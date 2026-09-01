# 🐛️ Browser boot chain — the six defects between "no plugins loaded" and a rendering 2D board

Traced live in the React OS dev server (`SEMIO_RENDERER=react`, port 6012) on 2026-08-30. Each defect
only became visible once the previous one was fixed, so they are listed in the order they surfaced.

## 1. Stale `VITE_SEMIO_APP_ID` in an already-running Vite

`🧑️‍💻️dev/🟦️component.ts` reads `import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId`. The env var is
baked when `runViteBunxDev` spawns Vite, from `🔣️playgrounds.json`'s `app` column. The catalog had
already been regenerated with the new `s.puzzle.puzzle2d@1/*#editor` id, but the *running* server still
carried the pre-regeneration `s.puzzle2d@1/*#editor`, so `ShellHost` rejected the pinned app.

Not a code defect — but note that **editing any file inside the Vite config's import graph
(`🧵️shard-client.ts`, `🌐plugin-web-materialize.ts`, …) triggers a config restart that reliably wedges
in this repo**. Kill the chain and cold-start instead of waiting on the restart.

## 2. `PluginRuntime.createApp` bypassed the instance-lifecycle lease

`createApp` hand-built `{ kind: "instance-open", payload: { instance, appId, actor, config, assets,
capabilities, quotas } }` and pushed it through the plain `submitTurn` path. The guest's own wire
decoder requires `activation-generation` and `request-sequence` on that event, so every first turn died
with `actor-lifecycle.invalid-authority` before the guest ever ran.

The correct API already existed in the same module and was already covered by its tests
(`shardClient.captureInstanceLifecycle` → `submitPluginLifecycleTurn({ kind: "open" })` → `receipt-ack`);
only production `createApp` had never been migrated to it. Fixed by migrating it, tracking the lease per
instance, and clearing that map in `destroyApp`/`dispose`.

## 3. jco's `useDirectParams` rewrite had never run on the puzzle component

`semio_s_plugin_puzzle_component.js` carried `useDirectParams: true` on all seven `taskReturn` bindings.
The `poll` result exceeds the flat-return limit, so its discriminant was read from the return-area
*pointer* — `caseIdx: 15179872` against a two-case `result`, surfacing as
`undefined is not iterable`. `rewriteJcoAsyncResultLifting` (which exists precisely for this, and is
idempotent) flips exactly one of the seven to `false`. Re-running it fixed the lift.

## 4. The generated bridge did not unwrap jco's `option` representation

WIT declares `lifecycle-receipt: option<receipt>`. jco's flat-lift path yields `{ tag: "none" }` /
`{ tag: "some", val }`, but `pluginComponentBridgeSource`'s `lifecycleReceipt`/`uiPatchReceipt` readers
expected the bare value or `undefined`, so a legitimately absent receipt raised
`actor-lifecycle.receipt-required`. Added `unwrapOption` to the generator and applied it to
`lifecycleReceipt`, `uiPatchReceipt` and `nextWake`.

## 5. `patchAckEvents` dropped the UI-patch receipt

`patchAckEvents` emitted `{ surface, revision }` with no `receipt`, so the bridge's patch-ack branch
dereferenced `payload.receipt.lifetime` on `undefined`. The ack is only meaningful for the turn whose
`uiPatchReceipt` produced the patches, so the function now takes that turn and decodes its receipt.

## 6. `resolveActionArgDef` assumed every string arg carries `options`

The Rust wire type declares `options` as `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so
a free-text string argument legitimately serialises with no `options` field at all — 91 of them in the
puzzle descriptor. `ShellHelpers` called `def.schema.options.map(...)` unconditionally and took the whole
shell render down. Reads through `actionArgStringOptions` now.

## 7. Two puzzle-2D-specific id/classification defects

- `🌉️wasm/🟦️component.ts` still registered the board session factory under `s.puzzle2d@1/*#editor` /
  `…#viewer`, so `Board2dHost` threw *"The current app has no registered board session factory."*
- All 38 `action_interactive_job` rows in the 2D editor were `BatchOnlyPendingRewrite`, and **only
  `Migrated` is UI-dispatchable**. Every interactive action — `setActiveExample`, the whole `brush*`
  family, `setFillCount`, `deleteSelection`, `setCamera` — was rejected at dispatch with
  `interactive-job.not-ui-safe`. 2D was simply never migrated: 5D already marks the same class of
  interactive editor actions (`deleteSelection`, `duplicateSelection`, `setSelectionFlag`, the pointer
  actions) as `Migrated`. All 38 flipped to `Migrated`.

The classification is compiled into the guest, and `🔣️descriptor.json` is extracted by running the
transpiled component under node — so this one needs a real wasm rebuild to take effect.

## Tooling note

`grep` silently returns no matches on this repo's emoji-named sources (it treats them as binary).
Every content search in this investigation had to go through `python3`. `grep -c` reporting nothing
where the string is present is the tell.

## 8. The wasm-pkg dev stub swallowed a pkg that was present

`playgroundFlowWasmDevStubPlugin` substitutes a `wasmMissing` stub for any `/pkg/` import it cannot
find on disk. When the **browser requests the module directly**, Vite hands `resolveId` its own
`/@fs/<absolute path>` URL rather than the specifier the importer wrote — and
`resolve(repoRoot, "/@fs/…")` is that same non-existent path, so the check failed and the stub won even
though `pkg/semio_puzzle.js` was sitting right there.

Symptom: `BoardSession.prototype.free` was `undefined` in the browser (the stub's `BoardSession` only
has `lodScaleJson`), so `Board2dHost` died with `session?.free is not a function` and never painted.
Fixed by stripping the `/@fs` prefix before the existence checks. With the real 157 KB wasm-bindgen
module served, all three board panes (Overview / Detail / Selection) construct their WebGPU sessions
and paint.

## Verified in the browser so far

- Shell chrome, example selector, panels, and the three board panes render.
- Board WebGPU sessions construct and paint their grid.
- Remaining: every editor action is still refused with
  `interactive-job.not-ui-safe … BatchOnlyPendingRewrite` until the guest is rebuilt with the
  `Migrated` classifications from §7.

# Lane K2 — merge-policy UI control wiring (report)

## Headline

**Genuinely wired now, for the direct-call path.** `PluginWasmHandle.setMergePolicy` /
`.resolveConflict` / `.readConflicts` are real, non-optional members of the adapted handle
(`adaptPluginHandle`, `PluginRuntime/🟦️component.tsx`), each riding a real `AppChannelClient` call
and decoding the real `AppFrame::MergeReport`/`AppFrame::Conflicts` reply. `ShellHost`'s
`dispatchSetMergePolicy`/`dispatchResolveConflict` call these directly (no `?.`), and a `readConflicts`
effect seeds the Conflicts panel with the guest's real roster on session start. **Still not wired**:
the async/unsolicited delivery path (a remote peer's merge pushing `MergeReport`/`Conflicts` frames
through `💻️os/🟦️backbone-worker.ts`'s relay) — that file never reads those frames at all (only
`AppFrame::Error` → a bare `"conflict"` event), and it is outside this lane's file list. See "Known
gap" below.

## Lease note (read first)

The task's stated lease listed `🎠️kernel/🟦️component.ts` as where `PluginWasmHandle`/`adaptPluginHandle`
live. In the live tree they do not — `kernel/🟦️component.ts:101` has an unrelated, narrower
`PluginWasmHandle` (5-function worker-ABI type, used by `loadPluginModuleViaWorker`); the actual
`adaptPluginHandle`/wide `PluginWasmHandle` the shell consumes live in
`PluginRuntime/🟦️component.tsx`, which is **not** in the stated `{ShellHost,ChromePanels,Shell,ShellSync}`
list. This file was already mid-flight with an uncommitted, unrelated diff from ticket
`26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` (an `ActorIdentity` region,
`git diff` confirmed before touching anything). Per CLAUDE.md's "work simultaneously with others on
the same files" and the fact that this file is the *only* place the fix can genuinely land, I edited
it region-locally (`//#region 🔖️Merge` in both the type and `adaptPluginHandle`'s body), re-read
before editing, and left the other lane's `ActorIdentity` region untouched — confirmed via
`git diff` after (their hunk is intact, only my two new hunks were added).

## What changed

- **`PluginRuntime/🟦️component.tsx`**: `PluginWasmHandle` type gains `setMergePolicy`,
  `resolveConflict`, `readConflicts` (required, not optional — a future regression removing the
  implementation is now a compile error, not a silent no-op). `adaptPluginHandle`'s returned object
  implements all three: `setMergePolicy` throws on an `AppFrame::Error` reply;
  `resolveConflict` throws on Error, else decodes both the `MergeReport` and (when present)
  `Conflicts` reply frames via `decodeMergeReportFromWire`/`decodeConflictsFromWire`; `readConflicts`
  decodes the `Conflicts` reply. Same shape/error-handling idiom as the file's existing
  `applyMutations`/`loadAppDocumentPack` (throw on Error, no swallowing).
- **`ShellHost/🟦️component.tsx`**:
  - `PendingAppChannelMethods` no longer carries `setMergePolicy?`/`resolveConflict?` — those two
    fields are deleted (the genuinely-wired methods don't need feature-detection). `openArtifact?`/
    `setDefaultApp?`/`clearDefaultApp?` are untouched (still out of scope for K2, still wire-dead —
    same gap lane 2-D flagged, not this ticket's ask).
  - New `pluginHandleFor(pluginId)` returns the real `PluginWasmHandle` (no cast).
  - `dispatchSetMergePolicy`/`dispatchResolveConflict` rewritten: no `?.` on the three methods;
    each now passes `session.instanceId` (the real per-instance channel the methods require — the
    old stub signatures never took one, which was itself part of why they could never have worked).
    `dispatchResolveConflict` now dispatches `SET_CONFLICTS` from the real `Conflicts` frame the
    reply carries, instead of relying solely on the (still-unwired) backbone bridge.
  - New effect: on session start/switch, calls `readConflicts` and dispatches `SET_CONFLICTS`, so the
    Conflicts panel shows the guest's real open-conflict roster immediately rather than staying
    empty until some other event fires.
  - Removed now-unused `mergePolicyAsU8`/`conflictResolutionAsU8` imports (the u8 conversion now
    happens inside `AppChannelClient.setMergePolicy`/`.resolveConflict` themselves, one layer down).
- **`🟦️component.ts` (os product, `AppChannelClient`) / `🎠️kernel/🟦️component.ts`**: no changes —
  both already fully implemented `setMergePolicy`/`resolveConflict`/`readConflicts` and their
  types/codecs (landed by an earlier lane; confirmed by reading, not assumed).
- **`ChromePanels`/`Shell`/`ShellSync`**: no changes needed — `ChromePanels`' Select/Accept/Discard
  already called through `SettingsHostApi.setMergePolicy`/`ConflictsHostApi.onResolve` (real required
  props from `ShellHost`, not the broken seam), and `Shell`'s `mergeReducer`/selectors already existed
  and needed no changes.

## `?.` removal

Removed at both call sites named in the task (`setMergePolicy`, `resolveConflict` in `ShellHost`).
`openArtifact` at line ~4136 (`ShellHost`) keeps its `?.` deliberately — it is a different, still-
genuinely-unwired method (`PluginWasmHandle` has no `openArtifact` member at all), out of this
lane's three-method scope.

## Known gap (explicitly out of this lease)

`💻️os/🟦️backbone-worker.ts`'s `emitEvent(..., {kind:"conflict", message: frame.Error.message})`
(line ~520) only ever fires on `AppFrame::Error`, and never inspects `AppFrame::MergeReport`/
`AppFrame::Conflicts` at all — so a REMOTE peer's merge that produces a real conflict still never
reaches `ShellHost`'s `event.kind === "conflict"` duck-typed bridge. That bridge (and `ArtifactEvent`'s
`"conflict"` shape) is defined in `💻️os/🟦️component.ts` (in this lease) but the worker file that would
populate it is not. Direct local calls (Settings toggle, Accept/Discard) are fully live end-to-end;
async delivery of a remote peer's conflict is not. This is the same gap lane 2-D's
`📓️w2-d-report.md` and J2's `📓️j2-runtime-proof.md` already named — narrowed here to its exact
remaining location, not closed.

## Verify (real numbers)

- `@semio-tech/framework-renderer-react` (`bunx vitest run --config 🧪️vitest.config.ts` from
  `📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`, direct invocation per lane
  2-D's documented nx-quick-budget workaround): before my change **306 passed, 9 failed, 315 total**
  (`🧪️k2-vitest-before.txt`); after, **307 passed, 9 failed, 316 total** (`🧪️k2-vitest-after.txt`) —
  same 9 pre-existing failures verbatim (adaptPluginHandle wire-encoding test-fixture issue, form
  builder cards, VFS scene, `resolveWindowActions`, `commandCategories` label drift, 2 mit-bestand
  asset-path regexes), plus the 1 new passing test this lane added. Isolated run of the new test:
  `bunx vitest run --config 🧪️vitest.config.ts -t "adaptPluginHandle exposes setMergePolicy"` → 1
  passed.
- `@semio-tech/framework-os` (`bunx vitest run --config 🧪️vitest.config.ts` from
  `💻️os/📦️packages/🟦️typescript`, untouched by this lane, run to confirm `AppChannelClient`'s
  existing `setMergePolicy`/`resolveConflict`/`readConflicts` coverage still passes): **334 passed, 2
  failed** — the 2 failures are a pre-existing missing wasm-pkg build artifact
  (`Cannot find module .../pkg/semio_framework_os.js`), unrelated to this change (`🧪️k2-os-vitest.txt`).
- `@semio-tech/ui-react` (untouched by this lane, checked per the brief's "don't regress" ask):
  **515 passed, 10 failed, 525 total** (`🧪️k2-ui-react-vitest.txt`) — same 10 failure names lane 2-D
  already catalogued as unrelated (`UnifiedGumball`, icon hover animation, `CanvasPickMenu`, navbar
  spacing, `VirtualFileSystem`/tree icons); the 505→510→515 passed / 515→520→525 total drift across
  lane 2-D → now is from other concurrent lanes' commits to the shared bundle file, not this lane.
- Root `bunx tsc --noEmit -p tsconfig.json`: **19 errors**, byte-identical file:line set to lane
  2-D's baseline (`🔱️trinity/…/🧠️lsp`, `🗄️stdio/…/🧬️schema` ×2, the vscode extension) — zero in any
  file this lane touched, before and after (`🧪️k2-tsc.txt`).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
  (outside the stated lease — see "Lease note" above)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
  (new coverage)

Logs: `🧪️k2-vitest-before.txt`, `🧪️k2-vitest-after.txt`, `🧪️k2-os-vitest.txt`,
`🧪️k2-ui-react-vitest.txt`, `🧪️k2-tsc.txt` (this folder).

# 📓️ terra — ts-hub report

Packet: `ts-hub`. Owned paths: `🌎️hub/📦️packages/🟦️typescript/**`, `🌎️hub/🔨️modules/**`. Scope: verify the
two hub TypeScript packages (unmeasured this session), repair genuine breakage, report named/unique test
sets, and census — without fixing — `🌎️hub/🔨️modules/🛡️admin/…/📤️dist/` for banned legacy symbols.

## 1. Named test sets (fresh, `--reporter=verbose`, this session)

### `os-hub-ts` (`🌎️hub/📦️packages/🟦️typescript`)
```
$ bun nx run os-hub-ts:test -- --reporter=verbose
↓ |os-hub-ts| 🧪️index.test.ts > boots the real hub and proves directory + presence-per-surface +
  document-scoped commands + admin kick + restart persistence
Test Files  1 skipped (1)
     Tests  1 skipped (1)
```
**exit=0.** The single test is intentionally gated behind `HUB_E2E=1` (documented in the file's own
header) — default `test` never touches cargo. `include: ["🧪️index.test.ts"]` names the one test file
that actually exists in the directory (verified: no other `*.test.ts` file present), so this is not an
instance of the filename-list trap; no `includeSource`, so no double-count risk either.

**Also ran for real** with `HUB_E2E=1` + `CARGO_TARGET_DIR=<scratchpad>/target-tshub` (foreground,
Monitor-watched to survive the 120s auto-background): the harness's own `buildHubBinary` step failed
building the real `os-hub` binary —
```
error: could not compile `semio-framework-os-kernel-db` (lib) due to 280 previous errors; 38 warnings
cargo build --manifest-path Cargo.toml exited with status 101
EXIT_CODE=1
```
Full output: `terra-ts-hub-e2e-run.txt` in this folder. This is the **same crate, same known-broken
state** other packets on this ticket have already reported (`semio-framework-os-kernel-db` red, unrelated
mid-flight Rust work) — not a ts-hub regression, and outside this packet's writable paths. The default
(non-`HUB_E2E`) gate, which is what `os-hub-ts:test` actually runs day-to-day, is unaffected and green.

### `@semio-tech/hub-admin` (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript`)
```
$ bun nx run os-hub-admin:test -- --reporter=verbose
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > has an identical key set in en and de
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > covers every admin.* namespace the app renders
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > substitutes {placeholder} vars
✓ 🧪️admin.test.tsx > SpacesPage > renders rows from a mocked admin client
✓ 🧪️admin.test.tsx > ConnectionsPage > updates live on a pushed connection message
Test Files  2 passed (2)
     Tests  5 passed (5)
```
**exit=0, unique count = 5** (2 test files: the in-source `📚️I18n` suite + `🧪️admin.test.tsx`).
Checked for both vitest traps: `include: ["🧪️admin.test.tsx"]` and `includeSource:
["../../🧱️elements/📚️I18n/🟦️component.tsx"]` name **different** files (no double-count), and both are
the only test-bearing files that exist under this package/its elements tree (verified: `grep -rl
"import.meta.vitest"` over all 8 `🧱️elements/**/🟦️component.tsx` finds only `📚️I18n`). Re-ran a second
time after my tsconfig edit (§2) to confirm no regression — identical 5/5.

Baselines re-verified twice, both runs identical — deterministic.

## 2. Repaired breakage (in-scope, owned files only)

`bunx tsc --noEmit` had never been run for either package this session (no `typecheck` nx target wires
it — vitest is the only gate that actually runs). I ran it standalone as an extra check and found:

- **`📜️script.ts` in both packages**: `error TS2339: Property 'dir' does not exist on type 'ImportMeta'`
  — both scripts use Bun's `import.meta.dir`, which isn't in `types: ["node"]` and no `bun-types` package
  is installed anywhere in the repo. The established repo convention (seen in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json`) is to
  exclude `📜️script.ts` from the tsc `include` set rather than install `bun-types`. **Fixed**: added
  `"📜️script.ts"` to `exclude` in both `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` and
  `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/tsconfig.json`. Re-verified: the `ImportMeta.dir`
  error is gone from both packages' `tsc --noEmit` output; `bun nx run os-hub-ts:test` and `bun nx run
  os-hub-admin:test` re-run clean afterward (§1), confirming vitest is unaffected by the tsconfig change.
  admin package's own files (`admin.test.tsx`, all 8 `🧱️elements/**/🟦️component.tsx`) now show **zero**
  tsc errors of their own.

## 3. Genuine breakage found, NOT fixed — root cause is outside owned paths

`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts` still shows **9 tsc errors**, all one root cause:
```
error TS2305: Module '"@semio-tech/framework-os"' has no exported member 'ArtifactPresencePeer'
error TS2305: … has no exported member 'ClientFrame'
error TS2305: … has no exported member 'ServerFrame'
error TS2305: … has no exported member 'decodePresencePeer'
error TS2305: … has no exported member 'decodeServerFrame'
error TS2305: … has no exported member 'encodeClientFrame'
error TS2305: … has no exported member 'encodePresencePeer'
error TS7006: Parameter 'raw' implicitly has an 'any' type.     (line 175, downstream of the above)
error TS7006: Parameter 'stage' implicitly has an 'any' type.   (line 289, downstream of the above)
```
Traced (not assumed): `ArtifactPresencePeer`/`ClientFrame`/`ServerFrame`/`decodePresencePeer`/
`decodeServerFrame`/`encodeClientFrame`/`encodePresencePeer` are real, exported from
`🧰️framework/🔨️modules/📡️replication/🟦️component.ts` (verified with `grep -n "^export"`), but
`🧰️framework/🛍️products/💻️os/🟦️component.ts` only **imports them as internal types** from
`@semio-tech/framework-replication` — it never re-exports them. `index.test.ts`'s own doc-comment (line
57) says it uses `@semio-tech/framework-os`'s `encodeClientFrame`/`decodeServerFrame` "never
hand-rolled", i.e. the **intent** was always that the OS glue barrel re-exports these; either it never
did, or a sibling's in-flight async/dedyn work dropped the re-export (`git diff HEAD` on
`🟦️component.ts` shows no working-tree delta, so if this is a regression it's already committed, not
mid-edit by a live sibling).

This is currently **latent**, not live: the only consumer, `🧪️index.test.ts`, only runs under
`HUB_E2E=1`, which is itself blocked by the unrelated `semio-framework-os-kernel-db` breakage (§1). But
it is a real defect that will surface the moment either blocker clears. The two implicit-`any` errors are
not independent bugs — they're `ServerFrame`/`ClientFrame` collapsing to `any` because the import already
failed; fixing the root cause should clear all 9 at once, so I did not hand-annotate `raw`/`stage`
locally (that would paper over the real type flow instead of restoring it).

**Not fixed**: the fix belongs in `🧰️framework/🛍️products/💻️os/🟦️component.ts` (re-export the 7 names,
or `export type {…}`/`export {…} from "@semio-tech/framework-replication"` at the glue layer), which is
outside `ts-hub`'s owned paths (`🌎️hub/**` only).

**Lease-request**: owner of `🧰️framework/🛍️products/💻️os/🟦️component.ts` — re-export
`ArtifactPresencePeer`, `ClientFrame`, `ServerFrame`, `decodePresencePeer`, `decodeServerFrame`,
`encodeClientFrame`, `encodePresencePeer` (currently importable only from `@semio-tech/framework-replication`)
so `@semio-tech/framework-os` actually provides what its own consumers document it as providing.

## 4. `🌎️hub/🔨️modules/🛡️admin/…/📤️dist/` census — classification only, NOT fixed

Per instruction: reporting presence/absence, not repairing (dist is a generated artifact — regenerate via
`bun nx run os-hub-admin:build`, never hand-edit).

- **Staleness**: every file under `📤️dist/` is dated **2026-08-17 19:31** — i.e. from at or before this
  ticket's own start, predating essentially all of this session's source changes. It reflects a
  pre-ticket (or very-early-ticket) build, not current source.
- **Banned-symbol scan** (full list from `important.md`'s "Replace, never wrap" table, `grep -rl` over
  every file in `📤️dist/`): `PluginWorkerClient`, `PluginModuleLease`, `WasmPluginRuntime`,
  `ExtensionRuntime`, `ProgramSupervisorState`, `PLUGIN_FUEL_BUDGET`, `PLUGIN_WORKER_UNRESPONSIVE_MS`,
  `INSTANCE_GUARD`, `host_port`, `install_io_fallback_dispatcher`, `set_host_backbone_channel`,
  `runSerialized`, `loadPluginModuleUncached` — **zero hits, all of them**.
- **`LeasePool` / `createLeasePool`**: 2 hits, both in the same bundle
  (`assets/🌐️index-BFUULK5r.js`) and both **benign**, verified by reading their actual context (not
  assumed from the grep alone):
  1. `createLeasePool:Wf` in a minified export-map — this is the **generic, relocated**
     `createLeasePool` that `important.md` explicitly sanctions surviving in
     `📦️packages/🟦️typescript/🟦️glue.ts` for its 3 non-plugin users. Not the banned kernel type.
  2. The literal string `"LeasePool evictNow (hot-swap reload eviction)"` — a bundled **vitest test
     description string** for that same generic `LeasePool`'s own test suite, incidentally pulled into
     the production bundle by the bundler; not a reference to the banned kernel `LeasePool`/
     `PluginModuleLease`.
  Neither is the banned kernel-owned `LeasePool`/`PluginModuleLease` this ticket targets for removal.
- **`exchange`**: not scanned as a bare word (too common in English UI copy to grep meaningfully); no
  compound identifier matching the WIT/plugin-exchange shape (`exchange::`, `ExchangeHandle`, etc.) turned
  up in the same pass that caught every other banned symbol at 0.

**Conclusion for the exit-gate question**: as far as *banned legacy symbols* go, this dist bundle is
already clean — a rebuild is not required to clear a banned-symbol finding here. It **is** stale relative
to current source, so if the gate cares about the bundle reflecting current source generally (not just
being free of banned symbols), a rebuild (`bun nx run os-hub-admin:build`) is still recommended before
treating this bundle as representative. I did not run that build myself (out of scope for "census, not
fix"; also a `--all-features`-shaped build call belongs to the coordinator per the ticket's build-ownership
rule).

## 5. Files touched

- `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` — added `📜️script.ts` to `exclude`.
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/tsconfig.json` — added `📜️script.ts` to `exclude`.
- Ticket folder additions: this report, `terra-ts-hub-e2e-run.txt` (full HUB_E2E build+test log).

No other production file touched. No `🌎️hub/🔨️modules/🛡️admin` element/component file needed a change —
their own tsc surface is already clean.

## 6. What the coordinator or a sibling should know

- Both hub TS packages' actual gates (`bun nx run os-hub-ts:test`, `bun nx run os-hub-admin:test`) are
  **green**, unique counts 1-skipped-by-design and 5/5 respectively — first baseline recorded for either
  this session.
- The `HUB_E2E=1` path is blocked purely by `semio-framework-os-kernel-db` (280 errors), already known to
  this ticket from other packets — not new, not mine to fix, not a ts-hub defect.
- New finding: `@semio-tech/framework-os` doesn't re-export 7 names its own consumer's doc-comment says it
  does (§3) — lease-request filed above. Latent today only because two unrelated things already gate the
  one file that imports them.
- Admin `📤️dist/` is clean of every banned legacy symbol (§4) but stale (pre-dates this session's source
  changes); rebuild recommended for freshness, not required for the banned-symbol gate.

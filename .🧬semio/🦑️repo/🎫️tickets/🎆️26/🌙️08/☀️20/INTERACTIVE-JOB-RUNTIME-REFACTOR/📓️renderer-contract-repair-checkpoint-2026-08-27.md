# Renderer Contract Repair Checkpoint

## Verified Gates

- Coordinator baseline typecheck: 385 diagnostics, captured in coordinator r2 output. The earlier 112 count was a truncated-stream mistake, not a passing gate.
- Executor typecheck r5: 163 diagnostics; r7: 133 diagnostics. Full captured files are retained. r6 typecheck did not finish with a retained capture and has no claimed count.
- Executor full React r5: 470 passed, one failed. The old tutorial command fixture still expected removed `ActionArgDef.control`; it now asserts the actual stored string schema and options.
- Executor full React r6: invalid run, two suites failed while opening existing dependency files with EINTR. Subsequent ordinary source reads also reported EINTR. No pass credit.
- Executor full React r7: 473 passed, zero failed, four files. Coordinator independently repeated 473/473 in its full r3 log, 21.27 seconds. This does not validate Wasm runtime or the unfinished parameter backends.
- Executor full React Presence regression: 474 passed, zero failed, four files before the dead Effect removal. The earlier targeted attempt selected zero tests and has no test credit.
- Fresh executor typecheck r9: 15 diagnostics, full retained capture `🧪️renderer-contract-typecheck-r9-2026-08-27.txt`. Historical r8 reported 47 but its capture is currently absent; use the fresh r9 capture for independently inspectable evidence. The remaining diagnostics are tutorial capture/restore (7), tutorial document-track naming (3), and Wasm factory declarations (5).

## Contract Repairs

### Latest Captured Checkpoint

- Typecheck r10: 9 diagnostics; r11: 8; r12: 8; r13 and r14: **7 diagnostics**, all in the unfinished tutorial full-local interaction producer/consumer join. Full captures retain every diagnostic. No exclusion or suppression was used.
- Tutorial document-track regression: red 474 passed / 1 failed, then green 475/475. The TypeScript mirror, slice, recorder and fixtures now use native `document`, not the removed `artifact` track key. The isolated native serde fixture is source-ready, unrun.
- Raster interaction regression: red 475 passed / 1 failed with one uncaught error, then green 476/476. The actual combined `syncInteraction` method replaces nonexistent split setters. Coordinator independently repeated 476/476.
- Board factory scope: initial red 476 passed / 1 failed; the first implementation full run then exposed an undefined shell prop binding (476 passed / 1 failed), which was fixed.
- Board lifecycle: red 479 passed / 1 failed reproduced freeing during pending attachment; green 480/480. The session remains owned until attachment settles, with one exact final release.
- Board isolation/retry: **484/484**, four files, 5.21 seconds total and 1.63 seconds execution, in `🧪️board-session-isolation-r1-2026-08-27.txt`. These additional laws were added after the initial scope implementation, so no earlier red is claimed for them. Actual DOM mounts cover two scopes with identical IDs, stale old attachment rejection after remount, and exact peer/gesture cleanup. Two module-loader laws cover concurrent deduplication and retry after network/initialization failures.
- Editor's remaining module assertion is replaced by the actual generated module type. Typecheck r15 again reports exactly **7 tutorial-only diagnostics**, with no loader errors.
- Final post-Editor renderer run: **484/484**, four files, 4.63 seconds total / 1.59 seconds execution, in `🧪️board-session-final-renderer-2026-08-27.txt`.
- Linked composition engine metadata: initial two attempts failed because the new test used the wrong fixture root (no semantic red credit); corrected red r3 fails on the absent parser and missing metadata. Green r2 passes **2 tests, 50 filtered**, with 3 valid and 6 hostile schema vectors, strict Ajv parity and TypeScript AST verification of all three actual composition entry imports. Full captures: `🧪️linked-session-engines-{red-r3,green-r2}-2026-08-27.txt`.
- Targeted `git diff --check` passed for the renderer/Board/dev/Puzzle/demonstrator/launch packet. No Rust compiler was run by this executor.

### Board Ownership and Build Boundary

`WasmSessionLoader` now constructs Graph/Raster/Map/Terrain through their actual generated module exports. Puzzle supplies its Board constructor at product composition, with exact plugin/app/u32-instance identity. The shell memoizes an instance-owned scope; no process-global Board peer or gesture map remains. Unregister/gesture release checks the exact registered peer object, so delayed old failures cannot remove a successor. Shared surface, Editor and Puzzle initialization use exact failed-attempt cache retirement rather than permanently caching rejection.

The Puzzle Rust script owns a real `wasm` target. Cargo playground metadata points to that crate. Product entrypoints (dev, multi-shell and demonstrator) register the Puzzle-owned factory. Registry generation passed via `NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_CACHE_PROJECT_GRAPH=false bun x nx run @semio-tech/plugin-registry:generate --skip-nx-cache`, refreshing 59 plugin crates, 60 playgrounds, 38 framework packages and generated launch entries. The normal isolated Nx plugin route returned no registry targets despite the native glob and direct callback seeing the configuration; disabling plugin isolation exposed the correct tasks. No graph metadata was fabricated or global cache deleted.

Actual Puzzle `.js`, `.d.ts` and `.wasm` artifacts are absent. **No actual Board Wasm runtime or product-composition typecheck is claimed.** Coordinator owns the pending `@semio-tech/puzzle-plugin:wasm` compiler gate. Dev and demonstrator package metadata now declare `semio.browserSessionFactories` module/engine pairs; the generic engine builder unions these with the selected playground's engines (all playgrounds for an all-app host). Demonstrator passes its own composition manifest explicitly. No Puzzle-always-build conditional was put into the generic builder. Runtime construction remains lazy. Internal workspace dependency edges were declared; no new external dependency was introduced.

Canonical commands for coordinator reproduction:

```sh
NX_DAEMON=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
NX_DAEMON=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache
NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-dev:test --skip-nx-cache --args='-t linkedSessionEngines'
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_CACHE_PROJECT_GRAPH=false bun x nx run @semio-tech/puzzle-plugin:wasm --skip-nx-cache
```

The last command is **unrun**. The preceding three have the exact scoped results above. Dag confirms the local-interaction schema stage is source-ready but runtime read/restore registration is not ready; the seven tutorial consumers must not be replaced with fake Presence/query fields.

Changed owner groups in this packet: renderer WasmSessionLoader, Board2dHost, Paint2dHost, Shell/ShellHost and facade/tests; manifest tutorial mirror/native fixture; UI tutorial slice; Puzzle app-owned browser factory/barrel, Rust build script/project/Cargo metadata; dev/multi-shell/demonstrator entrypoints; launch seed/generated launch; existing repo discovery compiler-import boundary and tests. All temporary captures remain in this ticket.

Remaining Board latency obligations are unchanged: fixture synchronization and native scene generation remain whole-operation paths; Board's render lifecycle is not a retained bounded rendering implementation. The DOM tests use typed sessions and validate host ownership, not GPU/Wasm behavior.

Current patches use the existing schema vocabulary for action argument storage, explicit resolved action/tool/argument labels, bilingual manifest-label resolution, current menu references, control IDs and icons, actual Three constructors, numeric projection variants, and native event types. The action-semantics fixture covers six action kinds against the existing native defaults; interactive jobs remain unclassified. Its strict Ajv test runs in the React suite; the native parity test is source-ready but unrun.

The authoritative schema metadata now includes the native execution `interactiveJob` field and exact nested staged-argument maps. Empty separator props use an empty-key record, avoiding the impossible intersection of `Record<string, never>` with the component discriminator. No native runtime behavior was changed for these metadata corrections.

## Remaining Cross-Owner Seams

- Tutorial capture/restore cannot use broadcast-only Presence or invented `selectionJson`. Dag owns the captured full-local interaction producer, including mode, granularity, selection anchors and exact revision authority. Consumer work waits for the coherent schema.
- `PatchWorld3dChrome` had no application producer. Its Effect/WIT record and variant, host method, all native transport mappings, TypeScript union, obsolete renderer consumer and root capability row are now removed cohesively. Live Presence has a strict four-vector schema, real DOM replacement regression and native serde fixture test (native unrun). Fresh Wasm rebuilding and the post-removal suite remain required.
- The shared surface Wasm module has Graph, Raster, Map and Terrain sessions, but no BoardSession. BoardSession remains puzzle-owned and no built/linked Puzzle wasm-bindgen package is currently present. Coordinator chose an app-owned registered factory and actual product build routing; no fake shared declaration or permanent undefined stub is acceptable.
- Dag's authoritative schema-generated Flow browser declarations are adopted directly; the handwritten consumer inventory and module casts are removed. Actual task results are `unknown`, and canvas operations replace the incorrectly declared `renderFrame` surface.
- Repository discovery's five transitive diagnostics are cleared in r9. The actual mutable map parameter now accepts owned `Map` values; compiler imports cross a minimal owned runtime-validated Bun capability boundary. No dependencies, ambient constructor declarations, `any`, typecheck exclusions or unrelated taxonomy edits were introduced.

## Bun Compiler Import Boundary

Canonical `@semio-tech/repo-lib:test --args=--test-name-pattern=registryCompilerImports` ran red (0 passed, 2 failed), then green (2 passed, 0 failed, 290 filtered, 21 assertions). Five language-neutral vectors compare the unchanged Bun parser with the existing third-party TypeScript AST parser. Eight malformed platform capabilities are rejected, and returned records cannot be rebound through a platform-owned object. Strict Ajv rejects extra fixture fields. The initial oracle incorrectly counted type-only imports; the fixture was corrected to Bun's existing runtime-dependency semantics before green. The redundant pre-existing `getWorkspaceRoot` import in the test entry was removed. Both complete run logs remain in this ticket.

## Scope and Credit

This checkpoint repairs runtime contracts and tests; it is not evidence of bounded large-document interactivity. Whole-map patching, hashing, validation and notification remain the next renderer packet. P2/P3 retained parameter factories, allocation ownership, targeted undo and runtime latest-wins publication remain unfinished.

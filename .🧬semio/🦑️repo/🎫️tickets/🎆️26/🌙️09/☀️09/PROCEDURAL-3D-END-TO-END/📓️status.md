# Procedural 3d End To End — status

Coordinator: Fable 5.1 (session ⚪9f5f6952). Started 2026-09-09 ~14:30 at HEAD 9b605a4550.
Repo MCP timed out at session start; ticket bookkeeping is done on disk.

## Waves

- **W0 audits (Sonnet, read-only, parallel ×6)** — boot path, feature inventory, build chain, test harness, kernel+preview, peers+gates. Reports: `📓️*-audit-2026-09-09.md`, `📓️feature-inventory-2026-09-09.md`.
- **W1 build + boot (Opus)** — get the procedural plugin compiling natively + wasm32-wasip2 and the playground serving (react first, then wgpu wasm), with a private target dir.
- **W2 feature waves (Opus, parallel per lane)** — windows, examples + preview, hover/selection, editor actions, tests/oracles. Scoped after W0 reports land.
- **W3 performance** — profile the runtime, optimize architecture at the limits.
- **W4 gates + runtime verification + close.**

## Log

- 14:30 ticket opened on disk; W0 dispatched.
- 14:18 boot lane started: `serve-generation3d-react-dev` (Nx-inferred from `[[package.metadata.semio.playground]] variant="generation3d"`, react port 6018) under nohup with private `CARGO_TARGET_DIR=$S/target-boot` seeded by rsync from `target/wasm32-wasip2/wasm-dev` (5.6 GB, ~3 MB/s under load 80 / swap 45 of 46 GB). Log `$S/boot-react.log`.
- 14:20 W0 partial findings (editor + examples + mutations, from inventory sub-audits):
  - All 14 mutations real (diff/inverse/7-fn tests/5 fixtures each). All 27 editor actions `Migrated`, `factory_type` set. No todo!/FIXME.
  - **Dead continuations**: `✅️flow-eval-resolve` and `🔺️flow-tessellate-resolve` are implemented but not in `Generation3dCommand` nor retained tool ids → `Effect::InvokeExtension{brep, evaluate|tessellate}` results have no dispatch path back. Prime suspect for "no 3d preview".
  - `context_menu` hardcodes empty selection (`✏️editor/🦀️.rs:1217`). `world-pointer-down`/`graph-pointer-down` are no-op stubs. `👥️set-contributions/` dir is empty. Viewer has no transient hover/selection; viewer command enum is `Noop` only.
  - Examples: only `.dsl.semio` assets are real; `.op/.spr/.pack.semio` are empty envelopes; example tests only assert asset non-empty (no geometry assertions).
  - Brep ops run in plugin `semio-s-plugin-flow-extension-brep` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep`) over kernel `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d`); generation3d itself has no kernel dep.
  - Assembly editor/viewer authored but not mounted (plugin root `🦀️.rs:91-98`).
- 14:21 boot lane restarted on the shared default target dir (rsync seed abandoned).
- 14:40 W0 landed: `📓️boot-path-audit-2026-09-09.md` (react via `serve-generation3d-react-dev`, 11-crate closure procedural+forms+9 flow-extensions, URL `?plugin=generation3d`), `📓️peers-and-gates-audit-2026-09-09.md` (launch.json generated from seed; gates: `verify dependencies literal-external`, `verify interactivity`, `verify taxonomy`, `verify rust-warnings`, `plugin-registry:generate/check`). Build-chain auditor lost its result (backgrounded monitor); its native `cargo check -p semio-s-plugin-procedural` keeps running in `$S/target-buildaudit`, log `$S/native-check-plugin.log`.
- 14:40 W1 dispatched (Opus ×3): boot-lane owner → `📓️wave1-boot-report-2026-09-09.md`; extension continuation wiring (flowEvalResolve/flowTessellateResolve) → `📓️extension-continuation-2026-09-09.md`; launch seed repair → `📓️launch-entries-2026-09-09.md`.
- 14:55 W0 complete except test-harness audit (pending): `📓️feature-inventory-2026-09-09.md` (IO codecs silently wrong for 7/9 formats; demo-session example unreachable; context menu empty selection), `📓️kernel-and-preview-audit-2026-09-09.md` (kernel = BREP subset of `semio-s-artifact-stdio-semio`; booleans have 15 failing tests hitting sphere-cut-with-torus + sphere-box-fuse; tessellation uncancellable; picking is client-side r3f raycast; no validate gate).
- 14:55 W2 dispatched (Opus ×3): boolean kernel fix → `📓️boolean-kernel-2026-09-09.md`; example geometry tests + oracle → `📓️example-geometry-tests-2026-09-09.md`; real IO codecs → `📓️io-codecs-2026-09-09.md`. Deferred until continuation agent lands (shared `✏️editor/🦀️.rs`): context menu selection, pointer-down stubs, perf/progress/cancel of tessellation.
- 14:53 W1 launch seed repaired → `📓️launch-entries-2026-09-09.md` (procedural 3d/2d dev rows env-fixed, hexagonal column rows rewired to VITE_SEMIO_DEFAULT_EXAMPLE, .claude/launch.json got procedural3d-react-attach + procedural3d-wgpu). Test-harness audit landed → `📓️test-harness-audit-2026-09-09.md` (TS examples 11/11 pass; python oracle lane blocked by unbuilt PyO3 host; no procedural3d E2E exists).
- 16:35 W1 continuation wiring landed → `📓️extension-continuation-2026-09-09.md`: **root cause of "no 3d preview" found** — (A) `Effect::InvokeExtension` with hand-minted `RequestId` never resolves (only registry-minted `req` via `Host::invoke_extension` is answered by `Event::Completed`), so every brep evaluate/tessellate result is discarded repo-wide (gen3d 104/105, gen2d 101, flow 106); (B) generation3d builds a throwaway `FlowEvalSession` per dispatch (no `ArtifactInstanceOperationOwner`). flowEvalResolve/flowTessellateResolve now first-class routes (29 rows), tests written but blocked from running by peer churn (`ArtifactCompositionFields`, `♻️retirement`, `set_sun`, `Generation3dViewCamera`).
- 16:35 boot lane attempt 1 failed at 15:29 on peer breakage (`semio-framework-os-kernel` OwnerRef derives — since repaired by peer at 15:13; plugin-host `ArtifactCompositionFields` bound churn); boot owner retrying.
- 16:36 W2b dispatched (Opus): SDK-level fix for (A) + retained owner for (B), migrating gen3d/gen2d/flow producers → `📓️extension-round-trip-2026-09-09.md`.
- 17:10 W2 IO codecs landed → `📓️io-codecs-2026-09-09.md`: real STL/OBJ/PLY/glTF/LAS/DWG export via stdio codecs, imports plant brep.io.import* neurons (LAS/PNG typed refusals), txt = full DSL round trip; 34/34 round-trip tests + python oracle lane green; gen3d crate checks with 0 errors/0 own warnings.

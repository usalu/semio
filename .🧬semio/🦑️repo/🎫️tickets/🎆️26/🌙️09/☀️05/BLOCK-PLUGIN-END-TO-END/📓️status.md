# 🧱️ Block plugin end to end — status

Apps under test: `block2d`, `block3d`, `block5d` of plugin `✏️s/🔌️plugins/🧱️block`
(`bun run dev:block:<2d|3d|5d>` → `bun ./📜️script.ts dev block <nd>` → framework-os-dev playground).
Ticket opened 2026-09-05 by session ⚪2adc84fa (Fable 5.1 coordinator). Repo MCP was down; bookkeeping is manual.
Ticket start commit: see `🗑️generated/start-commit.txt`.

## Definition of done
1. `cargo check -p semio-s-plugin-block` green natively and on `wasm32-wasip2`.
2. `bun nx run @semio-tech/block-plugin:test` green (lib test target compiles and passes).
3. Descriptor `🔣️.json` / `🛂️.descriptor.semio` regenerated via `describe`; registry check accepts block.
4. Each of the three playgrounds boots in the react renderer; every window renders non-empty; examples switch; editor actions dispatch (no `interactive-job.missing-owned-reducer` / bounded-factory faults).

## Log
- 2026-09-05 open: exploration fleet launched (2d/3d/5d editors, build+test infra, dev-boot path, TS side).
- block2d explored (`📓️explore-block2d-editor.md`): 9/9 actions `BatchOnlyPendingRewrite`, NO `bounded_first_step_tool_proofs!`/factory at all (→ `interactive-job.missing-factory`), boot snapshot is `Block2dSnapshot::default()` (empty), examples are `include_str!` DSL, `setActiveExample` handler complete but unreachable. All 604 `#[path]` mounts resolve. block5d has the full retained-factory pattern (`BLOCK5D_RETAINED_TOOL_IDS`, `Block5dRetainedCommandJobFactory`). Implementer W1 (block2d) launched.
- block5d explored (`📓️explore-block5d-editor.md`): dispatch healthy (factory_type set, 7/7 Migrated). Gaps: default boot empty (`Block5dSnapshot::default()`), orphan `app_5d_demo_session` module, io impls unregistered on `io_mechanism`, TS oracle path bug (W1 fixes).
- TS/io explored (`📓️explore-ts-viewer-io-schema.md`): 232 TS files are inert typed twins; io leaves `export {}`; Rust io: only JSON real, STL/OBJ/ZIP/PNG import silently return empty snapshots, TXT is an Err stub, io unregistered on the io mechanism in all subsets. W3 (io registration + txt via DSL + honest formats + TS json/txt twins) launched. Coordinator rewrote block-js `package.json` (was a verbatim cad-js copy; block TS imports no workspace packages).
- block3d explored (`📓️explore-block3d-editor.md`): 20 own actions all `Unclassified` (never `.action_interactive_job`), zero tool-proof/factory wiring, boot empty, `setCamera` unregistered, nakagin 1:500 representation references `/mesh/capsule_J.1to500.glb` which no catalog has (would throw `Unknown mesh asset`), orphan `demo-session` example. All 244 `#[path]` mounts resolve. W4 (block3d) launched.
- Infra explored (`📓️explore-build-test-infra.md`): 917/919 mounts resolve (2 are a generated-test-host quirk shared by cad/puzzle/procedural); block is already in the registry playground catalogs; `🔣️.json`/`🛂️.descriptor.semio` never generated at block's owner root (registry `check` warns); storybook scope `block` resolves but has zero stories; dev plugin-module wasm (2026-09-04 17:20) predates the latest block commit.
- Baseline native check (`🗑️generated/block-check-native-baseline.txt`, exit 101) never reached block: stdio's crate entry mounted `🪆️subsets/✳️base/🧬️schema/🦀️.rs`, which was renamed to `🧱️base` by its owner at 03:59 (after the check started) — a stale, queued-check artifact, not a live break.
- Dev-boot path explored (`📓️explore-dev-boot-path.md`): bare `dev block 2d` defaults to wgpu; react needs `SEMIO_RENDERER=react`; ports react 6024/6025/6026; engines (surface/editor/flow-core) all fresh on disk → `SKIP_ENGINE_BUILD=1` valid; block's load closure is {block, stdio}; neither has an owner-root `🔣️.json` (a full un-narrowed build materialises them); `SEMIO_PLUGIN_ONLY=block` unsafe cold. Registry `generate` gate probe: exit 0 (59 crates, 60 playgrounds). W5 (storybook block stories) launched.

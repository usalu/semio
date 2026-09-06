# 📓️ W-A — compile green: `semio-s-plugin-energy` (native + wasm32-wasip2)

Worker W-A. Crate `semio-s-plugin-energy` (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust`).
Private target dir `/Users/ueli/Documents/semio/target-energy-e2e`, `RUSTC_WRAPPER=`, `CARGO_BUILD_JOBS=4`.
Live logs: `/private/tmp/claude-501/-Users-ueli-Documents-semio/411af150-09ce-4f7f-a1b9-ac675c7c067d/scratchpad/w1-check-*.txt`.

## 1. Known-blocker resolution: the nonexistent `semio_framework::` path

`semio-framework` **does** exist as a crate (`🧰️framework/📦️packages/🦀️rust/Cargo.toml:2`), but
`semio-s-plugin-energy/Cargo.toml` does not depend on it, so `semio_framework::…` is unresolvable
from inside energy. The re-export energy already has in scope is `semio_framework_plugin`, whose
crate root does `pub use semio_framework::*;`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38888`) and whose `plugin_app_close_prelude`
re-exports `semio_framework::kernel::*` (`:38678`). `Effect` and `JobPlacement` are defined in
`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:359` / `:695`; `InteractiveJobClassification` is used through
`semio_framework_plugin::InteractiveJobClassification` by the compiling oracle puzzle
(`✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🦀️.rs:7356`).

Fix applied — no new dependency, no shim; the three sites now use the path energy's own crate entry
already uses (`semio_framework_plugin::kernel::…`):

| file | old | new |
|---|---|---|
| `…/✳️any/🧵️simulation-session/🦀️.rs:7` | `use semio_framework::kernel::{Effect, JobPlacement};` | `use semio_framework_plugin::kernel::{Effect, JobPlacement};` |
| `…/✳️any/✏️editor/🦀️.rs:259` | `-> Vec<semio_framework::kernel::Effect>` | `-> Vec<semio_framework_plugin::kernel::Effect>` |
| `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs:4` | `use semio_framework::InteractiveJobClassification;` | `use semio_framework_plugin::InteractiveJobClassification;` |

`grep -rn "semio_framework::" ✏️s/🔌️plugins/🔋️energy/ --include="*.rs"` now returns nothing.

## 2. Framework-API drift identified ahead of the compiler (oracle: `ArtifactEditor` trait + puzzle)

`ArtifactEditor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26623-26965`) is **sync**
except `command_from_intent`:
- `fn render(body_key, doc, cfg) -> UiAssemblyResult<ComponentTree>` (`:26878`) — energy had
  `async fn render(...) -> ComponentTree`.
- `fn pending_effects(doc, cfg) -> Vec<Effect>` (`:26875`) — energy had `async fn`.
- `fn initial_snapshot`, `fn handle`, `fn command_id` are all sync too — energy had `async fn`.
Puzzle confirms the shape: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:2089` is
`fn render(...) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>`.

## 3. Side item — `📦️packages/🟦️typescript/package.json`

Was a verbatim cad copy (`name: @semio-tech/energy-js` but a CAD description, `cad-js` nx scripts,
and 9 workspace deps). Energy's `🟦️.ts` re-exports only 11 relative artifact modules; a repo-wide
grep for non-relative TS imports under `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/` finds only `node:fs`,
`node:path`, `node:url` and `bun:test` — i.e. **zero package dependencies**. Rewritten to the
remodel shape: energy description, single `test` script pointing at `@semio-tech/energy-js:test`
(the only target in its `📋️project.json`), `"dependencies": {}`, `typescript` devDependency.

## 4. Command outputs

(appended below as runs complete)

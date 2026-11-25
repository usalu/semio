# Test crates in the flow production build (2026-11-24)

## Chain
`semio-hub-flow-component` (cdylib) -> `semio-hub-flow` -> `semio-s-artifact-flow-flow` (manifest `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`) -> `semio-s-artifact-flow-flow-{command-retirement-tests,unit-tests,unit-tests-2}` as `[dependencies]` -> those crates' own `[dependencies]` (`semio-s-artifact-flow-flow-editor` with `features=["testkit"]`, `unit_tests` with `features=["testkit"]`; the `testkit` feature of the unit-tests crate enables `semio-plugin/artifact-app-testing`, `semio-os-kernel/protocol-laws`, `semio-os-kernel/mutation-testing`).
Consumer check: the flow lib names none of the three crates; its test modules only include sources by `#[cfg(test)] #[path]`. `unit_tests` in the retirement-tests crate is used only in its `#[test]` fns (`🦀️.rs` line 141). `Cargo.lock` lists dev and normal dependencies identically and `📋️build-owners.json` has empty `packages` for these crates, so step 1 needs no lock or owner change.

## Step 1 (applied)
- flow artifact manifest: `semio-s-artifact-flow-flow-command-retirement-tests`, `-unit-tests`, `-unit-tests-2` moved from `[dependencies]` to the top of `[dev-dependencies]`.
- `…/✏️editor/🎮️commands/♻️retirement/🧪️tests/📦️packages/🦀️rust/Cargo.toml`: `unit_tests` (package `semio-s-artifact-flow-flow-unit-tests`, `features=["testkit"]`) moved to `[dev-dependencies]`.
Not compiled yet.

## The fourth leak: `semio-s-artifact-flow-flow-tests` is a production dependency
Path `…/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust`. Its lib root is wiring plus `🪞️mirror/🦀️.rs`, which mounts production files by `#[path]` and re-exports. Seven crates take it as `[dependencies]` and `pub use` from it (flow root `🦀️.rs`, `🚪️io/🦀️.rs`, mutations, viewer, editor, connect-media-ports mirror, retirement-tests mirror); `unit`, `unit-2`, `child-publication`, `retained` and the inferences-tests crate itself also list it.
Production items it holds (all small: 8 to 142 lines each), `A` = `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any`:
| item | source file under A | used by (non-test) |
|---|---|---|
| `io::text` | `🚪️io/📝️text/🦀️.rs` | flow `🚪️io`, mutations |
| `io::import::deserializers::artifacts::{txt,json}` | `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{🔤️txt/🔖️utf-8,🔣️json/🔖️rfc8259}/✳️any/🦀️.rs` | flow root, mutations |
| `io::export::serializers::artifacts::{txt,json}` | `🚪️io/📤️export/🧵️serializers/🗿️artifacts/{🔤️txt/🔖️utf-8,🔣️json/🔖️rfc8259}/✳️any/🦀️.rs` | flow root, mutations |
| `snapshot::schema` | none (alias of the schema crate) | flow root |
| `examples::demo` | `📚️examples/🎬️demo/🦀️.rs` | flow root, mutations, editor |
| `editor::flow::modes::edit::windows::main::transient` | `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🦀️.rs` | flow root, editor, connect-media-ports |
| `…::modes::generate::windows::{generations,preview}` | `✏️editor/🎭️modes/🧬️generate/🪟️windows/{🗂️generations,👁️preview}/🦀️.rs` | flow root, viewer, editor |
| `…::panels::inspection` | `✏️editor/📌️panels/🔍️inspection/🦀️.rs` | flow root, editor |
None of the moved source files declares a `cfg(test)` module itself. Their tests (`🧪️tests/🔬️unit/🦀️.rs` next to each) are mounted by the unit crates.

### Split design (script: `🐍️split-flow-flow-tests-crate.py`, not applied)
Three new production crates by domain, below `editor`/`flow` in the graph (they depend on `schema` and `inferences`, never on the tests crate):
| crate | directory under A | holds |
|---|---|---|
| `semio-s-artifact-flow-flow-io` | `🚪️io/📦️packages/🦀️rust` | text, import txt/json, export txt/json, `snapshot::schema` alias |
| `semio-s-artifact-flow-flow-examples-demo` | `📚️examples/🎬️demo/📦️packages/🦀️rust` | `examples::demo` |
| `semio-s-artifact-flow-flow-editor-surface` | `✏️editor/🖼️surface/📦️packages/🦀️rust` | transient, generations, preview, inspection |
Each has `Cargo.toml`, `🦀️.rs` (extern-crate aliases, `FlowSnapshot` re-export) and `🪞️mirror/🦀️.rs` with the same `standards::v1::subsets::any::…` / `editor::flow::…` module paths as the tests crate, so consumers only swap the crate name. Dependencies start as a path-relativised copy of the tests crate's `[dependencies]` and are pruned after the first check.
Edits the script makes:
1. consumers repointed: flow root `🦀️.rs`, `🚪️io/🦀️.rs`, mutations, viewer, editor, connect-media-ports mirror, retirement-tests mirror, and the `unit` / `unit-2` mirrors;
2. tests crate mirror: the production file modules are replaced by `pub use` of the new crates, so test sources keep resolving `crate::…` and no type is compiled twice; the tests crate gets the three crates as dependencies; it stays a dev-dependency of the unit crates only;
3. production manifests (flow, mutations, viewer, editor, connect-media-ports, child-publication, retained): drop `semio-s-artifact-flow-flow-tests`, add the new crates that their sources use;
4. artifact workspace `Cargo.toml` members (+3), `📋️build-owners.json` (+3 owners, `packages: []`), `📦️packages/🦀️rust/📜️script.ts` line ~196 package list (+3);
5. lock files containing the tests crate: `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock`, the artifact `Cargo.lock`, `🌎️hub/Cargo.lock` (new packages, changed dependency lists).
Expected iterations: a few `crate::…` path misses in the new mirrors (`crate::schema::widget_id`, `crate::flow_genesis_content_pack`, `pack::`), fixed by re-exports in the new roots. Check order: io, examples-demo, editor-surface, then each consumer, one `repo cargo --parallelism=1 -- check` at a time.

## Other leaks in the flow hub closure (631 crates; scan `🐍️hub-closure-test-leaks.py`, raw output in `🗑️generated/hub-flow-closure-test-leaks.txt`)
| production crate | leaking dependency | note |
|---|---|---|
| `semio-s-artifact-stdio-{json,md,txt}` | `semio-s-artifact-stdio-{json,md,txt}-tests` as `[dependencies]` | same carve pattern as flow; needs the same split (inventory in a later step) |
| `semio-os-kernel` | `semio-os-store-test-support` (`[dependencies]`, line 95) | no use found in the kernel lib sources; verify and move to dev or delete |
| `semio-plugin-instance-extension` | `semio-os-kernel` with `features=["mutation-testing"]` | no `mutation` use in its sources: drop the feature or make it dev-only |
| `semio-plugin-publication-mounted` | `semio-framework-test-fixtures` (`[dependencies]`) | already also in `[dev-dependencies]`; check lib use |
| `semio-framework-canvas-assets`, `semio-os-infinite-world-host`, `semio-plugin-instance-app` | `semio-framework-test-fixtures` | optional behind `testkit` / `artifact-app-testing`; not a leak unless something in the closure enables the feature |
| `semio-framework-job-testing-caller`, `semio-plugin-testing-{acceptance,caller,instance-laws,laws}` | `testkit`, `mutation-testing`, `artifact-app-testing` features | test-support crates by design: left alone |

## Status
Step 1 applied, nothing compiled. Step 2 (split script) prepared, held until the React lane finishes its flow hub build.

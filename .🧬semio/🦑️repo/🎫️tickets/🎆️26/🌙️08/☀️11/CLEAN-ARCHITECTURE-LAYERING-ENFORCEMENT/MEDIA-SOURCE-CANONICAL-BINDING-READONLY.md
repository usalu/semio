# Media Source Canonical Binding Follow-Up

Read-only source audit, 2026-10-14. No edits to source, tests, frontend, init, native, Root Source or AGENTS; no executed tests, runtime claims or lifecycle calls. This Markdown is the only new file. It refines the earlier recommendation: prefer an explicit Source test command to extending the broad `verify` family.

## Actual package APIs and minimal binding

Layout's `📦️packages/🦀️rust/📜️script.ts` already imports BundleScript, runOwnedCommand and path.resolve. Its actual final call is `runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-layout-layout", {commands:{verify:OwnedVerifyScript}, snapshotSqliteTests:[...]})`. OwnedVerifyScript is real; its existing cases are physical-codecs, layout-window-ownership, layout-document-contract and layout-frame-selection. It is not a preexisting media command.

The shared artifact runner `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:36` accepts `commands?: Readonly<Record<string, ScriptCommand>>` and `testCommands?: Readonly<Record<string, ScriptCommand>>`. At :89–92 it registers every `commands` entry directly. At :61–66 `testCommands` is routed under `test`. Therefore a new local BundleScript Source class registered in **commands** as `test-media-adoption-source` is the minimal explicit binding; it avoids making a test look like document verification and does not require shared runner changes. `run(segments:string[]):Promise<void>` is the established class API. Reject nonempty segments, resolve owned source from this.root, run strict TypeScript then Bun test using existing first-party runOwnedCommand and invocation cancellation. Do not introduce an external runtime dependency.

The proposed target, explicitly NOT authored or runnable yet, is `test-media-adoption-source` in Layout artifact `📋️project.json`, executor `repo:script`, cache:false, dependsOn:[], outputs:[], options cwd:{projectRoot}, command:`bun ./📜️script.ts test-media-adoption-source`. Its scoped inputs should cover package script plus owned export test/schema/corpus and the concrete Layout editor/export Rust sources inspected. Existing General Source targets demonstrate these target fields; existing Layout verify targets use repo:script and explicit package cwd. No need for workspaceCommand metadata for a directly launched test target.

## Authoring owner and portable corpus

Keep adoption at Layout export, whose Rust implementation owns LayoutMediaExportJob and whose unit test module is attached by `#[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests` at export Rust :4758–4759. Author portable test at export `🧪️tests/🟦️.ts`, corpus schema at export `🧪️tests/🧬️schema/🔣️.json`, and cases at export `🧪️tests/🧫️fixtures/🔣️.json`. These new paths are recommendations. Test-local schema describes an adoption test corpus, not the runtime Layout document schema. Keep it beside the test to avoid adding a production schema dependency for source text assertions. Define exact owner-relative source paths, expected generic mutable request/body signature and concrete Layout delegate in the corpus. Assert source resolved from import.meta.dir, strict Ajv corpus validation, and negative malformed corpus cases. Third-party Ajv remains test-only; no runtime API exports require it.

The one concrete assertion now embedded in General can move there unchanged in meaning. Corpus source paths must be relative to owned export/editor locations, never absolute workspace/ticket locations. If an independent transactional preparation oracle is included in S, hand-author S-specific expected phases from actual Layout admission; do not copy General's corpus and pretend it is a Layout runtime witness. A source assertion verifies adoption wiring only; the actual native admission is not executed by it.

Existing Layout document contract is a different owner: `🧬️schema/🧪️tests/🪪️document/🟦️.ts` imports its owned document vectors from subset `🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json` and calls the first-party document oracle. Do not bury media producer wiring in that document contract. Existing native `exact_layout_out_media_factory_dispatches_a_real_reserved_job` verifies the real factory, bus and reserved export job, but does not prove mutable-slot admit_original wiring. Leave native untouched in this move.

## General laws remain bound

General MediaTurnSourceScript has two exact source inputs: Media Turn source test and Cancellation Issuer source test. It strict-typechecks then Bun-tests both; MediaTurnNativeScript inherits source execution then runs `--offline --test media-turn --features media-turn-testing -- --nocapture`. Keep that binding and all nine Media source tests. In producer preparation retain Ajv validation, exact fixture phases/denial cases, independent SQLite transaction oracle, and generic SDK `body:&mut Option<Box<dyn ...ArtifactReservedJob+Send>>` assertion. Relocate only the S editor read/delegate check on line 42 after the held General lane is released.

Additional current binding: General package TestScript :65 also Bun-runs the Media Turn source file together with the history action source suite. Consequently this boundary fix repairs both the focused Media Source target and broad General plugin test. No independent S path exists in General Media Turn's schema or fixture: bounded rg for `✏️s`, `Layout`, `layout`, `s.layout` returned only test line 42 across the Turn subtree. `producerPreparation` references occur in its schema, fixture and source test. External schema catalog/document-index entries register the neutral schema path/id and contain no concrete Layout binding. Do not remove them. The generic tool-job-coverage Source test contains structural engine/ownership self-tests; it is not a suitable owner for a concrete Layout delegate assertion. Do not move this test there or alter its gate rosters.

## Seed and mirror launch placement

Both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` contain the General pair. Seed :72311 entry is `🎟️Original Media Turn Source 🦀️`, command `bun repo run @semio-tech/framework-plugin:test-media-turn-source --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype`, presentation group `4_gate`, order 900.8409. Its next entry is Original Media Turn Native. Mirror General source command currently appears at :93708 and native at :93724. The source entry carries prototype build mode, SEMIO_SCRIPT_POLICY owner `plugin:media-turn:source`, and ticket-generated artifact destination. Existing Layout document/window verification entries live elsewhere in the same inventories; a new explicit Layout Source entry should be placed with the media pair in **both** inventories, preserving existing ordering and using a distinct Layout policy owner and ticket output subfolder. Do not silently register only the generated mirror or reuse a General owner for S.

Proposed (not current) outer command after registration: `bun repo run @semio-tech/layout-layout-rs:test-media-adoption-source --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype`. Use existing launch format; source-only coverage needs no native capability blob. The shared artifact runner receives the invocation envelope before routing; the Source handler must preserve deadline/cancellation via the existing owned execution API. No exact new launch order or name was invented as already present.

## Bound batch and validation limits

Batch: three S-owned corpus/test files; Layout package script + project; both seed/mirror launch entries; then coordinated single-line General relocation. No new source/native/test framework dependency, no generic adopter roster, no deletion experiments, no artifact copies. General source fixture and schema require no edits. Source inspection and JSON inventory are the only validation performed; no tsc/Bun/Cargo/launch runtime execution occurred. Before authoring, recheck hashes because agents share this checkout.

## Proposed absolute path lengths

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts`: 144 characters.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🧬️schema/🔣️.json`: 155 characters.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🧫️fixtures/🔣️.json`: 157 characters.

## SHA-256 observed source snapshot

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📜️script.ts`: `3887a28910bb759739b6a4b66ecbb3919b2708ef76e9015d5ec7bd9cf2e30545`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📋️project.json`: `3122e3a623da5fd100131e59c7130a1b0ce56e8a69300e72a5a4f90c6c6c2042`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`: `315bbd7bf06ad889c72ac54b9fc817363e5a74cfe1a8b2f8ee290a0d42832e06`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json`: `aef1eb0d2acc9def08f5994176adbd8f643891a57b9fd100b6c4db9f8be42e06`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🟦️.ts`: `4335c92fa3c9a911d50d0ed9eb06ddcf838173f6ffb25832159a92543cb1ec47`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧬️schema/🔣️.json`: `20aeb5d0c1da8dc2d91d89992418f2e4deb8c75aca79ea79e686153316ffa8da`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🧫️fixtures/🔣️.json`: `1b1b35fc5620f47a478d70cc74b48f51cbf39587bff1e948dd68b2a0af2a702c`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`: `140fdb40f01dc932d362a53bc8dd18e9ed1196255ee36dd5a344f59f14d8eeed`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs`: `fda71a7d46f16c9a13f1a5bbe2dffa2c2644a9d67e40eab77dd740af22a8828c`
- `.vscode/launch.json`: `6c7a5afade01ee4a3a9f40661ee06bd518ed3ddb730b54c7558132a3e20b381b`
- `.vscode/🧩️launch.seed.jsonc`: `73a4c63e2e2f3eac7b5a0361427654fec6462cdc65e30a7f7932efd9d5a335af`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts`: `878d3826d6e22fe569b7151e218f05f5b3b7e08034cee043abcfbd4ed714c505`

# Norm Strict Plugin Assembly and Public Harness Audit

Status: source-closed for Norm package/document identities and source-credible for the current real-factory public harness; runtime unverified. This is a read-only current-tree audit. No build or runtime test was started here. Coordinator-reported `canonical51509` exit 0 predates the current package-identity insertion and identity cutover; it does not discharge the current public surface harness.

## Package identity ordering is source-closed, runtime unverified

The live root now calls `.label(...).version(...).package_id("semio:norm")` at `✏️s/🔌️plugins/📕️norm/🦀️.rs:74-78`. This is the only order valid because `package_id` is implemented for `PluginBuilder<Ready, _>` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:212-217`. It supersedes the earlier compiler session `25861` failure. No post-ordering compiler result was observed by this audit.

The exact value agrees with component metadata at `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml:11-13` and, once it is on the ready builder, satisfies the strict `semio:<plugin-id>` equality at `🏗️builder/🦀️.rs:624-628`.

## Superseded P0: the fifteen definition trees formerly used a different document identity

Every actual editor/viewer app uses the canonical three-segment document dialect `s.norm.<family>`; EN 1990 is representative at `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️.rs:105-109`. The same source shape exists for all fifteen families:

- DIN 4108, DIN 16798, DIN 18599;
- EN 1990 and EN 1991 through EN 1999;
- ISO 16757 and VDI 3805.

This discrepancy has now been corrected in the current tree. Each of the fifteen roots calls `assemble_definition("s.norm.<family>", ...)`; EN 1990 is the concrete source anchor at `.../📘️en1990/🦀️.rs:118-139`, where the definition, capability IDs, schema/inference/composer claims, and codec identity agree. A current exact source scan found no executable quoted legacy two-segment family identity under `✏️s/🔌️plugins/📕️norm`. The owner reports that its source namespace gate found and repaired 47 source/schema/fixture occurrences; that gate's terminal result is coordinator evidence, not an independently run result here.

The IO authority grammar accepts only `s.<plugin>.<artifact>` (`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:103-155`). The repaired EN 1990 IO rows now agree on `s.norm.en1990` at `.../📘️en1990/.../🚪️io/🦀️.rs:6-10,40-49,118-120`; this representative path includes the runtime import/export kinds, composer coordinate, and round-trip fixture. The builder itself still has an `if let Ok(ArtifactKindId::parse(...))` guard (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3474-3480`), allowing malformed identities from some other plugin to evade its claimed strict check. That framework-wide hardening is a separate residual audit, not a current Norm identity defect.

The prior deterministic composer miss is source-closed for Norm: the repaired IO coordinate matches the opened app's dialect. IO resolution remains exact and fails with `no composer registered` on a future mismatch (`🧰️framework/🔨️modules/🚪️io/🦀️.rs:1064-1074`), so a fresh process-backed composer/factory gate remains required before a runtime claim.

The smallest source-owned repair is one atomic family-wide identity cutover:

1. Source-closed: every root definition identity and its capability identities/claims now use the family’s canonical `s.norm.<family>` identity.
2. Source-closed by the 47-file canonicalization: nested schema, composer, IO `Dialect`, IO import/export kind, and round-trip fixture rows agree on that coordinate. A future cleanup may centralize those raw entries on `*_DIALECT`, but no current executable legacy value remains.
3. Still required: add a public factory law asserting, for every manifest app, that its `document_schema`, app definition dialect, corresponding document codec, definition-registry identity, and `io::resolve` key join on the same canonical family coordinate.

Do **not** change `computation.norm.<family>`. `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:169-209` explicitly reserves it for the distinct computed `report:out` artifact and uses it in the report media payload at lines 214-232. It is not the persisted/openable document dialect.

## Public surface harness: real factory path, source-credible projection and cleanup

The earlier source finding that the harness directly constructed empty-registry wrappers is superseded. The current [`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:55-100`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:55) does use the real `semio_s_plugin_norm::plugin()` result, its manifest definitions, definition registry, and `plugin.create_app(&row.app_id)` factory products. It drives `PluginApp::render` with an explicit English/native `ViewModel`, plus unknown-body and oversized-key negatives for every factory. The prior `serde_json::to_value(&tree.root)` compile/runtime failure is source-closed: `BuiltChildren` deliberately rejects serialization when populated ([`ui builder:325-332`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️builder.rs:325)), and the test now walks borrowed children while serializing only each typed component ([`surface test:31-43`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:31)).

- It compares the 30 fixture IDs with exactly the manifest app IDs and checks the definition registry has exactly `s.norm.<family>` roots.
- It visits all 30 registered factory products and renders **120** body invocations: the formerly counted 90 declared surface bodies plus the 30 inherited framework history bodies.
- `AppBuilder::build_definition` unconditionally injects `FRAMEWORK_HISTORY_BODY_KEY` when absent ([`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5274-5284`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5274)); its literal is `framework.body.history` ([`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:701`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:701)). The fixture now includes it in every row, and the test's `definition`-derived body-key set must equal the fixture before rendering. This is the required guard against silently omitting inherited panels.
- Each declared body yields a typed non-rejected tree; unknown body fallback is explicit; a >70k key is rejected. `project_and_retire` now uses the runtime producer's exact 384-node ceiling, captures any projection panic, unconditionally drops the tree, drains the retained-page authority, and resumes the original panic only after cleanup ([`surface test:31-56`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:31)). This supersedes both earlier source REDs: the prior 4,096-node test-only bound and the panic-path retained-page leak.

The cap is semantically aligned: `ComponentTreeProducer` increments its complete-node census and faults after `UI_BUILT_CHILD_RETIRE_SLOTS == 384` nodes ([`present.rs:195-214`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️present.rs:195)). The direct projection independently detects rejected backing and duplicate sibling keys before serializing each typed component. `close_built_node_page_one` returns true only when the global retained-page authority is terminally empty ([`ui builder:115-146`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️builder.rs:115)); 8,192 opportunities comfortably bound a 384-node accepted tree plus empty-page turns. This is source evidence only: the harness does not itself drive `ComponentTreeProducer`, renderer submission, WGPU, or a non-default snapshot.

The cleanup authority is intentionally process-global. This integration binary currently owns one public-surface test, so no current same-binary parallel owner is evident. A future second parallel UI test in that binary must not treat this drain as a per-tree token: it would need a serial owner or a scoped retirement API. That is a maintainability guard, not a present false-positive in this isolated test.

The registered entry points are `surface-render-source` and the exact `--test surface_render` runner in `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`. Neither was run by this audit. The source runner remains advisory; the exact Rust runner must execute the factory path after the current fixture and cleanup corrections.

## Public config harness: source-credible, runtime unexecuted

The config protocol harness is registered as the public Cargo integration test `config_mutation` (`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml:100-106`). Its oracle payload is correctly strict (`#[serde(deny_unknown_fields)]` at `🎚️config/🧪️tests/🦀️.rs:6-10`), including the hostile extra-field row at `🎚️config/🧪️tests/🔣️.json:10`. It also checks the runtime type is the schema-owned type, exactly one semantic descriptor, inverse, text, binary, and retired wire rejection (`🎚️config/🧪️tests/🦀️.rs:12-52`). The independent AJV companion owns the 13 text/25 binary neutral and hostile forms (`📜️script.ts:102-161`). Coordinator session `36156` is reported green for the source/AJV route; the public Rust runner was not run by this audit.

Its registered runners are `config-mutation-source` and the exact `--test config_mutation` runner at `📜️script.ts:102-130`.

## Acceptance boundary

Norm cannot claim a strict registered-plugin runtime pass until a post-cutover compiler result and the actual-factory public test execute green through their exact registered runners. A later pass must distinguish:

- source/compile: the coordinator-reported canonical library result;
- runtime: one real `plugin()` assembly plus all 30 `Plugin::create_app` products rendering the 120 fixture bodies (90 owned surface bodies plus 30 inherited history bodies) through their factory registry; and
- config protocol: Rust plus independent AJV oracle exercising the current 13 text and 25 binary vectors.

This audit intentionally makes no claim about native host activation, document open, interactive mutation publication, localisation parity, or component generation.

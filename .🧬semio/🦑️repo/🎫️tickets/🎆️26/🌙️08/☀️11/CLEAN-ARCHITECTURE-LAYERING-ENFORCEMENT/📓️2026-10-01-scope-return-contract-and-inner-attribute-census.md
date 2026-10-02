# Scope Return Contract And Inner Attribute Census

Read-only textual census, no tests/compiler run. Search scope: current Repo library TypeScript consumers and ordinary rg-visible Rust files under framework, S and Hub. Generated JSON fixture strings are separate evidence, not counted as actual Rust source attributes.

## Whole Return Assertions Versus Subsets

No direct `expect(inspectRustModuleGraphFacts(...)).toEqual(...)` whole-return assertion was found in the inspected library sources. Live tests select subsets:

- workspace-contract/🟦️.ts:4824–4825 maps modules to name/pathTarget.
- workspace-contract:7655–7656 checks one module conditional flag.
- native-source-ownership:29 filters modules by name/tests/conditional/pathTarget.
- source-direction:240 and351 call the higher inspectRustModuleGraph and compare contexts, not entire raw facts.
- rust-finite-target-consumption:173 contains an explicit TypeScript declaration string for inspectRustModuleGraphFacts return with modules/uses only. Update it with required scopes and any new exported selector included in extracted code dependencies.
- rust-finite-target-consumption:130 and rust-writable-path-authority:38 inject the actual function into generated/extracted test harnesses. If normalization now calls rustModuleScopeProof, inject and declare that symbol in both harnesses; failure otherwise is a real harness dependency error, not a production scanner regression.

Current direct production users: authoring/mutation-tree:194; normalization mutation/evidence:104,183; mutation/structural-reachability:327,342; normalization/🟦️.ts:4554; physical source execution:120; source/🔗️binding/🟦️.ts:180. The binding audit currently reports only module.unresolved; add unresolved scope diagnostics so root/inline rewriting is visible. Normalization:4565–4575 directly rechecks source-chain raw module facts after graph contexts; consume the shared scope proof rather than assume modules alone establish scope authority. Authoring and fallback filters require the same shared selector as already recorded.

## Actual Inner Attribute Heads

Ordinary Rust source matches produced these head counts (attribute occurrences, not unique files):

| Head | Count | Exact source examples |
| --- | --- | --- |
| allow |224| Hub rust root:11 `allow(async_fn_in_trait)`; Hub energy rust root:31 `allow(clippy::too_many_arguments)` |
| cfg |6| Hub inference/wal unit tests:3 `cfg(feature="native-artifact-execution")`; Puzzle5d wasm editor:7 target_arch/target_env predicate |
| deny |1| framework/value/derive deny-unknown-fields-enums test:14 `deny(unreachable_code)` |
| feature |2| OS/plugin Rust package root:3 `feature(associated_type_defaults)`; S/spatial-kernel engine session Rust package root:1 `feature(allocator_api)` |

Full exact paths for the two feature sources:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs:3`
`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🦀️.rs:1`

No recursion_limit/register_tool/doc/cfg_attr/warn/forbid/expect inner heads were found by this actual source query. Doc/cfg_attr are present in the newly authored closed attribute fixture/native source strings and need contract tests despite not appearing as physical first-party .rs examples. Do not expand an allow-list solely from possible system names without source or owned corpus evidence.

Allow/deny are lint metadata; feature changes language availability and should have an explicit owned syntax/authority classification rather than be called inert by association. cfg gates source presence and must retain all authored configurations. No observed head itself proves arbitrary nested token input expansion. Parse exact metadata structure and retain unsupported/null heads instead of treating unknown text as transparent.

The counts reflect this bounded rg-visible source snapshot and can change during concurrent work; they are not a whole ignored/hidden tree census. No passing behavior is asserted.

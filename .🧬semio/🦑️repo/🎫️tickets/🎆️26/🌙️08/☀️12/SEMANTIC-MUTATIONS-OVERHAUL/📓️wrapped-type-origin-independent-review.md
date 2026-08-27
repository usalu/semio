# Wrapped Type Origin Independent Review

FND-WRAPPED-TYPE-ORIGIN-21 is accepted for the bounded exact-origin/no-follow inspector after the final replay below, not for metadata-provider approval or global policy. Its first coordinator replay executed fourteen exact-origin vectors, compiling all six accepted Rust sources through rustc. Seven cases passed and seven failed: `🧪️wrapped-type-origin-root-review/🧪️first.log`, run `🧫️run-QVzYBl`, exit 1.

The failures are grouped public aliases at the root and child, a public inline-module declaration alias, an incorrect child-file module path, two disabled declarations incorrectly approved, and conflicting aliases resolved by last-write-wins instead of rejected as ambiguous.

`modulePath` is source-local: a declaration at the top of a mounted child file has `[]`, not the path of that file's mount. A declaration inside a public inline module in the direct primary retains that module path in the same physical source. These facts must match the accepted metadata declaration inspector's coordinate system.

The executor is correcting the existing proof using accepted Rust declaration/alias facts, retaining duplicate candidates and rejecting conditional declaration evidence. The six positive source compiler results remain independent syntax evidence; the inspector's exact origin and rejection results must all match before acceptance. Existing nineteen-case reachability preservation, expanded neutral tests, and final registered replay also remain required.

## Corrected Origin and Conditionality Replay

Root's corrected fourteen-case replay passed (`🧫️run-qp0Ryg`, exit 0). A subsequent independent expansion to twenty-four cases also passed (`🧫️run-w8hSyp`, `🧪️conditional-root.log`, exit 0). Six positive sources and ten deliberately rejected but syntactically legal conditional/competing sources compiled with rustc. The newly checked cases cover aggregate and variant `cfg`/`cfg_attr`, root inner conditionality, inline ancestor conditionality, and conditional competing declarations/reexports/variants. The executor's registered fixture now has thirty-three cases and reports 212 expectations; root's own final registered invocation remains pending.

## Virtual Filesystem Rejection Still Open

The independent virtual filesystem harness uses trapped `lstatSync` and `readFileSync` calls. Every source/file node in these cases exists only in memory; no excluded tree is created or inspected. It ran eighteen cases and exposed seven remaining failures: a nested source filename is accepted, four unsafe repository-base spellings are not rejected before access, and repository-root/ancestor symlinks are not inspected. The relative source guard does not validate its `repoRoot` boundary.

Evidence: `🧪️wrapped-type-origin-safety-review/🧪️root-first.log`, retained run `🧫️run-qbcGfw`, exit 1. The executor retains the bounded source ownership to require single-component filename/leaf identities and a raw-safe, no-follow repository-root ancestry. Relative mutation and child exclusion cases already passed; all eighteen virtual cases must pass, together with twenty-four origin cases and the registered regressions, before FND21 is accepted.

## Final Native Boundary Acceptance

The seven safety defects were corrected, followed by two independently found native-path defects: a Windows drive was rebased on POSIX and an extra drive colon was accepted. The final implementation requires native absolute input and one drive colon only. Root's fresh twenty-case trapped virtual replay passed with zero failures and exit 0: `🧪️origin-boundary-final/🧪️native-corrected-root.log`, run `🧫️run-iWf6pD`. The exact-origin/conditional tests were preserved; the permanent fixture now has 39 cases, including raw repository roots, a nested filename, root symlinks and ancestor symlinks.

The final registered selection passed two tests with 268 expectations, 294 filtered and exit 0. This includes the unchanged nineteen-case reachability selection and the expanded 39-case origin selection with real rustc checks. Transcript: `🧪️wrapped-type-origin-native-registered.log`; fixtures: `🧪️wrapped-type-origin-native-registered-artifacts`. The earlier unrelated WGPU taxonomy preload failure cleared without changes by this task.

Some older raw evidence directories referenced in the historical sections have since disappeared; the observations are historical, not claims that those raw paths remain available. The final paths above were freshly created and verified. The executor is released to the separately scoped Cargo binding authority packet; no aggregate metadata policy or provider-alias approval is implied by this acceptance.

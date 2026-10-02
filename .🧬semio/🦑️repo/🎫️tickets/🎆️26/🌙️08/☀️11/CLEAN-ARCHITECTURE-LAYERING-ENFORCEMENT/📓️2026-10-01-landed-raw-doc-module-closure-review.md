# Landed Raw Symbol, Documentation And Module Closure Review

Read-only current source review; no tests or compiler executed. Current native provider receipt remains Root's responsibility.

Canonical raw macro identity is now consistently used for template names, duplicate names, recursive calls, required metavariables, binding lookup, builtin input dispatch and outgoing include guard (discovery/🟦️.ts:6467,6480–6487,6642–6673,6747). Item keywords still compare authored token.text, so r#mod/r#fn do not accidentally become item syntax. The three previous raw macro golden offset pairs remain exact ASCII/UTF16 token starts: ordinary41/100+132, raw43/104+136, metavariable43/99. Retain canonical macro=load and authored raw token offsets independently.

Both previously identified direct normalization membership grants now check !unresolved: evidence/🟦️.ts:183, structural-reachability/🟦️.ts:333,345–346. This fixes those exact fallback holes. Required hostile consumer laws should exercise rejected mounts through their actual APIs rather than only graph construction.

## Remaining Concrete Boundaries

1. Inner attributes do not enter the new owned attribute validation path. rustAttributes recognizes only `#[...]`, while the added validation at discovery:6692–6698 handles only its ranges. `#![...]` reaches generic delimiter recursion, which inventories builtin-looking tokens without the doc-context/opaque-wrapper checks. Use a native-success inactive inner case `#![cfg_attr(any(), opaque(include_str!("absent.txt")))] fn main(){}`: native expands no input; the scanner should refuse opaque attribute provenance, not report a resolved doc input or a guessed missing file. Pair it with the equivalent outer attribute refusal. Inner doc itself is a valid positive but does not prove this boundary.

2. Documentation wrapper recognition allows a macro solely by canonical terminal name: the surrounding-pairs guard approves concat/env/include names without proving an unqualified builtin identity. A qualified custom `provider::concat!(include_str!(...))` or locally shadowed concat can consume tokens instead of expanding an input. Native closure requires a real owned helper provider that ignores its token argument and emits a string; absent such identity proof, refuse the wrapper. This is an authored macro identity issue, not a speculative procedural emission request. Direct builtin include dispatch has the same name-shadowing distinction if a repository owner claims exact expanded dependency equivalence.

3. MetadataHead now admits name=value at8338, but still silently drops unparseable heads in rustMetadataAttributes. Module unresolved at8702 only sees parsed items. A single undecodable path yields pathTarget=null without unresolved (paths.length1), allowing conventional candidate lookup rather than explicit refusal. These may be invalid native attributes, but the fail-closed graph contract should represent malformed metadata and null path proof explicitly, not infer a default mount. Record native-failure/gate-refusal separately from native-success dormant rewriting rows.

4. Whole-expansion coherence at execution:116 now verifies every row's canonical name, definition line, scope, monotonic offsets and local bounds. It still derives module candidate lexicalPath from the first reference and does not compare each reference.modulePath against that path. Generated current scanner rows share modulePath, so no direct authored bypass is established; the public metadata law should nevertheless mutate a later row's modulePath and require refusal. Likewise assert actual line numbers agree with source offsets when external callers supply canonical metadata.

## Documentation Corpus Needed Before Broad Coverage Claims

Current fixture cases202–244 cover outer doc, inner doc, concat doc and active cfg_attr doc. Add dormant doc to the all-config corpus, with expected native/gate difference made explicit: native omitted dependency versus authored gate input. Add string and comment decoys, outer+inner opaque refusals, nested cfg_attr doc, and unknown wrapper refusal. Actual UI builder doc sources at framework/ui/contract/builder:2045–2046 remain the required real-owner reference pair.

The native core law currently uses one uniform expected dependency set from fixture.references, except unused template. Therefore a dormant cfg_attr doc row cannot be inserted there with authored reference expectations and an absent native dependency without extending the closed fixture's independent native expected-input field or separating the dormant corpus law. Do not weaken all-config inventory to fit that harness. Physical deletion must be tested separately for active doc inputs; dormant missing input is a deliberate native-success/all-config-refusal case.

No full coverage or live whole-owner success is claimed by this review.

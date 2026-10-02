# Workflow Semantic Snapshot Ownership

## Persisted Scope

The existing `WorkflowSnapshot` root has seven persisted fields: schema, graph, parameters, parameter_bindings, inputs, input_bindings, and output_bindings. Its existing native root Value bridge and package-contract fixture use snake_case collection names; nested records use their authored camelCase names. This owner preserves those spellings.

The handcrafted schema has sixteen tables. Document and graph roots own ordered nodes and edges. Node ports carry their full persisted specification, including direction, class, form, optional literal kind, required flag, and multiplicity. Each edge owns its complete media contract, exactly one binary or document wire branch, and an optional conversion pair. Four parameter branches have independent numeric, categorical, toggle, and text entities; categorical options retain order and duplicate literal values. Three binding collections and input slots remain independent ordered entities.

Persisted string identifiers can be empty, duplicate, unresolved, or contain NUL and Unicode. Only surrogate SQLite entity relationships are foreign keys. Node geometry and all optional numeric parameter values have explicit binary64 word/class companions. Optional absence is independent of NaN's NULL query value. No native, JSON, or generic inferred snapshot carrier is stored.

The neutral fixture exercises every root, both wire branches, conversion presence/absence, both port directions and multiplicities, opposite direction vs input/output membership, present empty and absent optional kind, all parameter branches, duplicate literal IDs/options, unresolved endpoints, and twenty malformed independent SQL edits. Nine IEEE words cover signed zero, subnormal, finite extremes, infinities, and distinct NaN payloads.

## Current Validation

Schema, DDL, and neutral fixture authored first. Source/native providers remain absent at this stage. The upcoming registered source gate will measure the missing implementation before production functions are added. The root Cargo lane is occupied by the five existing parent owners; Workflow native baselines will execute through that same lane later.

The initial registered uncached quick source gate executed all 41 laws: 2 independent schema/DDL passes and 39 missing-public-implementation failures, 6 assertions in 324ms, Nx exit 1 after 26.5s. Full output is retained at `🗑️generated/root-workflow-source-red.log`. Only after this actual result, the explicit sixteen-table source codec and facade exports were mounted. Reconstruction reserves 512 bytes per input row before grouped maps and charges ordered lookup arrays separately; all entity identities and ordinals are checked without imposing uniqueness on literal domain identifiers.

The first mounted source gate then executed 41 laws: 40 passed and a single row-count fixture precondition failed, 99 assertions in 11.77s. The sixteen authored table populations `[1,1,2,3,2,2,4,1,1,3,1,1,2,2,2,2]` sum independently to 30; the prior expected 32 was corrected. Strict public types rejected an unreachable undefined-to-number cast produced by the neutral fixture's literal NULL type. Numeric fixture inputs now pass a strict runtime number guard; nullable presence retains its original NULL semantics. No production codec change or assertion removal was made for either correction.

The fresh registered uncached quick source/public gate passed all 41 laws, 102 assertions in 11.87s, plus strict public compilation. Nx exited 0 after 1m10s. Full output is `🗑️generated/root-workflow-source-admission.log`; the native owner and actual native I/O route remain separate pending obligations.

The actual native baseline executed all seven selected laws: six passed and the missing semantic owner failed. Nextest `3d0a47bd-2599-4cec-a8c7-8ce058385a41`, 175ms assertions, forty-two unrelated tests filtered. Both ordinary native encodings retained all full neutral fields and all twelve geometry/optional numeric raw-word slots. Only after this authentic result, the sixteen-table Rust owner, controlled metadata/record input/output, borrowed exact row forecast, and explicit ArtifactPack capability were mounted. The new implementation and actual native I/O laws remain unverified until the next registered native gate.
# 2026-10-02 04:47 UTC Native Verification

The registered uncached quick native target now passed all sixteen selected Workflow laws, zero selected skips: Nextest `0e1be4ac-2175-46bd-bdfe-720e7e927b09`, 1.936s assertions, forty-two unrelated tests filtered. The full source/public suite remains current at forty-one laws and 102 assertions. Native evidence includes all sixteen authored table populations, empty states, every raw numeric word through both genuine erased directions, all media forms/classes, independent Bun SQLite query/edit/renumbering, malformed semantic edits, exact row/value/file controls, interior UTF-8 and 2048-node cancellation, and the actual canonical editor/viewer I/O declaration. Output is retained in `🗑️generated/root-native-parent-repaired.log`.

This verifies the selected ownership suite. The full fifty-eight-test owning package remains a separate pending check; no universal artifact completion is inferred.

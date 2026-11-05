# Neural Failure Projection Ownership

The actual post-transfer Flow native compiler stopped with E0119: OriginalDomainProjection (FieldProjectionSource) and OriginalDomainFailure were each implemented twice for the same current OperatorWorkFailure enum. The type is defined in model/operator-record. Its dedicated failure leaf and existing independent serde_json law define the required typed projection, complete receipt, borrowed native string and original-turn delegation.

The extra engine/failure implementation differed semantically:

| Boundary | Actual type-owner contract | Duplicate engine projection |
|---|---|---|
| Evaluation root | primary, retained, optional receiptRefusal | variant, primary, retained, always receiptRefusal |
| Evaluation payload | primary.kind and original fields | primary.variant and value fields |
| Borrowed original text ordinal | [0,1] | [1,1] |
| Absent receipt refusal | omitted | explicit null |
| Domain code | neural.operator-evaluation | primary-specific code |

The existing law requires the model contract, including every current EvalError variant, retained receipt, exact payload, original source pointer and nested OriginalTurnFailure. No authored caller or schema evidence requires the differing engine wrapper shape/code. The repair therefore retains the OperatorWorkFailure trait implementations solely beside their actual type owner. It removes the duplicate pair, their unused progress helper and import from engine/failure, while preserving that leaf's distinct legitimate EvalError projection/failure implementation completely. No obsolete type, compatibility alias, formatted diagnostic mirror or hidden runtime dependency was introduced.

The existing type-owner law is strengthened to assert its stable domain code and that an absent receipt refusal is actually omitted and its ordinal refused. All existing variant, receipt, payload, native-pointer, malformed-path and exact original-turn checks remain. Independent review is pending; no native pass is claimed. Integration currently owns the serialized native lane. The next proof is the exact neural owner law, followed by the full Flow controller law and the declared Puzzle producer once these pass.

## Actual Native Proof

The exact neural model owner law passed through ordinary shared/prototype repo Cargo:1passed,0failed,0ignored,18filtered,0.00s native test time. It executed all eight current evaluation variants with complete receipts/payload, native text identity, malformed path refusal, delegated original-turn preservation, stable domain code and absent-receipt omission/refusal. The duplicate implementations no longer conflict at actual Rust compilation. Full temporary output is generated/neural-failure-owner.log. Independent review is recorded in neural-preflight.md.

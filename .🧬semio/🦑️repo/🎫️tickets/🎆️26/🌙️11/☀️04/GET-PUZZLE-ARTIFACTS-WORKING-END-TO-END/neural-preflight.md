# Neural Failure Ownership Preflight

Actual Flow73733 passed Source admission but compiler found duplicate OperatorWorkFailure projection/failure implementations. Read-only review only; no compiler/source/cap changes.

The type is defined by neural/engine/model/operator-record/🦀️.rs:41, with Evaluation carrying actual EvalError, retained receipt and optional receipt refusal, and Original carrying actual original-turn failure. The model/operator-record/failure implementation is the coherent single owner. Its authored neutral cases require evaluation fields primary,retained,optional receiptRefusal; its mounted native law requires primary.kind, borrowed payload at[0,1], exact receipt, invalid ordinal refusal and complete original branch.

Engine/failure duplicate is semantically different: extra root variant, always-present nullable receipt, primary.variant/value rather than primary.kind/original, shifted ordinals and primary-specific code versus model stable neural.operator-evaluation. It cannot simply replace model implementation without changing authored laws/schema. Current caller search found no specific engine-code dependency proving that alternative required.

Recommend remove only duplicate OperatorWorkFailure projection and domain-failure impls from engine/failure, plus unused progress helper/import; preserve its separate legitimate EvalError implementation. Model projection independently traverses EvalError through its typed Node and retains original typed fields, so removal does not lose EvalError public ownership. Do not disable/remove model laws or add compatibility shape. Actual next owner compile/law remains necessary.

## Bounded patch review

Actual engine/failure diff removes only duplicate OperatorWorkFailure trait pair and unused progress helper/import; separate EvalError implementation body remains unchanged. Canonical model owner remains sole required Operator projection/failure implementation, preserving typed original pointer identity and complete delegated Original branch. Its eight-variant JSON law retains all previous assertions and adds stable code, absent receiptRefusal object cardinality2, omitted key and refused ordinal2. This matches authored optional field semantics; no weakened/removed oracle or compatibility shape. No further concrete defect found; native compile/law result remains pending.

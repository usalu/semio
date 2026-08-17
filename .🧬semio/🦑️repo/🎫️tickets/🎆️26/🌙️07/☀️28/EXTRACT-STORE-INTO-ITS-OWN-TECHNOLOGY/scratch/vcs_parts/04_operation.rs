//#region 🔖️Operation
// 🎞️ CW3 kernel cut-over: `Operation`/`OperationDiff` moved to `protocol_command` (only additive
// defaulted methods + id-newtype-typed return values + `reconcile` becoming an instance method —
// see the shim block's comment near the top of this file), re-exported via the
// `🚧️TEMPORARY protocol shim`.

pub fn apply_operation<P, Operation>(projection: &P, operation: &Operation) -> P
where
    Operation: crate::Operation<P>,
{
    operation.diff(projection).apply(projection)
}

pub fn absorb_diff<P, Operation>(_projection: &P, existing: &mut Operation::Diff, incoming: Operation::Diff)
where
    Operation: crate::Operation<P>,
{
    existing.absorb(incoming);
}
//#endregion 🔖️Operation

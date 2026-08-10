use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::WiresSnapshot;
use crate::artifacts::wires::mutations::WiresMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &WiresMutation, base: &WiresSnapshot) -> WiresDiff {
    <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(mutation, base)
}

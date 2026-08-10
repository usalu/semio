use crate::artifacts::wires::WiresSnapshot;
use crate::artifacts::wires::mutations::WiresMutation;

pub fn inverse(base: &WiresSnapshot, mutation: &WiresMutation) -> Vec<WiresMutation> {
    <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(mutation, base)
}

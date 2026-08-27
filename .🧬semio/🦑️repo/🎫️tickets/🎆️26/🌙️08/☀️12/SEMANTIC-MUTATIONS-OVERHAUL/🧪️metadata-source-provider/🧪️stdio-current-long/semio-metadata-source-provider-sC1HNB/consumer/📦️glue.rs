extern crate derive;
use facade::MutationLeaf as GenuineTrait;
impl GenuineTrait for Payload {}
#[derive(derive::MutationLeaf)]
#[mutation_leaf(contract = ::facade::MutationLeaf)]
pub struct Payload;

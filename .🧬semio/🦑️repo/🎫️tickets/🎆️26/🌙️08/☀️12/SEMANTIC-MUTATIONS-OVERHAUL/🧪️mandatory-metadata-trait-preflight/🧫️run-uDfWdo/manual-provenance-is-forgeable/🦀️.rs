
struct Provenance { owner: &'static str }
trait Leaf { const PROVENANCE: Provenance; }
struct Forged;
impl Leaf for Forged { const PROVENANCE: Provenance = Provenance { owner: "claimed/🧬️mutations/insert-item" }; }
fn main() { println!("{}", Forged::PROVENANCE.owner); }


#[derive(Clone, Copy)]
struct Descriptor { kind: &'static str }
trait Leaf { const DESCRIPTOR: Descriptor; }
trait Mutation<P> {
    const DESCRIPTORS: &'static [Descriptor];
    fn descriptor(&self) -> &'static Descriptor;
}
struct Insert<T>(T);
struct Remove<T>(T);
impl<T> Leaf for Insert<T> { const DESCRIPTOR: Descriptor = Descriptor { kind: "insert-item" }; }
impl<T> Leaf for Remove<T> { const DESCRIPTOR: Descriptor = Descriptor { kind: "remove-item" }; }
enum Operations<T> { Insert(Insert<T>), Remove(Remove<T>) }

impl<T> Mutation<Vec<T>> for Operations<T> { const DESCRIPTORS: &'static [Descriptor] = &[<Insert<T> as Leaf>::DESCRIPTOR]; }
fn main() {}


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

impl<T> Mutation<Vec<T>> for Operations<T> {
    const DESCRIPTORS: &'static [Descriptor] = &[<Insert<T> as Leaf>::DESCRIPTOR, <Remove<T> as Leaf>::DESCRIPTOR];
    fn descriptor(&self) -> &'static Descriptor { match self { Self::Insert(_) => &<Insert<T> as Leaf>::DESCRIPTOR, Self::Remove(_) => &<Remove<T> as Leaf>::DESCRIPTOR } }
}
fn report<'a>(value: &'a str) -> String {
    let operation = Operations::Insert(Insert(value));
    let removed = Operations::Remove(Remove(value));
    assert_eq!(operation.descriptor().kind, "insert-item");
    assert_eq!(removed.descriptor().kind, "remove-item");
    <Operations<&'a str> as Mutation<Vec<&'a str>>>::DESCRIPTORS.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>().join(",")
}
fn main() { let owned = String::from("borrowed"); println!("{};{}", report(&owned), <Operations<u32> as Mutation<Vec<u32>>>::DESCRIPTORS.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>().join(",")); }

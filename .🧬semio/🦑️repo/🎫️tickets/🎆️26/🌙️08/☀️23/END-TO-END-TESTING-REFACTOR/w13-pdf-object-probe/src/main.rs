use lopdf::{Document, Object};
use std::collections::BTreeSet;

fn collect_refs(object: &Object, out: &mut BTreeSet<(u32, u16)>) {
    match object {
        Object::Reference(id) => { out.insert(*id); }
        Object::Array(items) => items.iter().for_each(|item| collect_refs(item, out)),
        Object::Dictionary(dict) => dict.iter().for_each(|(_, value)| collect_refs(value, out)),
        Object::Stream(stream) => stream.dict.iter().for_each(|(_, value)| collect_refs(value, out)),
        _ => {}
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("fixture path");
    let bytes = std::fs::read(&path).expect("read");
    let doc = Document::load_mem(&bytes).expect("parse");
    let mut referenced = BTreeSet::new();
    for (_, object) in doc.objects.iter() { collect_refs(object, &mut referenced); }
    doc.trailer.iter().for_each(|(_, value)| collect_refs(value, &mut referenced));
    let present: BTreeSet<(u32, u16)> = doc.objects.keys().copied().collect();
    let dangling: Vec<_> = referenced.difference(&present).copied().collect();
    println!("referenced={} present={} dangling={:?}", referenced.len(), present.len(), dangling);
    let orphans: Vec<_> = present.difference(&referenced).copied().collect();
    println!("orphans (present but unreferenced) count={} first={:?}", orphans.len(), &orphans[..orphans.len().min(20)]);
    let max = present.iter().map(|(n, _)| *n).max().unwrap();
    let missing_ids: Vec<u32> = (1..=max).filter(|n| !present.iter().any(|(p, _)| p == n)).collect();
    println!("missing ids in 1..={max}: {missing_ids:?}");
}

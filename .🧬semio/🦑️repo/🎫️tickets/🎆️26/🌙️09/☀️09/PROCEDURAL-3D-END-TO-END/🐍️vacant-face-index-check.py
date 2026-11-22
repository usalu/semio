"""🌳️ Check the production vacant-batch AVL mutation with the neutral Section face counts."""
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
source = Path("🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs").read_text()
fixture = json.loads(Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/disconnected-planar-section.json").read_text())
sizes = sorted({len(row["faces"]) for row in fixture["cases"]} | {5})

def excerpt(first, last):
    return source[source.index(first):source.index(last)]

code = """
use std::{borrow::Borrow, cmp::Ordering};
struct FoldIndexNode<K,V> { entry: Option<(K,V)>, retired_entry: Option<(K,V)>, left: Option<usize>, right: Option<usize>, height: usize, count: usize }
struct HistoryFoldIndex<K,V> { nodes: Vec<FoldIndexNode<K,V>>, root: Option<usize>, free_head: Option<usize>, displaced_keys: Vec<K>, mutation_epoch: u64 }
#[derive(Debug)] struct ValueError;
enum ValueRefusalKind { OwnershipLimit }
impl ValueError {
    fn literal(_: ValueRefusalKind, _: &str) -> Self { Self }
    fn invariant(_: &str) -> Self { Self }
}
#[derive(Clone, Copy, Default)] struct RetainedCloneGrant { maximum_items: usize, maximum_depth: usize }
#[derive(Default)] struct RetainedCloneProgress { copied_items: usize }
impl<K: Ord,V> HistoryFoldIndex<K,V> {
""" + excerpt("    fn mark_mutation(", "    /// 🌱️ Retains a Copy key") + excerpt(
    "    pub fn contains_key", "    /// 🧭️ Quotes the same original AVL"
) + excerpt("    fn has_vacant_batch_backing", "    /// 📐️ Quotes the next backing step") + excerpt(
    "    pub fn insert_vacant_batch_reserved", "    pub fn vacant_pair_demand"
) + excerpt("    fn insert_at(", "    /// 🎟️ Counts original arena slots") + """
}
fn validate(index: &HistoryFoldIndex<String,usize>, root: Option<usize>) -> (usize,usize) {
    let Some(slot) = root else { return (0,0); };
    let row = &index.nodes[slot];
    let (lh,ln) = validate(index,row.left);
    let (rh,rn) = validate(index,row.right);
    assert!(lh.abs_diff(rh) <= 1);
    assert_eq!((row.height,row.count),(lh.max(rh)+1,ln+rn+1));
    (row.height,row.count)
}
#[test]
fn vacant_face_rows_commit_after_validation_or_survive_refusal_intact() {
    for count in COUNTS {
        let mut index = HistoryFoldIndex {
            nodes: Vec::with_capacity(12), root: None, free_head: None, displaced_keys: Vec::new(), mutation_epoch: 0,
        };
        for i in 0..12 { index.insert(format!("old-{i:02}"), i); }
        let mut rows: Vec<_> = (0..count).map(|i| Some((format!("face-{i}"), i))).collect();
        let keys: Vec<_> = rows.iter().map(|row| row.as_ref().unwrap().0.as_ptr()).collect();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_depth: index.height(index.root) + count + 1 };
        assert!(index.insert_vacant_batch_reserved(&mut rows, grant).is_err());
        for i in 0..count { let key=format!("old-{i:02}"); let (root, removed)=index.remove_at(index.root,key.as_str()); index.root=root; assert_eq!(removed.unwrap().1,i); }
        let original = rows[count-1].replace(("old-11".into(), 999)).unwrap();
        assert!(index.insert_vacant_batch_reserved(&mut rows, grant).is_err());
        assert_eq!(index.get("old-11"), Some(&11));
        rows[count-1] = Some(original);
        let duplicate = rows[0].as_ref().unwrap().0.clone();
        let original = rows[count-1].replace((duplicate, 999)).unwrap();
        assert!(index.insert_vacant_batch_reserved(&mut rows, grant).is_err());
        rows[count-1] = Some(original);
        assert!(index.insert_vacant_batch_reserved(&mut rows, RetainedCloneGrant { maximum_items: 0, ..grant }).is_err());
        assert_eq!(index.count(index.root),12-count);
        assert_eq!(index.insert_vacant_batch_reserved(&mut rows,grant).unwrap().copied_items,1);
        assert!(rows.iter().all(Option::is_none));
        for (i, pointer) in keys.into_iter().enumerate() {
            let (key, &value) = index.get_key_value(&format!("face-{i}")).unwrap();
            assert_eq!((key.as_ptr(),value),(pointer,i));
        }
        assert_eq!((validate(&index,index.root).1,index.nodes.len()),(12,12));
    }
}
""".replace("COUNTS", repr(sizes))

with TemporaryDirectory(dir=ticket / "🗑️generated") as temporary:
    directory = Path(temporary)
    rust_source = directory / "batch.rs"
    rust_source.write_text(code)
    binary = directory / ("batch.exe" if os.name == "nt" else "batch")
    rustup = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20"]
    subprocess.run(rustup + ["rustc", "--edition=2024", "--test", "-Awarnings", str(rust_source), "-o", str(binary)], check=True)
    subprocess.run(rustup + [str(binary)], check=True)

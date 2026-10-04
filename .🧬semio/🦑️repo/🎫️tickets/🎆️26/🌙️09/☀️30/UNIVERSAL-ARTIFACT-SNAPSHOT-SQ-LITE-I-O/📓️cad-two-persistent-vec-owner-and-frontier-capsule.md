# CAD Two Persistent Owners And Concrete Rust Frontier Capsule

This is an unmounted input capsule. Root still owns the unchanged Native14 numerical baseline. Source50 is now GREEN after only its separately authorized semantic role correction; it does not authorize mounting this Rust owner implementation before the actual System allocation RED.

The closed neutral referenceIndex facet already defines nativeMapShape=mapOfOrderedReferences, literalUtf8Bytes comparison, invalidValue duplicate-key refusal and localRelationshipOnly aliases. The seven literal keys and independent BunSQLite CAST(key AS BLOB) order oracle remain. Source tests preserve an explicit empty __proto__ group and reconstruct independently renumbered group/reference surrogates. The actual Native owner still has all nine fields, four distinct optional model children, ordered drawings/nodes, complete raw binary64 identities and optional presence.

## Single Domain Owner In Both Persistent Fields

The proposed source lives next to the CAD domain schema at 🧬️schema/📎️references/🦀️.rs and is reexported as crate::CadReferenceIndex. It owns its entries directly; its native/Value map forms are intrinsic projections of this owner, with no BTree conversion or hidden mirror.

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadReferenceIndex {
    entries: Vec<(String, CadReferenceList)>,
}

impl CadReferenceIndex {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&String, &CadReferenceList)> {
        self.entries.iter().map(|(key, rows)| (key, rows))
    }
    pub fn values(&self) -> impl ExactSizeIterator<Item = &CadReferenceList> {
        self.entries.iter().map(|(_, rows)| rows)
    }
    fn position(&self, key: &str) -> Result<usize, usize> {
        self.entries.binary_search_by(|(owned, _)| owned.as_bytes().cmp(key.as_bytes()))
    }
    pub fn get(&self, key: &str) -> Option<&CadReferenceList> {
        self.position(key).ok().map(|index| &self.entries[index].1)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut CadReferenceList> {
        self.position(key).ok().map(|index| &mut self.entries[index].1)
    }
    pub fn contains_key(&self, key: &str) -> bool { self.position(key).is_ok() }
    pub fn insert(&mut self, key: String, rows: CadReferenceList) -> Option<CadReferenceList> {
        match self.position(&key) {
            Ok(index) => Some(std::mem::replace(&mut self.entries[index].1, rows)),
            Err(index) => { self.entries.insert(index, (key, rows)); None }
        }
    }
    pub fn remove(&mut self, key: &str) -> Option<CadReferenceList> {
        self.position(key).ok().map(|index| self.entries.remove(index).1)
    }
}

impl std::ops::Index<&str> for CadReferenceIndex {
    type Output = CadReferenceList;
    fn index(&self, key: &str) -> &Self::Output { self.get(key).expect("declared CAD reference key") }
}
```

These ordinary domain APIs serve existing edit/delta/query callers. Their allocation behavior is outside a claimed controlled SQLite constructor. Controlled construction preallocates its complete typed Vec and never invokes insert or a growing ordinary collector. Native malformed duplicate input must refuse; ordinary existing edit replacement remains an explicit operation at insert. No unchecked public sorted constructor is exposed.

Both 🧬️schema/📸️snapshot/🦀️.rs:58 and 🧬️schema/🦀️.rs:49 change their field directly to CadReferenceIndex. The existing Artifact::to_snapshot clone preserves this owner, and from_snapshot/set_snapshot move the same owner. Snapshot empty at artifact root:433, Artifact builder:157 and default inference:858 use CadReferenceIndex::new. Forest inference:807 returns CadReferenceIndex and fills the four real pane entries directly. All editor inspection/document/pane reads, seven inverse readers, diff apply at text:48/172, and existing index-based semantic tests consume this owner. CadDiff's Option<BTreeMap<String,CadReferenceList>> remains a distinct sparse mutation delta; applying its entries inserts directly into the persistent index, without manufacturing a persistent BTree owner. The separate infallible take and canonical Mutation inverse Result APIs remain.

## Controlled Literal Comparison And Ordering

The constructor uses in-place heapsort with a fallible bounded comparator. It allocates no sort scratch; no standard stable sort or uninterruptible whole long-string comparison is invoked in controlled paths. Each comparison has an explicit scalar frontier, and heap swaps have row work checkpoints.

```rust
fn compare_literal(left: &str, right: &str, control: &mut NativeDecodeControl<'_>) -> Result<std::cmp::Ordering, ValueError> {
    control.scoped_stage(|control| {
        let length = left.len().min(right.len());
        control.begin_stage(length)?;
        let mut position = 0;
        while position < length {
            let end = position.saturating_add(65536).min(length);
            let order = left.as_bytes()[position..end].cmp(&right.as_bytes()[position..end]);
            control.advance(end - position)?;
            if order != std::cmp::Ordering::Equal { return Ok(order); }
            position = end;
        }
        control.checkpoint()?;
        Ok(left.len().cmp(&right.len()))
    })
}

fn sift_reference_entries(entries: &mut [(String, CadReferenceList)], mut root: usize, end: usize, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    loop {
        let Some(child) = root.checked_mul(2).and_then(|value| value.checked_add(1)).filter(|child| *child < end) else { return Ok(()); };
        let selected = if child + 1 < end && compare_literal(&entries[child].0, &entries[child + 1].0, control)? == std::cmp::Ordering::Less { child + 1 } else { child };
        if compare_literal(&entries[root].0, &entries[selected].0, control)? != std::cmp::Ordering::Less { return Ok(()); }
        entries.swap(root, selected);
        control.step()?;
        root = selected;
    }
}

fn order_reference_entries(entries: &mut [(String, CadReferenceList)], control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        for root in (0..entries.len() / 2).rev() { sift_reference_entries(entries, root, entries.len(), control)?; control.step()?; }
        for end in (1..entries.len()).rev() { entries.swap(0, end); sift_reference_entries(entries, 0, end, control)?; control.step()?; }
        for index in 1..entries.len() {
            if compare_literal(&entries[index - 1].0, &entries[index].0, control)? == std::cmp::Ordering::Equal {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "CAD reference keys must be unique"));
            }
            control.step()?;
        }
        control.checkpoint()
    })
}
```

All comparison/order controls use the actual NativeDecodeControl callback and cumulative storage ledger. No heap-byte charge is attached to inline indices, fixed four-slot presence or the CadSnapshot/CadArtifact root. The zero-length comparison still checks cancellation, including empty literal keys.

## Native And Intrinsic Value Methods

The actual DslField trait has controlled shape/from/to methods returning ValueError, while ordinary from_value returns String. CadReferenceIndex implements Shape::Map over CadReferenceList in both shape methods; the controlled boxed shape uses the existing dsl::producer::boxed producer. The controlled from_value accepts only FieldValue::Map, reserves Vec<(String,CadReferenceList)> using allocate_vec(entries.len()), copies each literal key with copy_text, binds each list through its actual controlled DslField implementation, then orders/rejects duplicates under the same control. Each completed list and partial index is held by DecodedFieldOwner with a current typed cold retirement callback. Ordinary from_value preserves the identical map grammar and rejects duplicates through a domain error; there is no blanket ValueError-to-String classifier.

Controlled to_value emits FieldValue::Map by reserving exactly the actual Vec<(String,FieldValue)> slots, copying keys and calling the actual controlled list producer. The existing native_encoding::project_map requires BTreeMap and is not used. Shape/native binding remains the ordinary handcrafted CAD map contract, with no generic list serialization or carrier. DslField::retire_decoded transfers this actual index through its RetireOwned implementation.

The actual ToValue/FromValue methods preserve DslValue::Object. Controlled encoding can use the existing intrinsic encoding::map(self.iter(),control), whose owned output is an admitted Vec<(String,DslValue)>; this is not an owning-type adapter. Controlled hydration binds the borrowed object slice into the actual admitted domain Vec and uses the same bounded ordering/duplicate check. Ordinary hydration consumes owned object entries directly. The current object path methods (shape/key/read/edit) operate on get/get_mut/insert/remove and the actual per-reference list, preserving explicit empty groups and nested edit semantics. A default controlled index checkpoints and returns the empty Vec owner, which allocates zero backing; no dummy budget buffer is installed.

## Relational Reconstruction Frontier

The actual relational helper changes from synchronous validator to validate_sqlite_database_schema_controlled. It then uses the existing SqliteSnapshotControl::allocation_stage(ReconstructSnapshot,...) with NativeDecodeControl::new(remaining,callback), returning both typed result and native.owned_bytes on every terminal path. This closes the complete operation ledger across calls and settles refusals/cancellation, retaining debt after typed retirement. max_value_bytes remains complete SQL scalar authority; max_allocation_bytes owns concrete Vec/String construction. The count*512 and inline root charges are removed only after Root's actual numerical RED.

Five borrowed row indices become admitted Vec<&SqliteRow> with in-place checked rowid ordering and uniqueness. Four model slots use inline presence [bool;4] and exact declared-slot dispatch, with bounded row checkpoints. Drawings and nodes use admitted Vec<Option<&SqliteRow>> ordinal frontiers; each unique contiguous placement is checked before its actual semantic child is constructed. The references frontier is one admitted Vec<&SqliteRow> ordered by (parent alias, ordinal); owned references are appended exactly once into admitted lists, rather than per-group BTree maps or uncharged Vec growth. Unknown parent aliases, duplicate or missing ordinals and leftover references remain typed invalidValue. The final reference group Vec is preallocated at its exact group count, keeps explicit empty groups and sorts literal keys by the controlled comparator above.

Each actual partial CadReference, model/drawing child, list, index and complete Snapshot has current typed RetireOwned. Both persistent owners and CadReference receive field-by-field retirement declarations; the direct index retirement transfers its entries into the real typed Vec cursor. Cold cleanup uses current owned_retirement and bounded close grants. These cursors allocate scaffold storage, which is explicitly qualified; retirement is not claimed allocation free. Temporary borrowed row indices have no semantic children but their real Vec backing is admitted cumulatively.

## Mount And Verification Boundary

Before any mount, re-anchor both persistent-owner files and consumers against concurrent edits. Preserve the unchanged fourteen actual native laws for the numerical baseline, all old full raw-bit/literal/ordinal/enum/cancellation comparisons and independent SQLite oracles. Native duplicate-map and long-key ordering cancellation laws are staged only after the unchanged baseline; their parser/constructor contracts must gain genuine owning RED before distinct behavior activation. The observer must see the final native/schema/output/relational temporary buffers and exact full allocator requests; parser/type verification is not a compiler or runtime claim.

## Reanchor Input Identities

- 🧬️schema/🦀️.rs: 0aab70818c0c3a5e36523d97678516432adbfaf3290300a3409fa3be8aef0abb
- 🧬️schema/📸️snapshot/🦀️.rs: c11bb3ccb51201219f28035c66f016353ef77da29588b9bb3e5c86c251cf37de
- 🧬️schema/💡️inferences/🦀️.rs: b949eed409cf0cd6eae1262116e99b59e21c60e8ba435a60ab37dc06069c9fee
- 🧬️schema/🔺️diff/📝️text/🦀️.rs: b6aaa4882d1cb92aa00a7f9e9241c80e281f6c4c3b83f34112b202ca7a54f6e2


## Current Unmounted Two-Owner Readiness

Four adjacent first-party Rust capsules and the unapplied handcrafted atomic consumer patch are preserved in 📓️cad-unmounted-concrete-two-owner-implementation-receipt.md. Registered independent syntax is GREEN 3/14 and current active Source public types are GREEN (Nx39.5s). Source50/116 prior runtime receipt remains unchanged. Both active Rust persistent fields, old numeric charges/provider selection, and all fourteen Native laws remain unchanged; Root still owns genuine compiler/numerical qualification. No Rust numerical RED or paid-owner runtime GREEN is claimed. Current shared TextDraftView publication prerequisites were already legitimately paired by concurrent authors and were preserved.

# GIS Terrain Semantic Repair Capsules

These hand-authored capsules are staged, unmounted production edits. The actual lossy mutation equality and two/three-row native admission remain unchanged until Root obtains the genuine assertion RED, alongside the current full System allocator observer. No Cargo or Native command was executed by this worker. This document is input material and must remain with the ticket.

## Mounted Regression Selection

Registered `@semio-tech/gis-gisterrain-rs:test-snapshot-sqlite-native` selects the `sqlite_snapshot` prefix. The source candidate census is sixteen: the original ten laws, including the full actual-owner System observer, plus these six separate regression laws in the actual `📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` mount:

- `sqlite_snapshot_terrain_map_mutation_preserves_zero_word_change`
- `sqlite_snapshot_terrain_map_mutation_preserves_integer_variant_change`
- `sqlite_snapshot_terrain_map_mutation_identical_nan_word_is_noop`
- `sqlite_snapshot_terrain_map_mutation_distinguishes_nan_and_optional_owner`
- `sqlite_snapshot_terrain_native_encode_counts_complete_map_rows_before_work`
- `sqlite_snapshot_terrain_native_decode_counts_complete_map_rows`

The first three invoke the actual ChangeImportedFeatures mutation diff, then apply it and compare the whole actual snapshot through ImportedMap::same. Their small map contains no unrelated NaN that could mask the zero/tag equality defect. The fourth verifies distinct NaN words and unchanged/clear/replace/present-empty boundaries. Every mutation law retains the independently authored mesh and exaggeration word. The two row laws independently project the actual normalized SQL owner and count its rows: absent map/no mesh is two, present-empty/no mesh is three, and the full fixture includes all complete map rows plus independent mesh. Both native Text and Binary must accept the exact count and refuse minus one; encode refusal must precede positive EncodeNative progress. Nextest alone can authenticate selection and assertions.

## Exact Mutation Comparison Capsule

Target: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🔺️diff/🦀️.rs`.

Replace only the current `base.imported_map == payload.new_imported_map` condition:

```rust
let identical=match(&base.imported_map,&payload.new_imported_map){
    (None,None)=>true,
    (Some(base),Some(requested))=>base.same(requested),
    _=>false,
};
if identical {
    return protocol::MutationOutcome::empty().warning("mutation.no-op", "Imported features are already identical to the requested replacement.");
}
```

This preserves raw Float words, UInt/Int tags, octets and occurrence order; it does not change the ambient PartialEq trait or any other owner.

## Complete Native Row Admission Capsule

The handwritten SQL row rule is: document + parameters + optional mesh + optional map marker; each intrinsic occurrence owns one value row and one incoming relation row; each scalar except null additionally owns exactly one branch row. Object/member and array/element relations count through the child occurrence. Names and Float companion columns are cells in those rows, never synthetic extra entities. Empty arrays/objects have no child rows. Positions/routes/regions and root property lists are typed owner collections, not implicit intrinsic Array entities.

Stage a local module `📸️snapshot/🧮️row-admission/🦀️.rs`, mounted only after the assertion RED. The following capsule borrows the actual snapshot or actual decoded native field grammar. It does not clone the map, persist any String carrier, reserve a frontier, or synthesize allocation charges. A bounded recursive read-only census has a checkpoint at every actual occurrence; numerical reconstruction/frontier repair remains separate held work.

```rust
use super::GisTerrainSnapshot;
use crate::schema::ImportedMap;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
struct Rows{count:usize,maximum:usize}
impl Rows{
    fn add(&mut self,count:usize)->Result<(),ValueError>{
        self.count=self.count.checked_add(count).filter(|count|*count<=self.maximum).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"terrain native complete owner row limit exceeded"))?;
        Ok(())
    }
    fn tree(&mut self,value:&DslValue,depth:usize,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
        checkpoint()?;
        if depth>=64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"terrain native row census exceeds intrinsic depth limit"))}
        let branch=usize::from(matches!(value,DslValue::Bool(_)|DslValue::Number(_)|DslValue::String(_)|DslValue::Bytes(_)));
        self.add(2+branch)?;
        match value{
            DslValue::Array(items)=>for value in items{self.tree(value,depth+1,checkpoint)?;},
            DslValue::Object(members)=>for(_,value)in members{self.tree(value,depth+1,checkpoint)?;},
            _=>(),
        }
        Ok(())
    }
    fn feature(&mut self,value:&DslValue,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
        if !matches!(value,DslValue::Object(_)){return Err(invalid("terrain map feature requires complete object"))}
        self.tree(value,0,checkpoint)
    }
    fn property(&mut self,name:&str,value:&DslValue,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
        if matches!(name,"positions"|"routes"|"regions"){return Err(invalid("terrain map property uses reserved collection name"))}
        self.tree(value,0,checkpoint)
    }
    fn map(&mut self,map:&ImportedMap,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
        checkpoint()?;self.add(1)?;
        for records in[&map.positions,&map.routes,&map.regions]{for value in records{self.feature(value,checkpoint)?;}}
        for property in &map.properties{self.property(&property.name,&property.value,checkpoint)?;}
        Ok(())
    }
    fn native_map(&mut self,map:&RecordValue,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
        checkpoint()?;self.add(1)?;
        if map.fields.len()!=4{return Err(invalid("terrain native map field census differs"))}
        for field in 0..3{
            let Some(FieldValue::List(values))=map.fields.get(&field)else{return Err(invalid("terrain native collection requires list"))};
            for value in values{
                let FieldValue::Value(value)=value else{return Err(invalid("terrain native collection requires intrinsic value"))};
                self.feature(value,checkpoint)?;
            }
        }
        let Some(FieldValue::List(properties))=map.fields.get(&3)else{return Err(invalid("terrain native properties require list"))};
        for property in properties{
            let FieldValue::Record(property)=property else{return Err(invalid("terrain native property requires record"))};
            if property.fields.len()!=2{return Err(invalid("terrain native property field census differs"))}
            let Some(FieldValue::Text(name))=property.fields.get(&0)else{return Err(invalid("terrain native property name requires text"))};
            let Some(FieldValue::Value(value))=property.fields.get(&1)else{return Err(invalid("terrain native property requires intrinsic value"))};
            self.property(name,value,checkpoint)?;
        }
        Ok(())
    }
}
pub(super) fn snapshot(value:&GisTerrainSnapshot,maximum:usize,mut checkpoint:impl FnMut()->Result<(),ValueError>)->Result<usize,ValueError>{
    let mut rows=Rows{count:0,maximum};checkpoint()?;rows.add(2+usize::from(value.mesh.is_some()))?;
    if let Some(map)=&value.imported_map{rows.map(map,&mut checkpoint)?;}
    Ok(rows.count)
}
pub(super) fn native(value:&RecordValue,maximum:usize,mut checkpoint:impl FnMut()->Result<(),ValueError>)->Result<usize,ValueError>{
    let mut rows=Rows{count:0,maximum};checkpoint()?;
    let mesh=match value.fields.get(&2){None|Some(FieldValue::Absent)=>false,Some(FieldValue::Record(_))=>true,_=>return Err(invalid("terrain native optional mesh record differs"))};
    rows.add(2+usize::from(mesh))?;
    match value.fields.get(&1){None|Some(FieldValue::Absent)=>(),Some(FieldValue::Record(map))=>rows.native_map(map,&mut checkpoint)?,_=>return Err(invalid("terrain native optional map record differs"))}
    Ok(rows.count)
}
```

After the measured assertion RED, mount this local module in the actual snapshot root. In SQLite native encode, retain the current schema byte check and replace the two/three-row condition with the borrowed snapshot census before calling the controlled native encoder:

```rust
let maximum_rows=control.limits().max_rows;
super::row_admission::snapshot(self,maximum_rows,||control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0))?;
```

In `📸️snapshot/📦️pack/🦀️.rs` reconstruction, replace the two/three-row condition with the borrowed record census before constructing Terrain fields:

```rust
super::row_admission::native(source,maximum_rows,||control.checkpoint())?;
```

The capsule is a reviewable semantic proposal, not a compiler, runtime or allocation receipt. FieldValue metadata is anchored on the actual current derived native ImportedMap and ImportedProperty record grammar. Actual Nextest compiler diagnostics must be resolved before claiming the mounted implementation.

## Held Allocation Work And JSON Qualification

The current full System observer must measure the unchanged unpaid native pending Vec, BTreeMap row/index ownership, relation/result Vec, recursive tree construction and temporary unsigned decimal String first. None of those allocations has been repaired or newly charged by these capsules. Later repair must use actual paid buffers/frontiers, bounded stack-safe reconstruction, immutable captured limits, cancellable literal UTF-8/octets, exact variant/single-use/dense ordinal validation, and no semantic-count allocation charges.

TypeScript JSON projection is narrower: integral Float and unsafe JavaScript integer magnitudes are explicitly refused because its JSON number projection cannot retain the native variant. Rust pack_json can emit finite integral Float decimal lexemes and complete lexical u64/i64 magnitudes. This remains an admission qualification; no stable Rust roundtrip loss was established by these regression declarations. Full typed transport and normalized SQL retain all nine domains in both implementations.

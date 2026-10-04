# Higher TextError Controlled Snapshot Refusal Floor

Actual JPG/DXF/PDF snapshot metadata and controlled projection helpers carry ValueError directly. Schema shape failures author InvalidValue. Existing under-key paths retain the same error. DXF/PDF parser-facing construction carries Result<T,TextError> inside the ValueError-owned control scope and converts only the outer control at authored span1:1, retaining inner source errors. PDF intrinsic octet ValueError constructors explicitly name schema admission and control errors pass directly. Full original source, inverse and authored text are captured before mutation. The owned shared32-case refusal corpus is already admitted in the actual TextError test-first RED; these higher real native consumers have not been compiled or run yet.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Text representation codec surface for `stdio.jpg` (snapshot).

use super::*;
use pack::value::{FromValue, ToValue};

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JpgSnapshotText = String;
//#endregion 🚚️Carrier

const FIELDS:[(u16,&str,dsl::Shape,bool);17]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"width",dsl::Shape::UInt,false),
    (3,"height",dsl::Shape::UInt,false),
    (4,"pixels",dsl::Shape::Bytes64,false),
    (5,"reEncodeQuality",dsl::Shape::Value,false),
    (6,"jfifVersion",dsl::Shape::Value,false),
    (7,"jfifDensityUnits",dsl::Shape::Value,false),
    (8,"jfifXDensity",dsl::Shape::UInt,false),
    (9,"jfifYDensity",dsl::Shape::UInt,false),
    (10,"jfifThumbnail",dsl::Shape::Value,false),
    (11,"frame",dsl::Shape::Value,false),
    (12,"sofMarker",dsl::Shape::UInt,false),
    (13,"arithmetic",dsl::Shape::Bool,false),
    (14,"quantTables",dsl::Shape::Value,false),
    (15,"huffmanTables",dsl::Shape::Value,false),
    (16,"restartInterval",dsl::Shape::Value,false),
    (17,"otherSegments",dsl::Shape::Value,false),
];
pub fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}


pub fn to_record(snapshot:&JpgSnapshot)->dsl::RecordValue {
    use dsl::FieldValue as V;
    dsl::RecordValue { fields: [
        (1,V::Text(snapshot.schema.clone())),(2,V::UInt(snapshot.width.into())),(3,V::UInt(snapshot.height.into())),(4,V::Bytes64(snapshot.pixels.clone())),
        (5,V::Value(snapshot.re_encode_quality.to_value())),(6,V::Value(snapshot.jfif_version.to_value())),(7,V::Value(snapshot.jfif_density_units.to_value())),(8,V::UInt(snapshot.jfif_x_density.into())),(9,V::UInt(snapshot.jfif_y_density.into())),
        (10,V::Value(snapshot.jfif_thumbnail.to_value())),(11,V::Value(snapshot.frame.to_value())),(12,V::UInt(snapshot.sof_marker.into())),(13,V::Bool(snapshot.arithmetic)),
        (14,V::Value(snapshot.quant_tables.to_value())),(15,V::Value(snapshot.huffman_tables.to_value())),(16,V::Value(snapshot.restart_interval.to_value())),(17,V::Value(snapshot.other_segments.to_value())),
    ].into_iter().collect() }
}

/// 🛫️ Projects the seventeen owned fields with cumulative allocation and exact stage control.
pub fn to_record_controlled(snapshot:&JpgSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,semio_framework_diagnostic::TextError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,String>{
        use dsl::FieldValue as V;
        control.begin_stage(17)?;let mut record=dsl::native_encoding::EncodedRecord::new(17,control)?;
        record.insert(1,V::Text(control.copy_text(&snapshot.schema)?));control.step()?;
        record.insert(2,V::UInt(snapshot.width.into()));control.step()?;
        record.insert(3,V::UInt(snapshot.height.into()));control.step()?;
        record.insert(4,V::Bytes64(control.copy_bytes(&snapshot.pixels)?));control.step()?;
        record.insert(5,project(&snapshot.re_encode_quality,control)?);control.step()?;
        record.insert(6,project(&snapshot.jfif_version,control)?);control.step()?;
        record.insert(7,project(&snapshot.jfif_density_units,control)?);control.step()?;
        record.insert(8,V::UInt(snapshot.jfif_x_density.into()));control.step()?;
        record.insert(9,V::UInt(snapshot.jfif_y_density.into()));control.step()?;
        record.insert(10,project(&snapshot.jfif_thumbnail,control)?);control.step()?;
        record.insert(11,project(&snapshot.frame,control)?);control.step()?;
        record.insert(12,V::UInt(snapshot.sof_marker.into()));control.step()?;
        record.insert(13,V::Bool(snapshot.arithmetic));control.step()?;
        record.insert(14,project(&snapshot.quant_tables,control)?);control.step()?;
        record.insert(15,project(&snapshot.huffman_tables,control)?);control.step()?;
        record.insert(16,project(&snapshot.restart_interval,control)?);control.step()?;
        record.insert(17,project(&snapshot.other_segments,control)?);control.step()?;
        Ok(record.take())
    })).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))
}
fn project<T:ToValue>(value:&T,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(dsl::FieldValue::Value).map_err(|error|error.to_string())})}

fn error(key:&str)->semio_framework_diagnostic::TextError { semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JPG owned snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)) }
fn value<T:FromValue>(record:&mut dsl::RecordValue,id:u16,key:&str)->Result<T,semio_framework_diagnostic::TextError> {
    match record.fields.remove(&id) { Some(dsl::FieldValue::Value(value))=>T::from_value(value).map_err(|failure|semio_framework_diagnostic::TextError::from_value_error(failure, semio_framework_diagnostic::TextSpan::at(1,1))),_=>Err(error(key)) }
}
fn unsigned<T:TryFrom<u64>>(record:&mut dsl::RecordValue,id:u16,key:&str)->Result<T,semio_framework_diagnostic::TextError> {
    match record.fields.remove(&id) { Some(dsl::FieldValue::UInt(value))=>T::try_from(value).map_err(|_|error(key)),_=>Err(error(key)) }
}

pub fn from_record(mut record:dsl::RecordValue)->Result<JpgSnapshot,semio_framework_diagnostic::TextError> {
    if record.fields.len()!=17 || record.fields.keys().any(|id|!(1..=17).contains(id)) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JPG owned snapshot requires exactly seventeen declared fields",semio_framework_diagnostic::TextSpan::at(1,1))); }
    let schema=match record.fields.remove(&1){Some(dsl::FieldValue::Text(value))=>value,_=>return Err(error("schema"))};
    let width=unsigned(&mut record,2,"width")?;let height=unsigned(&mut record,3,"height")?;
    let pixels=match record.fields.remove(&4){Some(dsl::FieldValue::Bytes64(value))=>value,_=>return Err(error("pixels"))};
    let re_encode_quality=value(&mut record,5,"reEncodeQuality")?;let jfif_version=value(&mut record,6,"jfifVersion")?;let jfif_density_units=value(&mut record,7,"jfifDensityUnits")?;
    let jfif_x_density=unsigned(&mut record,8,"jfifXDensity")?;let jfif_y_density=unsigned(&mut record,9,"jfifYDensity")?;let jfif_thumbnail=value(&mut record,10,"jfifThumbnail")?;let frame=value(&mut record,11,"frame")?;let sof_marker=unsigned(&mut record,12,"sofMarker")?;
    let arithmetic=match record.fields.remove(&13){Some(dsl::FieldValue::Bool(value))=>value,_=>return Err(error("arithmetic"))};
    let quant_tables=value(&mut record,14,"quantTables")?;let huffman_tables=value(&mut record,15,"huffmanTables")?;let restart_interval=value(&mut record,16,"restartInterval")?;let other_segments=value(&mut record,17,"otherSegments")?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
}

fn object<'a,const N:usize>(value:&'a dsl::DslValue,keys:[&str;N],control:&mut dsl::NativeDecodeControl<'_>)->Result<[&'a dsl::DslValue;N],String>{
    let dsl::DslValue::Object(entries)=value else{return Err("JPG expected an owned entity".into());};if entries.len()!=N{return Err("JPG entity field count differs from its declared schema".into());}let mut fields=[None;N];control.scoped_stage(|control|->Result<(),String>{control.begin_stage(N)?;for(key,value)in entries{control.step()?;let index=keys.iter().position(|expected|*expected==key).ok_or("JPG entity names an unknown field")?;if fields[index].replace(value).is_some(){return Err("JPG entity repeats a field".into());}}Ok(())})?;Ok(fields.map(|value|value.expect("exact complete field set")))
}
fn number<T:TryFrom<u64>>(value:&dsl::DslValue)->Result<T,String>{let dsl::DslValue::Number(value)=value else{return Err("JPG expected an integer".into());};T::try_from(value.as_u64().ok_or("JPG integer is negative or non-integral")?).map_err(|_|"JPG integer is out of its owned domain".into())}
fn octets(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Vec<u8>,String>{let dsl::DslValue::Bytes(value)=value else{return Err("JPG expected intrinsic octets".into());};control.copy_bytes(value)}
fn list<T>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>,make:impl Fn(&dsl::DslValue,&mut dsl::NativeDecodeControl<'_>)->Result<T,String>)->Result<Vec<T>,String>{let dsl::DslValue::Array(values)=value else{return Err("JPG expected an ordered list".into());};control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=control.allocate_vec::<T>(values.len())?;for value in values{control.step()?;output.push(control.scoped_stage(|control|make(value,control))?);}Ok(output)})}
fn optional<T>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>,make:impl FnOnce(&dsl::DslValue,&mut dsl::NativeDecodeControl<'_>)->Result<T,String>)->Result<Option<T>,String>{if matches!(value,dsl::DslValue::Null){Ok(None)}else{make(value,control).map(Some)}}
fn fixed<T,const N:usize>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<[T;N],String>where T:TryFrom<u64>{let dsl::DslValue::Array(values)=value else{return Err("JPG expected a fixed numeric array".into());};if values.len()!=N{return Err("JPG fixed numeric array arity mismatch".into());}list(value,control,|value,_|number(value))?.try_into().map_err(|_|"JPG fixed numeric array arity mismatch".into())}
fn thumbnail(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JfifThumbnail,String>{let[width,height,rgb]=object(value,["width","height","rgbData"],control)?;Ok(JfifThumbnail{width:number(width)?,height:number(height)?,rgb_data:octets(rgb,control)?})}
fn component(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgFrameComponent,String>{let[id,h,v,quant]=object(value,["id","hSampling","vSampling","quantTableId"],control)?;Ok(JpgFrameComponent{id:number(id)?,h_sampling:number(h)?,v_sampling:number(v)?,quant_table_id:number(quant)?})}
fn frame(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgFrameHeader,String>{let[precision,width,height,components]=object(value,["precision","width","height","components"],control)?;Ok(JpgFrameHeader{precision:number(precision)?,width:number(width)?,height:number(height)?,components:list(components,control,component)?})}
fn quantization(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgQuantTable,String>{let[id,precision,values]=object(value,["id","precision","values"],control)?;Ok(JpgQuantTable{id:number(id)?,precision:number(precision)?,values:fixed(values,control)?})}
fn huffman(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgHuffmanTable,String>{let[id,class,bits,values]=object(value,["id","class","bits","values"],control)?;let class=match class{dsl::DslValue::String(value)if value=="dc"=>JpgHuffmanClass::Dc,dsl::DslValue::String(value)if value=="ac"=>JpgHuffmanClass::Ac,_=>return Err("JPG Huffman class is outside its declared enum".into())};Ok(JpgHuffmanTable{id:number(id)?,class,bits:fixed(bits,control)?,values:octets(values,control)?})}
fn segment(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgSegment,String>{let[marker,data]=object(value,["marker","data"],control)?;Ok(JpgSegment{marker:number(marker)?,data:octets(data,control)?})}
fn field_value(record:&dsl::RecordValue,id:u16)->Result<&dsl::DslValue,String>{match record.get(id){Some(dsl::FieldValue::Value(value))=>Ok(value),_=>Err(format!("JPG root field {id} has no declared value"))}}
fn field_unsigned<T:TryFrom<u64>>(record:&dsl::RecordValue,id:u16)->Result<T,String>{match record.get(id){Some(dsl::FieldValue::UInt(value))=>T::try_from(*value).map_err(|_|format!("JPG root integer {id} is out of range")),_=>Err(format!("JPG root field {id} is not UInt"))}}
pub fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgSnapshot,semio_framework_diagnostic::TextError>{
    (||->Result<JpgSnapshot,String>{if record.fields.len()!=17||record.fields.keys().any(|id|!(1..=17).contains(id)){return Err("JPG owned snapshot requires exactly seventeen declared fields".into());}control.begin_stage(17)?;
    let schema=match record.get(1){Some(dsl::FieldValue::Text(value))=>control.copy_text(value)?,_=>return Err("JPG schema is not Text".into())};control.step()?;
    let width=field_unsigned(record,2)?;control.step()?;let height=field_unsigned(record,3)?;control.step()?;
    let pixels=match record.get(4){Some(dsl::FieldValue::Bytes64(value))=>control.copy_bytes(value)?,_=>return Err("JPG pixels are not intrinsic octets".into())};control.step()?;
    let re_encode_quality=optional(field_value(record,5)?,control,|value,_|number(value))?;control.step()?;
    let jfif_version=match control.scoped_stage(|control|fixed::<u8,2>(field_value(record,6)?,control))?{[major,minor]=>(major,minor)};control.step()?;
    let jfif_density_units=match field_value(record,7)?{dsl::DslValue::String(value)if value=="aspect"=>JfifDensityUnits::Aspect,dsl::DslValue::String(value)if value=="pixelsPerInch"=>JfifDensityUnits::PixelsPerInch,dsl::DslValue::String(value)if value=="pixelsPerCm"=>JfifDensityUnits::PixelsPerCm,_=>return Err("JPG density unit is outside its declared enum".into())};control.step()?;
    let jfif_x_density=field_unsigned(record,8)?;control.step()?;let jfif_y_density=field_unsigned(record,9)?;control.step()?;
    let jfif_thumbnail=control.scoped_stage(|control|optional(field_value(record,10)?,control,thumbnail))?;control.step()?;
    let frame=control.scoped_stage(|control|optional(field_value(record,11)?,control,frame))?;control.step()?;let sof_marker=field_unsigned(record,12)?;control.step()?;
    let arithmetic=match record.get(13){Some(dsl::FieldValue::Bool(value))=>*value,_=>return Err("JPG arithmetic is not Bool".into())};control.step()?;
    let quant_tables=control.scoped_stage(|control|list(field_value(record,14)?,control,quantization))?;control.step()?;
    let huffman_tables=control.scoped_stage(|control|list(field_value(record,15)?,control,huffman))?;control.step()?;
    let restart_interval=optional(field_value(record,16)?,control,|value,_|number(value))?;control.step()?;
    let other_segments=control.scoped_stage(|control|list(field_value(record,17)?,control,segment))?;control.step()?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
    })().map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Complete six-root owned DXF snapshot persistence.
use super::*;
use pack::value::{DslValue,FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";
const FIELDS:[(u16,&str,dsl::Shape,bool);6]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"headerVars",dsl::Shape::Value,false),
    (3,"tables",dsl::Shape::Value,false),
    (4,"otherTables",dsl::Shape::Value,false),
    (5,"blocks",dsl::Shape::Value,false),
    (6,"entities",dsl::Shape::Value,false),
];
pub(super) fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(super) fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(super) fn to_record(snapshot:&DxfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[(1,V::Text(snapshot.schema.clone())),(2,V::Value(snapshot.header_vars.to_value())),(3,V::Value(snapshot.tables.to_value())),(4,V::Value(snapshot.other_tables.to_value())),(5,V::Value(snapshot.blocks.to_value())),(6,V::Value(snapshot.entities.to_value()))].into_iter().collect()}
}

pub(super) fn to_record_controlled(snapshot:&DxfSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,semio_framework_diagnostic::TextError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,String>{
        control.begin_stage(6)?;let mut record=dsl::native_encoding::EncodedRecord::new(6,control)?;
        record.insert(1,dsl::FieldValue::Text(control.copy_text(&snapshot.schema)?));control.step()?;
        record.insert(2,project(&snapshot.header_vars,control)?);control.step()?;
        record.insert(3,project(&snapshot.tables,control)?);control.step()?;
        record.insert(4,project(&snapshot.other_tables,control)?);control.step()?;
        record.insert(5,project(&snapshot.blocks,control)?);control.step()?;
        record.insert(6,project(&snapshot.entities,control)?);control.step()?;
        Ok(record.take())
    })).map_err(|message|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1)))
}
fn project<T:ToValue>(value:&T,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(dsl::FieldValue::Value).map_err(|error|error.to_string())})}
pub(super) fn from_record(record:&dsl::RecordValue)->Result<DxfSnapshot,semio_framework_diagnostic::TextError>{
    if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DXF snapshot contains an undeclared root field",semio_framework_diagnostic::TextSpan::at(1,1)));}
    let mut fields=Vec::with_capacity(6);
    for(id,key)in[(1,"schema"),(2,"headerVars"),(3,"tables"),(4,"otherTables"),(5,"blocks"),(6,"entities")]{
        let value=match record.get(id){Some(dsl::FieldValue::Text(value))if id==1=>DslValue::String(value.clone()),Some(dsl::FieldValue::Value(value))if id>1=>value.clone(),_=>return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))};
        fields.push((key.into(),value));
    }
    DxfSnapshot::from_value(DslValue::Object(fields)).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))
}

fn owned<T:FromValue>(record:&dsl::RecordValue,id:u16,key:&str,control:&mut dsl::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,semio_framework_diagnostic::TextError>{
    let Some(dsl::FieldValue::Value(value))=record.get(id)else{return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))};
    let value=T::from_value_controlled(value,control).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.under(key), semio_framework_diagnostic::TextSpan::at(1,1)))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))?;Ok(owner)
}
pub(super) fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<DxfSnapshot,semio_framework_diagnostic::TextError>{
    let error=|message:String|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1));
    control.scoped_stage(|control|{
        control.begin_stage(6).map_err(error)?;if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(error("DXF snapshot contains an undeclared root field".into()));}
        let Some(dsl::FieldValue::Text(schema))=record.get(1)else{return Err(error("DXF snapshot schema is missing or has a different shape".into()));};let schema=control.copy_text(schema).map_err(error)?;control.step().map_err(error)?;
        let header_vars=owned::<Vec<DxfHeaderVar>>(record,2,"headerVars",control)?;let tables=owned::<DxfTables>(record,3,"tables",control)?;let other_tables=owned::<Vec<DxfOtherTable>>(record,4,"otherTables",control)?;let blocks=owned::<Vec<DxfBlock>>(record,5,"blocks",control)?;let entities=owned::<Vec<DxfEntity>>(record,6,"entities",control)?;
        Ok(DxfSnapshot{schema,header_vars:header_vars.take(),tables:tables.take(),other_tables:other_tables.take(),blocks:blocks.take(),entities:entities.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📝️ Explicit complete owned snapshot record fields and canonical tagged semantic values.
use super::*;
use pack::value::{ToValue,FromValue,DslValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";

const FIELDS:[(u16,&str,dsl::Shape,bool);31]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"declaredVersion",dsl::Shape::Text,false),
    (3,"pages",dsl::Shape::Value,false),
    (4,"fonts",dsl::Shape::Value,false),
    (5,"images",dsl::Shape::Value,false),
    (6,"forms",dsl::Shape::Value,false),
    (7,"extGStates",dsl::Shape::Value,false),
    (8,"shadings",dsl::Shape::Value,false),
    (9,"patterns",dsl::Shape::Value,false),
    (10,"colorSpaces",dsl::Shape::Value,false),
    (11,"properties",dsl::Shape::Value,false),
    (12,"outlines",dsl::Shape::Value,false),
    (13,"namedDestinations",dsl::Shape::Value,false),
    (14,"pageLabels",dsl::Shape::Value,false),
    (15,"embeddedFiles",dsl::Shape::Value,false),
    (16,"outputIntents",dsl::Shape::Value,false),
    (17,"acroForm",dsl::Shape::Value,true),
    (18,"optionalContent",dsl::Shape::Value,true),
    (19,"pageLayout",dsl::Shape::Value,true),
    (20,"pageMode",dsl::Shape::Value,true),
    (21,"viewerPreferences",dsl::Shape::Value,true),
    (22,"openAction",dsl::Shape::Value,true),
    (23,"language",dsl::Shape::Value,true),
    (24,"markInfo",dsl::Shape::Value,true),
    (25,"metadata",dsl::Shape::Value,true),
    (26,"documentId",dsl::Shape::Value,true),
    (27,"encryption",dsl::Shape::Value,true),
    (28,"info",dsl::Shape::Value,false),
    (29,"catalogExtra",dsl::Shape::Value,false),
    (30,"objects",dsl::Shape::Value,false),
    (31,"trailer",dsl::Shape::Value,false),
];
pub(super) fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(super) fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(super) fn to_record(value:&PdfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[
        (1,V::Text(value.schema.clone())),(2,V::Text(value.declared_version.clone())),
        (3,V::Value(value.pages.to_value())),(4,V::Value(value.fonts.to_value())),(5,V::Value(value.images.to_value())),(6,V::Value(value.forms.to_value())),(7,V::Value(value.ext_g_states.to_value())),(8,V::Value(value.shadings.to_value())),(9,V::Value(value.patterns.to_value())),(10,V::Value(value.color_spaces.to_value())),(11,V::Value(value.properties.to_value())),(12,V::Value(value.outlines.to_value())),(13,V::Value(value.named_destinations.to_value())),(14,V::Value(value.page_labels.to_value())),(15,V::Value(value.embedded_files.to_value())),(16,V::Value(value.output_intents.to_value())),
        (17,V::Value(value.acro_form.to_value())),(18,V::Value(value.optional_content.to_value())),(19,V::Value(value.page_layout.to_value())),(20,V::Value(value.page_mode.to_value())),(21,V::Value(value.viewer_preferences.to_value())),(22,V::Value(value.open_action.to_value())),(23,V::Value(value.language.to_value())),(24,V::Value(value.mark_info.to_value())),(25,V::Value(value.metadata.to_value())),(26,V::Value(crate::standards::v1_7::subsets::base::schema::snapshot::octets::document_id_to_value(&value.document_id))),(27,V::Value(value.encryption.to_value())),
        (28,V::Value(value.info.to_value())),(29,V::Value(value.catalog_extra.to_value())),(30,V::Value(value.objects.to_value())),(31,V::Value(value.trailer.to_value())),
    ].into_iter().collect()}
}
/// 🛫️ Projects each literal root slot with bounded allocation and partial-record retirement.
pub(super) fn to_record_controlled(value:&PdfSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,semio_framework_diagnostic::TextError>{
    let error=|message:String|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1));
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<dsl::RecordValue,String>{
        control.begin_stage(FIELDS.len())?;
        let mut record=dsl::os_dsl::native_encoding::EncodedRecord::new(FIELDS.len(),control)?;
        record.insert(1,dsl::FieldValue::Text(control.copy_text(&value.schema)?));control.step()?;
        record.insert(2,dsl::FieldValue::Text(control.copy_text(&value.declared_version)?));control.step()?;
        macro_rules! field{($id:literal,$key:literal,$output:expr)=>{{let output=$output.map_err(|failure|failure.under($key).to_string())?;record.insert($id,dsl::FieldValue::Value(output));control.step()?;}};}
        field!(3,"pages",value.pages.to_value_controlled(control));
        field!(4,"fonts",value.fonts.to_value_controlled(control));
        field!(5,"images",value.images.to_value_controlled(control));
        field!(6,"forms",value.forms.to_value_controlled(control));
        field!(7,"extGStates",value.ext_g_states.to_value_controlled(control));
        field!(8,"shadings",value.shadings.to_value_controlled(control));
        field!(9,"patterns",value.patterns.to_value_controlled(control));
        field!(10,"colorSpaces",value.color_spaces.to_value_controlled(control));
        field!(11,"properties",value.properties.to_value_controlled(control));
        field!(12,"outlines",value.outlines.to_value_controlled(control));
        field!(13,"namedDestinations",value.named_destinations.to_value_controlled(control));
        field!(14,"pageLabels",value.page_labels.to_value_controlled(control));
        field!(15,"embeddedFiles",value.embedded_files.to_value_controlled(control));
        field!(16,"outputIntents",value.output_intents.to_value_controlled(control));
        field!(17,"acroForm",value.acro_form.to_value_controlled(control));
        field!(18,"optionalContent",value.optional_content.to_value_controlled(control));
        field!(19,"pageLayout",value.page_layout.to_value_controlled(control));
        field!(20,"pageMode",value.page_mode.to_value_controlled(control));
        field!(21,"viewerPreferences",value.viewer_preferences.to_value_controlled(control));
        field!(22,"openAction",value.open_action.to_value_controlled(control));
        field!(23,"language",value.language.to_value_controlled(control));
        field!(24,"markInfo",value.mark_info.to_value_controlled(control));
        field!(25,"metadata",value.metadata.to_value_controlled(control));
        field!(26,"documentId",octets::document_id_to_value_controlled(&value.document_id,control));
        field!(27,"encryption",value.encryption.to_value_controlled(control));
        field!(28,"info",value.info.to_value_controlled(control));
        field!(29,"catalogExtra",value.catalog_extra.to_value_controlled(control));
        field!(30,"objects",value.objects.to_value_controlled(control));
        field!(31,"trailer",value.trailer.to_value_controlled(control));
        Ok(record.take())
    })).map_err(error)
}
fn field(record:&dsl::RecordValue,id:u16,key:&str)->Result<DslValue,semio_framework_diagnostic::TextError>{match record.get(id){Some(dsl::FieldValue::Text(value))if id<=2=>Ok(DslValue::String(value.clone())),Some(dsl::FieldValue::Value(value))if id>2=>Ok(value.clone()),None|Some(dsl::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(DslValue::Null),_=>Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("PDF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))}}
pub(super) fn from_record(record:&dsl::RecordValue)->Result<PdfSnapshot,semio_framework_diagnostic::TextError>{
    if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PDF snapshot contains an undeclared root field",semio_framework_diagnostic::TextSpan::at(1,1)));}
    PdfSnapshot::from_value(DslValue::object([
        ("schema".into(),field(record,1,"schema")?),("declaredVersion".into(),field(record,2,"declaredVersion")?),
        ("pages".into(),field(record,3,"pages")?),("fonts".into(),field(record,4,"fonts")?),("images".into(),field(record,5,"images")?),("forms".into(),field(record,6,"forms")?),("extGStates".into(),field(record,7,"extGStates")?),("shadings".into(),field(record,8,"shadings")?),("patterns".into(),field(record,9,"patterns")?),("colorSpaces".into(),field(record,10,"colorSpaces")?),("properties".into(),field(record,11,"properties")?),("outlines".into(),field(record,12,"outlines")?),("namedDestinations".into(),field(record,13,"namedDestinations")?),("pageLabels".into(),field(record,14,"pageLabels")?),("embeddedFiles".into(),field(record,15,"embeddedFiles")?),("outputIntents".into(),field(record,16,"outputIntents")?),
        ("acroForm".into(),field(record,17,"acroForm")?),("optionalContent".into(),field(record,18,"optionalContent")?),("pageLayout".into(),field(record,19,"pageLayout")?),("pageMode".into(),field(record,20,"pageMode")?),("viewerPreferences".into(),field(record,21,"viewerPreferences")?),("openAction".into(),field(record,22,"openAction")?),("language".into(),field(record,23,"language")?),("markInfo".into(),field(record,24,"markInfo")?),("metadata".into(),field(record,25,"metadata")?),("documentId".into(),field(record,26,"documentId")?),("encryption".into(),field(record,27,"encryption")?),
        ("info".into(),field(record,28,"info")?),("catalogExtra".into(),field(record,29,"catalogExtra")?),("objects".into(),field(record,30,"objects")?),("trailer".into(),field(record,31,"trailer")?),
    ])).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))
}

fn borrowed_field<'a>(record:&'a dsl::RecordValue,id:u16,key:&str)->Result<&'a DslValue,semio_framework_diagnostic::TextError>{match record.get(id){Some(dsl::FieldValue::Value(value))=>Ok(value),None|Some(dsl::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(&DslValue::Null),_=>Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("PDF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))}}
fn owned<T:FromValue>(record:&dsl::RecordValue,id:u16,key:&str,control:&mut dsl::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,semio_framework_diagnostic::TextError>{let value=T::from_value_controlled(borrowed_field(record,id,key)?,control).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.under(key), semio_framework_diagnostic::TextSpan::at(1,1)))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))?;Ok(owner)}
pub(super)fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<PdfSnapshot,semio_framework_diagnostic::TextError>{
    let error=|message:String|semio_framework_diagnostic::TextError::new(message,semio_framework_diagnostic::TextSpan::at(1,1));
    control.scoped_stage(|control|{
        control.begin_stage(31).map_err(error)?;if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(error("PDF snapshot contains an undeclared root field".into()));}
        let text=|id,key:&str,control:&mut dsl::NativeDecodeControl<'_>|{let Some(dsl::FieldValue::Text(value))=record.get(id)else{return Err(error(format!("PDF snapshot field {key} is missing or has a different shape")))};let value=control.copy_text(value).map_err(error)?;control.step().map_err(error)?;Ok::<_,semio_framework_diagnostic::TextError>(value)};
        let schema=text(1,"schema",control)?;let declared_version=text(2,"declaredVersion",control)?;
        let pages=owned::<Vec<PdfPage>>(record,3,"pages",control)?;
        let fonts=owned::<Vec<PdfFont>>(record,4,"fonts",control)?;
        let images=owned::<Vec<PdfImage>>(record,5,"images",control)?;
        let forms=owned::<Vec<PdfFormXObject>>(record,6,"forms",control)?;
        let ext_g_states=owned::<Vec<PdfExtGState>>(record,7,"extGStates",control)?;
        let shadings=owned::<Vec<PdfShading>>(record,8,"shadings",control)?;
        let patterns=owned::<Vec<PdfPattern>>(record,9,"patterns",control)?;
        let color_spaces=owned::<Vec<PdfNamedColorSpace>>(record,10,"colorSpaces",control)?;
        let properties=owned::<Vec<PdfNamedProperties>>(record,11,"properties",control)?;
        let outlines=owned::<Vec<PdfOutlineItem>>(record,12,"outlines",control)?;
        let named_destinations=owned::<Vec<PdfNamedDestination>>(record,13,"namedDestinations",control)?;
        let page_labels=owned::<Vec<PdfPageLabelRange>>(record,14,"pageLabels",control)?;
        let embedded_files=owned::<Vec<PdfEmbeddedFile>>(record,15,"embeddedFiles",control)?;
        let output_intents=owned::<Vec<PdfOutputIntent>>(record,16,"outputIntents",control)?;
        let acro_form=owned::<Option<PdfAcroForm>>(record,17,"acroForm",control)?;
        let optional_content=owned::<Option<PdfOptionalContent>>(record,18,"optionalContent",control)?;
        let page_layout=owned::<Option<PdfPageLayout>>(record,19,"pageLayout",control)?;
        let page_mode=owned::<Option<PdfPageMode>>(record,20,"pageMode",control)?;
        let viewer_preferences=owned::<Option<PdfViewerPreferences>>(record,21,"viewerPreferences",control)?;
        let open_action=owned::<Option<PdfOpenAction>>(record,22,"openAction",control)?;
        let language=owned::<Option<String>>(record,23,"language",control)?;
        let mark_info=owned::<Option<PdfMarkInfo>>(record,24,"markInfo",control)?;
        let metadata=owned::<Option<String>>(record,25,"metadata",control)?;
        let document_id=octets::document_id_from_value_controlled(borrowed_field(record,26,"documentId")?,control).map_err(|value|error(value.under("documentId").to_string()))?;control.step().map_err(error)?;
        let encryption=owned::<Option<PdfEncryption>>(record,27,"encryption",control)?;
        let info=owned::<PdfInfo>(record,28,"info",control)?;
        let catalog_extra=owned::<Vec<PdfDictEntry>>(record,29,"catalogExtra",control)?;
        let objects=owned::<Vec<PdfIndirectObject>>(record,30,"objects",control)?;
        let trailer=owned::<Vec<PdfDictEntry>>(record,31,"trailer",control)?;
        Ok(PdfSnapshot{schema,declared_version,pages:pages.take(),fonts:fonts.take(),images:images.take(),forms:forms.take(),ext_g_states:ext_g_states.take(),shadings:shadings.take(),patterns:patterns.take(),color_spaces:color_spaces.take(),properties:properties.take(),outlines:outlines.take(),named_destinations:named_destinations.take(),page_labels:page_labels.take(),embedded_files:embedded_files.take(),output_intents:output_intents.take(),acro_form:acro_form.take(),optional_content:optional_content.take(),page_layout:page_layout.take(),page_mode:page_mode.take(),viewer_preferences:viewer_preferences.take(),open_action:open_action.take(),language:language.take(),mark_info:mark_info.take(),metadata:metadata.take(),document_id,encryption:encryption.take(),info:info.take(),catalog_extra:catalog_extra.take(),objects:objects.take(),trailer:trailer.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🦀️.rs

```rust
//! 🧬️ Explicit COS intrinsic byte payloads preserve their owner-tagged logical shape.
use super::*;
use pack::value::{DslValue as V,ToValue,FromValue,ValueError as E};
impl ToValue for PdfObject{
    fn to_value(&self)->V{
        let(tag,payload)=match self{
            Self::Null=>return V::Object(vec![("kind".into(),V::String("null".into()))]),
            Self::Bool(value)=>("bool",value.to_value()),Self::Int(value)=>("int",value.to_value()),Self::Real(value)=>("real",value.to_value()),
            Self::Str(bytes)=>("str",pack::value::bytes::to_value(bytes)),Self::Name(value)=>("name",value.to_value()),Self::Array(value)=>("array",value.to_value()),Self::Dict(value)=>("dict",value.to_value()),Self::Ref(value)=>("ref",value.to_value()),
            Self::Stream{dict,data,filters}=>return V::Object(vec![("kind".into(),V::String("stream".into())),("dict".into(),dict.to_value()),("data".into(),pack::value::bytes::to_value(data)),("filters".into(),filters.to_value())]),
        };
        let mut fields=vec![("kind".into(),V::String(tag.into()))];match payload{V::Object(values)=>fields.extend(values),value=>fields.push(("value".into(),value))};V::Object(fields)
    }
    fn to_value_controlled(&self,control:&mut pack::value::NativeEncodeControl<'_>)->Result<V,E>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            let(tag,count)=match self{Self::Null=>("null",1),Self::Real(_)=>("real",4),Self::Ref(_)=>("ref",3),Self::Stream{..}=>("stream",4),Self::Bool(_)=>("bool",2),Self::Int(_)=>("int",2),Self::Str(_)=>("str",2),Self::Name(_)=>("name",2),Self::Array(_)=>("array",2),Self::Dict(_)=>("dict",2)};
            control.begin_stage(count).map_err(E::new)?;let mut fields=V::object_encoding_controlled(count,control)?;cos_output(fields.get_mut(),"kind",&tag,control)?;
            match self{
                Self::Null=>{},
                Self::Bool(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Int(value)=>cos_output(fields.get_mut(),"value",value,control)?,
                Self::Name(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Array(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Dict(value)=>cos_output(fields.get_mut(),"value",value,control)?,
                Self::Str(bytes)=>cos_output_bytes(fields.get_mut(),"value",bytes,control)?,
                Self::Real(value)=>{cos_output(fields.get_mut(),"negative",&value.negative,control)?;cos_output(fields.get_mut(),"coefficient",&value.coefficient,control)?;cos_output(fields.get_mut(),"scale",&value.scale,control)?;},
                Self::Ref(value)=>{cos_output(fields.get_mut(),"num",&value.num,control)?;cos_output(fields.get_mut(),"gen",&value.gen,control)?;},
                Self::Stream{dict,data,filters}=>{cos_output(fields.get_mut(),"dict",dict,control)?;cos_output_bytes(fields.get_mut(),"data",data,control)?;cos_output(fields.get_mut(),"filters",filters,control)?;},
            }
            Ok(V::Object(fields.take()))
        }))
    }

}
impl FromValue for PdfObject{
    fn from_value(value:V)->Result<Self,E>{
        let fields=value.into_object()?;let tag=fields.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str()).ok_or_else(||E::new("PDF COS object requires a kind"))?;
        let get=|key:&str|fields.iter().find(|(name,_)|name==key).map(|(_,value)|value.clone()).ok_or_else(||E::new(format!("PDF COS object requires {key}")));
        if tag=="stream"{if fields.iter().any(|(key,_)|!matches!(key.as_str(),"kind"|"dict"|"data"|"filters")){return Err(E::new("PDF COS stream has an undeclared field"));}return Ok(Self::Stream{dict:Vec::<PdfDictEntry>::from_value(get("dict")?)?,data:pack::value::bytes::from_value(get("data")?)?,filters:Vec::<PdfStreamFilter>::from_value(get("filters")?)?});}
        let payload=||get("value");
        Ok(match tag{
            "null"=>Self::Null,"bool"=>Self::Bool(bool::from_value(payload()?)?),"int"=>Self::Int(i64::from_value(payload()?)?),"real"=>Self::Real(PdfDecimal::from_value(V::Object(fields.iter().filter(|(key,_)|key!="kind").cloned().collect()))?),
            "str"=>Self::Str(pack::value::bytes::from_value(payload()?)?),"name"=>Self::Name(String::from_value(payload()?)?),"array"=>Self::Array(Vec::<PdfObject>::from_value(payload()?)?),"dict"=>Self::Dict(Vec::<PdfDictEntry>::from_value(payload()?)?),
            "ref"=>Self::Ref(ObjRef::from_value(V::Object(fields.iter().filter(|(key,_)|key!="kind").cloned().collect()))?),
            _=>return Err(E::new("unknown PDF COS object kind")),
        })
    }
    fn from_value_controlled(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Self,E>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            let V::Object(fields)=value else{return Err(E::new("PDF COS object requires a tagged object"))};
            let tag=fields.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str()).ok_or_else(||E::new("PDF COS object requires a kind"))?;
            let keys:&[&str]=match tag{"null"=>&["kind"],"real"=>&["kind","negative","coefficient","scale"],"ref"=>&["kind","num","gen"],"stream"=>&["kind","dict","data","filters"],"bool"|"int"|"str"|"name"|"array"|"dict"=>&["kind","value"],_=>return Err(E::new("unknown PDF COS object kind"))};
            if fields.len()!=keys.len()||keys.iter().any(|key|fields.iter().filter(|(name,_)|name==key).count()!=1){return Err(E::new("PDF COS object has missing, duplicate or undeclared fields"))}
            control.begin_stage(keys.len()).map_err(E::new)?;control.step().map_err(E::new)?;
            let get=|key:&str|fields.iter().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||E::new(format!("PDF COS object requires {key}")));
            Ok(match tag{
                "null"=>Self::Null,
                "bool"=>Self::Bool(cos_owned::<bool>(get("value")?,control)?.take()),
                "int"=>Self::Int(cos_owned::<i64>(get("value")?,control)?.take()),
                "str"=>{let data=cos_bytes(get("value")?,control)?;control.step().map_err(E::new)?;Self::Str(data)},
                "name"=>Self::Name(cos_owned::<String>(get("value")?,control)?.take()),
                "array"=>Self::Array(cos_owned::<Vec<Self>>(get("value")?,control)?.take()),
                "dict"=>Self::Dict(cos_owned::<Vec<PdfDictEntry>>(get("value")?,control)?.take()),
                "real"=>{let negative=cos_owned::<bool>(get("negative")?,control)?;let coefficient=cos_owned::<String>(get("coefficient")?,control)?;let scale=cos_owned::<u32>(get("scale")?,control)?;Self::Real(PdfDecimal{negative:negative.take(),coefficient:coefficient.take(),scale:scale.take()})},
                "ref"=>{let num=cos_owned::<u32>(get("num")?,control)?;let gen=cos_owned::<u16>(get("gen")?,control)?;Self::Ref(ObjRef{num:num.take(),gen:gen.take()})},
                "stream"=>{let dict=cos_owned::<Vec<PdfDictEntry>>(get("dict")?,control)?;let data=cos_bytes(get("data")?,control)?;control.step().map_err(E::new)?;let filters=cos_owned::<Vec<PdfStreamFilter>>(get("filters")?,control)?;Self::Stream{dict:dict.take(),data,filters:filters.take()}},
                _=>unreachable!(),
            })
        }))
    }
    fn retire_decoded(self){
        let mut pending=vec![self];while let Some(value)=pending.pop(){match value{
            Self::Array(mut values)=>pending.append(&mut values),Self::Dict(values)|Self::Stream{dict:values,..}=>pending.extend(values.into_iter().map(|entry|entry.value)),
            Self::Null|Self::Bool(_)|Self::Int(_)|Self::Real(_)|Self::Str(_)|Self::Name(_)|Self::Ref(_)=>{},
        }}
    }
    fn edit_value_at_path(&mut self,path:&[&str],edit:pack::value::ValueEdit)->Result<(),E>{pack::value::edit_through_value(self,path,edit)}
}
/// 🪪️ The document ID is exactly two independently owned intrinsic octet strings.
pub(crate)fn document_id_to_value(value:&Option<[Vec<u8>;2]>)->V{value.as_ref().map_or(V::Null,|value|V::Array(vec![pack::value::bytes::to_value(&value[0]),pack::value::bytes::to_value(&value[1])]))}
pub(crate)fn document_id_from_value(value:V)->Result<Option<[Vec<u8>;2]>,E>{match value{V::Null=>Ok(None),V::Array(mut values)if values.len()==2=>{let right=pack::value::bytes::from_value(values.pop().unwrap())?;let left=pack::value::bytes::from_value(values.pop().unwrap())?;Ok(Some([left,right]))},_=>Err(E::new("PDF document ID requires exactly two intrinsic octet strings"))}}

fn cos_owned<T:FromValue>(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,E>{let value=T::from_value_controlled(value,control)?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(E::new)?;Ok(owner)}
fn cos_bytes(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Vec<u8>,E>{match value{V::Bytes(bytes)=>control.copy_bytes(bytes).map_err(E::new),V::Array(_)=>Vec::<u8>::from_value_controlled(value,control),_=>Err(E::new("PDF intrinsic octet string requires bytes or canonical octets"))}}
/// 🪪️ Borrowed document ID admission copies its two explicit octet fields under one control.
pub(crate)fn document_id_from_value_controlled(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Option<[Vec<u8>;2]>,E>{control.scoped_stage(|control|match value{V::Null=>Ok(None),V::Array(values)if values.len()==2=>{control.begin_stage(2).map_err(E::new)?;let left=cos_bytes(&values[0],control)?;control.step().map_err(E::new)?;let right=cos_bytes(&values[1],control)?;control.step().map_err(E::new)?;Ok(Some([left,right]))},_=>Err(E::new("PDF document ID requires exactly two intrinsic octet strings"))})}

fn cos_output<T:ToValue+?Sized>(fields:&mut Vec<(String,V)>,key:&str,value:&T,control:&mut pack::value::NativeEncodeControl<'_>)->Result<(),E>{
    let key_owned=control.copy_text(key).map_err(E::new)?;let value=value.to_value_controlled(control).map_err(|error|error.under(key))?;fields.push((key_owned,value));control.step().map_err(E::new)
}
fn cos_output_bytes(fields:&mut Vec<(String,V)>,key:&str,bytes:&[u8],control:&mut pack::value::NativeEncodeControl<'_>)->Result<(),E>{
    let key_owned=control.copy_text(key).map_err(E::new)?;let value=pack::value::bytes::to_value_controlled(bytes,control).map_err(|error|error.under(key))?;fields.push((key_owned,value));control.step().map_err(E::new)
}
/// 🪪️ Projects the two intrinsic document-ID octets through one admitted native output frontier.
pub(crate)fn document_id_to_value_controlled(value:&Option<[Vec<u8>;2]>,control:&mut pack::value::NativeEncodeControl<'_>)->Result<V,E>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|match value{
        None=>().to_value_controlled(control),Some(values)=>{control.begin_stage(2).map_err(E::new)?;let mut output=Vec::<V>::guard_decoded(control.allocate_vec(2).map_err(E::new)?);for(index,value)in values.iter().enumerate(){output.get_mut().push(pack::value::bytes::to_value_controlled(value,control).map_err(|error|error.under(index))?);control.step().map_err(E::new)?;}Ok(V::Array(output.take()))}
    }))
}

#[cfg(test)]
#[path="🧪️tests/🛫️output/🦀️.rs"]
mod output_tests;

```


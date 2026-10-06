//! 🕸️ Writes rich graph fields directly into the admitted native output buffer.
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot,SemioGraphNode,SemioGraphPort,SemioGraphPortKind,SemioGraphEdge};
use crate::standards::v1::subsets::{base::io::sqlite::snapshot::native_encoding::{self,Writer},value::io::sqlite::snapshot::native_encoding as values};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,ValueError};
pub(crate) fn encode(snapshot:&SemioGraphSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{native_encoding::encode_admitted(encoding,"semio s.stdio.semio.graph.dsl v1\n","s.stdio.semio.graph.pack v1",control,|control|crate::standards::v1::subsets::graph::io::sqlite::snapshot::admit_values(snapshot,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,control),|writer,encoding|fields(snapshot,writer,encoding))}
fn optional(value:Option<&str>,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{
 match(value,encoding){(None,SnapshotEncoding::Binary)=>writer.byte(0),(None,SnapshotEncoding::Text)=>writer.bytes(b"-"),(Some(value),SnapshotEncoding::Binary)=>{writer.byte(1)?;writer.string(value,encoding)},(Some(value),SnapshotEncoding::Text)=>{writer.bytes(b"[")?;writer.string(value,encoding)?;writer.bytes(b"]")}}
}
fn port(port:&SemioGraphPort,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{
 writer.delimiter(b"[",encoding)?;writer.string(&port.name,encoding)?;writer.delimiter(b",",encoding)?;writer.byte(if encoding==SnapshotEncoding::Binary{match port.kind{SemioGraphPortKind::In=>0,SemioGraphPortKind::Out=>1,SemioGraphPortKind::InOut=>2}}else{crate::standards::v1::subsets::graph::io::text::snapshot::enc_port_kind(port.kind) as u8})?;
 writer.delimiter(b",",encoding)?;writer.string(&port.category,encoding)?;writer.delimiter(b",",encoding)?;writer.list(&port.properties,encoding,|writer,item|values::entry(item,writer,encoding,0))?;writer.delimiter(b"]",encoding)
}
fn node(node:&SemioGraphNode,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{
 writer.delimiter(b"[",encoding)?;for field in [&node.id.value,&node.kind,&node.label]{writer.string(field,encoding)?;writer.delimiter(b",",encoding)?;}for value in [node.position.x,node.position.y,node.width,node.height]{writer.hex_number(value,encoding)?;writer.delimiter(b",",encoding)?;}
 writer.list(&node.ports,encoding,|writer,item|port(item,writer,encoding))?;writer.delimiter(b",",encoding)?;writer.list(&node.properties,encoding,|writer,item|values::entry(item,writer,encoding,0))?;writer.delimiter(b"]",encoding)
}
fn edge(edge:&SemioGraphEdge,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{
 writer.delimiter(b"[",encoding)?;for field in [&edge.id.value,&edge.source.value,&edge.target.value,&edge.kind,&edge.label]{writer.string(field,encoding)?;writer.delimiter(b",",encoding)?;}optional(edge.source_port.as_deref(),writer,encoding)?;writer.delimiter(b",",encoding)?;optional(edge.target_port.as_deref(),writer,encoding)?;writer.delimiter(b",",encoding)?;writer.list(&edge.properties,encoding,|writer,item|values::entry(item,writer,encoding,0))?;writer.delimiter(b"]",encoding)
}
pub(crate) fn fields(snapshot:&SemioGraphSnapshot,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{writer.entities(1)?;if encoding==SnapshotEncoding::Binary{writer.byte(1)?}else{writer.bytes(b"schema=")?}writer.string(&snapshot.schema,encoding)?;writer.delimiter(b"\nnodes=",encoding)?;writer.list(&snapshot.nodes,encoding,|writer,item|node(item,writer,encoding))?;writer.delimiter(b"\nedges=",encoding)?;writer.list(&snapshot.edges,encoding,|writer,item|edge(item,writer,encoding))}

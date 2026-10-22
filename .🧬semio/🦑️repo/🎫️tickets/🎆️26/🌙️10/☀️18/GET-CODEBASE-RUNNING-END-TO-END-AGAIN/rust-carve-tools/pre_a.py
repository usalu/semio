import sys,os,re,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import *
E=F+"/🔨️modules/🎒️pack/🌱️value/🛫️encode"
B=E+"/🫳️borrowed"
CUR=B+"/⏳️cursor"
pkg=CUR+"/📦️packages/🦀️rust"
P=lambda n,extra="":pathdep(pkg,n,extra)
deps=[P("semio-framework-value"),P("semio-framework-dsl-record"),P("semio-framework-value-paged-text"),P("semio-framework-value-persistent"),P("semio-framework-hash"),P("semio-framework-pack-format"),P("semio-framework-pack-record-preflight"),P("semio-framework-replication-codec"),P("semio-framework-pack-record-schema")]
dev=[P("semio-framework-pack-record-encode"),P("semio-framework-pack-error"),P("semio-framework-dsl-record-derive"),P("semio-framework-trace"),P("semio-framework-trace-memory-testing"),'blake3 = "1.8.2"','serde_json = { version = "1.0.140", features = ["float_roundtrip"] }']
newcrate(CUR,"semio-framework-pack-record-encode-cursor","pack-record-encode-cursor","Retained borrowed Pack cursors: canonical record symbols and ordinal emission, retained document frames and owned wire emission","../../🦀️.rs",deps,dev,features='default = ["deflate"]\ndeflate = ["semio-framework-replication-codec/deflate"]\n',workspace=F)
# cursor root imports + mounts + allocator
p=CUR+"/🦀️.rs"
sub(p,"use super::*;\nuse semio_framework_dsl_record::{BorrowedRecordSpec, BorrowedShape as B};\n",
"""use semio_framework_value::{Number, ValueError, ValueRefusalKind};
use std::cmp::Ordering;
use std::mem::size_of;
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_FALLBACK, ELEM_INT, ELEM_STR, ELEM_UINT, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE};
#[cfg(test)]
use semio_framework_value::{DslValue, native_encoding::NativeEncodeControl};
use semio_framework_dsl_record::{BorrowedRecordSpec, BorrowedShape as B};
""")
s=get(p)
anchor="const SYMBOL_LIMIT: usize = isize::MAX as usize;\n"
assert s.count(anchor)==1
texts[p]=s.replace(anchor,'''#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

//#region 🎒️BorrowedDocumentCursor
#[cfg(feature = "deflate")]
#[path = "../🎒️document/⏳️cursor/🦀️.rs"]
mod borrowed_document;
#[cfg(feature = "deflate")]
pub use borrowed_document::{BorrowedPackDocumentCursor, BorrowedPackDocumentFailure, BorrowedPackDocumentProgress};
//#endregion 🎒️BorrowedDocumentCursor

//#region 📨️OwnedWireCursor
#[path = "../📨️wire/🦀️.rs"]
mod owned_wire;
pub use owned_wire::{OwnedProjectedPackWireCursor, OwnedProjectedPackWireReady, OwnedProjectedPackWireStep};
//#endregion 📨️OwnedWireCursor

'''+anchor)
# L1 root: unmount
r=E+"/🦀️.rs"
sub(r,'''#[path="🫳️borrowed/⏳️cursor/🦀️.rs"]
mod borrowed_cursor;
pub use borrowed_cursor::{BorrowedProjectedPackCursor,BorrowedProjectedPackProgress,BorrowedProjectedPackFailure,BorrowedPackSection};
#[cfg(feature="deflate")]
#[path="🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs"]
mod borrowed_document;
#[cfg(feature="deflate")]
pub use borrowed_document::{BorrowedPackDocumentCursor,BorrowedPackDocumentProgress,BorrowedPackDocumentFailure};
#[path="🫳️borrowed/📨️wire/🦀️.rs"]
mod owned_wire;
pub use owned_wire::{OwnedProjectedPackWireCursor,OwnedProjectedPackWireReady,OwnedProjectedPackWireStep};
''','')
# document tests: crate:: -> L1
sub(B+"/🎒️document/⏳️cursor/🧪️tests/🦀️.rs"," crate::encode_document_controlled("," semio_framework_pack_record_encode::encode_document_controlled(")
# pack aggregator
PK=F+"/🔨️modules/🎒️pack/📦️packages/🦀️rust"
s=get(PK+"/🦀️.rs")
old73=[l for l in s.split("\n") if l.startswith("    pub use semio_framework_pack_record_encode::{BorrowedPackSource")][0]
new73=old73
for n in ["BorrowedProjectedPackCursor, ","BorrowedProjectedPackFailure, ","BorrowedProjectedPackProgress, ","OwnedProjectedPackWireCursor, ","OwnedProjectedPackWireReady, ","OwnedProjectedPackWireStep, "]:
    assert n in new73,n
    new73=new73.replace(n,"")
sub(PK+"/🦀️.rs",old73,new73+"\n    pub use semio_framework_pack_record_encode_cursor::{BorrowedProjectedPackCursor, BorrowedProjectedPackFailure, BorrowedProjectedPackProgress, OwnedProjectedPackWireCursor, OwnedProjectedPackWireReady, OwnedProjectedPackWireStep};")
sub(PK+"/🦀️.rs","    pub use semio_framework_pack_record_encode::BorrowedPackSection;\n    #[cfg(feature=\"deflate\")]\n    pub use semio_framework_pack_record_encode::{BorrowedPackDocumentCursor,BorrowedPackDocumentProgress,BorrowedPackDocumentFailure};\n",
 "    pub use semio_framework_pack_record_encode_cursor::BorrowedPackSection;\n    #[cfg(feature=\"deflate\")]\n    pub use semio_framework_pack_record_encode_cursor::{BorrowedPackDocumentCursor,BorrowedPackDocumentProgress,BorrowedPackDocumentFailure};\n")
sub(PK+"/Cargo.toml",'semio-framework-pack-record-encode = { path = "../../🌱️value/🛫️encode/📦️packages/🦀️rust" }\n','semio-framework-pack-record-encode = { path = "../../🌱️value/🛫️encode/📦️packages/🦀️rust" }\nsemio-framework-pack-record-encode-cursor = { path = "../../🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/📦️packages/🦀️rust" }\n')
# registries
insert_sorted_member(F+"/Cargo.toml",os.path.relpath(pkg,F))
add_build_owner("pack-record-encode-cursor",os.path.relpath(pkg,R))
flush()

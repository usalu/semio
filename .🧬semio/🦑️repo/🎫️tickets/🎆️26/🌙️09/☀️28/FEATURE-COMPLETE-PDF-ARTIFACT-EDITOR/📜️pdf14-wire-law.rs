//! 📚 Exact PDF 1.4 native receiver contract against independently encoded authored frames.
extern crate self as dsl;
extern crate self as store;

mod pack_rt { pub const OP_BINARY_FORMAT: u8 = 1; }
mod original_receiving {
    pub use semio_framework_replication_codec::OpBinaryReceivingCursor;
    pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
}
macro_rules! tag { ($name:ident, $tag:expr) => {mod $name {pub const TAG: u8 = $tag;}}; }
tag!(insert_page, 0);
tag!(remove_page, 1);
tag!(move_page, 2);
tag!(resize_page, 3);
tag!(replace_page_text, 4);
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedWorkOutcome};
use semio_framework_replication_codec::{OpBinary, OpBinaryReceiving};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use standards::v1_4::subsets::base::schema::mutations::*;
mod standards { pub mod v1_4 { pub mod subsets { pub mod base { pub mod schema {
    pub mod snapshot {
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub struct PageDoc { pub width: f64, pub height: f64, pub text: String }
    }
    pub mod mutations {
        use super::snapshot::PageDoc;
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)] pub struct InsertPage { pub index: usize, pub page: PageDoc }
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)] pub struct RemovePage { pub index: usize }
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)] pub struct MovePage { pub from: usize, pub to: usize }
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)] pub struct ResizePage { pub index: usize, pub width: f64, pub height: f64 }
        #[derive(Debug, PartialEq, semio_framework_value::RetireOwned)] pub struct ReplacePageText { pub index: usize, pub text: String }
    }
} } } } }
#[derive(Debug, PartialEq, semio_framework_value::RetireOwned)]
enum PdfMutation { InsertPage(InsertPage), RemovePage(RemovePage), MovePage(MovePage), ResizePage(ResizePage), ReplacePageText(ReplacePageText) }
macro_rules! from {($($name:ident),+) => {$(impl From<$name> for PdfMutation { fn from(value: $name) -> Self { Self::$name(value) } })+};}
from!(InsertPage, RemovePage, MovePage, ResizePage, ReplacePageText);

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> { unreachable!() }
    fn decode_op(_: &[u8]) -> Result<Self, semio_framework_replication_error::ProtocolError> { unreachable!() }
}

#[path = "owner.rs"]
mod receiver;

impl OpBinaryReceiving for PdfMutation {
    type DomainFailure = semio_framework_value::ValueError;
    type Receiving = receiver::Pdf14OperationReceiving;
    fn admit_receiving(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self::Receiving,RetainedCloneProgress)>> {
        Self::Receiving::admit(source,range,grant)
    }
}

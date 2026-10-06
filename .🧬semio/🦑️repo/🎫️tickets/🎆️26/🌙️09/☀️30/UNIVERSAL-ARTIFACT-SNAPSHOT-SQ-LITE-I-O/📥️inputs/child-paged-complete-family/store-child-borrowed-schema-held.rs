//! 🫳️ Literal owned child and link descriptors retain their original identities in immutable backing.
use super::{ArtifactChild,OwnerRef,LinkPin,ArtifactLink};
use semio_framework_dsl_record::{BorrowedDslField,BorrowedDslRecord,BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout,borrowed_record};
static CHILD_FIELDS:&[F]=&[F::new(0,"child_id",H::Text),F::new(1,"target",<crate::os_io::ArtifactRef as BorrowedDslField>::SHAPE)];
static OWNER_FIELDS:&[F]=&[F::new(0,"parent",H::Text),F::new(1,"slot",H::Text),F::new(2,"child_id",H::Text)];
static PIN_LABELS:&[(&'static str,u32)]=&[("head",0),("checkpoint",1),("snapshot",2)];
static PIN_FIELDS:&[F]=&[
 F::new(0,"kind",H::Enum(PIN_LABELS)),
 F{optional:true,..F::new(1,"checkpoint_id",H::Text)},
 F{optional:true,..F::new(2,"blob_hash",H::Text)},
 F{optional:true,..F::new(3,"blob_size",H::UInt)},
 F{optional:true,..F::new(4,"blob_media_type",H::Text)},
];
static LINK_FIELDS:&[F]=&[F::new(0,"target",<crate::os_io::ArtifactRef as BorrowedDslField>::SHAPE),F::new(1,"pin",H::Record(borrowed_record::<LinkPin>)),F::new(2,"role",H::Text)];
impl<S> BorrowedDslRecord for ArtifactChild<S>{const RECORD:R=R{keyword:None,layout:RecordLayout::Inline,fields:CHILD_FIELDS};}
impl<S> BorrowedDslField for ArtifactChild<S>{const SHAPE:H=H::Record(borrowed_record::<Self>);}
impl BorrowedDslRecord for OwnerRef{const RECORD:R=R{keyword:None,layout:RecordLayout::Inline,fields:OWNER_FIELDS};}
impl BorrowedDslField for OwnerRef{const SHAPE:H=H::Record(borrowed_record::<Self>);}
impl BorrowedDslRecord for LinkPin{const RECORD:R=R{keyword:None,layout:RecordLayout::Inline,fields:PIN_FIELDS};}
impl BorrowedDslField for LinkPin{const SHAPE:H=H::Record(borrowed_record::<Self>);}
impl BorrowedDslRecord for ArtifactLink{const RECORD:R=R{keyword:None,layout:RecordLayout::Inline,fields:LINK_FIELDS};}
impl BorrowedDslField for ArtifactLink{const SHAPE:H=H::Record(borrowed_record::<Self>);}

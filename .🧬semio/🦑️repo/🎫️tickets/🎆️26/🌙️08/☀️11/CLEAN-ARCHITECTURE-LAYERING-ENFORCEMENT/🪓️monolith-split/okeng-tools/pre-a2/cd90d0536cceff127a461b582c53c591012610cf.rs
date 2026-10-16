//! 🏭️ The selected producer quotes and admits its original typed catalog without constructing a substitute source.
use super::{DocumentStoreOwners,DocumentStoreOwnersAdmissionError};
use semio_framework_value::{ToValue,FromValue,ValueError,RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use crate::os_spr::Mutation;

/// 🎟️ Every replay branch uses this same original catalog producer under its existing caller grant.
pub trait DocumentStoreOwnerProducer<P,M>:'static
where P:Clone+ToValue+FromValue,M:Clone+ToValue+FromValue+Mutation<P>,
{
 fn source_demands()->Result<RetirementDemand,ValueError>;
 fn admit_sources(grant:RetainedCloneGrant)->Result<(DocumentStoreOwners<P,M>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,M>>;
}

/// 🗃️ The authored generic native source constructor retains its existing bounded factories and disposer.
pub struct BoundedDocumentStoreOwnerProducer;
impl<P,M> DocumentStoreOwnerProducer<P,M> for BoundedDocumentStoreOwnerProducer
where P:Clone+ToValue+FromValue+semio_framework_value::retirement::RetireOwned+Sync,
M:Clone+ToValue+FromValue+Mutation<P>+crate::os_spr::OpBinary+crate::os_spr::OpText+semio_framework_value::retirement::RetireOwned+semio_framework_pack_json::ArtifactCanonicalJsonTree+Send+Sync+'static,
{
 fn source_demands()->Result<RetirementDemand,ValueError>{super::bounded_artifact_store_owners_source_demands::<P,M>()}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(DocumentStoreOwners<P,M>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,M>>{super::bounded_artifact_store_owners::<P,M>(grant)}
}

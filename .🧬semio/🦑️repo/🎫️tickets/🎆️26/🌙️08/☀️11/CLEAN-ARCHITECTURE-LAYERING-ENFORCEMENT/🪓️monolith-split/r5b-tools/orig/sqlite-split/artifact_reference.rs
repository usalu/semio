//! 📦️ Canonical artifact identity and dialect persist in explicit scalar columns.
use super::*;
use semio_framework_artifact_reference::{ArtifactRef,ArtifactDialect};
pub(super) fn write(out:&mut Projection<'_,'_>,reference:&ArtifactRef)->Result<i64,ValueError>{out.insert("pdf_artifact_reference",&[C::Text(&reference.artifact_id),C::Text(&reference.dialect.artifact_kind),C::Text(&reference.dialect.standard),C::Text(&reference.dialect.subset)])}
pub(super) fn read(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<ArtifactRef,ValueError>{let row=reader.take("pdf_artifact_reference",key,5)?;Ok(ArtifactRef{artifact_id:reader.text(row,1)?,dialect:ArtifactDialect{artifact_kind:reader.text(row,2)?,standard:reader.text(row,3)?,subset:reader.text(row,4)?}})}

//! 🎯️ Borrows the Terrain snapshot field each mutation replaces.

use super::GisTerrainMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

impl GisTerrainMutation {
    fn target_segment(&self, index: usize) -> Result<&'static str, ValueError> {
        if index != 0 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Terrain conflict path has one segment"));
        }
        Ok(match self {
            Self::ChangeExaggeration(_) => "exaggeration",
            Self::ChangeImportedFeatures(_) => "importedMap",
            Self::ReplaceElevationSource(_) => "elevationSource",
        })
    }
}

impl ArtifactOperationTargets for GisTerrainMutation {
    fn len(&self) -> usize { 1 }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.target_segment(index).map(str::len)
    }

    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, ValueError> {
        let suffix = self.target_segment(index)?.as_bytes().get(offset..)
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "Terrain conflict path offset is outside the segment"))?;
        let count = suffix.len().min(output.len());
        output[..count].copy_from_slice(&suffix[..count]);
        Ok(count)
    }
}

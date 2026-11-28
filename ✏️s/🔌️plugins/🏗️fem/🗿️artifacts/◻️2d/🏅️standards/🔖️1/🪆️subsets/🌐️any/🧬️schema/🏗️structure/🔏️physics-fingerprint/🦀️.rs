//! 🔏️ Physics fingerprint of a FEM document: a stable 64-bit digest of everything a solve depends on, so an edit that changes only display settings is told apart from one that needs a new solve.

use super::FemAnalysisSettings;
use std::fmt::{self, Debug, Write};

/// 🔏️ One analysis setting and whether it only changes how results are drawn. This single table is the declared list of display-only fields, mirrored by `x-semio-display-only` in the schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnalysisField {
    pub name: &'static str,
    pub display_only: bool,
}

/// 🔏️ FNV-1a, 64 bit: the fingerprint hash every language reproduces byte for byte (offset `cbf29ce484222325`, prime `100000001b3`).
#[derive(Clone, Copy, Debug)]
pub struct PhysicsHasher(u64);

impl Default for PhysicsHasher {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl PhysicsHasher {
    pub fn bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.0 = bytes.iter().fold(self.0, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
        self
    }

    pub fn debug<T: Debug + ?Sized>(&mut self, value: &T) -> &mut Self {
        let _ = write!(self, "{value:?};");
        self
    }

    pub fn analysis(&mut self, settings: &FemAnalysisSettings) -> &mut Self {
        for (field, bits) in FemAnalysisSettings::ANALYSIS_FIELDS.iter().zip(settings.field_bits()).filter(|(field, _)| !field.display_only) {
            self.bytes(field.name.as_bytes()).bytes(b"=").bytes(&bits.to_le_bytes()).bytes(b";");
        }
        self
    }

    pub fn finish(&self) -> u64 {
        self.0
    }
}

impl fmt::Write for PhysicsHasher {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.bytes(text.as_bytes());
        Ok(())
    }
}

/// 🔏️ A finished generation answers an edited revision, with no new solve and no stale mark, exactly when the edit left the physics fingerprint unchanged; any physics edit, or a solve still running, takes the re-solve path.
pub fn rebinds_generation(finished: bool, bound_fingerprint: u64, edited_fingerprint: u64) -> bool {
    finished && bound_fingerprint == edited_fingerprint
}

impl FemAnalysisSettings {
    pub const ANALYSIS_FIELDS: [AnalysisField; 3] = [
        AnalysisField { name: "modalCount", display_only: false },
        AnalysisField { name: "bucklingCount", display_only: false },
        AnalysisField { name: "deformationScale", display_only: true },
    ];

    fn field_bits(&self) -> [u64; 3] {
        let Self { modal_count, buckling_count, deformation_scale } = self;
        [*modal_count as u64, *buckling_count as u64, deformation_scale.to_bits()]
    }

    /// 🔏️ Names of the settings that never change a solve, in declaration order.
    pub fn display_only_fields() -> impl Iterator<Item = &'static str> {
        Self::ANALYSIS_FIELDS.iter().filter(|field| field.display_only).map(|field| field.name)
    }

    /// 🔏️ Digest of the settings with every display-only field left out: modal and buckling counts are physics, the deformation scale is not.
    pub fn physics_fingerprint(&self) -> u64 {
        PhysicsHasher::default().analysis(self).finish()
    }
}

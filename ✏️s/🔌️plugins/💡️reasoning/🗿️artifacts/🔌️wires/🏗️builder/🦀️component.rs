//! 🏗️ WiresBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::wires::{WiresDiff, WiresMutation, WiresSnapshot};
use crate::artifacts::wires::standards::v1::builder::WiresBuilder as WiresRawBuilder;

#[derive(Clone, Debug)]
pub struct WiresBuilder(WiresRawBuilder);

impl ArtifactBuilder for WiresBuilder {
    type Snapshot = WiresSnapshot;
    type Mutation = WiresMutation;
    type Diff = WiresDiff;
    fn empty() -> Self { Self(WiresRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(WiresRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(WiresRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(WiresRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}

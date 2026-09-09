//! 🧬️ Generation2d snapshot schema — artifact-lane fields only.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation2dSnapshot
/// 🧬️ Generation2dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, Default)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dSnapshot {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation2dSnapshot


impl Generation2dSnapshot {
    /// 🧊️ Explicit cold-only disposal of a detached projection. Both fields reject a bare drop —
    /// `fixture.layout` is an `OrderedMap<WidgetLayout>` whose root must be retired
    /// (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), and `generation` carries its own
    /// retirement ladder — so an owned projection is CLOSED, never dropped.
    pub fn retire_cold(self) {
        self.fixture.retire_cold();
        self.generation.retire_cold();
    }
}

/// 📸️ A projection read that CLOSES itself — the ONE shape a test holds an owned
/// [`Generation2dSnapshot`] in, the 2d twin of `Generation3dSnapshotRead`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
pub struct Generation2dSnapshotRead(Option<Generation2dSnapshot>);

#[cfg(test)]
impl Generation2dSnapshotRead {
    pub fn new(snapshot: Generation2dSnapshot) -> Self {
        Self(Some(snapshot))
    }

    /// 📤️ Hands the projection on to a caller that takes ownership of the retirement itself.
    pub fn into_inner(mut self) -> Generation2dSnapshot {
        self.0.take().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl std::ops::Deref for Generation2dSnapshotRead {
    type Target = Generation2dSnapshot;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl std::ops::DerefMut for Generation2dSnapshotRead {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl Drop for Generation2dSnapshotRead {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            snapshot.retire_cold();
        }
    }
}

#[cfg(test)]
impl PartialEq for Generation2dSnapshotRead {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

#[cfg(test)]
impl PartialEq<Generation2dSnapshot> for Generation2dSnapshotRead {
    fn eq(&self, other: &Generation2dSnapshot) -> bool {
        **self == *other
    }
}

#[cfg(test)]
impl std::fmt::Debug for Generation2dSnapshotRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, formatter)
    }
}

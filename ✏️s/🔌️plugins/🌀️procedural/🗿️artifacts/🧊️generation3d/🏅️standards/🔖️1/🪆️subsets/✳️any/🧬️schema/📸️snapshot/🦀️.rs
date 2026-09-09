//! 🧬️ Generation3d snapshot schema — artifact-lane fields only.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::FlowFixture;
pub use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation3dSnapshot
/// 🧬️ Generation3dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation3d")]
pub struct Generation3dSnapshot {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation3dSnapshot

impl Default for Generation3dSnapshot {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default().into() }
    }
}

impl Generation3dSnapshot {
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
/// [`Generation3dSnapshot`] in. `VcsArtifactApp::snapshot`/`ArtifactStore::snapshot` hand back an
/// owned projection, and a bare drop of one aborts the whole test binary on
/// `ordered-map root must be explicitly retired before drop`; this dereferences to the projection
/// and retires it on the way out (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
pub struct Generation3dSnapshotRead(Option<Generation3dSnapshot>);

#[cfg(test)]
impl Generation3dSnapshotRead {
    pub fn new(snapshot: Generation3dSnapshot) -> Self {
        Self(Some(snapshot))
    }

    /// 📤️ Hands the projection on to a caller that takes ownership of the retirement itself.
    pub fn into_inner(mut self) -> Generation3dSnapshot {
        self.0.take().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl std::ops::Deref for Generation3dSnapshotRead {
    type Target = Generation3dSnapshot;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl std::ops::DerefMut for Generation3dSnapshotRead {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("a projection read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl Drop for Generation3dSnapshotRead {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            snapshot.retire_cold();
        }
    }
}

#[cfg(test)]
impl PartialEq for Generation3dSnapshotRead {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

#[cfg(test)]
impl PartialEq<Generation3dSnapshot> for Generation3dSnapshotRead {
    fn eq(&self, other: &Generation3dSnapshot) -> bool {
        **self == *other
    }
}

#[cfg(test)]
impl std::fmt::Debug for Generation3dSnapshotRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, formatter)
    }
}

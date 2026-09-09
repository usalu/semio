//! 🧬️ Generation2d diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation2dDiff
/// 🧬️ Generation2dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Generation2dArtifact>>,
    #[state(artifact)]
    pub fixture: Option<FlowFixture>,
    #[state(artifact)]
    pub generation: Option<GenerationPlayRoot>,
}
//#endregion 🔖️Generation2dDiff

impl Generation2dDiff {
    /// 🧊️ Explicit cold-only disposal of a detached sparse delta. Every inhabited side carries the
    /// same retirement law the projection does — `fixture.layout` is an `OrderedMap<WidgetLayout>`
    /// whose root must be retired (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) and
    /// `generation` owns its own ladder — so an owned diff is CLOSED, never dropped.
    pub fn retire_cold(self) {
        let Self { artifact, fixture, generation } = self;
        if let Some(artifact) = artifact {
            let Generation2dArtifact { fixture, generation } = *artifact;
            fixture.retire_cold();
            generation.retire_cold();
        }
        if let Some(fixture) = fixture {
            fixture.retire_cold();
        }
        if let Some(generation) = generation {
            generation.retire_cold();
        }
    }
}

/// 🔺️ A sparse-delta read that CLOSES itself — the ONE shape a test holds an owned
/// [`Generation2dDiff`] in, the diff twin of `Generation2dSnapshotRead`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
pub struct Generation2dDiffRead(Option<Generation2dDiff>);

#[cfg(test)]
impl Generation2dDiffRead {
    pub fn new(diff: Generation2dDiff) -> Self {
        Self(Some(diff))
    }
}

#[cfg(test)]
impl std::ops::Deref for Generation2dDiffRead {
    type Target = Generation2dDiff;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("a delta read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl Drop for Generation2dDiffRead {
    fn drop(&mut self) {
        if let Some(diff) = self.0.take() {
            diff.retire_cold();
        }
    }
}

#[cfg(test)]
impl std::fmt::Debug for Generation2dDiffRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, formatter)
    }
}

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️Helpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::Generation2dArtifact;
//#endregion 🔁️Re-exports

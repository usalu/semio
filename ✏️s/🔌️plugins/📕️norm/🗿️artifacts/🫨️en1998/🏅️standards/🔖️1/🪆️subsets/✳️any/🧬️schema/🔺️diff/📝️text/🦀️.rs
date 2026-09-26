//! 🔺️ En1998 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1998Artifact;
use crate::En1998Snapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

fn apply_fields(next: &mut En1998Snapshot, diff: &En1998Diff) {
    if let Some(value) = &diff.annex {
        next.annex = value.clone();
    }
    if let Some(value) = &diff.site {
        next.site = value.clone();
    }
    if let Some(value) = &diff.buildings {
        next.buildings = value.clone();
    }
    if let Some(value) = &diff.bridges {
        next.bridges = value.clone();
    }
    if let Some(value) = &diff.assessments {
        next.assessments = value.clone();
    }
    if let Some(value) = &diff.silos {
        next.silos = value.clone();
    }
    if let Some(value) = &diff.tanks {
        next.tanks = value.clone();
    }
    if let Some(value) = &diff.foundations {
        next.foundations = value.clone();
    }
    if let Some(value) = &diff.retaining_walls {
        next.retaining_walls = value.clone();
    }
    if let Some(value) = &diff.towers {
        next.towers = value.clone();
    }
}

impl En1998Diff {
    pub fn apply_to_artifact(&self, artifact: &En1998Artifact) -> protocol::MutationApplyResult<En1998Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut snap = artifact.to_snapshot();
        apply_fields(&mut snap, self);
        Ok(En1998Artifact::from_snapshot(snap))
    }
}

impl MutationDiff<En1998Snapshot> for En1998Diff {
    fn apply(&self, snapshot: &En1998Snapshot) -> protocol::MutationApplyResult<En1998Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        apply_fields(&mut next, self);
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(site);
        take!(buildings);
        take!(bridges);
        take!(assessments);
        take!(silos);
        take!(tanks);
        take!(foundations);
        take!(retaining_walls);
        take!(towers);
    }
}

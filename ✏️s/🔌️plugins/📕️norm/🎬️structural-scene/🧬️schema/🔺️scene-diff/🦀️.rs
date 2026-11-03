//! 🔺️ Scene delta: the wire form of an incremental scene update; `apply(diff(a, b), a) == b` is enforced by the diff engine, not here.

use super::*;

use std::collections::BTreeSet;

records! {
    /// 🩹️ Everything that changed between two scene revisions; unchanged entities are absent.
    pub struct SceneDelta {
        pub base: SceneRevision,
        pub revision: SceneRevision,
        pub bounds: Option<Aabb>,
        pub entities: Vec<EntityPatch>,
        pub annotations: Vec<AnnotationPatch>,
        pub legends: Option<Vec<Legend>>,
        pub layers: Option<Vec<Layer>>,
        pub tracks: Option<Vec<ParameterTrack>>,
        pub owners: Option<Vec<Ownership>>,
        pub description: Option<LocalizedCopy>,
    }
}

variants! {
    /// 🧷️ One entity-level change; `index` is the document-order position after the whole delta applied.
    pub enum EntityPatch {
        Add { index: u32, entity: SceneEntity },
        Replace { entity: SceneEntity },
        Remove { id: String },
    }

    /// 🏷️ One annotation-level change; `index` is the position after the whole delta applied.
    pub enum AnnotationPatch {
        Add { index: u32, annotation: Annotation },
        Replace { annotation: Annotation },
        Remove { id: String },
    }
}

impl EntityPatch {
    pub fn id(&self) -> &str {
        match self {
            Self::Add { entity, .. } | Self::Replace { entity } => &entity.entity.id,
            Self::Remove { id } => id,
        }
    }
}

impl AnnotationPatch {
    pub fn id(&self) -> &str {
        match self {
            Self::Add { annotation, .. } | Self::Replace { annotation } => annotation.id(),
            Self::Remove { id } => id,
        }
    }
}

impl SceneDelta {
    pub fn is_empty(&self) -> bool {
        self.bounds.is_none() && self.entities.is_empty() && self.annotations.is_empty() && self.legends.is_none() && self.layers.is_none() && self.tracks.is_none() && self.owners.is_none() && self.description.is_none()
    }
}

impl Validate for EntityPatch {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), self.id());
        match self {
            Self::Add { entity, .. } | Self::Replace { entity } => entity.check(&at(path, "entity"), out),
            Self::Remove { .. } => {}
        }
    }
}

impl Validate for AnnotationPatch {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), self.id());
        match self {
            Self::Add { annotation, .. } | Self::Replace { annotation } => annotation.check(&at(path, "annotation"), out),
            Self::Remove { .. } => {}
        }
    }
}

impl Validate for SceneDelta {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.bounds.check(&at(path, "bounds"), out);
        each(&self.entities, path, "entities", out);
        each(&self.annotations, path, "annotations", out);
        if let Some(legends) = &self.legends {
            each(legends, path, "legends", out);
        }
        if let Some(layers) = &self.layers {
            each(layers, path, "layers", out);
        }
        if let Some(tracks) = &self.tracks {
            each(tracks, path, "tracks", out);
        }
        self.description.check(&at(path, "description"), out);
        expect(out, &at(path, "revision"), self.base.document_seq <= self.revision.document_seq && self.base.report_seq <= self.revision.report_seq, "revision-not-before-base");
        let once = |ids: Vec<&str>| ids.iter().collect::<BTreeSet<_>>().len() == ids.len();
        expect(out, &at(path, "entities"), once(self.entities.iter().map(EntityPatch::id).collect()), "one-patch-per-entity");
        expect(out, &at(path, "annotations"), once(self.annotations.iter().map(AnnotationPatch::id).collect()), "one-patch-per-annotation");
    }
}

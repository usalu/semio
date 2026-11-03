//! 🧱️ Scene builder: append-only entity drafts with accessible names, utilisation binding and sealed content hashes, plus assembly of fragments into a validated scene.

use super::description::{describe, kind_text};
use super::{EntityEvaluation, ProjectionContext};
use crate::tessellation::geometry_bounds;
use crate::*;
use std::collections::BTreeSet;
use std::sync::Arc;

/// 🧩️ Everything one entity (or the global pass) contributed; the unit the fragment cache stores.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fragment {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    pub legends: Vec<Legend>,
    pub layers: Vec<Layer>,
    pub tracks: Vec<ParameterTrack>,
    pub owners: Vec<Ownership>,
    pub bounds: Option<Aabb>,
}

/// 🏗️ Collects one fragment: entities through drafts, annotations, legends and layers directly.
#[derive(Debug, Default)]
pub struct SceneBuilder {
    fragment: Fragment,
    owner: Option<EntityRef>,
}

/// ✏️ A pending entity; nothing is recorded until [`EntityDraft::add`].
#[derive(Debug)]
pub struct EntityDraft<'a> {
    builder: &'a mut SceneBuilder,
    entity: SceneEntity,
    layer: Option<Layer>,
}

fn union(a: Option<Aabb>, b: Aabb) -> Aabb {
    match a {
        None => b,
        Some(a) => Aabb::new(
            Vec3::new(a.min.x.min(b.min.x), a.min.y.min(b.min.y), a.min.z.min(b.min.z)),
            Vec3::new(a.max.x.max(b.max.x), a.max.y.max(b.max.y), a.max.z.max(b.max.z)),
        ),
    }
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draft(&mut self, entity: &EntityRef, geometry: Geometry) -> EntityDraft<'_> {
        let (layer_id, en, de) = kind_text(entity.kind);
        let layer = Layer { id: layer_id.into(), name: LocalizedCopy::new(en, de), visible: true, locked: false };
        let blank = LocalizedCopy::new("", "");
        EntityDraft {
            builder: self,
            layer: Some(layer),
            entity: SceneEntity {
                entity: entity.clone(),
                geometry,
                appearance: Appearance::role(MaterialRole::Neutral),
                utilisation: None,
                label: None,
                accessible_name: blank.clone(),
                accessible_detail: blank,
                layer: layer_id.into(),
                pick: PickPolicy::Whole,
                handles: Vec::new(),
                content_hash: String::new(),
            },
        }
    }

    pub fn member(&mut self, e: &EntityRef, g: MemberGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Member(g))
    }

    pub fn plate(&mut self, e: &EntityRef, g: PlateGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Plate(g))
    }

    pub fn shell(&mut self, e: &EntityRef, g: ShellGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Shell(g))
    }

    pub fn solid(&mut self, e: &EntityRef, g: SolidGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Solid(g))
    }

    pub fn fastener(&mut self, e: &EntityRef, g: FastenerGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Fastener(g))
    }

    pub fn support(&mut self, e: &EntityRef, g: SupportGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Support(g))
    }

    pub fn load(&mut self, e: &EntityRef, g: LoadGlyph) -> EntityDraft<'_> {
        self.draft(e, Geometry::Load(g))
    }

    pub fn surface(&mut self, e: &EntityRef, g: SurfaceGeometry) -> EntityDraft<'_> {
        self.draft(e, Geometry::Surface(g))
    }

    pub fn group(&mut self, e: &EntityRef, members: Vec<EntityRef>) -> EntityDraft<'_> {
        self.draft(e, Geometry::Group { members })
    }

    /// 🔗️ From now on annotations and tracks belong to `owner` (`None` makes them global to the scene).
    pub fn owned_by(&mut self, owner: Option<EntityRef>) {
        self.owner = owner;
    }

    fn link(&mut self, kind: OwnedKind, id: &str) {
        if let Some(owner) = &self.owner {
            self.fragment.owners.push(Ownership { kind, id: id.into(), owner: owner.clone() });
        }
    }

    pub fn annotate(&mut self, annotation: Annotation) {
        self.link(OwnedKind::Annotation, annotation.id());
        self.fragment.annotations.push(annotation);
    }

    pub fn track(&mut self, track: ParameterTrack) {
        self.link(OwnedKind::Track, &track.id);
        self.fragment.tracks.push(track);
    }

    pub fn dimension(&mut self, id: &str, from: Vec3, to: Vec3, offset: Vec3, value: Quantity, binding: Option<FieldBinding>) {
        self.annotate(Annotation::Dimension { id: id.into(), from, to, offset, value, binding });
    }

    pub fn label(&mut self, id: &str, anchor: Vec3, text: LocalizedCopy, tone: Tone) {
        self.annotate(Annotation::Label { id: id.into(), anchor, text, tone });
    }

    pub fn legend(&mut self, legend: Legend) {
        self.fragment.legends.push(legend);
    }

    pub fn layer(&mut self, layer: Layer) {
        if !self.fragment.layers.iter().any(|l| l.id == layer.id) {
            self.fragment.layers.push(layer);
        }
    }

    /// 🧩️ Appends a whole fragment, its ownership rows included, as produced by an artifact's own collector.
    pub fn absorb(&mut self, other: Fragment) {
        let Fragment { entities, annotations, legends, layers, tracks, owners, bounds } = other;
        self.fragment.entities.extend(entities);
        self.fragment.annotations.extend(annotations);
        self.fragment.legends.extend(legends);
        self.fragment.layers.extend(layers);
        self.fragment.tracks.extend(tracks);
        self.fragment.owners.extend(owners);
        if let Some(b) = bounds {
            self.fragment.bounds = Some(union(self.fragment.bounds.take(), b));
        }
    }

    pub fn into_fragment(self) -> Fragment {
        self.fragment
    }
}

impl EntityDraft<'_> {
    pub fn named(mut self, en: &str, de: &str) -> Self {
        self.entity.accessible_name = LocalizedCopy::new(en, de);
        self
    }

    pub fn detail(mut self, en: &str, de: &str) -> Self {
        self.entity.accessible_detail = LocalizedCopy::new(en, de);
        self
    }

    pub fn labelled(mut self, en: &str, de: &str) -> Self {
        self.entity.label = Some(LocalizedCopy::new(en, de));
        self
    }

    pub fn layer(mut self, id: &str, en: &str, de: &str) -> Self {
        self.entity.layer = id.into();
        self.layer = Some(Layer { id: id.into(), name: LocalizedCopy::new(en, de), visible: true, locked: false });
        self
    }

    pub fn role(mut self, role: MaterialRole) -> Self {
        self.entity.appearance.role = role;
        self
    }

    pub fn style(mut self, style: StyleState) -> Self {
        self.entity.appearance.style = style;
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.entity.appearance.opacity = opacity;
        self
    }

    pub fn hatch(mut self, hatch: Hatch) -> Self {
        self.entity.appearance.hatch = Some(hatch);
        self
    }

    pub fn deformation(mut self, overlay: DeformationOverlay) -> Self {
        self.entity.appearance.deformation = Some(overlay);
        self
    }

    pub fn pick(mut self, pick: PickPolicy) -> Self {
        self.entity.pick = pick;
        self
    }

    pub fn handle(mut self, id: &str) -> Self {
        self.entity.handles.push(id.into());
        self
    }

    /// 🌡️ Binds the governing check (highest utilisation) and per-station samples, switches the colour channel to utilisation and marks pending entities stale.
    pub fn utilise(mut self, ev: &EntityEvaluation) -> Self {
        if let Some(w) = ev.worst() {
            self.entity.utilisation = Some(UtilisationBinding { check_id: w.check_id.clone(), ratio: w.utilisation, status: w.status, per_station: ev.stations.clone(), stale: ev.stale });
            self.entity.appearance.channel = ColourChannel::Utilisation;
        }
        if ev.stale {
            self.entity.appearance.style = StyleState::Stale;
        }
        self
    }

    /// ➕️ Seals the content hash, records the entity, its layer and its bounds.
    pub fn add(self) {
        let EntityDraft { builder, entity, layer } = self;
        let entity = entity.sealed();
        if let Some(b) = geometry_bounds(&entity.geometry) {
            builder.fragment.bounds = Some(union(builder.fragment.bounds.take(), b));
        }
        if let Some(layer) = layer {
            builder.layer(layer);
        }
        builder.fragment.entities.push(entity);
    }
}

/// 🧩️ Concatenates fragments in order without validating: entities, annotations, tracks and ownership rows as given, first layer and legend definitions win, bounds are united. Layers, legends and description are the neutral defaults a projection's `finish` replaces.
pub fn merge(fragments: &[Arc<Fragment>], revision: SceneRevision) -> StructuralScene {
    let mut scene = StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision,
        bounds: Aabb::new(Vec3::ZERO, Vec3::ZERO),
        entities: Vec::new(),
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: Vec::new(),
        owners: Vec::new(),
        description: LocalizedCopy::new("Scene", "Szene"),
    };
    scene.entities.reserve_exact(fragments.iter().map(|f| f.entities.len()).sum());
    let (mut layer_ids, mut legend_ids, mut track_ids): (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    let mut bounds: Option<Aabb> = None;
    for f in fragments {
        scene.entities.extend(f.entities.iter().cloned());
        scene.annotations.extend(f.annotations.iter().cloned());
        scene.layers.extend(f.layers.iter().filter(|l| layer_ids.insert(l.id.clone())).cloned());
        scene.legends.extend(f.legends.iter().filter(|l| legend_ids.insert(l.id.clone())).cloned());
        scene.tracks.extend(f.tracks.iter().filter(|t| track_ids.insert(t.id.clone())).cloned());
        scene.owners.extend(f.owners.iter().filter(|o| o.kind != OwnedKind::Track || track_ids.contains(&o.id)).cloned());
        if let Some(b) = &f.bounds {
            bounds = Some(union(bounds, b.clone()));
        }
    }
    scene.bounds = bounds.unwrap_or(scene.bounds);
    scene.description = describe(&scene);
    scene
}

/// 👁️ Applies the layer visibility of the projection context.
pub fn apply_visibility(scene: &mut StructuralScene, ctx: &ProjectionContext) {
    if let Some(visible) = &ctx.visible_layers {
        for layer in &mut scene.layers {
            layer.visible = visible.contains(&layer.id);
        }
    }
}

/// 🎬️ [`merge`] with the neutral description, layer visibility and validation; `finish` runs between merge and validation and owns the global parts (layers, legends, description, bounds).
pub fn assemble_with(fragments: &[Arc<Fragment>], revision: SceneRevision, ctx: &ProjectionContext, finish: impl FnOnce(&mut StructuralScene)) -> Result<StructuralScene, Vec<Violation>> {
    let mut scene = merge(fragments, revision);
    finish(&mut scene);
    apply_visibility(&mut scene, ctx);
    scene.validate().map(|()| scene)
}

/// 🎬️ Assembly with the neutral global parts only.
pub fn assemble(fragments: &[Arc<Fragment>], revision: SceneRevision, ctx: &ProjectionContext) -> Result<StructuralScene, Vec<Violation>> {
    assemble_with(fragments, revision, ctx, |_| {})
}

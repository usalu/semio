//! 🫧️ Lane overlays of the viewport: drag grips, the inline dimension-line numeric entry of the open drag, the insert-tool cursor ghost and the selection and hover of the other users, merged into the annotation layer of a lowered viewport. See the viewport3d crate.

use crate::viewport3d::lowering::LoweredViewport;
use crate::viewport3d::overlays::number;
use crate::viewport3d::parts::entity_anchor;
use crate::viewport3d::picking::{resolve, PickTarget};
use crate::viewport3d::pose::{add, mul, v3, V3};
use semio_s_plugin_norm_structural_scene::{EntityKind, Handle, HandleKind, StructuralScene, Vec3};
use crate::accessibility::{resolve_variant, A11yPrefs};
use crate::{DragPreview, GhostSource, LaneFrame, PeerLane};
use semio_s_plugin_norm_structural_scene::colour_maps::Variant;
use semio_plugin::{World3dAnnotation, World3dAnnotationLayer, World3dMarker, World3dMarkerShape, World3dText, World3dTone, WORLD3D_ANNOTATIONS_MAX};

/// 🏷️ English and German name of an entity kind for the insert ghost.
pub fn kind_name(kind: EntityKind) -> (&'static str, &'static str) {
    match kind {
        EntityKind::Member => ("member", "Bauteil"),
        EntityKind::Plate => ("plate", "Platte"),
        EntityKind::Shell => ("shell", "Schale"),
        EntityKind::Solid => ("solid", "Volumenkörper"),
        EntityKind::Fastener => ("fastener", "Verbindungsmittel"),
        EntityKind::Support => ("support", "Lager"),
        EntityKind::Load => ("load", "Last"),
        EntityKind::Surface => ("surface", "Fläche"),
        EntityKind::Group => ("group", "Gruppe"),
    }
}

fn marker(id: String, position: V3, shape: World3dMarkerShape, text: World3dText, tone: World3dTone) -> World3dAnnotation {
    World3dAnnotation::Marker(World3dMarker { id, position, shape, axis: None, text, tone })
}

fn grip_marker(handle: &Handle, text: World3dText, tone: World3dTone) -> World3dAnnotation {
    let axis = matches!(handle.kind, HandleKind::Translate { .. } | HandleKind::ScalarAlong { .. }).then(|| direction(&handle.kind));
    World3dAnnotation::Marker(World3dMarker { id: format!("handle:{}", handle.id), position: v3(&handle.anchor), shape: World3dMarkerShape::Ring, axis, text, tone })
}

fn anchor_of(scene: &StructuralScene, entity_id: &str) -> Option<V3> {
    scene.entities.iter().find(|e| e.entity.id == entity_id).map(|e| entity_anchor(&e.geometry))
}

fn direction(kind: &HandleKind) -> V3 {
    match kind {
        HandleKind::Translate { axis } | HandleKind::ScalarAlong { axis } | HandleKind::Angular { axis, .. } => v3(axis),
        HandleKind::TranslatePlane { normal } | HandleKind::FaceOffset { normal } => v3(normal),
        HandleKind::Radial { .. } | HandleKind::DimensionLine | HandleKind::CountStepper | HandleKind::ChoiceStepper => [1.0, 0.0, 0.0],
    }
}

/// 👁️ How the grips are drawn right now: the handle of the open drag, the keyboard-focused handle and the platform preferences.
#[derive(Clone, Copy, Debug, Default)]
pub struct GripView<'a> {
    pub dragging: Option<&'a str>,
    pub focused: Option<&'a str>,
    pub prefs: A11yPrefs,
}

/// 🕹️ One grip marker per handle of the selection, plus a cross at the keyboard-focused one so focus never relies on colour alone. The dragged grip is toned primary; with forced colours or more contrast every grip takes the primary tone and the shapes carry the state. Grips never animate, so reduced motion changes nothing about them.
pub fn grips(handles: &[Handle], view: &GripView) -> Vec<World3dAnnotation> {
    let contrast = resolve_variant(view.prefs) == Variant::HighContrast;
    handles
        .iter()
        .flat_map(|h| {
            let tone = if contrast || view.dragging == Some(h.id.as_str()) { World3dTone::Primary } else { World3dTone::Info };
            let name = World3dText::new(h.accessible_name.en.clone(), h.accessible_name.de.clone());
            let focus = (view.focused == Some(h.id.as_str())).then(|| marker(format!("focus:{}", h.id), v3(&h.anchor), World3dMarkerShape::Cross, World3dText::new(format!("Focused: {}", h.accessible_name.en), format!("Fokussiert: {}", h.accessible_name.de)), World3dTone::Primary));
            std::iter::once(grip_marker(h, name, tone)).chain(focus)
        })
        .collect()
}

/// 📏️ The dimension line of one drag: from the grip along its axis to the dragged value, labelled with the typed entry and a caret while the user types, else with the previewed value; both languages are carried and `who` prefixes the label of another user.
pub fn drag_dimension(handle: &Handle, drag: &DragPreview, id: String, tone: World3dTone, who: Option<&str>) -> World3dAnnotation {
    let from = v3(&handle.anchor);
    let to = match &handle.kind {
        HandleKind::Angular { axis, centre } => {
            let a = v3(axis);
            let c = v3(centre);
            let offset = [from[0] - c[0], from[1] - c[1], from[2] - c[2]];
            let length = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt().max(f64::MIN_POSITIVE);
            let along = (offset[0] * a[0] + offset[1] * a[1] + offset[2] * a[2]) / (length * length);
            let radial = [offset[0] - along * a[0], offset[1] - along * a[1], offset[2] - along * a[2]];
            let radius = (radial[0] * radial[0] + radial[1] * radial[1] + radial[2] * radial[2]).sqrt();
            handle.kind.angular_point(drag.s, radius).map_or(from, |p| [p.x + along * a[0], p.y + along * a[1], p.z + along * a[2]])
        }
        kind => add(from, mul(direction(kind), drag.s - drag.origin)),
    };
    let unit = handle.writes.first().map_or("", |w| w.unit.as_str());
    let choice = handle.writes.first().and_then(|w| w.quantise.as_ref()?.token_at(w.map.apply(drag.s)));
    let label = |german: bool| {
        if let (Some(choice), true) = (choice, drag.entry.is_empty()) {
            let text = if german { &choice.label.de } else { &choice.label.en };
            return who.map_or(text.clone(), |who| format!("{who}: {text}"));
        }
        let shown = if drag.entry.is_empty() { handle.writes.first().map_or_else(|| number(drag.s, german, 3), |w| number(w.value_at(drag.s), german, 3)) } else { format!("{}\u{258F}", drag.entry) };
        let text = format!("{shown} {unit}").trim_end().to_string();
        who.map_or(text.clone(), |who| format!("{who}: {text}"))
    };
    World3dAnnotation::dimension(id, from, to, [0.0; 3], World3dText::new(label(false), label(true))).with_tone(tone)
}

/// 📏️ The inline dimension line of the open drag of this user.
pub fn dimension_entry(handles: &[Handle], frame: &LaneFrame) -> Option<World3dAnnotation> {
    let drag = frame.transient.drag.as_ref()?;
    let handle = handles.iter().find(|h| h.id == drag.handle)?;
    Some(drag_dimension(handle, drag, format!("entry:{}", handle.id), World3dTone::Primary, None))
}

/// 🖐️ The in-progress drags of the other users, drawn as dimension lines at the grips they hold.
pub fn peer_drags(handles: &[Handle], frame: &LaneFrame) -> Vec<World3dAnnotation> {
    frame.transient.peers.iter().filter_map(|peer| handles.iter().find(|h| h.id == peer.drag.handle).map(|handle| drag_dimension(handle, &peer.drag, format!("peer:{}:drag:{}", peer.user, handle.id), World3dTone::Info, Some(&peer.user)))).collect()
}

/// 👻️ The insert-tool ghost: a cross at the cursor labelled with the kind that a click would insert.
pub fn insert_ghost(kind: EntityKind, at: &Vec3) -> World3dAnnotation {
    let (en, de) = kind_name(kind);
    marker("insert:ghost".to_string(), v3(at), World3dMarkerShape::Cross, World3dText::new(format!("Insert {en}"), format!("{de} einfügen")), World3dTone::Primary)
}

/// 👥️ The selection and the hover of one other user, drawn at the scene entities they address and labelled with their name.
pub fn peer_marks(scene: &StructuralScene, peer: &PeerLane) -> Vec<World3dAnnotation> {
    let at = |address: &String| match resolve(scene, address) {
        Some(PickTarget::Entity { entity, .. }) => anchor_of(scene, &entity.id).map(|position| (entity.id, position)),
        _ => None,
    };
    let selected = peer.selected.iter().filter_map(at).map(|(id, position)| marker(format!("peer:{}:select:{id}", peer.actor), position, World3dMarkerShape::Ring, World3dText::new(format!("{} selected this", peer.actor), format!("{} hat dies ausgewählt", peer.actor)), World3dTone::Info));
    let hovered = peer.hovered.iter().filter_map(at).map(|(id, position)| marker(format!("peer:{}:hover:{id}", peer.actor), position, World3dMarkerShape::Dot, World3dText::new(format!("{} points here", peer.actor), format!("{} zeigt hierher", peer.actor)), World3dTone::Neutral));
    selected.chain(hovered).collect()
}

/// 🫧️ Merges every lane overlay into the annotation layer of `lowered`.
pub fn apply(mut lowered: LoweredViewport, scene: &StructuralScene, lanes: &LaneOverlays) -> LoweredViewport {
    let (handles, peer_handles, frame) = (&lanes.handles[..], &lanes.peer_handles[..], &lanes.frame);
    let dragging = frame.transient.drag.as_ref().map(|d| d.handle.as_str());
    let mut items = lowered.annotations.take().map_or_else(Vec::new, |layer| layer.items);
    items.extend(grips(handles, &GripView { dragging, focused: lanes.focused.as_deref(), prefs: lanes.prefs }));
    items.extend(dimension_entry(handles, frame));
    items.extend(peer_drags(peer_handles, frame));
    items.extend(frame.transient.ghost.as_ref().and_then(|g| match &g.source {
        GhostSource::Insert { kind, at, .. } => Some(insert_ghost(*kind, at)),
        _ => None,
    }));
    items.extend(frame.peers.iter().flat_map(|peer| peer_marks(scene, peer)));
    items.truncate(WORLD3D_ANNOTATIONS_MAX);
    lowered.annotations = (!items.is_empty()).then(|| World3dAnnotationLayer::new(items).titled(World3dText::new("Annotations", "Beschriftungen")));
    lowered
}

/// 🫧️ What the live lanes add to a viewport paint: the grips of the selection, the grips the other users hold, the insert palette and the lane frame (open drag, ghost, other users), the keyboard-focused grip and the accessibility preferences.
#[derive(Clone, Debug, Default)]
pub struct LaneOverlays {
    pub handles: Vec<Handle>,
    pub peer_handles: Vec<Handle>,
    pub palette: Vec<PaletteEntry>,
    pub frame: LaneFrame,
    pub focused: Option<String>,
    pub prefs: A11yPrefs,
}

impl LaneOverlays {
    /// 🫧️ The overlays of a paint of the artifact `domain`: the lane frame, the grip that has the keyboard (the grip-focus registry the shell hook writes) and the accessibility preferences the shell last reported for the platform; the grips of the selection are filled in by the window.
    pub fn of_frame(frame: &LaneFrame, domain: &str) -> Self {
        Self { frame: frame.clone(), focused: crate::focused_grip(domain), prefs: crate::platform_a11y_prefs(), ..Self::default() }
    }
}

/// ➕ One tool of the insert palette as the toolbar shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct PaletteEntry {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

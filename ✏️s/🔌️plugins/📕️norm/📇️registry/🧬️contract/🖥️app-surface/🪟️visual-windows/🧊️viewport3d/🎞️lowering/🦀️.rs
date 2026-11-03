//! 🎞️ Lowering of a locale-free `StructuralScene` to the framework's `World3dScene` lanes: instanced meshes, utilisation colours and scalar field, section, highlights, annotations, pick targets and instance deltas.

use super::camera::{preset_pose, CameraPose};
use super::colour::{entity_rgb, field_rgb, ramp_rgb, rgb_hex, styled, Rgb};
use super::fields::{self, ResolvedField, Site};
use super::playback::{self, PlaybackState};
use super::config::{Lang, Viewport3dWindowConfig};
use super::description::{describe, entity_spoken_name};
use super::measure::MeasureState;
use super::mesh::MeshData;
use super::overlays;
use super::parts::{entity_points, fnv_hex, Part, PartContext, SceneTessellator};
use super::picking::{entity_address, granularity, resolve, PickTarget};
use super::pose::*;
use super::scene::{EntityKind, Geometry, LocalizedCopy, PickPolicy, SceneEntity, StructuralScene, StyleState};
use semio_framework_pack_json::{json, Value};
use semio_framework_plugin::{World3dText, World3dHighlight, World3dModellingOptions, World3dScalarDomain, World3dScalarField, World3dScalarRange, World3dSection, World3dSubElementStyle, World3dTone};
use std::collections::{BTreeSet, HashMap, HashSet};

pub struct ViewportInputs<'a> {
    pub config: &'a Viewport3dWindowConfig,
    pub selected: &'a [String],
    pub hovered: Option<&'a str>,
    pub measure: &'a MeasureState,
    pub lang: Lang,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoweringStats {
    pub entities: usize,
    pub instances: usize,
    pub meshes: usize,
    pub truncated: usize,
}

#[derive(Clone, Debug)]
pub struct LoweredViewport {
    pub meshes: Vec<Value>,
    pub instances: Vec<Value>,
    pub scalar_field: Option<World3dScalarField>,
    pub annotations: Option<semio_framework_plugin::World3dAnnotationLayer>,
    pub options: World3dModellingOptions,
    pub selection_ids: Vec<String>,
    pub hovered: Option<String>,
    pub pick_targets_json: Option<String>,
    pub camera: CameraPose,
    pub stats: LoweringStats,
    pub description: String,
    pub readout: Option<LocalizedCopy>,
}

fn r6(x: f64) -> f64 {
    (x * 1.0e6).round() / 1.0e6
}

fn r3(v: V3) -> [f64; 3] {
    [r6(v[0]), r6(v[1]), r6(v[2])]
}

fn priority(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Member | EntityKind::Plate | EntityKind::Shell | EntityKind::Solid | EntityKind::Surface => 0,
        EntityKind::Support => 1,
        EntityKind::Load => 2,
        EntityKind::Fastener => 3,
        EntityKind::Group => 4,
    }
}

fn load_kind(entity: &SceneEntity) -> Option<super::scene::LoadKind> {
    match &entity.geometry {
        Geometry::Load(g) => Some(g.case().kind),
        _ => None,
    }
}

fn station_value(stations: &[super::scene::StationValue], t: f64) -> f64 {
    let Some(first) = stations.first() else { return 0.0 };
    if t <= first.at { return first.value; }
    stations.windows(2).find(|w| t <= w[1].at).map_or(stations[stations.len() - 1].value, |w| w[0].value + (w[1].value - w[0].value) * (t - w[0].at) / (w[1].at - w[0].at).max(1e-12))
}

struct ResolvedFieldInfo {
    id: String,
    symbol: &'static str,
}

fn vertex_values(part: &Part, instance: usize, field: &ResolvedField) -> Vec<f64> {
    let count = part.mesh.vertex_count();
    (0..count).map(|v| {
        let station = part.station.map(|(t0, t1)| t0 + part.mesh.point(v)[2].clamp(0.0, 1.0) * (t1 - t0));
        let site = match part.grid {
            Some((rows, columns)) if rows >= 2 && columns >= 2 && count == 12 * (rows - 1) * (columns - 1) => Site { vertex: fields::grid_node(v, rows, columns), vertices: rows * columns, station, instance },
            _ => Site { vertex: v, vertices: count, station, instance },
        };
        field.value(site)
    }).collect()
}

fn mesh_json(id: &str, mesh: &MeshData, colour: impl Fn(usize) -> Rgb) -> Value {
    let colors: Vec<f64> = (0..mesh.vertex_count()).flat_map(|i| colour(i)).map(r6).collect();
    let round = |v: &Vec<f64>| v.iter().map(|x| r6(*x)).collect::<Vec<_>>();
    json!({ "id": id, "data": { "positions": round(&mesh.positions), "normals": round(&mesh.normals), "colors": colors, "indices": mesh.indices.clone() } })
}

/// 🎞️ Lowers a scene in its static view: one `World3dScene` worth of lanes, deterministic and locale-free except the instance label language.
pub fn lower(scene: &StructuralScene, inputs: &ViewportInputs, tess: &dyn SceneTessellator) -> LoweredViewport {
    lower_dynamic(scene, inputs, &PlaybackState::default(), tess)
}

/// 🎬️ Lowers a scene at one ephemeral local-only transport state over the scene's own parameter tracks: active scalar field with baked per-vertex ramp colours and its legend, track-selected fields and layers, and mode shapes at the current oscillation phase.
pub fn lower_dynamic(scene: &StructuralScene, inputs: &ViewportInputs, playback: &PlaybackState, tess: &dyn SceneTessellator) -> LoweredViewport {
    let config = inputs.config;
    let tracks = scene.tracks.as_slice();
    let track = playback.track.as_deref().and_then(|id| playback::find(tracks, id));
    let tsample = track.map(|t| playback::sample(t, playback.value)).unwrap_or_default();
    let selection = fields::selection(config.active_field.as_deref(), track.map(|t| (t, &tsample)));
    let (lo, hi) = fields::domain(scene, &selection).unwrap_or((0.0, 1.0));
    let frame = playback::frame(playback, tracks, config.deformation_phase);
    let profile = config.device.profile();
    let density = if config.label_density.rank() <= profile.max_label_density.rank() { config.label_density } else { profile.max_label_density };
    let resting: Vec<playback::TrackSample> = scene.tracks.iter().filter(|t| Some(t.id.as_str()) != track.map(|c| c.id.as_str())).map(|t| playback::sample(t, t.min)).collect();
    let samples: Vec<&playback::TrackSample> = std::iter::once(&tsample).chain(resting.iter()).collect();
    let layer_visible = |id: &str| scene.layers.iter().find(|l| l.id == id).is_some_and(|l| l.visible) && !config.hidden_layers.iter().any(|h| h == id) && config.solo_layer.as_deref().is_none_or(|s| s == id) && samples.iter().all(|s| !s.managed_layers.iter().any(|m| m == id) || s.layer.as_deref() == Some(id));
    let diag = len(sub(v3(&scene.bounds.max), v3(&scene.bounds.min))).max(1.0);
    let ctx = PartContext { scene, lod: profile.lod, glyph: diag * 0.06 * config.load_glyph_scale, deformation_scale: config.deformation_scale, frame };
    let selected: HashSet<String> = inputs.selected.iter().filter_map(|a| match resolve(scene, a) { Some(PickTarget::Entity { entity, .. }) => Some(entity.id), _ => None }).collect();
    let hovered = inputs.hovered.and_then(|a| match resolve(scene, a) { Some(PickTarget::Entity { entity, .. }) => Some(entity.id), _ => None });

    let mut order: Vec<usize> = (0..scene.entities.len()).collect();
    order.sort_by_key(|&i| priority(scene.entities[i].entity.kind));
    let mut placed: HashMap<usize, Vec<Part>> = HashMap::new();
    let (mut budget, mut truncated) = (profile.instance_budget, 0);
    for i in order {
        let e = &scene.entities[i];
        let hidden_load = load_kind(e).is_some_and(|k| config.hidden_load_kinds.contains(&k));
        if !layer_visible(&e.layer) || hidden_load { continue; }
        let parts = tess.parts(e, &ctx);
        if parts.len() > budget { truncated += 1; budget = 0; continue; }
        budget -= parts.len();
        placed.insert(i, parts);
    }

    let mut meshes: Vec<Value> = Vec::new();
    let mut mesh_ids: BTreeSet<String> = BTreeSet::new();
    let mut instances: Vec<Value> = Vec::new();
    let selected_util = scene.entities.iter().position(|e| selected.contains(&e.entity.id) && e.utilisation.is_some());
    let worst_util = scene.entities.iter().enumerate().filter(|(i, e)| placed.contains_key(i) && e.utilisation.is_some()).max_by(|a, b| a.1.utilisation.as_ref().map_or(0.0, |u| u.ratio).total_cmp(&b.1.utilisation.as_ref().map_or(0.0, |u| u.ratio))).map(|(i, _)| i);
    let with_fields = |i: &usize| placed.contains_key(i) && fields::resolve(&scene.entities[*i], &selection).is_some();
    let selected_field = (0..scene.entities.len()).find(|i| with_fields(i) && selected.contains(&scene.entities[*i].entity.id));
    let any_field = (0..scene.entities.len()).find(with_fields);
    let field_focus = if selected_util.is_some() { None } else { selected_field.or_else(|| (config.active_field.is_some() || worst_util.is_none()).then_some(any_field).flatten()) };
    let focus = selected_util.or(worst_util).filter(|_| field_focus.is_none());
    let mut field_values: Option<(String, Vec<Option<f64>>)> = None;
    let mut field_legend: Option<(ResolvedFieldInfo, f64)> = None;

    for (i, e) in scene.entities.iter().enumerate() {
        let Some(parts) = placed.get(&i) else { continue };
        let Some(address) = entity_address(e.entity.kind, &e.entity.id) else { continue };
        let gran = granularity(e.entity.kind);
        let muted = matches!(e.appearance.style, StyleState::Dimmed | StyleState::Ghost);
        for (n, part) in parts.iter().enumerate() {
            let role = part.role.as_ref().unwrap_or(&e.appearance.role);
            let base = if part.ink { super::colour::ink_rgb(entity_rgb(e, config, role)) } else { entity_rgb(e, config, role) };
            let baked: Option<Vec<f64>> = fields::resolve(e, &selection).filter(|_| !part.ink).map(|r| vertex_values(part, n, &r));
            let colours: Option<Vec<Rgb>> = baked.as_ref().map(|values| values.iter().map(|v| styled(e, field_rgb(config.ramp, *v, lo, hi))).collect());
            let stations = e.utilisation.as_ref().and_then(|u| u.per_station.as_deref()).zip(part.station).filter(|_| colours.is_none());
            let mesh_id = match (&colours, stations) {
                (Some(c), _) => format!("{}:{}:{}", part.key, e.entity.id, fnv_hex(&c.iter().map(|rgb| rgb_hex(*rgb)).collect::<String>())),
                (None, Some((_, (t0, t1)))) => format!("{}:{}:{:.4}:{:.4}", part.key, e.entity.id, t0, t1),
                (None, None) => format!("{}@{}", part.key, rgb_hex(base)),
            };
            if mesh_ids.insert(mesh_id.clone()) {
                meshes.push(match (&colours, stations) {
                    (Some(c), _) => mesh_json(&mesh_id, &part.mesh, |v| c[v]),
                    (None, Some((st, (t0, t1)))) => mesh_json(&mesh_id, &part.mesh, |v| ramp_rgb(config.ramp, station_value(st, t0 + part.mesh.point(v)[2].clamp(0.0, 1.0) * (t1 - t0)), config.domain_max)),
                    (None, None) => mesh_json(&mesh_id, &part.mesh, |_| base),
                });
            }
            if field_focus == Some(i) && field_values.is_none() && !part.ghost {
                if let (Some(values), Some(r)) = (&baked, fields::resolve(e, &selection)) {
                    let (scale, symbol) = fields::scale_of(r.unit, config.unit);
                    field_values = Some((mesh_id.clone(), values.iter().map(|v| Some(v * scale)).collect()));
                    field_legend = Some((ResolvedFieldInfo { id: r.id.to_string(), symbol }, scale));
                }
            }
            if focus == Some(i) && field_values.is_none() && !part.ghost {
                let values = match stations {
                    Some((st, (t0, t1))) => (0..part.mesh.vertex_count()).map(|v| Some(station_value(st, t0 + part.mesh.point(v)[2].clamp(0.0, 1.0) * (t1 - t0)))).collect(),
                    None => vec![e.utilisation.as_ref().map(|u| u.ratio); part.mesh.vertex_count()],
                };
                field_values = Some((mesh_id.clone(), values));
            }
            let id = if n == 0 { address.clone() } else { format!("{address}#{n}") };
            let mut record = json!({
                "id": id,
                "interactionGranularityId": gran,
                "meshId": mesh_id,
                "position": r3(part.pose.position),
                "rotation": [r6(part.pose.rotation[0]), r6(part.pose.rotation[1]), r6(part.pose.rotation[2]), r6(part.pose.rotation[3])],
                "scale": r3(part.pose.scale),
                "label": entity_spoken_name(e, inputs.lang),
                "selected": selected.contains(&e.entity.id),
                "hovered": hovered.as_deref() == Some(e.entity.id.as_str()),
                "disabled": muted || part.ghost,
                "provisional": matches!(e.appearance.style, StyleState::Ghost),
            });
            if n > 0 {
                if let Some(object) = record.as_object_mut() { object.insert("interactionId", json!(address.clone())); }
            }
            instances.push(record);
        }
    }

    let scalar_field = field_values.map(|(mesh_id, values)| match &field_legend {
        Some((info, scale)) => {
            let title = fields::title(&info.id);
            World3dScalarField::new(mesh_id, World3dScalarDomain::Vertex, values, config.ramp.framework(), World3dScalarRange { min: lo * scale, max: hi * scale }, World3dText::new(title.en, title.de)).with_unit(info.symbol).with_ticks(5)
        }
        None => World3dScalarField::new(mesh_id, World3dScalarDomain::Vertex, values, config.ramp.framework(), World3dScalarRange { min: 0.0, max: config.domain_max }, Viewport3dWindowConfig::legend_title()).with_ticks(5),
    });
    let annotations = overlays::build(scene, config, density, &|layer| layer_visible(layer), inputs.measure);
    let style = |tone| World3dSubElementStyle { hover: Some(World3dTone::Info), selected: Some(tone), width_px: Some(2.0) };
    let options = World3dModellingOptions {
        pick_filter: None,
        section: config.section.as_ref().map(|s| { let plane = World3dSection::new(v3(&s.origin), v3(&s.normal)); if s.cap { plane.capped(World3dTone::Neutral) } else { plane } }),
        highlight: Some(World3dHighlight { face: Some(style(World3dTone::Primary)), edge: Some(style(World3dTone::Primary)), vertex: Some(style(World3dTone::Primary)) }),
    };
    let selection_ids: Vec<String> = scene.entities.iter().filter(|e| selected.contains(&e.entity.id)).filter_map(|e| entity_address(e.entity.kind, &e.entity.id)).collect();
    let hovered = scene.entities.iter().find(|e| hovered.as_deref() == Some(e.entity.id.as_str())).and_then(|e| entity_address(e.entity.kind, &e.entity.id));
    let stats = LoweringStats { entities: placed.len(), instances: instances.len(), meshes: meshes.len(), truncated };
    let readout = track.map(|t| playback::readout(t, playback.value));
    LoweredViewport {
        pick_targets_json: pick_targets(scene, &placed),
        camera: preset_pose(scene, config.preset),
        description: match (&readout, inputs.lang) {
            (Some(r), Lang::En) => format!("{} {}.", describe(scene, inputs.lang, &stats, &selection_ids), r.en),
            (Some(r), Lang::De) => format!("{} {}.", describe(scene, inputs.lang, &stats, &selection_ids), r.de),
            (None, _) => describe(scene, inputs.lang, &stats, &selection_ids),
        },
        readout,
        meshes, instances, scalar_field, annotations, options, selection_ids, hovered, stats,
    }
}

/// 🧲️ Sub-part pick targets (`vertex`, `edge`, `face`) of plates whose pick policy asks for them, in the pick-target lane's `{kind, id, point, points, typology, selectable}` shape.
fn pick_targets(scene: &StructuralScene, placed: &HashMap<usize, Vec<Part>>) -> Option<String> {
    let mut targets: Vec<Value> = Vec::new();
    for (i, e) in scene.entities.iter().enumerate().filter(|(i, _)| placed.contains_key(i)) {
        if matches!(e.pick, PickPolicy::Whole | PickPolicy::None) { continue; }
        let (Geometry::Plate(_), Some(address)) = (&e.geometry, entity_address(e.entity.kind, &e.entity.id)) else { continue };
        let ring = entity_points(&e.geometry);
        let n = ring.len();
        match e.pick {
            PickPolicy::Vertex => targets.extend(ring.iter().enumerate().map(|(k, p)| json!({ "kind": "vertex", "id": format!("{address}:v{k}"), "point": r3(*p), "points": [r3(*p)], "typology": "norm.plate", "selectable": true }))),
            PickPolicy::Edge => targets.extend((0..n).map(|k| { let (a, b) = (ring[k], ring[(k + 1) % n]); json!({ "kind": "edge", "id": format!("{address}:e{k}"), "point": r3(lerp(a, b, 0.5)), "points": [r3(a), r3(b)], "typology": "norm.plate", "selectable": true }) })),
            PickPolicy::Face => targets.push(json!({ "kind": "face", "id": format!("{address}:f"), "point": r3(mul(ring.iter().fold([0.0; 3], |s, p| add(s, *p)), 1.0 / n.max(1) as f64)), "points": ring.iter().map(|p| r3(*p)).collect::<Vec<_>>(), "typology": "norm.plate", "selectable": true })),
            _ => {}
        }
    }
    targets.truncate(512);
    (!targets.is_empty()).then(|| semio_framework_pack_json::to_string(&Value::Array(targets)))
}

/// 🚚️ Delta between two instance sets keyed by instance id: `{base, revision, count, changed[], removed[]}` as the host's incremental lane expects.
pub fn instances_delta(base: u64, revision: u64, previous: &[Value], next: &[Value]) -> Value {
    let id = |v: &Value| v.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
    let before: HashMap<String, &Value> = previous.iter().map(|v| (id(v), v)).collect();
    let after: HashSet<String> = next.iter().map(id).collect();
    let changed: Vec<Value> = next.iter().filter(|v| before.get(&id(v)).is_none_or(|old| *old != *v)).cloned().collect();
    let removed: Vec<String> = previous.iter().map(id).filter(|i| !after.contains(i)).collect();
    json!({ "base": base, "revision": revision, "count": next.len(), "changed": changed, "removed": removed })
}

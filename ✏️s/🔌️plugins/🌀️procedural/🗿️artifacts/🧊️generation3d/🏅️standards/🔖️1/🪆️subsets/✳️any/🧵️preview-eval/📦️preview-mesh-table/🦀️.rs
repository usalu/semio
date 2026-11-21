//! 📦️ Shared preview mesh publication for an exact evaluation, session and display quality.
use super::*;
use std::{cell::{Cell, RefCell}, collections::{BTreeMap, HashMap}, rc::Rc, sync::atomic::{AtomicU64, Ordering}};

/// 📌️ The mesh and resolved source used by one directional viewport instance.
pub struct PreviewMeshPlacement {
    pub mesh_id: String,
    pub source_handle: Option<String>,
}

/// 🖼️ A completed mesh table shared by hover, selection and camera republications.
pub struct PreviewMeshTable {
    pub bounds: Option<PreviewBounds>,
    pub meshes_json: String,
    pub placements: BTreeMap<String, PreviewMeshPlacement>,
    pub hover_aliases: HashMap<String, String>,
    pub hover_revision: u64,
}

struct CachedPreviewMeshTable {
    source: String,
    policy: PreviewEvaluationSource,
    residency: Option<(u64, u64)>,
    tolerance_bits: u64,
    show_mode: String,
    widgets: Vec<String>,
    sources: Vec<Option<String>>,
    table: Rc<PreviewMeshTable>,
}

thread_local! {
    static PREVIEW_MESH_TABLE: RefCell<Option<CachedPreviewMeshTable>> = const { RefCell::new(None) };
    static PREVIEW_TESSELLATIONS: Cell<u64> = const { Cell::new(0) };
}

static PREVIEW_HOVER_REVISION: AtomicU64 = AtomicU64::new(1);

/// 🔢️ Counts resident mesh misses while preparing a new preview table.
pub fn preview_tessellation_count() -> u64 {
    PREVIEW_TESSELLATIONS.with(Cell::get)
}

/// 🧹️ Clears the last preview table before taking a cold measurement.
pub fn reset_preview_mesh_table() {
    PREVIEW_MESH_TABLE.with(|table| table.borrow_mut().take());
}

type MeshResolution<'session> = Option<(String, &'session MeshData)>;

/// 🪟️ Reuses a table while the evaluated geometry, visible widgets and resolved meshes agree.
pub fn preview_mesh_table(
    evaluation: &PreviewEvaluation<'_>,
    preview_ids: &[String],
    session: Option<&FlowEvalSession>,
    show_mode: &str,
    tolerance: f64,
) -> Option<Rc<PreviewMeshTable>> {
    let document = evaluation.geometry()?;
    let items: Vec<_> = preview_ids.iter().flat_map(|id| {
        preview_viewport_items_for_widget(document, id).into_iter().map(move |item| (id.as_str(), item))
    }).collect();
    let lookup = session.map(|session| evaluation.mesh_lookup(session));
    let resolved: Vec<MeshResolution<'_>> = items.iter().map(|(id, item)| {
        lookup.as_ref()?.resolve(id, item).map(|(handle, mesh)| (handle.into_owned(), mesh))
    }).collect();
    let source = evaluation.source();
    let residency = session.map(|session| (session.preview_mesh_cache_identity(), session.preview_mesh_epoch()));
    let tolerance_bits = tolerance.to_bits();
    Some(PREVIEW_MESH_TABLE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(cached) = slot.as_ref().filter(|cached| {
            cached.residency == residency && cached.tolerance_bits == tolerance_bits
                && cached.policy == evaluation.evaluation_source
                && cached.show_mode == show_mode && cached.widgets == preview_ids && cached.source == source
                && cached.sources.iter().map(Option::as_deref).eq(resolved.iter().map(|row| row.as_ref().map(|(handle, _)| handle.as_str())))
        }) {
            return Rc::clone(&cached.table);
        }
        let table = Rc::new(build_mesh_table(&items, &resolved, show_mode));
        *slot = Some(CachedPreviewMeshTable {
            source: source.to_owned(),
            policy: evaluation.evaluation_source,
            residency,
            tolerance_bits,
            show_mode: show_mode.to_owned(),
            widgets: preview_ids.to_vec(),
            sources: resolved.into_iter().map(|row| row.map(|(handle, _)| handle)).collect(),
            table: Rc::clone(&table),
        });
        table
    }))
}

fn build_mesh_table(items: &[(&str, PreviewChannelItem<'_>)], resolved: &[MeshResolution<'_>], show_mode: &str) -> PreviewMeshTable {
    use semio_framework_pack_json::{Object, Value};
    let mut bounds = PreviewMeshBounds::default();
    let mut meshes = Vec::new();
    let mut mesh_ids = PreviewMeshIds::default();
    let mut placements = BTreeMap::new();
    for ((widget, item), resolution) in items.iter().zip(resolved) {
        let instance = item.address(widget).instance;
        let inline = item.inline.as_ref();
        let source_handle = resolution.as_ref().map(|(handle, _)| handle.as_str());
        let mesh_id = if let Some(mesh) = mesh_ids.get(source_handle, inline) {
            mesh.clone()
        } else {
            let data = match inline {
                Some(inline) => inline_preview_mesh(inline),
                None => {
                    if resolution.is_none() { PREVIEW_TESSELLATIONS.with(|count| count.set(count.get().saturating_add(1))); }
                    resolution.as_ref().map(|(_, mesh)| (*mesh).clone())
                }
            };
            let Some(data) = data.map(|mesh| apply_show_mode_mesh(mesh, show_mode)).filter(mesh_has_preview_geometry) else { continue };
            let mesh_id = format!("eval-{instance}");
            let mut mesh = Object::new();
            mesh.insert("id", Value::String(mesh_id.clone()));
            mesh.insert("role", Value::String(preview_mesh_role(inline, &data).into()));
            bounds.include(&data);
            mesh.insert("data", Value::from(data));
            meshes.push(Value::Object(mesh));
            mesh_ids.insert(source_handle, inline, mesh_id.clone());
            if let (None, Some((_, native))) = (inline, resolution) {
                mesh_ids.track_native_hover(native, &mesh_id);
            }
            mesh_id
        };
        placements.insert(instance, PreviewMeshPlacement { mesh_id, source_handle: source_handle.map(str::to_owned) });
    }
    PreviewMeshTable {
        bounds: bounds.finish(),
        meshes_json: semio_framework_pack_json::to_string(&Value::Array(meshes)),
        placements,
        hover_aliases: mesh_ids.into_hover_aliases(),
        hover_revision: PREVIEW_HOVER_REVISION.fetch_add(1, Ordering::Relaxed),
    }
}

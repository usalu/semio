//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link CommandBus}.

pub mod command_bus {
// #region command_bus
//! 🎯 Command routing between renderer and app controllers.

use serde_json::Value;
use std::collections::HashMap;

pub trait CommandHandler: Send {
    fn id(&self) -> &str;
    fn handle(&mut self, command: &str, args: Option<&Value>) -> Vec<String>;
}

pub struct CommandBus {
    controllers: HashMap<String, Box<dyn CommandHandler>>,
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn CommandHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub fn dispatch(&mut self, controller_id: &str, command: &str, args: Option<&Value>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(command, args))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler {
        id: String,
    }

    impl CommandHandler for EchoHandler {
        fn id(&self) -> &str {
            &self.id
        }

        fn handle(&mut self, command: &str, _args: Option<&Value>) -> Vec<String> {
            vec![format!("{command}:ok")]
        }
    }

    #[test]
    fn dispatches_to_registered_handler() {
        let mut bus = CommandBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let ops = bus.dispatch("app", "ping", None);
        assert_eq!(ops, vec!["ping:ok"]);
    }
}
// #endregion command_bus
}

pub mod layout {
// #region layout
//! 📐 Window layouts, panel tab constants, and engagement rails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDescriptor {
    pub controller_id: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
}
//#endregion 🔖Command

//#region 🔖PanelTabConstants
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ID: &str = "framework.panel.document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL: &str = "Document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID: &str = "framework.panel.document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ID: &str = "framework.panel.parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: &str = "Parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: &str = "framework.panel.parameters";
//#endregion 🔖PanelTabConstants

//#region 🔖WindowLayout
fn kind_window() -> String {
    "window".into()
}

fn kind_stack() -> String {
    "stack".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutWindowNode {
    #[serde(default = "kind_window")]
    pub kind: String,
    pub window_kind_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutStackNode {
    #[serde(default = "kind_stack")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
    pub active_window_kind_id: Option<String>,
    pub children: Vec<WindowLayoutWindowNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutAxisNode {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    pub children: Vec<WindowLayoutChild>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowLayoutChild {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowLayoutRoot {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayout {
    pub root: WindowLayoutRoot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedLayout {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    pub layout: WindowLayout,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_path: Option<Vec<String>>,
}

pub fn create_window_layout(
    window_kind_id: impl Into<String>,
    title: Option<String>,
    instance_id: Option<String>,
    template_id: Option<String>,
) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode {
        kind: kind_window(),
        window_kind_id: window_kind_id.into(),
        title,
        instance_id,
        template_id,
    }
}

pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: kind_stack(),
            size: None,
            active_window_kind_id: None,
            children: window_kind_ids
                .iter()
                .enumerate()
                .map(|(index, id)| {
                    create_window_layout(
                        id.clone(),
                        titles.and_then(|rows| rows.get(index).cloned()),
                        None,
                        None,
                    )
                })
                .collect(),
        }),
    }
}

pub fn create_default_layout(
    window_ids: &[String],
    direction: &str,
    sizes: Option<&[f64]>,
    titles: Option<&[String]>,
) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: direction.into(),
            size: None,
            children: window_ids
                .iter()
                .enumerate()
                .map(|(index, id)| {
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: kind_stack(),
                        size: sizes.and_then(|rows| rows.get(index).copied()),
                        active_window_kind_id: None,
                        children: vec![create_window_layout(
                            id.clone(),
                            titles
                                .and_then(|rows| rows.get(index).cloned())
                                .or_else(|| Some(id.clone())),
                            None,
                            None,
                        )],
                    })
                })
                .collect(),
        }),
    }
}

pub fn create_tab_stack_layout(window_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    create_stack_layout(window_ids, titles)
}

pub fn create_named_layout(
    id: impl Into<String>,
    label: impl Into<String>,
    layout: WindowLayout,
    origin: impl Into<String>,
    icon_id: Option<String>,
    group_path: Option<Vec<String>>,
) -> NamedLayout {
    NamedLayout {
        id: id.into(),
        label: label.into(),
        icon_id,
        layout,
        origin: origin.into(),
        group_path,
    }
}

pub fn merge_named_layouts(base: &[NamedLayout], extension: &[NamedLayout]) -> Vec<NamedLayout> {
    let mut merged: HashMap<String, NamedLayout> = HashMap::new();
    for entry in base {
        merged.insert(entry.id.clone(), entry.clone());
    }
    for entry in extension {
        merged.insert(entry.id.clone(), entry.clone());
    }
    merged.into_values().collect()
}

/// 🧭 Collects every `window_kind_id` referenced by a layout tree.
pub fn collect_window_kind_ids_from_layout(layout: &WindowLayout) -> Vec<String> {
    let mut ids = Vec::new();
    collect_window_kind_ids_from_root(&layout.root, &mut ids);
    ids
}

fn collect_window_kind_ids_from_root(root: &WindowLayoutRoot, out: &mut Vec<String>) {
    match root {
        WindowLayoutRoot::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
        WindowLayoutRoot::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
    }
}

fn collect_window_kind_ids_from_children(children: &[WindowLayoutChild], out: &mut Vec<String>) {
    for child in children {
        match child {
            WindowLayoutChild::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
            WindowLayoutChild::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
        }
    }
}

fn collect_window_kind_ids_from_stack(stack: &WindowLayoutStackNode, out: &mut Vec<String>) {
    for window in &stack.children {
        out.push(window.window_kind_id.clone());
    }
}
//#endregion 🔖WindowLayout

//#region 🔖WindowMeasure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WindowMeasure {
    Select {
        id: String,
        label: Option<String>,
        value: String,
        items: Vec<MeasureSelectItem>,
        on_change: CommandDescriptor,
    },
    Slider {
        id: String,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        on_change: CommandDescriptor,
    },
    Toggle {
        id: String,
        icon_id: String,
        label: Option<String>,
        pressed: bool,
        text: Option<String>,
        on_change: CommandDescriptor,
    },
    Group {
        id: String,
        label: String,
        default_open: Option<bool>,
        children: Vec<WindowMeasure>,
    },
}
//#endregion 🔖WindowMeasure

//#region 🔖WindowEngagement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementOption {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_repeat_last: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_abort: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementStatus {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementPossible {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementRingOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementToggleGroupOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WindowEngagementControl {
    Slider {
        id: Option<String>,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        unit: Option<String>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
        on_commit: Option<CommandDescriptor>,
    },
    Stepper {
        id: Option<String>,
        label: Option<String>,
        value: f64,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        unit: Option<String>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
        on_commit: Option<CommandDescriptor>,
    },
    Ring {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementRingOption>,
        disabled: Option<bool>,
        on_select: Option<CommandDescriptor>,
    },
    ToggleGroup {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementToggleGroupOption>,
        disabled: Option<bool>,
        on_select: Option<CommandDescriptor>,
    },
    Select {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        placeholder: Option<String>,
        items: Vec<WindowEngagementSelectItem>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<WindowEngagementOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<WindowEngagementInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<WindowEngagementControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<WindowEngagementControl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<WindowEngagementStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
}

pub fn default_viewport_engagement() -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "framework.viewport.status".into(),
            text: "Viewport".into(),
        }]),
        possible_engagements: None,
    }
}
//#endregion 🔖WindowEngagement
// #endregion layout
}

pub mod mesh {
// #region mesh
//! 🔺 Shared mesh geometry: primitives, compact JSON, OBJ/GLB interchange.

use serde::{Deserialize, Serialize};

//#region MeshData
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshData {
    #[serde(default)]
    pub positions: Vec<f32>,
    #[serde(default)]
    pub normals: Vec<f32>,
    #[serde(default)]
    pub colors: Vec<f32>,
    #[serde(default)]
    pub indices: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub face_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vertex_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_positions: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_is_seam: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_texture_base64: Option<String>,
}

impl MeshData {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                e0[1] * e1[2] - e0[2] * e1[1],
                e0[2] * e1[0] - e0[0] * e1[2],
                e0[0] * e1[1] - e0[1] * e1[0],
            ];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.chunks_exact_mut(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }

    pub fn aabb(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for chunk in self.positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(chunk[axis]);
                max[axis] = max[axis].max(chunk[axis]);
            }
        }
        (min, max)
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices
            .extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        push_triangle(&mut mesh, a, b, c);
        push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

pub fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::PI;
        let phi1 = v1 * std::f32::consts::PI;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = sphere_point(radius, phi0, theta0);
            let p10 = sphere_point(radius, phi0, theta1);
            let p01 = sphere_point(radius, phi1, theta0);
            let p11 = sphere_point(radius, phi1, theta1);
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11);
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01);
            }
        }
    }
    mesh.compute_normals();
    mesh
}

fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * phi.cos(),
        radius * sin_phi * theta.sin(),
    ]
}

pub fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]),
        normalize3([1.0, t, 0.0]),
        normalize3([-1.0, -t, 0.0]),
        normalize3([1.0, -t, 0.0]),
        normalize3([0.0, -1.0, t]),
        normalize3([0.0, 1.0, t]),
        normalize3([0.0, -1.0, -t]),
        normalize3([0.0, 1.0, -t]),
        normalize3([t, 0.0, -1.0]),
        normalize3([t, 0.0, 1.0]),
        normalize3([-t, 0.0, -1.0]),
        normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[
                [face[0], a, c],
                [face[1], b, a],
                [face[2], c, b],
                [a, b, c],
            ]);
        }
        faces = next;
    }
    let mut mesh = MeshData::default();
    for face in faces {
        let a = scale3(verts[face[0] as usize], radius);
        let b = scale3(verts[face[1] as usize], radius);
        let c = scale3(verts[face[2] as usize], radius);
        push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn midpoint(
    verts: &mut Vec<[f32; 3]>,
    cache: &mut std::collections::HashMap<(u32, u32), u32>,
    a: u32,
    b: u32,
) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([
        (verts[a as usize][0] + verts[b as usize][0]) * 0.5,
        (verts[a as usize][1] + verts[b as usize][1]) * 0.5,
        (verts[a as usize][2] + verts[b as usize][2]) * 0.5,
    ]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

pub fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        push_triangle(&mut mesh, p00, p01, p11);
        push_triangle(&mut mesh, p00, p11, p10);
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0);
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::TAU;
        let phi1 = v1 * std::f32::consts::TAU;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0);
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1);
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0);
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1);
            push_triangle(&mut mesh, p00, p10, p11);
            push_triangle(&mut mesh, p00, p11, p01);
        }
    }
    mesh.compute_normals();
    mesh
}

fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1),
        "vertex-marker" => mesh_ico_sphere(1.0, 1),
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12),
        "icoSphere" => mesh_ico_sphere(0.5, 1),
        "plane" => mesh_plane(1.0, 1.0),
        "cylinder" => mesh_cylinder(0.5, 1.0, 16),
        "cone" => mesh_cone(0.5, 1.0, 16),
        "torus" => mesh_torus(0.5, 0.15, 16, 12),
        _ => mesh_box(1.0, 1.0, 1.0),
    }
}

/** @emoji 🔩 Builds mesh data from indexed brep tessellation buffers. */
pub fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData {
        positions: positions.to_vec(),
        normals: normals.to_vec(),
        indices: indices.to_vec(),
        ..MeshData::default()
    };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals();
    }
    mesh
}
//#endregion Primitives

//#region Obj
pub fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
    let mut out = format!("o {object_name}\n");
    for chunk in mesh.positions.chunks_exact(3) {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.chunks_exact(3) {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.chunks_exact(3) {
        let a = tri[0] + 1;
        let b = tri[1] + 1;
        let c = tri[2] + 1;
        if mesh.normals.len() == mesh.positions.len() {
            out.push_str(&format!("f {a}//{a} {b}//{b} {c}//{c}\n"));
        } else {
            out.push_str(&format!("f {a} {b} {c}\n"));
        }
    }
    out
}
//#endregion Obj

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("invalid glb header".into());
    }
    let mut offset = 12usize;
    let mut json = None;
    let mut bin = None;
    while offset + 8 <= bytes.len() {
        let chunk_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[offset + 4..offset + 8];
        offset += 8;
        let end = offset + chunk_len;
        if end > bytes.len() {
            break;
        }
        let chunk = &bytes[offset..end];
        if chunk_type == b"JSON" {
            json = Some(String::from_utf8_lossy(chunk).to_string());
        } else if chunk_type == b"BIN\x00" {
            bin = Some(chunk.to_vec());
        }
        offset = end;
    }
    let json = json.ok_or_else(|| "glb missing json chunk".to_string())?;
    let bin = bin.ok_or_else(|| "glb missing bin chunk".to_string())?;
    let root: serde_json::Value = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    let accessors = root
        .get("accessors")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing accessors".to_string())?;
    let buffer_views = root
        .get("bufferViews")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing bufferViews".to_string())?;
    let meshes = root
        .get("meshes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing meshes".to_string())?;
    let primitive = meshes[0]
        .get("primitives")
        .and_then(|v| v.as_array())
        .and_then(|v| v.first())
        .ok_or_else(|| "glb missing primitive".to_string())?;
    let position_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("POSITION"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing POSITION".to_string())? as usize;
    let normal_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("NORMAL"))
        .and_then(|v| v.as_u64());
    let index_accessor = primitive
        .get("indices")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing indices".to_string())? as usize;
    let positions = read_accessor_f32_vec3(&accessors[position_accessor], &buffer_views, &bin)?;
    let normals = if let Some(index) = normal_accessor {
        read_accessor_f32_vec3(&accessors[index as usize], &buffer_views, &bin)?
    } else {
        Vec::new()
    };
    let indices = read_accessor_u32(&accessors[index_accessor], &buffer_views, &bin)?;
    let mut mesh = MeshData {
        positions,
        normals,
        colors: Vec::new(),
        indices,
        ..Default::default()
    };
    if mesh.normals.is_empty() {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn read_accessor_f32_vec3(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<f32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count * 3);
    for index in 0..count {
        let base = start + index * 12;
        if base + 12 > bin.len() {
            break;
        }
        for axis in 0..3 {
            let value = f32::from_le_bytes(bin[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
            out.push(value);
        }
    }
    Ok(out)
}

fn read_accessor_u32(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<u32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let base = start + index * 4;
        if base + 4 > bin.len() {
            break;
        }
        out.push(u32::from_le_bytes(bin[base..base + 4].try_into().unwrap()));
    }
    Ok(out)
}

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_has_triangles() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.normals.len(), mesh.positions.len());
    }

    #[test]
    fn obj_contains_faces() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let obj = mesh_to_obj(&mesh, "box");
        assert!(obj.contains("o box"));
        assert!(obj.contains("f "));
    }

    #[test]
    fn glb_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let glb = mesh_to_glb(&mesh);
        let decoded = mesh_from_glb(&glb).expect("decode glb");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[test]
    fn primitive_kinds() {
        assert!(mesh_from_kind("sphere").vertex_count() > 0);
        assert!(mesh_from_kind("box").vertex_count() > 0);
    }
}
// #endregion mesh
}

pub mod platform {
// #region platform
//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared command bus.

use crate::command_bus::CommandBus;
use crate::ui::AppDefinition;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PanelVisibility {
    pub left_side_panel: bool,
    pub right_side_panel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformSpec {
    pub id: String,
    pub name: String,
    pub default_active_app_id: Option<String>,
    pub initial_panel_visibility: Option<PanelVisibility>,
}

pub struct Platform {
    pub command_bus: CommandBus,
    pub apps: Vec<AppDefinition>,
    pub active_app_id: String,
    pub generation: u64,
    pub chrome_generation: u64,
    pub uri: String,
    pub panel_visibility: PanelVisibility,
    pub id: String,
    pub name: String,
    generation_counter: AtomicU64,
    chrome_generation_counter: AtomicU64,
}

impl Platform {
    pub fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self {
            command_bus: CommandBus::new(),
            apps: Vec::new(),
            active_app_id: spec.default_active_app_id.clone().unwrap_or_default(),
            generation: 0,
            chrome_generation: 0,
            uri: "/".into(),
            panel_visibility,
            id: spec.id,
            name: spec.name,
            generation_counter: AtomicU64::new(0),
            chrome_generation_counter: AtomicU64::new(0),
        }
    }

    pub fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify();
    }

    pub fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps
            .iter()
            .find(|app| app.id == self.active_app_id)
            .or_else(|| self.apps.first())
    }

    pub fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome();
    }

    pub fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome();
    }

    pub fn notify(&mut self) {
        self.generation = self.generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }

    pub fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ModeDefinition, WindowKindDefinition};

    #[test]
    fn adds_first_app_as_active() {
        let mut platform = Platform::new(None);
        platform.add_app(AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: vec![ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
                layout_id: None,
            }],
            default_mode_id: Some("edit".into()),
            window_kinds: vec![WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: crate::SurfaceKind::Canvas2d,
                icon_id: None,
                measures: Vec::new(),
                engagement: None,
                params_schema: None,
                model_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }],
            panel_tabs: vec![],
            keybindings: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
        });
        assert_eq!(platform.active_app_id, "draw-play");
    }
}
// #endregion platform
}

pub mod tools {
// #region tools
//! 🧰 Declarative per-mode toolbar tool trees.

use crate::layout::CommandDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ToolCategory {
    Selection,
    Tools,
    Commands,
    History,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ToolNode {
    Separator {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
    },
    Button {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        on_press: CommandDescriptor,
    },
    Toggle {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        on_change: CommandDescriptor,
    },
    Collection {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        children: Vec<ToolNode>,
    },
}

impl ToolNode {
    pub fn category(&self) -> ToolCategory {
        match self {
            ToolNode::Separator { .. } => ToolCategory::Tools,
            ToolNode::Button { category, .. } => category.unwrap_or(ToolCategory::Commands),
            ToolNode::Toggle { category, .. } => category.unwrap_or(ToolCategory::Tools),
            ToolNode::Collection { category, .. } => category.unwrap_or(ToolCategory::Tools),
        }
    }

    pub fn with_category(mut self, category: ToolCategory) -> Self {
        match &mut self {
            ToolNode::Button { category: slot, .. }
            | ToolNode::Toggle { category: slot, .. }
            | ToolNode::Collection { category: slot, .. } => *slot = Some(category),
            ToolNode::Separator { .. } => {}
        }
        self
    }
}

pub fn tool_separator(id: impl Into<String>) -> ToolNode {
    ToolNode::Separator {
        id: id.into(),
        order: None,
        disabled: None,
    }
}

pub fn tool_button(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    on_press: CommandDescriptor,
) -> ToolNode {
    let label = label.into();
    ToolNode::Button {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        disabled: None,
        category: None,
        on_press,
    }
}

pub fn tool_toggle(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    pressed: bool,
    on_change: CommandDescriptor,
) -> ToolNode {
    let label = label.into();
    ToolNode::Toggle {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        pressed: Some(pressed),
        disabled: None,
        category: None,
        on_change,
    }
}

pub fn tool_collection(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    children: Vec<ToolNode>,
) -> ToolNode {
    let label = label.into();
    ToolNode::Collection {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        disabled: None,
        category: None,
        children,
    }
}
// #endregion tools
}

pub mod ui {
// #region ui
//! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

use crate::layout::NamedLayout;
use crate::layout::WindowEngagement;
use crate::layout::WindowLayout;
use crate::layout::WindowMeasure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Command
pub use crate::layout::{CommandDescriptor, StyleSpec};
//#endregion 🔖Command

//#region 🔖Primitives
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiStackNode {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTextNode {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiButtonNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub icon_id: String,
    pub label: String,
    pub command: CommandDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSeparatorNode {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInputNode {
    pub id: String,
    pub input_kind: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectNode {
    pub id: String,
    pub value: String,
    pub items: Vec<UiSelectItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiToggleNode {
    pub id: String,
    pub icon_id: String,
    pub pressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiVec3Node {
    pub id: String,
    pub value: Option<[f64; 3]>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueNode {
    pub entries: Vec<UiKeyValueEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSliderNode {
    pub id: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNumberStepperNode {
    pub id: String,
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    pub on_absolute: CommandDescriptor,
    pub on_delta: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiRingNode {
    pub id: String,
    pub orb_id: String,
    pub t: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIconSelectNode {
    pub id: String,
    pub value: String,
    pub uniform: bool,
    pub classifier_kind: String,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiControlNode {
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Vec3(UiVec3Node),
    Button(UiButtonNode),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiFieldNode {
    pub id: String,
    pub label: String,
    pub child: UiControlNode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemAction {
    pub icon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub command: CommandDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_on_hover: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
    pub default_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhover_command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<UiTreeItemAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UiTreeItemNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<UiControlNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

impl UiTreeItemNode {
    /** @emoji 🌳 Builds a tree item with optional extensions unset. */
    pub fn base(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub items: Vec<UiTreeItemNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeNode {
    pub sections: Vec<UiTreeSectionNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_change: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInspectorFieldGroup {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub fields: Vec<UiNode>,
}

pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
//#endregion 🔖Primitives

//#region 🔖InspectorHelpers
pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
    if values.len() <= 1 {
        return true;
    }
    values.windows(2).all(|pair| pair[0] == pair[1])
}

pub struct UiInspectorMixedText {
    pub value: String,
    pub placeholder: Option<String>,
}

pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedText {
        value: if uniform {
            values.first().cloned().unwrap_or_default()
        } else {
            String::new()
        },
        placeholder: if uniform {
            None
        } else {
            Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
        },
    }
}

pub struct UiInspectorMixedNumber {
    pub value: f64,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedNumber {
        value: if uniform {
            *values.first().unwrap_or(&0.0)
        } else {
            f64::NAN
        },
        uniform,
    }
}

pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
    ui_inspector_mixed_text(values)
}

pub struct UiInspectorMixedToggle {
    pub pressed: bool,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedToggle {
        pressed: uniform && values.first().copied().unwrap_or(false),
        uniform,
    }
}

pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
    ui_inspector_mixed_number(values)
}

pub struct UiInspectorMixedVec3 {
    pub value: Option<[f64; 3]>,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_vec3(values: &[[f64; 3]]) -> UiInspectorMixedVec3 {
    let serialized: Vec<String> = values
        .iter()
        .map(|row| serde_json::to_string(row).unwrap_or_default())
        .collect();
    let uniform = ui_inspector_all_equal(&serialized);
    UiInspectorMixedVec3 {
        value: if uniform { values.first().copied() } else { None },
        uniform,
    }
}

pub fn ui_inspector_readonly_field(
    id: impl Into<String>,
    label: impl Into<String>,
    value: impl Into<String>,
) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id,
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: None,
            on_change: CommandDescriptor {
                controller_id: String::new(),
                command: String::new(),
                args: None,
            },
        }),
    })
}

pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
    let sections: Vec<UiSectionNode> = groups
        .iter()
        .filter(|group| !group.fields.is_empty())
        .map(|group| UiSectionNode {
            id: group.id.clone(),
            label: Some(group.label.clone()),
            default_open: Some(group.default_open.unwrap_or(true)),
            children: group.fields.clone(),
        })
        .collect();
    ui_declarative_sections_to_tree(&sections)
}

pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .map(|section| UiTreeSectionNode {
            id: section.id.clone(),
            label: section.label.clone(),
            default_open: Some(section.default_open.unwrap_or(true)),
            items: section
                .children
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index))
                })
                .collect(),
        })
        .collect();
    UiNode::Tree(if tree_sections.is_empty() {
        UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "empty".into(),
                label: None,
                default_open: None,
                items: vec![UiTreeItemNode {
                    id: "empty".into(),
                    label: "—".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    command: None,
                    hover_command: None,
                    unhover_command: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }],
            }],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
        }
    } else {
        UiTreeNode {
            sections: tree_sections,
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
        }
    })
}

fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
    match node {
        UiNode::Text(text) => UiTreeItemNode {
            id: format!("{}.text", fallback_id),
            label: text.value.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        UiNode::Field(field) => {
            let description = if let UiControlNode::Input(input) = &field.child {
                input
                    .placeholder
                    .clone()
                    .or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) })
            } else {
                None
            };
            UiTreeItemNode {
                id: field.id.clone(),
                label: field.label.clone(),
                description,
                icon_id: None,
                selected: None,
                default_open: None,
                command: None,
                hover_command: None,
                unhover_command: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: Some(field.child.clone()),
                is_hidden: None,
            }
        }
        UiNode::Button(button) => UiTreeItemNode {
            id: button.id.clone().unwrap_or(fallback_id),
            label: button.label.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(UiControlNode::Button(button.clone())),
            is_hidden: None,
        },
        UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
        UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
        UiNode::Vec3(vec3) => tree_control_item(vec3.id.clone(), UiControlNode::Vec3(vec3.clone())),
        UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => {
            tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone()))
        }
        UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon_select) => {
            tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone()))
        }
        UiNode::Separator(_) => UiTreeItemNode {
            id: format!("{}.sep", fallback_id),
            label: "—".into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        other => UiTreeItemNode {
            id: fallback_id,
            label: format!("{other:?}"),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    }
}

fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode {
        id,
        label: String::new(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: Some(control),
        is_hidden: None,
    }
}
//#endregion 🔖InspectorHelpers

//#region 🔖ComponentScenes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceKind {
    #[serde(rename = "canvas-2d")]
    Canvas2d,
    #[serde(rename = "world-3d")]
    World3d,
    #[serde(rename = "node-graph")]
    NodeGraph,
    #[serde(rename = "text-editor")]
    TextEditor,
    Table,
    Raster,
    #[serde(rename = "virtualFileSystem")]
    VirtualFileSystem,
    #[serde(rename = "gis2d-map")]
    GisMap,
}

impl SurfaceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canvas2d => "canvas-2d",
            Self::World3d => "world-3d",
            Self::NodeGraph => "node-graph",
            Self::TextEditor => "text-editor",
            Self::Table => "table",
            Self::Raster => "raster",
            Self::VirtualFileSystem => "virtualFileSystem",
            Self::GisMap => "gis2d-map",
        }
    }

    pub fn is_viewport(self) -> bool {
        matches!(self, Self::World3d | Self::NodeGraph | Self::Canvas2d)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dScene {
    pub camera_json: String,
    #[serde(default = "world3d_default_meshes_json")]
    pub meshes_json: String,
    pub instances_json: String,
    #[serde(default = "world3d_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortices_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attractions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volumes_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldMeshLodEntry {
    pub lod: f64,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldLodRecord {
    #[serde(default = "default_true")]
    pub automatic: bool,
    #[serde(default = "default_manual_lod")]
    pub manual: f64,
    #[serde(default = "default_distance_reference")]
    pub distance_reference: f64,
    #[serde(default)]
    pub depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    pub show_grid: bool,
    #[serde(default)]
    pub grid_datum: Option<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldChunkingRecord {
    pub chunk_size: f64,
    pub max_distance: f64,
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_distance_reference() -> f64 {
    100.0
}

fn default_grid_factor() -> f64 {
    10.0
}

fn default_true() -> bool {
    true
}

pub fn world3d_default_lod_json() -> String {
    serde_json::json!({
        "automatic": true,
        "manual": 100.0,
        "distanceReference": 100.0,
        "depthVariable": false,
        "gridFactor": 10.0,
        "gridSnapEnabled": false,
        "showGrid": true,
        "gridDatum": [0.0, 0.0, 0.0],
    })
    .to_string()
}

pub fn world3d_chunking_json(chunk_size: f64, max_distance: f64) -> String {
    serde_json::json!({
        "chunkSize": chunk_size,
        "maxDistance": max_distance,
    })
    .to_string()
}

pub fn world3d_default_selection_json() -> String {
    r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
}

pub fn world3d_default_meshes_json() -> String {
    "[]".into()
}

pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
    serde_json::json!({
        "position": position,
        "target": target,
        "up": [0.0, 0.0, 1.0],
        "fov": fov,
    })
    .to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_items_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_off_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixture_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_peers_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_carets_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_spans_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableScene {
    pub columns_json: String,
    pub rows_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterScene {
    pub width: u32,
    pub height: u32,
    pub pixels_base64: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualFileSystemScene {
    pub schema_json: String,
    pub rows_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_ids_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_drop_enabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapScene {
    pub map_fixture_json: String,
    pub camera_json: String,
    #[serde(default = "gis_map_default_render_mode")]
    pub render_mode: String,
    #[serde(default = "gis_map_default_vector_style")]
    pub vector_style: String,
    #[serde(default = "gis_map_default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "gis_map_default_tile_url_template")]
    pub tile_url_template: String,
    #[serde(default = "gis_map_default_vector_tile_url_template")]
    pub vector_tile_url_template: String,
    #[serde(default = "gis_map_default_layer_visibility_json")]
    pub layer_visibility_json: String,
    #[serde(default = "gis_map_default_layer_stroke_scale_json")]
    pub layer_stroke_scale_json: String,
    #[serde(default = "gis_map_default_selection_json")]
    pub selection_json: String,
    #[serde(default = "gis_map_default_hover_json")]
    pub hover_json: String,
    #[serde(default = "gis_map_default_selection_method")]
    pub selection_method: String,
    #[serde(default = "gis_map_default_selection_mode")]
    pub selection_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
}

pub fn gis_map_default_render_mode() -> String {
    "combined".into()
}

pub fn gis_map_default_vector_style() -> String {
    "colored".into()
}

pub fn gis_map_default_lod_mode() -> String {
    "automatic".into()
}

pub fn gis_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}

pub fn gis_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}

pub fn gis_map_default_layer_visibility_json() -> String {
    r#"{"raster":true,"water":true,"land":true,"roads":true,"buildings":true,"borders":true,"labels":true,"positions":true,"positionLabels":true,"routes":true,"regions":true}"#.into()
}

pub fn gis_map_default_layer_stroke_scale_json() -> String {
    r#"{"raster":1,"water":1,"land":1,"roads":1,"buildings":1,"borders":1,"labels":1,"positions":1,"positionLabels":1,"routes":1,"regions":1}"#.into()
}

pub fn gis_map_default_selection_json() -> String {
    r#"{"positions":[],"routes":[]}"#.into()
}

pub fn gis_map_default_hover_json() -> String {
    "null".into()
}

pub fn gis_map_default_selection_method() -> String {
    "rectangle".into()
}

pub fn gis_map_default_selection_mode() -> String {
    "default".into()
}

impl GisMapScene {
    /** @emoji 🗺️ Builds a GIS map scene with optional extensions unset. */
    pub fn base(map_fixture_json: String, camera_json: String) -> Self {
        Self {
            map_fixture_json,
            camera_json,
            render_mode: gis_map_default_render_mode(),
            vector_style: gis_map_default_vector_style(),
            lod_mode: gis_map_default_lod_mode(),
            tile_url_template: gis_map_default_tile_url_template(),
            vector_tile_url_template: gis_map_default_vector_tile_url_template(),
            layer_visibility_json: gis_map_default_layer_visibility_json(),
            layer_stroke_scale_json: gis_map_default_layer_stroke_scale_json(),
            selection_json: gis_map_default_selection_json(),
            hover_json: gis_map_default_hover_json(),
            selection_method: gis_map_default_selection_method(),
            selection_mode: gis_map_default_selection_mode(),
            context_menu_json: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiExternalSlotNode {
    pub plugin_id: String,
    pub app_id: String,
    pub body_key: String,
    pub params_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiComponentSceneNode {
    pub surface_id: String,
    pub controller_id: String,
    pub component_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pane_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_2d: Option<Canvas2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_3d: Option<World3dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_graph: Option<NodeGraphScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_editor: Option<TextEditorScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster: Option<RasterScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_file_system: Option<VirtualFileSystemScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gis_map: Option<GisMapScene>,
}
//#endregion 🔖ComponentScenes

//#region 🔖UiNode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiNode {
    Stack(UiStackNode),
    Text(UiTextNode),
    Button(UiButtonNode),
    Separator(UiSeparatorNode),
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Vec3(UiVec3Node),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
    Field(UiFieldNode),
    Section(UiSectionNode),
    Tree(UiTreeNode),
    ComponentScene(UiComponentSceneNode),
    ExternalSlot(UiExternalSlotNode),
}

impl NodeGraphScene {
    /** @emoji 🕸️ Builds a node-graph scene with optional extensions unset. */
    pub fn base(nodes_json: String, edges_json: String, viewport_json: String) -> Self {
        Self {
            nodes_json,
            edges_json,
            viewport_json,
            editable: None,
            operators_json: None,
            context_menu_json: None,
            find_items_json: None,
            selection_json: None,
            hover_json: None,
            preview_off_json: None,
            lod_json: None,
            catalogue_json: None,
            controls_json: None,
            clusters_json: None,
            computing_json: None,
            capabilities_json: None,
            fixture_json: None,
            presence_peers_json: None,
        }
    }
}

impl TextEditorScene {
    /** @emoji ✍️ Builds a text-editor scene with optional extensions unset. */
    pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
        Self {
            buffer,
            language,
            selection_json,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
        }
    }
}

//#region 🔖SceneCommands
/** @emoji 🎮 Renderer-to-plugin command names for node-graph surfaces. */
pub mod node_graph_commands {
    pub const SELECT: &str = "nodeGraphSelect";
    pub const HOVER: &str = "nodeGraphHover";
    pub const EDIT: &str = "nodeGraphEdit";
    pub const VIEWPORT: &str = "nodeGraphViewport";
    pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
}

/** @emoji ✍️ Renderer-to-plugin command names for text-editor surfaces. */
pub mod text_editor_commands {
    pub const EDIT: &str = "textEdit";
    pub const SELECT: &str = "textSelect";
    pub const HOVER: &str = "textHover";
    pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
    pub const COMMIT_RENAME: &str = "commitRename";
    pub const FORMAT_DOCUMENT: &str = "formatDocument";
}

/** @emoji 🗺️ Renderer-to-plugin command names for GIS map surfaces. */
pub mod gis_map_commands {
    pub const SET_CAMERA: &str = "setCamera";
    pub const SET_FEATURE_SELECTION: &str = "setFeatureSelection";
    pub const SET_HOVER: &str = "setHover";
    pub const SET_SELECTION_METHOD: &str = "setSelectionMethod";
    pub const SET_SELECTION_MODE: &str = "setSelectionMode";
    pub const CLEAR_SELECTION: &str = "clearSelection";
    pub const SELECT_ALL: &str = "selectAll";
    pub const DESELECT: &str = "deselect";
    pub const FOCUS_FEATURE: &str = "focusFeature";
    pub const OPEN_SOURCE: &str = "openSource";
    pub const SET_LAYER_STROKE_SCALE: &str = "setLayerStrokeScale";
    pub const FIT_WORLD: &str = "fitWorld";
}
//#endregion 🔖SceneCommands

pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: None,
        children,
    })
}

impl Default for UiNode {
    fn default() -> Self {
        ui_stack_vertical(vec![])
    }
}

pub fn ui_text(value: impl Into<String>) -> UiNode {
    UiNode::Text(UiTextNode {
        value: value.into(),
        emphasize: None,
        data_attributes: None,
    })
}

/** @emoji 🔌 Renders a contributing plugin body inline at this tree position. */
pub fn ui_external_slot(
    plugin_id: impl Into<String>,
    app_id: impl Into<String>,
    body_key: impl Into<String>,
    params_json: impl Into<String>,
) -> UiNode {
    UiNode::ExternalSlot(UiExternalSlotNode {
        plugin_id: plugin_id.into(),
        app_id: app_id.into(),
        body_key: body_key.into(),
        params_json: params_json.into(),
    })
}

fn component_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    component_kind: SurfaceKind,
    pane_id: Option<String>,
    binding_id: Option<String>,
    canvas_2d: Option<Canvas2dScene>,
    world_3d: Option<World3dScene>,
    node_graph: Option<NodeGraphScene>,
    text_editor: Option<TextEditorScene>,
    table: Option<TableScene>,
    raster: Option<RasterScene>,
    virtual_file_system: Option<VirtualFileSystemScene>,
    gis_map: Option<GisMapScene>,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind,
        pane_id,
        binding_id,
        canvas_2d,
        world_3d,
        node_graph,
        text_editor,
        table,
        raster,
        virtual_file_system,
        gis_map,
    })
}

pub fn build_canvas_2d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: Canvas2dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Canvas2d,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_world_3d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: World3dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::World3d,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_node_graph_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: NodeGraphScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::NodeGraph,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_text_editor_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TextEditorScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::TextEditor,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
    )
}

pub fn build_table_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TableScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Table,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
    )
}

pub fn build_raster_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: RasterScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Raster,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
    )
}

pub fn build_virtual_file_system_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: VirtualFileSystemScene,
    pane_id: Option<String>,
    binding_id: Option<String>,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::VirtualFileSystem,
        pane_id,
        binding_id,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
    )
}

pub fn build_gis_map_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: GisMapScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::GisMap,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
    )
}
//#endregion 🔖UiNode

//#region 🔖Manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub command: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<crate::tools::ToolNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    pub label: String,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measures: Vec<WindowMeasure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement: Option<WindowEngagement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_projection_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    pub fn side(&self) -> &'static str {
        match self {
            PanelGroup::Workbench | PanelGroup::Display => "left",
            PanelGroup::Details | PanelGroup::Settings => "right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub id: String,
    pub label: String,
    pub group: PanelGroup,
    pub body_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDefinition {
    pub id: String,
    pub label: String,
    pub document: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    pub controller_id: String,
    pub modes: Vec<ModeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    pub window_kinds: Vec<WindowKindDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
}

/// 🧭 Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDefinition {
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub document: Vec<String>,
    pub yields: String,
}

/// 🪜 Formats a canonical app document for chrome.
pub fn app_document_label(document: &[String]) -> String {
    document.join(" · ")
}

/// 🗂️ Formats a window tab within its canonical app document.
pub fn app_window_document_label(app: &AppDefinition, window_label: &str) -> String {
    let mut document = app.document.clone();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.trim().to_lowercase();
    if !normalized_window.is_empty()
        && normalized_window != normalized_app
        && document.last().is_none_or(|segment| segment.to_lowercase() != normalized_window)
    {
        document.push(normalized_window);
    }
    app_document_label(&document)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    pub label: String,
    pub document_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Contribution {
    FormsQuestionKind {
        app_id: String,
        question_kind: String,
        label: String,
        icon_id: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        default_value_json: String,
        params_body_key: String,
        preview_body_key: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub programs: Vec<ProgramDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<Contribution>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
}

//#region 🔖Kernel
pub mod kernel {
//! 🧠 Local-first command kernel contracts: commands, operations, capabilities, window I/O.

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandInvocationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelVersion(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowKindId(pub String);
//#endregion 🔖Identifiers

//#region 🔖HybridLogicalTimestamp
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self {
        Self {
            actor,
            physical_ms,
            logical: 0,
        }
    }

    pub fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub fn cmp_key(&self) -> (u64, u64) {
        (self.physical_ms, self.logical)
    }
}
//#endregion 🔖HybridLogicalTimestamp

//#region 🔖Capability
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceKind {
    Model,
    Projection,
    Window,
    Asset,
    Network,
    Backbone,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequirement {
    pub resource: ResourceKind,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub subject: PluginInstanceId,
    pub resource: ResourceId,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGrant {
    pub token: CapabilityToken,
    pub capability: Capability,
}
//#endregion 🔖Capability

//#region 🔖Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDef {
    pub id: CommandId,
    pub input_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub deterministic: bool,
    pub produces_operations: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub id: CommandInvocationId,
    pub app: AppInstanceId,
    pub command: CommandId,
    pub input: Value,
    pub actor: ActorId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causal_context: Vec<OperationId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub level: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostEffect {
    OpenWindow { kind: WindowKindId, params: Value },
    CloseWindow { window: WindowHandle },
    Notify { message: String },
    RequestSync,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDiff {
    pub schema_id: SchemaId,
    pub payload: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingCommand,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseOperation {
    pub target_operation: OperationId,
    pub inverse_diff: ModelDiff,
    pub base_version: ModelVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub undo_policy: UndoPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelOperation {
    pub id: OperationId,
    pub model: ModelHandle,
    pub base_version: ModelVersion,
    pub command_id: CommandInvocationId,
    pub diff: ModelDiff,
    pub inverse: InverseOperation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub author: ActorId,
    pub timestamp: HybridLogicalTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoGroup {
    pub command_id: CommandInvocationId,
    pub operations: Vec<OperationId>,
    pub inverse_operations: Vec<InverseOperation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    pub output: Value,
    pub operations: Vec<KernelOperation>,
    pub inverse_group: UndoGroup,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_effects: Vec<HostEffect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<AppEvent>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContext {
    pub invocation: CommandInvocation,
    pub model_projection: Value,
    pub view_state: super::ViewState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_capabilities: Vec<CapabilityGrant>,
}
//#endregion 🔖Command

//#region 🔖Sync
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PayloadHash(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpEnvelope {
    pub id: OperationId,
    pub actor: ActorId,
    pub model: ModelId,
    pub schema_version: SchemaVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deps: Vec<OperationId>,
    pub payload_hash: PayloadHash,
    pub diff: ModelDiff,
    pub inverse: InverseOperation,
}
//#endregion 🔖Sync

//#region 🔖Window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    pub invocation: CommandInvocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDef {
    pub id: WindowKindId,
    pub params_schema: SchemaId,
    pub model_projection_schema: SchemaId,
    pub input_event_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInput {
    pub window: WindowHandle,
    pub params: Value,
    pub model_projection: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<WindowEvent>,
    pub size: PhysicalSize,
    pub scale_factor: f64,
    pub theme: Theme,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub ui: super::UiNode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandRequest>,
}
//#endregion 🔖Window

//#region 🔖MergeStrategy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelKind {
    PlainRecord,
    OrderedSequence,
    TextSequence,
    TombstonedGraph,
    ContentAddressedBlob,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeStrategyKind {
    LwwRegister,
    OrderedSequence,
    TextSequence,
    TombstonedGraphSet,
    ContentAddressedBlob,
}

impl ModelKind {
    pub fn merge_strategy(&self) -> MergeStrategyKind {
        match self {
            ModelKind::PlainRecord => MergeStrategyKind::LwwRegister,
            ModelKind::OrderedSequence => MergeStrategyKind::OrderedSequence,
            ModelKind::TextSequence => MergeStrategyKind::TextSequence,
            ModelKind::TombstonedGraph => MergeStrategyKind::TombstonedGraphSet,
            ModelKind::ContentAddressedBlob => MergeStrategyKind::ContentAddressedBlob,
        }
    }
}
//#endregion 🔖MergeStrategy
}
//#endregion 🔖Kernel

#[cfg(test)]
mod app_document_tests {
    use super::app_document_label;

    #[test]
    fn formats_app_document_for_chrome() {
        assert_eq!(
            app_document_label(&["semio".into(), "puzzle".into(), "3d".into()]),
            "semio · puzzle · 3d"
        );
    }
}
//#endregion 🔖Manifest
// #endregion ui
}


pub use command_bus::{CommandBus, CommandHandler};
pub use layout::{
    collect_window_kind_ids_from_layout, create_default_layout, create_named_layout, create_stack_layout,
    create_tab_stack_layout, create_window_layout, merge_named_layouts, CommandDescriptor, NamedLayout,
    StyleSpec, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, default_viewport_engagement,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
pub use mesh::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_torus, mesh_uv_sphere, MeshData,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use tools::{tool_button, tool_collection, tool_separator, tool_toggle, ToolCategory, ToolNode};
pub use ui::*;
pub use ui::kernel::{
    ActorId, AppEvent, AppInstanceId, AssetHandle, Capability, CapabilityGrant, CapabilityRequirement,
    CapabilityToken, CommandContext, CommandDef, CommandId, CommandInvocation, CommandInvocationId,
    CommandRequest, CommandResult, Diagnostic, HostEffect, HybridLogicalTimestamp, InverseOperation,
    KernelOperation, MergeStrategyKind, ModelDiff, ModelHandle, ModelId, ModelKind, ModelVersion,
    OpEnvelope, OperationId, PayloadHash, PhysicalSize, PluginInstanceId, ResourceId, ResourceKind,
    Rights, SchemaId, SchemaVersion, Scope, Theme, UndoGroup, UndoPolicy, WindowEvent, WindowHandle,
    WindowInput, WindowKindDef, WindowKindId, WindowOutput,
};

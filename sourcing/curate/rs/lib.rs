//! 🛒 `sourcing_curate` — document model for the sourcing app's curate workflow: a filtered stock of
//! catalogue object kinds, user-adjustable filters, and a curated set with per-kind counts.

use serde::{Deserialize, Serialize};
use vcs::{Operation, OperationDiff};

pub const SOURCING_CURATE_SCHEMA: &str = "sourcing.curate/v1";

//#region 🔖Typology
/// 🌳 One node in a module's typology tree — object kinds reference a node by its path of segment ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypologyNode {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TypologyNode>,
}

impl TypologyNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>, children: Vec<TypologyNode>) -> Self {
        Self { id: id.into(), label: label.into(), children }
    }
}

/// 🔎 Whether `path` (a sequence of segment ids from the root) resolves to a node in `root`'s tree.
pub fn typology_contains(root: &TypologyNode, path: &[String]) -> bool {
    match path.split_first() {
        None => true,
        Some((head, rest)) if *head == root.id => {
            if rest.is_empty() {
                true
            } else {
                root.children.iter().any(|child| typology_contains(child, rest))
            }
        }
        _ => false,
    }
}

/// 📋 Flattens a typology tree into `(full path from root, label)` pairs, depth-first, for filter UIs.
pub fn typology_flatten(root: &TypologyNode) -> Vec<(Vec<String>, String)> {
    fn walk(node: &TypologyNode, prefix: &[String], out: &mut Vec<(Vec<String>, String)>) {
        let mut path = prefix.to_vec();
        path.push(node.id.clone());
        out.push((path.clone(), node.label.clone()));
        for child in &node.children {
            walk(child, &path, out);
        }
    }
    let mut out = Vec::new();
    walk(root, &[], &mut out);
    out
}
//#endregion 🔖Typology

//#region 🔖Geometry
/// 📦 A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GeometryRecipe {
    Box { width: f64, height: f64, depth: f64 },
    Frame { width: f64, height: f64, depth: f64, profile: f64 },
    Slab { width: f64, depth: f64, thickness: f64 },
    Mesh { positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32> },
}

/// 🧱 Flat indexed triangle mesh data, ready for `mesh_from_indexed` at the plugin boundary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshDataSpec {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

/// ➕ Appends `other` onto `base`, offsetting `other`'s indices past `base`'s existing vertex count.
pub fn append_mesh_spec(base: &mut MeshDataSpec, other: MeshDataSpec) {
    let vertex_offset = (base.positions.len() / 3) as u32;
    base.positions.extend(other.positions);
    base.normals.extend(other.normals);
    base.indices.extend(other.indices.into_iter().map(|i| i + vertex_offset));
}

/// 📐 Builds an axis-aligned box mesh centered at the origin, with per-face flat normals.
fn box_mesh_spec(width: f64, height: f64, depth: f64) -> MeshDataSpec {
    let (hw, hh, hd) = ((width * 0.5) as f32, (height * 0.5) as f32, (depth * 0.5) as f32);
    // 6 faces * 4 verts, wound counter-clockwise when viewed from outside along the face normal.
    let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
        ([0.0, 0.0, 1.0], [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]]),
        ([0.0, 0.0, -1.0], [[hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]]),
        ([1.0, 0.0, 0.0], [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]]),
        ([-1.0, 0.0, 0.0], [[-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]]),
        ([0.0, 1.0, 0.0], [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]]),
        ([0.0, -1.0, 0.0], [[-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]]),
    ];
    let mut spec = MeshDataSpec::default();
    for (normal, corners) in faces {
        let base = (spec.positions.len() / 3) as u32;
        for corner in corners {
            spec.positions.extend(corner);
            spec.normals.extend(normal);
        }
        spec.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    spec
}

/// 🪟 Builds a rectangular frame (4 mitred boxes: top/bottom rails, left/right stiles) around an opening.
fn frame_mesh_spec(width: f64, height: f64, depth: f64, profile: f64) -> MeshDataSpec {
    let mut spec = MeshDataSpec::default();
    let mut add = |w: f64, h: f64, cx: f64, cy: f64| {
        let mut piece = box_mesh_spec(w, h, depth);
        for i in (0..piece.positions.len()).step_by(3) {
            piece.positions[i] += cx as f32;
            piece.positions[i + 1] += cy as f32;
        }
        append_mesh_spec(&mut spec, piece);
    };
    let half_h = height * 0.5;
    let half_w = width * 0.5;
    add(width, profile, 0.0, half_h - profile * 0.5);
    add(width, profile, 0.0, -half_h + profile * 0.5);
    let stile_h = height - profile * 2.0;
    add(profile, stile_h, -half_w + profile * 0.5, 0.0);
    add(profile, stile_h, half_w - profile * 0.5, 0.0);
    spec
}

/// 🧱 Realizes a `GeometryRecipe` into flat mesh data.
pub fn mesh_spec_for(recipe: &GeometryRecipe) -> MeshDataSpec {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => box_mesh_spec(*width, *height, *depth),
        GeometryRecipe::Frame { width, height, depth, profile } => frame_mesh_spec(*width, *height, *depth, *profile),
        GeometryRecipe::Slab { width, depth, thickness } => box_mesh_spec(*width, *thickness, *depth),
        GeometryRecipe::Mesh { positions, normals, indices } => MeshDataSpec { positions: positions.clone(), normals: normals.clone(), indices: indices.clone() },
    }
}

/// 📏 The largest bounding dimension of a recipe's geometry, used to normalize grid-cell scale.
pub fn bounding_extent(recipe: &GeometryRecipe) -> f64 {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => width.max(*height).max(*depth),
        GeometryRecipe::Frame { width, height, depth, .. } => width.max(*height).max(*depth),
        GeometryRecipe::Slab { width, depth, thickness } => width.max(*depth).max(*thickness),
        GeometryRecipe::Mesh { positions, .. } => positions.chunks(3).flat_map(|p| p.iter().map(|v| v.abs() as f64 * 2.0)).fold(0.0_f64, f64::max).max(1e-6),
    }
}
//#endregion 🔖Geometry

//#region 🔖ObjectKind
/// 🧱 A catalogue object KIND: identity ∘ typology reference ∘ availability ∘ geometry (composition, not subclassing).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKind {
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub typology_path: Vec<String>,
    pub availability: u32,
    pub geometry: GeometryRecipe,
}
//#endregion 🔖ObjectKind

//#region 🔖Document
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSort {
    pub column_id: String,
    pub direction: SortDirection,
}

/// 🔍 The pool table's active filter set — narrows `CurateDocument::stock` down to `filtered_stock()`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub module_ids: Vec<String>,
    #[serde(default)]
    pub typology_path: Vec<String>,
    #[serde(default)]
    pub min_availability: u32,
    #[serde(default)]
    pub sort: Option<TableSort>,
}

/// 🧺 One curated object kind and how many units of it have been picked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CuratedItem {
    pub object_id: String,
    pub count: u32,
}

/// 🖱️ Ephemeral cross-window UI state — which single object is selected for the preview window.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurateRuntime {
    #[serde(default)]
    pub selected_object_id: Option<String>,
}

/// 🛒 The curate document: a stock of catalogue kinds ∘ filters ∘ a curated set ∘ ephemeral runtime state.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurateDocument {
    #[serde(default)]
    pub stock: Vec<ObjectKind>,
    #[serde(default)]
    pub filters: Filters,
    #[serde(default)]
    pub curated: Vec<CuratedItem>,
    #[serde(default)]
    pub runtime: CurateRuntime,
}

impl CurateDocument {
    /// 🔎 The stock kinds that currently satisfy every active filter dimension.
    pub fn filtered_stock(&self) -> Vec<&ObjectKind> {
        self.stock
            .iter()
            .filter(|kind| {
                let query = self.filters.query.trim().to_lowercase();
                let matches_query = query.is_empty() || kind.name.to_lowercase().contains(&query);
                let matches_module = self.filters.module_ids.is_empty() || self.filters.module_ids.contains(&kind.module_id);
                let matches_typology = self.filters.typology_path.is_empty() || kind.typology_path.starts_with(&self.filters.typology_path);
                let matches_availability = kind.availability >= self.filters.min_availability;
                matches_query && matches_module && matches_typology && matches_availability
            })
            .collect()
    }

    /// 🔢 How many units of `object_id` are currently in the curated set (0 if absent).
    pub fn curated_count(&self, object_id: &str) -> u32 {
        self.curated.iter().find(|item| item.object_id == object_id).map(|item| item.count).unwrap_or(0)
    }

    /// ➕➖ Adjusts the curated count for `object_id` by `delta`, clamped to `0..=availability`; removes the
    /// entry entirely when the count reaches 0. Silently no-operations if `object_id` isn't in the stock.
    pub fn curate_delta(&mut self, object_id: &str, delta: i64) {
        let Some(kind) = self.stock.iter().find(|kind| kind.id == object_id) else { return };
        let next = (self.curated_count(object_id) as i64 + delta).clamp(0, kind.availability as i64) as u32;
        self.curate_set(object_id, next);
    }

    /// 🎯 Sets the curated count for `object_id` directly, clamped to `0..=availability`; removes the
    /// entry when the count is 0. Silently no-operations if `object_id` isn't in the stock.
    pub fn curate_set(&mut self, object_id: &str, count: u32) {
        let Some(kind) = self.stock.iter().find(|kind| kind.id == object_id) else { return };
        let clamped = count.min(kind.availability);
        match self.curated.iter_mut().find(|item| item.object_id == object_id) {
            Some(item) if clamped == 0 => {
                let id = item.object_id.clone();
                self.curated.retain(|item| item.object_id != id);
            }
            Some(item) => item.count = clamped,
            None if clamped > 0 => self.curated.push(CuratedItem { object_id: object_id.to_string(), count: clamped }),
            None => {}
        }
    }
}
//#endregion 🔖Document

//#region 🔖Operations
/// 🛒 Curate document operation: currently always a wholesale swap — every action recomputes the
/// full document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SourcingOperation {
    SetDocument { document: CurateDocument },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcingDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<CurateDocument>,
}

impl OperationDiff<CurateDocument> for SourcingDiff {
    fn apply(&self, projection: &CurateDocument) -> CurateDocument {
        self.document.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}

impl Operation<CurateDocument> for SourcingOperation {
    type Diff = SourcingDiff;

    fn diff(&self, _projection: &CurateDocument) -> Self::Diff {
        match self {
            SourcingOperation::SetDocument { document } => SourcingDiff { document: Some(document.clone()) },
        }
    }

    fn backwards(&self, projection: &CurateDocument) -> Vec<Self> {
        match self {
            SourcingOperation::SetDocument { .. } => vec![SourcingOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer/printer for `CurateDocument`'s `.curate` DSL (`🔖Dsl`) and for
/// `SourcingOperation`'s one-line op text (`🔖OpText`) — both share the same `marker key=value ...
/// "trailing text"` line grammar `vcs`'s own structural lines use, hand-rolled locally since `vcs`'s
/// escaping helpers are private to that crate.
mod curate_dsl {
    use super::{CuratedItem, CurateDocument, CurateRuntime, Filters, GeometryRecipe, ObjectKind, SortDirection, SourcingOperation, TableSort};
    use std::collections::HashMap;
    use vcs::{TextError, TextSpan};

    //#region 🔖Lexer
    /// 🔐 Escapes `\`, `"` and newlines so arbitrary source text fits inside one quoted field.
    fn escape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out
    }

    fn unescape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        let mut chars = value.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🔎 Finds the char index of the unescaped opening `"` of a trailing quoted field, mirroring
    /// `vcs`'s private `find_unescaped_trailing_quote` (kept in lock-step, see that doc comment).
    fn find_unescaped_trailing_quote(chars: &[char]) -> Option<usize> {
        if chars.is_empty() || *chars.last().unwrap() != '"' {
            return None;
        }
        let last = chars.len() - 1;
        let mut i = last;
        while i > 0 {
            i -= 1;
            if chars[i] == '"' {
                let mut backslashes = 0;
                let mut j = i;
                while j > 0 && chars[j - 1] == '\\' {
                    backslashes += 1;
                    j -= 1;
                }
                if backslashes % 2 == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    /// 🧾 One parsed `marker key=value ...` line plus its optional trailing quoted text field.
    struct KvLine {
        marker: String,
        fields: HashMap<String, String>,
        text: Option<String>,
    }

    fn parse_kv_line(line: &str, line_no: u32) -> Result<KvLine, TextError> {
        let chars: Vec<char> = line.chars().collect();
        let (head, text) = match find_unescaped_trailing_quote(&chars) {
            Some(open) => {
                let content: String = chars[open + 1..chars.len() - 1].iter().collect();
                let head: String = chars[..open].iter().collect();
                (head.trim_end().to_string(), Some(unescape_text(&content)))
            }
            None => (line.to_string(), None),
        };
        let mut tokens = head.split_whitespace();
        let marker = tokens
            .next()
            .ok_or_else(|| TextError::new("expected a marker", TextSpan::at(line_no, 1)))?
            .to_string();
        let mut fields = HashMap::new();
        for token in tokens {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| TextError::new(format!("expected key=value token, got '{token}'"), TextSpan::at(line_no, 1)))?;
            fields.insert(key.to_string(), value.to_string());
        }
        Ok(KvLine { marker, fields, text })
    }

    fn field<'a>(fields: &'a HashMap<String, String>, key: &str, line_no: u32) -> Result<&'a str, TextError> {
        fields.get(key).map(|value| value.as_str()).ok_or_else(|| TextError::new(format!("missing field '{key}'"), TextSpan::at(line_no, 1)))
    }

    fn parse_u32(value: &str, key: &str, line_no: u32) -> Result<u32, TextError> {
        value.parse::<u32>().map_err(|_| TextError::new(format!("expected integer for '{key}', got '{value}'"), TextSpan::at(line_no, 1)))
    }

    /// 🛤️ `-` marks an empty `/`-joined path (an empty path would otherwise print as `""`, ambiguous
    /// with a genuine one-empty-segment path).
    fn join_path(segments: &[String]) -> String {
        if segments.is_empty() { "-".to_string() } else { segments.join("/") }
    }

    fn split_path(value: &str) -> Vec<String> {
        if value == "-" { Vec::new() } else { value.split('/').map(String::from).collect() }
    }

    fn join_ids(ids: &[String]) -> String {
        if ids.is_empty() { "-".to_string() } else { ids.join(",") }
    }

    fn split_ids(value: &str) -> Vec<String> {
        if value == "-" { Vec::new() } else { value.split(',').map(String::from).collect() }
    }
    //#endregion 🔖Lexer

    //#region 🔖Geometry
    fn join_f32(values: &[f32]) -> String {
        values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    }

    fn join_u32_list(values: &[u32]) -> String {
        values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    }

    fn split_f32_csv(value: &str, line_no: u32) -> Result<Vec<f32>, TextError> {
        if value.is_empty() {
            return Ok(Vec::new());
        }
        value.split(',').map(|part| part.parse::<f32>().map_err(|_| TextError::new(format!("expected number, got '{part}'"), TextSpan::at(line_no, 1)))).collect()
    }

    fn split_u32_csv(value: &str, line_no: u32) -> Result<Vec<u32>, TextError> {
        if value.is_empty() {
            return Ok(Vec::new());
        }
        value.split(',').map(|part| part.parse::<u32>().map_err(|_| TextError::new(format!("expected integer, got '{part}'"), TextSpan::at(line_no, 1)))).collect()
    }

    fn parse_f64_field(value: &str, line_no: u32) -> Result<f64, TextError> {
        value.parse::<f64>().map_err(|_| TextError::new(format!("expected number, got '{value}'"), TextSpan::at(line_no, 1)))
    }

    /// 📤 Prints a `GeometryRecipe` as one whitespace-free token: `box:w,h,d` | `frame:w,h,d,profile` |
    /// `slab:w,d,t` | `mesh:pos,pos,...;norm,norm,...;idx,idx,...`.
    fn print_geometry(recipe: &GeometryRecipe) -> String {
        match recipe {
            GeometryRecipe::Box { width, height, depth } => format!("box:{width},{height},{depth}"),
            GeometryRecipe::Frame { width, height, depth, profile } => format!("frame:{width},{height},{depth},{profile}"),
            GeometryRecipe::Slab { width, depth, thickness } => format!("slab:{width},{depth},{thickness}"),
            GeometryRecipe::Mesh { positions, normals, indices } => format!("mesh:{};{};{}", join_f32(positions), join_f32(normals), join_u32_list(indices)),
        }
    }

    /// 📥 Parses one geometry token (see {@link print_geometry}).
    fn parse_geometry(token: &str, line_no: u32) -> Result<GeometryRecipe, TextError> {
        let (kind, rest) = token
            .split_once(':')
            .ok_or_else(|| TextError::new(format!("expected 'kind:params' geometry, got '{token}'"), TextSpan::at(line_no, 1)))?;
        match kind {
            "box" => {
                let parts: Vec<&str> = rest.split(',').collect();
                if parts.len() != 3 {
                    return Err(TextError::new(format!("expected 3 box params, got '{rest}'"), TextSpan::at(line_no, 1)));
                }
                Ok(GeometryRecipe::Box { width: parse_f64_field(parts[0], line_no)?, height: parse_f64_field(parts[1], line_no)?, depth: parse_f64_field(parts[2], line_no)? })
            }
            "frame" => {
                let parts: Vec<&str> = rest.split(',').collect();
                if parts.len() != 4 {
                    return Err(TextError::new(format!("expected 4 frame params, got '{rest}'"), TextSpan::at(line_no, 1)));
                }
                Ok(GeometryRecipe::Frame {
                    width: parse_f64_field(parts[0], line_no)?,
                    height: parse_f64_field(parts[1], line_no)?,
                    depth: parse_f64_field(parts[2], line_no)?,
                    profile: parse_f64_field(parts[3], line_no)?,
                })
            }
            "slab" => {
                let parts: Vec<&str> = rest.split(',').collect();
                if parts.len() != 3 {
                    return Err(TextError::new(format!("expected 3 slab params, got '{rest}'"), TextSpan::at(line_no, 1)));
                }
                Ok(GeometryRecipe::Slab { width: parse_f64_field(parts[0], line_no)?, depth: parse_f64_field(parts[1], line_no)?, thickness: parse_f64_field(parts[2], line_no)? })
            }
            "mesh" => {
                let segments: Vec<&str> = rest.splitn(3, ';').collect();
                if segments.len() != 3 {
                    return Err(TextError::new(format!("expected 'positions;normals;indices', got '{rest}'"), TextSpan::at(line_no, 1)));
                }
                Ok(GeometryRecipe::Mesh {
                    positions: split_f32_csv(segments[0], line_no)?,
                    normals: split_f32_csv(segments[1], line_no)?,
                    indices: split_u32_csv(segments[2], line_no)?,
                })
            }
            other => Err(TextError::expected(format!("unknown geometry kind '{other}'"), TextSpan::at(line_no, 1), "box | frame | slab | mesh")),
        }
    }
    //#endregion 🔖Geometry

    //#region 🔖Sections
    fn print_stock_item(kind: &ObjectKind, out: &mut String) {
        out.push_str(&format!(
            "  kind id={} module={} availability={} typology={} geometry={} \"{}\"\n",
            kind.id,
            kind.module_id,
            kind.availability,
            join_path(&kind.typology_path),
            print_geometry(&kind.geometry),
            escape_text(&kind.name),
        ));
    }

    fn parse_stock_item(line: &str, line_no: u32) -> Result<ObjectKind, TextError> {
        let parsed = parse_kv_line(line, line_no)?;
        if parsed.marker != "kind" {
            return Err(TextError::expected(format!("expected a 'kind' line, got '{}'", parsed.marker), TextSpan::at(line_no, 1), "kind"));
        }
        Ok(ObjectKind {
            id: field(&parsed.fields, "id", line_no)?.to_string(),
            name: parsed.text.ok_or_else(|| TextError::new("kind requires a quoted name field", TextSpan::at(line_no, 1)))?,
            module_id: field(&parsed.fields, "module", line_no)?.to_string(),
            typology_path: split_path(field(&parsed.fields, "typology", line_no)?),
            availability: parse_u32(field(&parsed.fields, "availability", line_no)?, "availability", line_no)?,
            geometry: parse_geometry(field(&parsed.fields, "geometry", line_no)?, line_no)?,
        })
    }

    fn print_curated_item(item: &CuratedItem, out: &mut String) {
        out.push_str(&format!("  pick {} {}\n", item.object_id, item.count));
    }

    fn parse_curated_item(line: &str, line_no: u32) -> Result<CuratedItem, TextError> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 3 || tokens[0] != "pick" {
            return Err(TextError::expected(format!("expected 'pick <objectId> <count>', got '{line}'"), TextSpan::at(line_no, 1), "pick <objectId> <count>"));
        }
        Ok(CuratedItem { object_id: tokens[1].to_string(), count: parse_u32(tokens[2], "count", line_no)? })
    }

    fn print_filters(filters: &Filters) -> String {
        let sort = match &filters.sort {
            Some(sort) => format!("{}:{}", sort.column_id, if sort.direction == SortDirection::Desc { "desc" } else { "asc" }),
            None => "-".to_string(),
        };
        format!(
            "filters modules={} typology={} minAvailability={} sort={} \"{}\"\n",
            join_ids(&filters.module_ids),
            join_path(&filters.typology_path),
            filters.min_availability,
            sort,
            escape_text(&filters.query),
        )
    }

    fn parse_filters(line: &str, line_no: u32) -> Result<Filters, TextError> {
        let parsed = parse_kv_line(line, line_no)?;
        if parsed.marker != "filters" {
            return Err(TextError::expected(format!("expected a 'filters' line, got '{}'", parsed.marker), TextSpan::at(line_no, 1), "filters"));
        }
        let sort_field = field(&parsed.fields, "sort", line_no)?;
        let sort = if sort_field == "-" {
            None
        } else {
            let (column_id, direction) = sort_field
                .split_once(':')
                .ok_or_else(|| TextError::new(format!("expected 'columnId:asc|desc', got '{sort_field}'"), TextSpan::at(line_no, 1)))?;
            Some(TableSort { column_id: column_id.to_string(), direction: if direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } })
        };
        Ok(Filters {
            query: parsed.text.unwrap_or_default(),
            module_ids: split_ids(field(&parsed.fields, "modules", line_no)?),
            typology_path: split_path(field(&parsed.fields, "typology", line_no)?),
            min_availability: parse_u32(field(&parsed.fields, "minAvailability", line_no)?, "minAvailability", line_no)?,
            sort,
        })
    }

    fn print_runtime(runtime: &CurateRuntime) -> String {
        format!("runtime selected={}\n", runtime.selected_object_id.clone().unwrap_or_else(|| "-".to_string()))
    }

    fn parse_runtime(line: &str, line_no: u32) -> Result<CurateRuntime, TextError> {
        let parsed = parse_kv_line(line, line_no)?;
        if parsed.marker != "runtime" {
            return Err(TextError::expected(format!("expected a 'runtime' line, got '{}'", parsed.marker), TextSpan::at(line_no, 1), "runtime"));
        }
        let selected = field(&parsed.fields, "selected", line_no)?;
        Ok(CurateRuntime { selected_object_id: if selected == "-" { None } else { Some(selected.to_string()) } })
    }
    //#endregion 🔖Sections

    //#region 🔖Document
    /// 📥 Parses a full `.curate` document: `stock`/`curated` sections (one two-space-indented item
    /// per line) plus single `filters`/`runtime` lines, in any order — see {@link print_document}.
    pub fn parse_document(text: &str) -> Result<CurateDocument, TextError> {
        let mut stock = Vec::new();
        let mut filters = Filters::default();
        let mut curated = Vec::new();
        let mut runtime = CurateRuntime::default();
        let mut section: Option<&str> = None;

        for (index, raw_line) in text.lines().enumerate() {
            let line_no = index as u32 + 1;
            if raw_line.trim().is_empty() {
                continue;
            }
            if let Some(indented) = raw_line.strip_prefix("  ") {
                match section {
                    Some("stock") => stock.push(parse_stock_item(indented, line_no)?),
                    Some("curated") => curated.push(parse_curated_item(indented, line_no)?),
                    _ => return Err(TextError::new("indented line outside a 'stock' or 'curated' section", TextSpan::at(line_no, 1))),
                }
                continue;
            }
            let trimmed = raw_line.trim();
            let marker = trimmed.split_whitespace().next().unwrap_or("");
            match marker {
                "stock" => section = Some("stock"),
                "curated" => section = Some("curated"),
                "filters" => {
                    filters = parse_filters(trimmed, line_no)?;
                    section = None;
                }
                "runtime" => {
                    runtime = parse_runtime(trimmed, line_no)?;
                    section = None;
                }
                other => return Err(TextError::expected(format!("unknown section '{other}'"), TextSpan::at(line_no, 1), "stock | filters | curated | runtime")),
            }
        }
        Ok(CurateDocument { stock, filters, curated, runtime })
    }

    /// 📤 Prints a `CurateDocument` back to its `.curate` DSL form (see {@link parse_document}).
    pub fn print_document(document: &CurateDocument) -> String {
        let mut out = String::new();
        out.push_str("stock\n");
        for kind in &document.stock {
            print_stock_item(kind, &mut out);
        }
        out.push_str(&print_filters(&document.filters));
        out.push_str("curated\n");
        for item in &document.curated {
            print_curated_item(item, &mut out);
        }
        out.push_str(&print_runtime(&document.runtime));
        out
    }
    //#endregion 🔖Document

    //#region 🔖Operation
    /// 📥 Parses a single one-line `SourcingOperation`: `setDocument "<escaped .curate document>"`.
    pub fn parse_operation(line: &str) -> Result<SourcingOperation, TextError> {
        let parsed = parse_kv_line(line, 1)?;
        match parsed.marker.as_str() {
            "setDocument" => {
                let text = parsed.text.ok_or_else(|| TextError::new("setDocument requires a quoted document field", TextSpan::at(1, 1)))?;
                Ok(SourcingOperation::SetDocument { document: parse_document(&text)? })
            }
            other => Err(TextError::expected(format!("unknown sourcing operation '{other}'"), TextSpan::at(1, 1), "setDocument")),
        }
    }

    /// 📤 Prints a `SourcingOperation` back to its one-line op text (see {@link parse_operation}).
    pub fn print_operation(operation: &SourcingOperation) -> String {
        match operation {
            SourcingOperation::SetDocument { document } => format!("setDocument \"{}\"", escape_text(&print_document(document))),
        }
    }
    //#endregion 🔖Operation
}

impl vcs::DocumentDsl for CurateDocument {
    const EXTENSION: &'static str = "curate";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        curate_dsl::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        curate_dsl::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for SourcingOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        curate_dsl::parse_operation(line)
    }

    fn print_op(&self) -> String {
        curate_dsl::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖Modules
/// 🧩 A sourcing module composes a typology subtree, demo catalogue kinds, and preview meshing for one
/// object family (e.g. beams, windows, slabs) — modules are trait objects, not subclasses of a base app.
pub trait SourcingModule {
    fn module_id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn typology(&self) -> TypologyNode;
    fn demo_kinds(&self) -> Vec<ObjectKind>;
    /// 🧱 Realizes a kind's preview mesh; defaults to the generic geometry recipe realization.
    fn preview_mesh(&self, kind: &ObjectKind) -> MeshDataSpec {
        mesh_spec_for(&kind.geometry)
    }
}

pub mod beams {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct BeamsModule;

    impl SourcingModule for BeamsModule {
        fn module_id(&self) -> &'static str {
            "beams"
        }
        fn label(&self) -> &'static str {
            "Beams"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new(
                "beams",
                "Beams",
                vec![
                    TypologyNode::new("solid-timber", "Solid Timber", vec![TypologyNode::new("glulam", "Glulam", vec![]), TypologyNode::new("kvh", "KVH", vec![])]),
                    TypologyNode::new("steel", "Steel", vec![TypologyNode::new("ipe", "IPE", vec![]), TypologyNode::new("hea", "HEA", vec![])]),
                ],
            )
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "beam-glulam-gl24h".into(),
                    name: "Glulam GL24h 200×400".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "glulam".into()],
                    availability: 24,
                    geometry: GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 },
                },
                ObjectKind {
                    id: "beam-kvh-c24".into(),
                    name: "KVH C24 100×200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "kvh".into()],
                    availability: 60,
                    geometry: GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 4.0 },
                },
                ObjectKind {
                    id: "beam-steel-ipe200".into(),
                    name: "Steel IPE 200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "ipe".into()],
                    availability: 12,
                    geometry: GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 5.0 },
                },
                ObjectKind {
                    id: "beam-steel-hea160".into(),
                    name: "Steel HEA 160".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "hea".into()],
                    availability: 8,
                    geometry: GeometryRecipe::Box { width: 0.16, height: 0.152, depth: 5.0 },
                },
            ]
        }
    }
}

pub mod windows {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct WindowsModule;

    impl SourcingModule for WindowsModule {
        fn module_id(&self) -> &'static str {
            "windows"
        }
        fn label(&self) -> &'static str {
            "Windows"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("windows", "Windows", vec![TypologyNode::new("casement", "Casement", vec![]), TypologyNode::new("fixed", "Fixed", vec![]), TypologyNode::new("tilt-turn", "Tilt & Turn", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "window-casement-100x120".into(),
                    name: "Casement Window 100×120".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "casement".into()],
                    availability: 18,
                    geometry: GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 },
                },
                ObjectKind {
                    id: "window-fixed-150x150".into(),
                    name: "Fixed Window 150×150".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "fixed".into()],
                    availability: 10,
                    geometry: GeometryRecipe::Frame { width: 1.5, height: 1.5, depth: 0.06, profile: 0.06 },
                },
                ObjectKind {
                    id: "window-tilt-turn-120x140".into(),
                    name: "Tilt & Turn Window 120×140".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "tilt-turn".into()],
                    availability: 14,
                    geometry: GeometryRecipe::Frame { width: 1.2, height: 1.4, depth: 0.09, profile: 0.09 },
                },
            ]
        }
    }
}

pub mod slabs {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct SlabsModule;

    impl SourcingModule for SlabsModule {
        fn module_id(&self) -> &'static str {
            "slabs"
        }
        fn label(&self) -> &'static str {
            "Slabs"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("slabs", "Slabs", vec![TypologyNode::new("concrete", "Concrete", vec![]), TypologyNode::new("clt", "CLT", vec![]), TypologyNode::new("hollow-core", "Hollow Core", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "slab-concrete-240".into(),
                    name: "Concrete Slab 240mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "concrete".into()],
                    availability: 30,
                    geometry: GeometryRecipe::Slab { width: 2.4, depth: 1.2, thickness: 0.24 },
                },
                ObjectKind {
                    id: "slab-clt-160".into(),
                    name: "CLT Slab 160mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "clt".into()],
                    availability: 20,
                    geometry: GeometryRecipe::Slab { width: 2.95, depth: 1.25, thickness: 0.16 },
                },
                ObjectKind {
                    id: "slab-hollow-core-265".into(),
                    name: "Hollow Core Slab 265mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "hollow-core".into()],
                    availability: 16,
                    geometry: GeometryRecipe::Slab { width: 1.2, depth: 6.0, thickness: 0.265 },
                },
            ]
        }
    }
}

/// 🧩 Every sourcing module known to this crate, in stable order.
pub fn sourcing_modules() -> Vec<Box<dyn SourcingModule>> {
    vec![Box::new(beams::BeamsModule), Box::new(windows::WindowsModule), Box::new(slabs::SlabsModule)]
}

/// 🔎 Looks up a single module by id.
pub fn module_for(module_id: &str) -> Option<Box<dyn SourcingModule>> {
    sourcing_modules().into_iter().find(|module| module.module_id() == module_id)
}
//#endregion 🔖Modules

//#region 🔖GridLayout
/// 🔢 Places item `index` of `count` total on a `ceil(sqrt(count))`-column grid, centered at the origin,
/// with `cell` spacing between slots — used to lay out the "all objects" 3D grid window.
pub fn grid_placement(count: usize, index: usize, cell: f64) -> (f64, f64) {
    if count == 0 {
        return (0.0, 0.0);
    }
    let columns = (count as f64).sqrt().ceil().max(1.0) as usize;
    let rows = count.div_ceil(columns);
    let column = index % columns;
    let row = index / columns;
    let x = (column as f64 - (columns as f64 - 1.0) * 0.5) * cell;
    let z = (row as f64 - (rows as f64 - 1.0) * 0.5) * cell;
    (x, z)
}

/// 📏 The uniform scale factor that fits a recipe's largest dimension inside a `cell`-sized grid slot.
pub fn grid_scale(recipe: &GeometryRecipe, cell: f64) -> f64 {
    let extent = bounding_extent(recipe);
    if extent <= 0.0 {
        1.0
    } else {
        cell / extent
    }
}
//#endregion 🔖GridLayout

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn filtered_stock_matches_query() {
        let mut document = sample_document();
        document.filters.query = "glulam".into();
        let filtered = document.filtered_stock();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "beam-glulam-gl24h");
    }

    #[test]
    fn filtered_stock_matches_module() {
        let mut document = sample_document();
        document.filters.module_ids = vec!["slabs".into()];
        let filtered = document.filtered_stock();
        assert!(filtered.iter().all(|kind| kind.module_id == "slabs"));
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn filtered_stock_matches_typology_prefix() {
        let mut document = sample_document();
        document.filters.typology_path = vec!["beams".into(), "steel".into()];
        let filtered = document.filtered_stock();
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|kind| kind.typology_path.starts_with(&["beams".to_string(), "steel".to_string()])));
    }

    #[test]
    fn filtered_stock_matches_min_availability() {
        let mut document = sample_document();
        document.filters.min_availability = 20;
        let filtered = document.filtered_stock();
        assert!(filtered.iter().all(|kind| kind.availability >= 20));
        assert!(!filtered.is_empty());
    }

    #[test]
    fn curate_delta_clamps_to_availability_and_zero_floor() {
        let mut document = sample_document();
        document.curate_delta("beam-steel-hea160", 100);
        assert_eq!(document.curated_count("beam-steel-hea160"), 8);
        document.curate_delta("beam-steel-hea160", -1000);
        assert_eq!(document.curated_count("beam-steel-hea160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_delta_unknown_object_is_noop() {
        let mut document = sample_document();
        document.curate_delta("does-not-exist", 5);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_set_removes_entry_at_zero() {
        let mut document = sample_document();
        document.curate_set("slab-clt-160", 5);
        assert_eq!(document.curated_count("slab-clt-160"), 5);
        document.curate_set("slab-clt-160", 0);
        assert_eq!(document.curated_count("slab-clt-160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn typology_contains_and_flatten() {
        let module = beams::BeamsModule;
        let tree = module.typology();
        assert!(typology_contains(&tree, &["beams".into(), "steel".into(), "ipe".into()]));
        assert!(!typology_contains(&tree, &["beams".into(), "concrete".into()]));
        let flattened = typology_flatten(&tree);
        assert!(flattened.iter().any(|(path, _)| path == &vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]));
    }

    fn assert_mesh_spec_is_valid(spec: &MeshDataSpec) {
        assert!(!spec.positions.is_empty());
        assert_eq!(spec.positions.len() % 3, 0);
        assert_eq!(spec.positions.len(), spec.normals.len());
        assert_eq!(spec.indices.len() % 3, 0);
        let vertex_count = (spec.positions.len() / 3) as u32;
        assert!(spec.indices.iter().all(|&i| i < vertex_count));
    }

    #[test]
    fn box_recipe_produces_valid_mesh() {
        assert_mesh_spec_is_valid(&mesh_spec_for(&GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }));
    }

    #[test]
    fn frame_recipe_concatenates_four_pieces_into_a_valid_mesh() {
        let spec = mesh_spec_for(&GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 });
        assert_mesh_spec_is_valid(&spec);
        let single_box = box_mesh_spec(1.0, 0.08, 0.08);
        assert_eq!(spec.positions.len(), single_box.positions.len() * 4);
        assert_eq!(spec.indices.len(), single_box.indices.len() * 4);
    }

    #[test]
    fn grid_placement_centers_around_origin() {
        let positions: Vec<(f64, f64)> = (0..9).map(|i| grid_placement(9, i, 2.0)).collect();
        let sum_x: f64 = positions.iter().map(|(x, _)| x).sum();
        let sum_z: f64 = positions.iter().map(|(_, z)| z).sum();
        assert!(sum_x.abs() < 1e-9);
        assert!(sum_z.abs() < 1e-9);
        let unique: std::collections::HashSet<(i64, i64)> = positions.iter().map(|(x, z)| ((x * 1000.0) as i64, (z * 1000.0) as i64)).collect();
        assert_eq!(unique.len(), 9);
    }

    #[test]
    fn grid_scale_normalizes_to_cell_size() {
        let recipe = GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 };
        let scale = grid_scale(&recipe, 2.0);
        assert!((bounding_extent(&recipe) * scale - 2.0).abs() < 1e-9);
    }

    //#region 🔖Dsl
    #[test]
    fn curate_document_dsl_round_trips_sample_and_empty() {
        vcs::test_support::assert_dsl_round_trip(&sample_document());
        vcs::test_support::assert_dsl_round_trip(&CurateDocument::default());
    }

    #[test]
    fn curate_document_dsl_round_trips_a_mesh_kind_and_a_curated_selection() {
        let mut document = CurateDocument {
            stock: vec![ObjectKind {
                id: "beam-mesh-custom".into(),
                name: "Custom \"Beam\" \\ Mesh".into(),
                module_id: "beams".into(),
                typology_path: vec!["beams".into(), "steel".into()],
                availability: 5,
                geometry: GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] },
            }],
            ..Default::default()
        };
        document.curate_set("beam-mesh-custom", 2);
        document.filters.module_ids = vec!["beams".into(), "windows".into()];
        document.filters.typology_path = vec!["beams".into(), "steel".into()];
        document.filters.min_availability = 1;
        document.filters.query = "steel \"ipe\"".into();
        document.filters.sort = Some(TableSort { column_id: "availability".into(), direction: SortDirection::Desc });
        document.runtime.selected_object_id = Some("beam-mesh-custom".into());
        vcs::test_support::assert_dsl_round_trip(&document);
    }
    //#endregion 🔖Dsl

    //#region 🔖OpText
    #[test]
    fn set_document_op_text_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&SourcingOperation::SetDocument { document: sample_document() });
        vcs::test_support::assert_op_line_round_trip(&SourcingOperation::SetDocument { document: CurateDocument::default() });
    }
    //#endregion 🔖OpText

    //#region 🔖DslAndOpTextStore
    #[test]
    fn curate_document_text_round_trips_through_a_vcs_store() {
        let envelope = vcs::create_document_vcs_envelope(SOURCING_CURATE_SCHEMA, "sourcing-curate-test", sample_document(), None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let mut next = store.projection().expect("projection").clone();
        next.curate_delta("beam-glulam-gl24h", 3);
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![SourcingOperation::SetDocument { document: next }], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslAndOpTextStore
}
//#endregion 🔖Tests

"""🗺️ Stage shared terrain face topology and borrowed scene export after NoData storage."""

from pathlib import Path
import difflib
import json
import re

TICKET = Path(__file__).resolve().parent
base_script = TICKET / "stage-terrain-nodata.py"
baseline = {"__file__": str(base_script)}
exec(compile(base_script.read_text(encoding="utf-8").split("\npatch = []", 1)[0], str(base_script), "exec"), baseline)
GIS = baseline["GIS"]
SURFACE = baseline["SURFACE"]
base = baseline["changes"]
changes = {}

def original(path):
    """📖 Returns the staged storage version or the current live source."""
    return base[path] if path in base else path.read_text(encoding="utf-8") if path.exists() else ""

def read(path):
    """🪡 Returns the current staged transformation."""
    return changes.get(path, original(path))

def edit(path, old, new, count=1):
    """🧷 Refuses a replacement whose input anchors have moved."""
    text = read(path)
    assert text.count(old) == count, (path, old, text.count(old), count)
    changes[path] = text.replace(old, new)

core = SURFACE / "🦀️.rs"
grid = SURFACE / "📐️grid-projection/🦀️.rs"
topology = SURFACE / "🔺️grid-topology/🦀️.rs"
changes[topology] = '''//! 🔺️ Shared north-first triangle topology for authored and streamed terrain.

use super::missing_samples::is_missing;

/// 🧩 Bounded grid geometry with optional canonical south-first missing ordinals.
#[derive(Clone, Copy)]
pub struct TerrainGridTopology<'a> {
    width: u32,
    height: u32,
    missing_samples: Option<&'a str>,
}

/// 📐 One half of a cell and the three vertices in renderer winding order.
#[derive(Clone, Copy)]
pub struct TerrainGridFace {
    pub vertices: [u32; 3],
    row: u32,
    column: u32,
    half: usize,
}

impl TerrainGridFace {
    /// ⚖️ Interpolates in north-first grid coordinates on this face.
    pub fn weights(self, [x, y]: [f64; 2]) -> [f64; 3] {
        let u = x - f64::from(self.column);
        let v = y - f64::from(self.row);
        if self.half == 0 { [1.0 - u - v, v, u] } else { [1.0 - v, 1.0 - u, u + v - 1.0] }
    }
}

impl<'a> TerrainGridTopology<'a> {
    /// 🗺️ Uses admitted dimensions and a validated authored mask.
    pub fn new([width, height]: [u32; 2], missing_samples: Option<&'a str>) -> Self {
        assert!((2..=65).contains(&width) && (2..=65).contains(&height));
        Self { width, height, missing_samples }
    }

    /// 🕳️ Reports whether validity topology is present.
    pub fn has_missing(self) -> bool { self.missing_samples.is_some() }

    /// 🧭 Converts one north-first vertex ID to a canonical south-first sample.
    pub fn sample(self, vertex: u32) -> [u32; 2] {
        [vertex % self.width, self.height - 1 - vertex / self.width]
    }

    /// 🎯 Tests a north-first row and column for authored data.
    pub fn is_present(self, row: u32, column: u32) -> bool {
        row < self.height && column < self.width
            && !is_missing(self.missing_samples, ((self.height - 1 - row) * self.width + column) as usize)
    }

    /// 📏 Returns the fixed two faces of a cell in winding and emission order.
    pub(super) fn cell_vertices(width: u32, row: u32, column: u32) -> [[u32; 3]; 2] {
        let i = row * width + column;
        [[i, i + width, i + 1], [i + 1, i + width, i + width + 1]]
    }

    /// 🔺 Returns each face with three valid vertices.
    pub fn cell_faces(self, row: u32, column: u32) -> [Option<TerrainGridFace>; 2] {
        let vertices = Self::cell_vertices(self.width, row, column);
        std::array::from_fn(|half| {
            let vertices = vertices[half];
            vertices.into_iter().all(|vertex| self.is_present(vertex / self.width, vertex % self.width))
                .then_some(TerrainGridFace { vertices, row, column, half })
        })
    }

    /// 🧱 Visits supported faces in the original renderer's row, cell and diagonal order.
    pub fn faces(self) -> impl Iterator<Item = TerrainGridFace> + 'a {
        (0..self.height - 1).flat_map(move |row| {
            (0..self.width - 1).flat_map(move |column| self.cell_faces(row, column).into_iter().flatten())
        })
    }

    /// 🔗 Checks whether this vertex participates in any supported face.
    pub fn vertex_has_face(self, row: u32, column: u32) -> bool {
        if !self.has_missing() { return true; }
        if !self.is_present(row, column) { return false; }
        let vertex = row * self.width + column;
        (row.saturating_sub(1)..=row.min(self.height - 2)).any(|cell_row| {
            (column.saturating_sub(1)..=column.min(self.width - 2)).any(|cell_column| {
                self.cell_faces(cell_row, cell_column).into_iter().flatten().any(|face| face.vertices.contains(&vertex))
            })
        })
    }

    /// 🧭 Finds a supported closed face, including either side of a grid line or cell diagonal.
    pub fn face_at(self, [x, y]: [f64; 2]) -> Option<TerrainGridFace> {
        const TOLERANCE: f64 = 1.0e-11;
        let cells = |value: f64, extent: u32| {
            let limit = f64::from(extent - 1);
            let index = |value: f64| value.floor().clamp(0.0, limit - 1.0) as u32;
            index(value - TOLERANCE)..=index(value + TOLERANCE)
        };
        if !(-TOLERANCE..=f64::from(self.width - 1) + TOLERANCE).contains(&x)
            || !(-TOLERANCE..=f64::from(self.height - 1) + TOLERANCE).contains(&y) { return None; }
        for row in cells(y, self.height) {
            for column in cells(x, self.width) {
                if let Some(face) = self.cell_faces(row, column).into_iter().flatten()
                    .find(|face| face.weights([x, y]).into_iter().all(|weight| weight >= -TOLERANCE)) {
                    return Some(face);
                }
            }
        }
        None
    }
}
'''
edit(core, "pub use grid_projection::TerrainGridMeshProjection;", '''pub use grid_projection::TerrainGridMeshProjection;
#[path = "🔺️grid-topology/🦀️.rs"]
mod grid_topology;
pub use grid_topology::{TerrainGridFace, TerrainGridTopology};''')
edit(grid, "use super::{normalize3, projection};", "use super::{TerrainGridTopology, normalize3, projection};")
old = read(grid)
start = old.index("    pub fn normal(")
end = old.index("    pub fn uv(", start)
changes[grid] = old[:start] + '''    pub fn normal(&self, row: u32, column: u32, height: impl Fn(u32, u32) -> f64) -> [f32; 3] {
        self.normal_present(row, column, |row, column| Some(height(row, column)))
    }

    /// ⛰️ Uses a one-sided derivative when an adjacent sample is absent.
    pub fn normal_present(&self, row: u32, column: u32, height: impl Fn(u32, u32) -> Option<f64>) -> [f32; 3] {
        let Some(center) = height(row, column) else { return [0.0, 0.0, 1.0]; };
        let west = column.saturating_sub(1);
        let east = (column + 1).min(self.cells[0]);
        let north = row.saturating_sub(1);
        let south = (row + 1).min(self.cells[1]);
        let rise = |(lower, before): (u32, Option<f64>), (upper, after): (u32, Option<f64>), coordinate: u32, scale| {
            let before_distance = if before.is_some() { coordinate.abs_diff(lower) } else { 0 };
            let after_distance = if after.is_some() { coordinate.abs_diff(upper) } else { 0 };
            let distance = before_distance + after_distance;
            if distance == 0 { return 0.0; }
            (after.unwrap_or(center) - before.unwrap_or(center)) * self.exaggeration / (f64::from(distance) * scale)
        };
        let x = rise((west, height(row, west)), (east, height(row, east)), column, self.cell_size[0]);
        let y = rise((south, height(south, column)), (north, height(north, column)), row, self.cell_size[1]);
        let (x, y, z) = normalize3(-x, -y, 1.0);
        [x as f32, y as f32, z as f32]
    }

''' + old[end:]
edit(grid, '''        let stride = self.cells[0] + 1;
        let i = row * stride + column;
        [i, i + stride, i + 1, i + 1, i + stride, i + stride + 1]''',
     '''        let [first, second] = TerrainGridTopology::cell_vertices(self.cells[0] + 1, row, column);
        [first[0], first[1], first[2], second[0], second[1], second[2]]''')

edit(core, "Some(build_terrain_grid_mesh(&heights, self.bounds, [origin_lon, origin_lat], exaggeration, dimensions))", '''Some(build_terrain_grid_mesh(&heights, self.bounds, [origin_lon, origin_lat], exaggeration, dimensions, self.missing_samples.as_deref()))
    }

    /// 🗜️ Keeps just the vertices addressed by faces for artifact and workflow exchange.
    pub fn compact_mesh(&self, origin_lon: f64, origin_lat: f64, exaggeration: f64) -> Option<TerrainTileMeshJson> {
        let mut mesh = self.mesh(origin_lon, origin_lat, exaggeration)?;
        if self.missing_samples.is_some() && !mesh.indices.is_empty() {
            mesh.compact_vertices();
        }
        Some(mesh)''')
edit(core, "exaggeration, [n, n])", "exaggeration, [n, n], None)")
edit(core, "exaggeration: f64, dimensions: [u32; 2]) -> TerrainTileMeshJson", "exaggeration: f64, dimensions: [u32; 2], missing_samples: Option<&str>) -> TerrainTileMeshJson")
edit(core, '''    let minimum = heights.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = heights.iter().copied().fold(f64::NEG_INFINITY, f64::max);''', '''    let topology = TerrainGridTopology::new(dimensions, missing_samples);
    let mut indices = Vec::with_capacity(((width - 1) * (rows - 1) * 6) as usize);
    indices.extend(topology.faces().flat_map(|face| face.vertices));
    if indices.is_empty() {
        return TerrainTileMeshJson { positions: Vec::new(), normals: Vec::new(), indices, uvs: Vec::new() };
    }
    let used_heights = heights.iter().copied().enumerate().filter_map(|(ordinal, sample)| {
        let ordinal = ordinal as u32;
        topology.vertex_has_face(ordinal / width, ordinal % width).then_some(sample)
    });
    let (minimum, maximum) = used_heights.fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), sample| {
        (min.min(sample), max.max(sample))
    });''')
edit(core, '''    let mut uvs = Vec::with_capacity(heights.len() * 2);
    let mut indices = Vec::with_capacity(((width - 1) * (rows - 1) * 6) as usize);''', '''    let mut uvs = Vec::with_capacity(heights.len() * 2);''')
edit(core, "normals.extend(projection.normal(row, column, height));", """normals.extend(projection.normal_present(row, column, |r, c| {
                topology.is_present(r, c).then(|| height(r, c))
            }));""")
edit(core, '''    for row in 0..rows - 1 {
        for column in 0..width - 1 {
            indices.extend(projection.triangles(row, column));
        }
    }
    TerrainTileMeshJson { positions, normals, indices, uvs }
}

fn normalize3''', '''    TerrainTileMeshJson { positions, normals, indices, uvs }
}

impl TerrainTileMeshJson {
    fn compact_vertices(&mut self) {
        let mut remap = vec![u32::MAX; self.positions.len() / 3];
        for &vertex in &self.indices { remap[vertex as usize] = 0; }
        let mut next = 0;
        for (old, new) in remap.iter_mut().enumerate().filter(|(_, index)| **index != u32::MAX) {
            self.positions.copy_within(old * 3..old * 3 + 3, next * 3);
            self.normals.copy_within(old * 3..old * 3 + 3, next * 3);
            self.uvs.copy_within(old * 2..old * 2 + 2, next * 2);
            *new = next as u32;
            next += 1;
        }
        self.positions.truncate(next * 3);
        self.normals.truncate(next * 3);
        self.uvs.truncate(next * 2);
        for vertex in &mut self.indices { *vertex = remap[*vertex as usize]; }
    }
}

fn normalize3''')

pick = SURFACE / "🎯️pick/🦀️.rs"
edit(pick, "TerrainSessionCore, ELEVATION_GRID_TILE", "TerrainGridTopology, TerrainSessionCore, ELEVATION_GRID_TILE")
edit(pick, '''        let distance = mesh.indices.as_chunks::<3>().0.iter()
            .filter_map(|&[a, b, c]| triangle_hit(origin, direction, vertex(a), vertex(b), vertex(c)))
            .min_by(f64::total_cmp)?;''', '''        let (distance, face) = mesh.indices.as_chunks::<3>().0.iter().filter_map(|&face| {
            let [a, b, c] = face.map(vertex);
            triangle_hit(origin, direction, a, b, c).map(|distance| (distance, face))
        }).min_by(|(first, _), (second, _)| first.total_cmp(second))?;''')
edit(pick, '''        Some(TerrainGridHit {
            position,
            coordinates: [lon.clamp(west, east), lat.clamp(south, north)],
            sample: [sample_index((lon - west) / (east - west), width), sample_index((lat - south) / (north - south), height)],
        })''', '''        let fractions = [(lon - west) / (east - west), (lat - south) / (north - south)];
        let sample = if grid.missing_samples.is_none() {
            [sample_index(fractions[0], width), sample_index(fractions[1], height)]
        } else {
            let topology = TerrainGridTopology::new([width, height], grid.missing_samples.as_deref());
            let x = fractions[0] * f64::from(width - 1);
            let y = fractions[1] * f64::from(height - 1);
            let distance = |[column, row]: [u32; 2]| {
                (x - f64::from(column)).powi(2) + (y - f64::from(row)).powi(2)
            };
            face.map(|vertex| topology.sample(vertex)).into_iter()
                .min_by(|&a, &b| distance(a).total_cmp(&distance(b)).then_with(|| a.cmp(&b)))?
        };
        Some(TerrainGridHit { position, coordinates: [lon.clamp(west, east), lat.clamp(south, north)], sample })''')

overlay = SURFACE / "🗺️overlay/🦀️.rs"
edit(overlay, "use super::{TerrainSessionCore,", "use super::{TerrainGridFace, TerrainGridTopology, TerrainSessionCore,")
edit(overlay, "    width: usize,", "    topology: TerrainGridTopology<'a>,")
old = read(overlay)
start = old.index("    fn point(")
end = old.index("    fn segments(", start)
changes[overlay] = old[:start] + '''    fn point_on_face(&self, face: TerrainGridFace, coordinate: [f64; 2]) -> [f32; 3] {
        std::array::from_fn(|axis| {
            face.vertices.into_iter().zip(face.weights(coordinate)).map(|(vertex, weight)| {
                weight * f64::from(self.positions[3 * vertex as usize + axis])
            }).sum::<f64>() as f32
        })
    }

    fn point(&self, coordinate: [f64; 2]) -> Option<[f32; 3]> {
        let face = self.topology.face_at(coordinate)?;
        Some(self.point_on_face(face, coordinate))
    }

''' + old[end:]
edit(overlay, '''            let mut points = stops.into_iter().map(|t| self.point(at(t)));
            if let Some(mut previous) = points.next() {
                for point in points {
                    output.push([previous, point]);
                    previous = point;
                }
            }''', '''            for pair in stops.windows(2) {
                let [begin, end] = [pair[0], pair[1]];
                let Some(face) = self.topology.face_at(at(begin.midpoint(end))) else { continue; };
                output.push([self.point_on_face(face, at(begin)), self.point_on_face(face, at(end))]);
            }''')
edit(overlay, '''    fn overlay_grid<'a>(&self, positions: &'a [f32]) -> Option<OverlayGrid<'a>> {
        let (inline_bounds, dimensions) = match &self.elevation_grid {
            Some(grid) => (Some(grid.bounds), grid.dimensions()?),
            None => (None, [TERRAIN_GRID_RESOLUTION; 2]),''', '''    fn overlay_grid<'a>(&'a self, positions: &'a [f32]) -> Option<OverlayGrid<'a>> {
        let (inline_bounds, dimensions, missing) = match &self.elevation_grid {
            Some(grid) => (Some(grid.bounds), grid.dimensions()?, grid.missing_samples.as_deref()),
            None => (None, [TERRAIN_GRID_RESOLUTION; 2], None),''')
edit(overlay, "            width: dimensions[0] as usize,", "            topology: TerrainGridTopology::new(dimensions, missing),")
edit(overlay, "Some(grid.point(grid.coordinates([self.origin_lon, self.origin_lat]))[2])", "grid.point(grid.coordinates([self.origin_lon, self.origin_lat])).map(|point| point[2])")
edit(overlay, "let mut point = grid.point(coordinate);", "let mut point = grid.point(coordinate)?;")

scene_imported_map = GIS / "🧬️schema/🗺️imported-map/🦀️.rs"
s = read(scene_imported_map)
m = re.search(r"(pub fn gis_terrain_mesh_buffers\b[\s\S]*?)(?=\n\})", s)
assert m and ".mesh(" in m.group(1), m.group(1) if m else s
changes[scene_imported_map] = s[:m.start()] + m.group(1).replace(".mesh(", ".compact_mesh(", 1) + s[m.end():]

camera = GIS / "🚪️io/📝️text/📸️snapshot/🦀️.rs"
edit(camera, '''    let centre_millimetres = [rows[south][west], rows[south][east], rows[north][west], rows[north][east]].into_iter().map(f64::from).sum::<f64>() / 4.0;
    let center_elevation = centre_millimetres / 1000.0 * descriptor.exaggeration;''', '''    let centre_millimetres = if let Some(mask) = source.missing_samples.as_deref() {
        use semio_framework_surface::terrain::TerrainGridTopology;
        let topology = TerrainGridTopology::new([width as u32, height as u32], Some(mask));
        let center = [(width - 1) as f64 / 2.0, (height - 1) as f64 / 2.0];
        let sample_height = |vertex| {
            let [column, row] = topology.sample(vertex);
            f64::from(rows[row as usize][column as usize])
        };
        if let Some(face) = topology.face_at(center) {
            face.vertices.into_iter().zip(face.weights(center)).map(|(vertex, weight)| sample_height(vertex) * weight).sum()
        } else {
            let distance = |vertex: u32| {
                (f64::from(vertex % width as u32) - center[0]).powi(2)
                    + (f64::from(vertex / width as u32) - center[1]).powi(2)
            };
            topology.faces().flat_map(|face| face.vertices)
                .min_by(|&a, &b| distance(a).total_cmp(&distance(b)).then_with(|| a.cmp(&b)))
                .map_or_else(|| {
                    1000.0 * descriptor.overlays.positions.iter().filter_map(|position| position.elevation).fold(200.0, f64::max)
                }, sample_height)
        }
    } else {
        [rows[south][west], rows[south][east], rows[north][west], rows[north][east]].into_iter().map(f64::from).sum::<f64>() / 4.0
    };
    let center_elevation = centre_millimetres / 1000.0 * descriptor.exaggeration;''')

media = GIS / "✏️editor/📤️media-export/🦀️.rs"
scene = GIS / "✏️editor/📤️media-export/📐️scene/🦀️.rs"
changes[scene] = '''use super::TerrainGridMeshProjection;
use crate::schema::snapshot::ELEVATION_MAX_SIDE;
use semio_framework_pack_json::{JsonWriteNode, JsonWriteSource};
use semio_framework_surface::terrain::TerrainGridTopology;
use semio_framework_value::{Number, ValueError, ValueRefusalKind};

struct RowSelection {
    bits: [u128; ELEVATION_MAX_SIDE],
    end: [u16; ELEVATION_MAX_SIDE],
}

impl Default for RowSelection {
    fn default() -> Self { Self { bits: [0; ELEVATION_MAX_SIDE], end: [0; ELEVATION_MAX_SIDE] } }
}

impl RowSelection {
    fn before(&self, row: usize) -> usize {
        row.checked_sub(1).map_or(0, |previous| usize::from(self.end[previous]))
    }

    fn push(&mut self, row: usize, bits: u128) {
        self.bits[row] = bits;
        self.end[row] = (self.before(row) + bits.count_ones() as usize) as u16;
    }

    fn rank(&self, row: usize, column: usize) -> u32 {
        (self.before(row) + (self.bits[row] & ((1_u128 << column) - 1)).count_ones() as usize) as u32
    }

    fn select(&self, rows: usize, index: usize) -> (u32, u32) {
        let row = self.end[..rows].partition_point(|&end| usize::from(end) <= index);
        let mut rank = (index - self.before(row)) as u32;
        let mut bits = self.bits[row];
        let mut column = 0;
        for width in [64, 32, 16, 8, 4, 2, 1] {
            let lower = bits & ((1_u128 << width) - 1);
            let count = lower.count_ones();
            if rank < count {
                bits = lower;
            } else {
                bits >>= width;
                rank -= count;
                column += width;
            }
        }
        (row as u32, column)
    }

    fn count(&self, rows: usize) -> usize { self.before(rows) }
}

/// 📐️ Per-row rank tables keep the retained scene projection bounded without any allocation.
#[derive(Default)]
pub struct TerrainSceneRange {
    pub geometry: Option<TerrainGridMeshProjection>,
    row: usize,
    range: Option<[i32; 2]>,
    vertices: RowSelection,
    faces: RowSelection,
}

semio_framework_value::artifact_retire_leaf!(TerrainSceneRange);

impl TerrainSceneRange {
    pub const ROW_COPY_BYTES: usize = ELEVATION_MAX_SIDE * (size_of::<i32>() + 2 * size_of::<u8>())
        + 2 * (size_of::<u128>() + size_of::<u16>()) + size_of::<Self>();

    pub fn step(&mut self, rows: &[Vec<i32>], width: usize, missing: Option<&str>) -> Result<bool, ValueError> {
        if self.row == rows.len() { return Ok(true); }
        let row = &rows[rows.len() - 1 - self.row];
        if row.len() != width { return Err(invalid_terrain()); }
        let topology = TerrainGridTopology::new([width as u32, rows.len() as u32], missing);
        let mut vertices = 0_u128;
        for (column, &sample) in row.iter().enumerate() {
            if topology.vertex_has_face(self.row as u32, column as u32) {
                vertices |= 1 << column;
                self.range = Some(self.range.map_or([sample; 2], |[min, max]| [min.min(sample), max.max(sample)]));
            }
        }
        self.vertices.push(self.row, vertices);
        if self.row + 1 < rows.len() {
            let mut faces = 0_u128;
            for column in 0..width - 1 {
                for (half, face) in topology.cell_faces(self.row as u32, column as u32).into_iter().enumerate() {
                    if face.is_some() { faces |= 1 << (2 * column + half); }
                }
            }
            self.faces.push(self.row, faces);
        }
        self.row += 1;
        Ok(self.row == rows.len())
    }
}

/// 🏔️ Lends the retained face and vertex ranks with original elevation samples.
pub struct TerrainSceneView<'a> {
    pub rows: &'a [Vec<i32>],
    pub dimensions: (usize, usize),
    pub projection: TerrainGridMeshProjection,
    pub range: &'a TerrainSceneRange,
    pub missing: Option<&'a str>,
}

impl TerrainSceneView<'_> {
    fn height(&self, row: u32, column: u32) -> f64 {
        f64::from(self.rows[self.dimensions.1 - 1 - row as usize][column as usize]) / 1000.0
    }

    fn vertex_count(&self) -> usize { self.range.vertices.count(self.dimensions.1) }
    fn face_count(&self) -> usize { self.range.faces.count(self.dimensions.1 - 1) }

    fn scalar_at(&self, column: usize, index: usize) -> Result<JsonWriteNode<'_>, ValueError> {
        let (width, height) = self.dimensions;
        let topology = TerrainGridTopology::new([width as u32, height as u32], self.missing);
        let sample = |components| {
            let (row, column) = self.range.vertices.select(height, index / components);
            (row, column, index % components)
        };
        let floating = match column {
            0 | 1 if index < self.vertex_count() * 3 => {
                let (row, col, axis) = sample(3);
                if column == 0 {
                    self.projection.position(row, col, self.height(row, col))[axis]
                } else {
                    self.projection.normal_present(row, col, |r, c| topology.is_present(r, c).then(|| self.height(r, c)))[axis]
                }
            }
            3 if index < self.face_count() * 3 => {
                let (row, half_cell) = self.range.faces.select(height - 1, index / 3);
                let points = self.projection.triangles(row, half_cell / 2);
                let vertex = points[(half_cell % 2) as usize * 3 + index % 3] as usize;
                return Ok(JsonWriteNode::Number(Number::UInt(u64::from(self.range.vertices.rank(vertex / width, vertex % width)))));
            }
            4 if index < self.vertex_count() * 2 => {
                let (row, col, axis) = sample(2);
                let [min, max] = self.range.range.ok_or_else(absent)?;
                TerrainGridMeshProjection::uv(self.height(row, col), f64::from(min) / 1000.0, f64::from(max) / 1000.0)[axis]
            }
            _ => return Err(absent()),
        };
        Ok(JsonWriteNode::Float32(floating))
    }
}

impl JsonWriteSource for TerrainSceneView<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        let vertices = self.vertex_count();
        Ok(match path {
            [] => JsonWriteNode::Object(2),
            [0] => JsonWriteNode::String("mesh.document"),
            [1] => JsonWriteNode::Object(if vertices == 0 { 4 } else { 5 }),
            [1, 0 | 1] => JsonWriteNode::Array(vertices * 3),
            [1, 2] => JsonWriteNode::Array(0),
            [1, 3] => JsonWriteNode::Array(self.face_count() * 3),
            [1, 4] if vertices != 0 => JsonWriteNode::Array(vertices * 2),
            [1, column, index] => self.scalar_at(*column, *index)?,
            _ => return Err(absent()),
        })
    }

    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        match path {
            [] => ["schema", "mesh"].get(index),
            [1] if index != 4 || self.vertex_count() != 0 => ["positions", "normals", "colors", "indices", "uvs"].get(index),
            _ => None,
        }.copied().ok_or_else(absent)
    }
}

fn absent() -> ValueError {
    ValueError::invariant("the original GIS terrain JSON projection path is absent")
}

fn invalid_terrain() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "the authored terrain elevation grid is invalid")
}
'''
edit(media, "use crate::schema::snapshot::ELEVATION_MAX_SIDE;\n", "")
edit(media, "ELEVATION_MAX_SIDE * size_of::<i32>() + size_of::<TerrainSceneRange>()", "TerrainSceneRange::ROW_COPY_BYTES")
edit(media, '''        let complete = state.step(&source.rows_millimetres, width).map_err(|_| if port == "elevation:out" { "no authored elevation source" } else { "The authored terrain elevation grid is invalid" })?;
        if port == "scene:out" && complete {''', '''        if port == "elevation:out" { return Ok(true); }
        let complete = state.step(&source.rows_millimetres, width, source.missing_samples.as_deref())
            .map_err(|_| "The authored terrain elevation grid is invalid")?;
        if complete {''')
edit(media, "range: *range", "range, missing: source.missing_samples.as_deref()")
edit(core, "return TerrainTileMeshJson { positions: Vec::new(), normals: Vec::new(), indices, uvs: Vec::new() };",
     "return TerrainTileMeshJson { positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), uvs: Vec::new() };")
edit(topology, "vertices.into_iter().all(|vertex| self.is_present(vertex / self.width, vertex % self.width))",
     "(!self.has_missing() || vertices.into_iter().all(|vertex| self.is_present(vertex / self.width, vertex % self.width)))")

edit(core, '''    let used_heights = heights.iter().copied().enumerate().filter_map(|(ordinal, sample)| {
        let ordinal = ordinal as u32;
        topology.vertex_has_face(ordinal / width, ordinal % width).then_some(sample)
    });''', '''    let mut used = [false; 65 * 65];
    for &index in &indices { used[index as usize] = true; }
    let referenced = |row: u32, column: u32| used[(row * width + column) as usize];
    let used_heights = heights.iter().copied().zip(used).filter_map(|(sample, used)| used.then_some(sample));''')
edit(core, '''            positions.extend(projection.position(row, column, height(row, column)));
            normals.extend(projection.normal_present(row, column, |r, c| {
                topology.is_present(r, c).then(|| height(r, c))
            }));
            uvs.extend(TerrainGridMeshProjection::uv(height(row, column), minimum, maximum));''', '''            let sample = if referenced(row, column) { height(row, column) } else { minimum };
            positions.extend(projection.position(row, column, sample));
            normals.extend(projection.normal_present(row, column, |r, c| referenced(r, c).then(|| height(r, c))));
            uvs.extend(TerrainGridMeshProjection::uv(sample, minimum, maximum));''')
edit(scene, "topology.is_present(r, c).then(|| self.height(r, c))", "topology.vertex_has_face(r, c).then(|| self.height(r, c))")

fixtures = SURFACE / "🧪️tests/🧫️fixtures/🕳️nodata/🔣️.json"
fixture = json.loads((TICKET / "terrain-nodata.json").read_text(encoding="utf-8"))
cases = []
for case in fixture["cases"]:
    if case["name"] in ("mixed-extrema-and-holes", "all-nodata", "dense-valid-extrema"):
        cases.append({key: value for key, value in case.items() if key not in ("asciiGrid", "gdal")})
cases.append({
    "name": "isolated-valid-extreme",
    "source": {"bounds": [5, 50, 5.25, 50.25],
               "rowsMillimetres": [[2147483647, 0, 4000], [0, 3000, 2000], [0, 0, 1000]], "missingSamples": "4a00"},
    "facesNorthFirst": [[1, 4, 2], [2, 4, 5]],
})
changes[fixtures] = json.dumps(cases, ensure_ascii=False, indent=2) + "\n"
unit = SURFACE / "🧪️tests/🔬️unit/🦀️.rs"
changes[unit] = read(unit) + '''
#[test]
fn missing_terrain_keeps_existing_diagonals_for_mesh_picks_and_drape() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🕳️nodata/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let source: TerrainElevationGrid = from_json_str(&case["source"].to_string(), JsonMemberPolicy::Reject).unwrap();
        let [west, south, east, north] = source.bounds;
        let [width, height] = source.dimensions().unwrap();
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(west, north);
        assert!(session.set_elevation_grid(Some(source.clone())));
        let mesh = session.terrain_tile_mesh(0, 0, 0).unwrap();
        let expected: Vec<[u32; 3]> = serde_json::from_value(case["facesNorthFirst"].clone()).unwrap();
        assert_eq!(mesh.indices, expected.iter().flatten().copied().collect::<Vec<_>>(), "{}", case["name"]);
        let compact = source.compact_mesh(west, north, 1.0).unwrap();
        assert_eq!(compact.indices.len(), mesh.indices.len());
        if mesh.indices.is_empty() {
            assert!(mesh.positions.is_empty() && mesh.normals.is_empty() && mesh.uvs.is_empty());
            assert!(session.terrain_origin_elevation(0, 0, 0).is_none());
            assert!(session.pick_elevation_grid([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]).is_none());
        }
        let geographic = |x, y| [west + (east - west) * x / f64::from(width - 1), north - (north - south) * y / f64::from(height - 1)];
        if let Some(queries) = case["surfaceQueriesNorthFirst"].as_array() {
            for query in queries {
                let [x, y]: [f64; 2] = serde_json::from_value(query["xy"].clone()).unwrap();
                let coordinate = geographic(x, y);
                let overlays = TerrainOverlays { positions: vec![TerrainOverlayPosition {
                    id: "point".into(), name: "Point".into(), coordinates: coordinate, elevation: None
                }], ..Default::default() };
                let drape = session.overlay_tile(0, 0, 0, &overlays).unwrap();
                let row = y.floor().min(f64::from(height - 2)) as usize;
                let column = x.floor().min(f64::from(width - 2)) as usize;
                let i = row * width as usize + column;
                let u = x - column as f64;
                let v = y - row as f64;
                let px = f64::from(mesh.positions[3 * i]) * (1.0 - u) + f64::from(mesh.positions[3 * (i + 1)]) * u;
                let py = f64::from(mesh.positions[3 * i + 1]) * (1.0 - v) + f64::from(mesh.positions[3 * (i + width as usize) + 1]) * v;
                let hit = session.pick_elevation_grid([px, py, 3_000_000.0], [0.0, 0.0, -1.0]);
                if let Some(mm) = query["millimetres"].as_f64() {
                    assert!((f64::from(drape.positions[0].point[2]) - mm / 1000.0).abs() < 0.01, "{query}");
                    let hit = hit.expect("surviving face hit");
                    assert!((hit.position[2] - mm / 1000.0).abs() < 0.01, "{query}");
                    assert!(!missing_samples::is_missing(source.missing_samples.as_deref(), (hit.sample[1] * width + hit.sample[0]) as usize));
                } else {
                    assert!(drape.positions.is_empty() && hit.is_none(), "{query}");
                }
            }
        }
        if let Some(routes) = case["horizontalDrapeNorthFirst"].as_array() {
            for route in routes {
                let y = route["y"].as_f64().unwrap();
                let expected: Vec<[f64; 2]> = serde_json::from_value(route["intervals"].clone()).unwrap();
                for reverse in [false, true] {
                    let mut coordinates = vec![geographic(0.0, y), geographic(f64::from(width - 1), y)];
                    if reverse { coordinates.reverse(); }
                    let input = TerrainOverlays { routes: vec![TerrainOverlayRoute {
                        id: "route".into(), name: "Route".into(), coordinates,
                    }], ..Default::default() };
                    let output = session.overlay_tile(0, 0, 0, &input).unwrap();
                    let segments = &output.routes[0].segments;
                    let position = |x: f32| f64::from(x) / f64::from(mesh.positions[3 * (width - 1) as usize]) * f64::from(width - 1);
                    let mut actual: Vec<[f64; 2]> = segments.iter().map(|[a, b]| {
                        let (a, b) = (position(a[0]), position(b[0]));
                        [a.min(b), a.max(b)]
                    }).collect();
                    actual.sort_by(|a, b| a[0].total_cmp(&b[0]));
                    let mut merged: Vec<[f64; 2]> = Vec::new();
                    for [start, end] in actual {
                        if let Some(last) = merged.last_mut().filter(|last| start <= last[1] + 1.0e-6) {
                            last[1] = last[1].max(end);
                        } else {
                            merged.push([start, end]);
                        }
                    }
                    assert_eq!(merged.len(), expected.len(), "{route}");
                    for (actual, expected) in merged.iter().flatten().zip(expected.iter().flatten()) {
                        assert!((actual - expected).abs() < 1.0e-6, "{route}");
                    }
                }
            }
        }
        if case["name"] == "isolated-valid-extreme" {
            assert!(mesh.positions.iter().skip(2).step_by(3).all(|z| *z <= 4.0));
            assert!(!compact.positions.contains(&(i32::MAX as f32 / 1000.0)));
        }
    }
}
'''
browser = SURFACE.parent / "🧪️tests/🕸️browser/🟦️.ts"
changes[browser] = read(browser) + '''
test("NoData terrain keeps triangle gaps in the browser surface and Three raycaster", async () => {
 const cases = (await import("../../🏔️terrain/🧪️tests/🧫️fixtures/🕳️nodata/🔣️.json")).default;
 const module = await import(fileUrl(file("framework_surface.js")).href);
 await module.default({ module_or_path: await WebAssembly.compile(readFileSync(file("framework_surface_bg.wasm"))) });
 const session = new module.TerrainSession();
 const geometry = new BufferGeometry();
 const material = new MeshBasicMaterial({ side: DoubleSide });
 try {
  for (const entry of cases) {
   const [west, , , north] = entry.source.bounds;
   session.set_project_origin(west!, north!);
   expect(session.set_elevation_grid_json(JSON.stringify(entry.source))).toBe(true);
   const data = JSON.parse(session.terrain_tile_mesh_json(0, 0, 0));
   expect(data.indices).toEqual(entry.facesNorthFirst.flat());
   if (!data.indices.length) {
    expect(data.positions).toEqual([]);
    expect(JSON.parse(session.pickElevationGridJson(JSON.stringify({ origin: [0, 0, 10], direction: [0, 0, -1] })))).toBeNull();
    continue;
   }
   geometry.setAttribute("position", new Float32BufferAttribute(data.positions, 3));
   geometry.setIndex(data.indices);
   const mesh = new Mesh(geometry, material);
   const width = entry.source.rowsMillimetres[0]!.length;
   const height = entry.source.rowsMillimetres.length;
   for (const query of entry.surfaceQueriesNorthFirst ?? []) {
    const [x, y] = query.xy;
    const column = Math.min(Math.floor(x!), width - 2);
    const row = Math.min(Math.floor(y!), height - 2);
    const i = 3 * (row * width + column);
    const px = data.positions[i] + (x! - column) * (data.positions[i + 3] - data.positions[i]);
    const py = data.positions[i + 1] + (y! - row) * (data.positions[i + 3 * width + 1] - data.positions[i + 1]);
    const point = [px, py, 3_000_000] as const;
    const hit = JSON.parse(session.pickElevationGridJson(JSON.stringify({ origin: point, direction: [0, 0, -1] })));
    if (query.millimetres === null) {
     expect(hit).toBeNull();
     expect(new Raycaster(new Vector3(...point), new Vector3(0, 0, -1)).intersectObject(mesh)).toHaveLength(0);
    } else {
     expect(hit.position[2]).toBeCloseTo(query.millimetres / 1000, 2);
    }
   }
  }
 } finally {
  geometry.dispose();
  material.dispose();
  session.free();
 }
});
'''

edit(unit, "fn missing_terrain_keeps_existing_diagonals_for_mesh_picks_and_drape() {\n", '''fn missing_terrain_keeps_existing_diagonals_for_mesh_picks_and_drape() {
    use super::overlay::{TerrainOverlayPosition, TerrainOverlayRoute, TerrainOverlays};
''')
changes[scene] += '''
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_rank_tables_preserve_first_and_last_compacted_ordinals() {
        let mut selection = RowSelection::default();
        selection.push(0, 1 | (1 << 127));
        selection.push(1, 0);
        selection.push(2, 1 << 64);
        assert_eq!(selection.count(3), 3);
        for (ordinal, (row, column)) in [(0, 0), (0, 127), (2, 64)].into_iter().enumerate() {
            assert_eq!(selection.select(3, ordinal), (row, column));
            assert_eq!(selection.rank(row as usize, column as usize), ordinal as u32);
        }
    }
}
'''

def patch(delta, previous):
    """🧾 Forms an ordered sheet of unified diffs."""
    hunks = []
    for path, staged in sorted(delta.items(), key=lambda item: str(item[0])):
        if not staged or staged == previous(path):
            continue
        assert len(str(path.resolve()).encode()) <= 256, path
        old = previous(path)
        hunks.extend(difflib.unified_diff(old.splitlines(keepends=True), staged.splitlines(keepends=True),
                                         f"a/{path}" if old else "/dev/null", f"b/{path}", n=3))
    return "".join(hunks)

(TICKET / "terrain-nodata-surface.patch").write_text(patch(changes, original), encoding="utf-8")
generated = TICKET / "🗑️generated" / "nodata-surface"
generated.mkdir(parents=True, exist_ok=True)
live = lambda path: path.read_text(encoding="utf-8") if path.exists() else ""
(generated / "combined.patch").write_text(patch(base | changes, live), encoding="utf-8")
(generated / "fixture.json").write_text(changes[fixtures], encoding="utf-8")
(generated / "mask.rs").write_text(base[baseline["MASK"]], encoding="utf-8")
(generated / "topology.rs").write_text(changes[topology], encoding="utf-8")
(generated / "grid.rs").write_text(changes[grid], encoding="utf-8")
(generated / "scene.rs").write_text(changes[scene], encoding="utf-8")

def without_value_derive(text):
    """✂️ Isolates real geometry from the application-only wire wrappers."""
    text = re.sub(r"^#\[value\([^\n]+\)\]\n", "", text, flags=re.M)
    text = re.sub(r"^    #\[value\([^\n]+\)\]\n", "", text, flags=re.M)
    text = re.sub(r"^#\[cfg_attr\(test, derive\(serde::Deserialize\)\)\]\n", "", text, flags=re.M)
    text = re.sub(r", (?:semio_framework_value::)?(?:FromValue|ToValue)(?=[,)])", "", text)
    text = re.sub(r"^#\[derive\((?:semio_framework_value::)?(?:FromValue|ToValue)(?:, (?:semio_framework_value::)?(?:FromValue|ToValue))*\)\]\n", "", text, flags=re.M)
    return text

surface_text = changes[core]
projection = surface_text.split("//#region Projection\n", 1)[1].split("//#endregion Projection", 1)[0]
mesh_start = surface_text.index("pub struct TerrainTileMeshJson")
mesh_end = surface_text.index("//#endregion TerrainTileMesh", mesh_start)
mesh = surface_text[mesh_start:mesh_end]
mesh = mesh[:mesh.index("fn build_terrain_tile_mesh")] + mesh[mesh.index("pub use grid_projection::TerrainGridMeshProjection;"):]
mesh = mesh.replace('#[path = "🔺️grid-topology/🦀️.rs"]\nmod grid_topology;', "")
mesh = mesh.replace('#[path = "🕳️missing-samples/🦀️.rs"]\npub mod missing_samples;', "")
mesh = without_value_derive(mesh)
pick_text = changes[pick].split("use super", 1)[1]
pick_text = "use super" + pick_text
pick_text = re.sub(r"^use semio_framework_pack_json::.*?;\n", "", pick_text, flags=re.M)
pick_text = pick_text.split("\n/// ⛰️ World position", 1)[0].split("\n", 1)[0] + "\n" + pick_text[pick_text.index("/// ⛰️ World position"):]
start = pick_text.index("    /// 🌐 Decodes")
end = pick_text.index("\n}\n\nfn triangle_hit", start)
pick_text = without_value_derive(pick_text[:start] + pick_text[end:])
overlay_text = changes[overlay][changes[overlay].index("use super"):].split("#[cfg(test)]\n#[path", 1)[0]
overlay_text = re.sub(r"^use semio_framework_pack_json::.*?;\n|^use semio_framework_value::.*?;\n", "", overlay_text, flags=re.M)
start = overlay_text.index("    /// 🌐 Compute")
overlay_text = without_value_derive(overlay_text[:start] + "}\n")
(generated / "projection.rs").write_text(projection, encoding="utf-8")
(generated / "mesh.rs").write_text(mesh, encoding="utf-8")
(generated / "pick.rs").write_text(pick_text, encoding="utf-8")
(generated / "overlay.rs").write_text(overlay_text, encoding="utf-8")

harness = r'''extern crate self as semio_framework_surface;
extern crate self as semio_framework_pack_json;
extern crate self as semio_framework_value;

#[macro_export]
macro_rules! artifact_retire_leaf { ($($type:ty),*) => {}; }

pub enum Number { UInt(u64) }
pub enum JsonWriteNode<'a> { Object(usize), Array(usize), String(&'a str), Number(Number), Float32(f32) }
pub struct ValueError;
pub enum ValueRefusalKind { InvalidValue }
impl std::fmt::Debug for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("value error") }
}
impl ValueError {
    pub fn invariant(_: &str) -> Self { Self }
    pub fn literal(_: ValueRefusalKind, _: &str) -> Self { Self }
}
pub trait JsonWriteSource {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError>;
    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError>;
}

mod schema { pub mod snapshot { pub const ELEVATION_MAX_SIDE: usize = 65; } }
pub mod terrain {
    include!("projection.rs");
    #[path = "mask.rs"] pub mod missing_samples;
    #[path = "topology.rs"] mod grid_topology;
    #[path = "grid.rs"] mod grid_projection;
    #[path = "pick.rs"] pub mod pick;
    #[path = "overlay.rs"] pub mod overlay;
    include!("mesh.rs");

    const ELEVATION_GRID_TILE: (u32, u32, u32) = (0, 0, 0);
    const TERRAIN_GRID_RESOLUTION: u32 = 33;

    pub struct TerrainSessionCore {
        pub origin_lon: f64,
        pub origin_lat: f64,
        pub exaggeration: f64,
        pub elevation_grid: Option<TerrainElevationGrid>,
    }
    impl TerrainSessionCore {
        fn tile_mesh(&self, z: u32, x: u32, y: u32) -> Option<TerrainTileMeshJson> {
            if (z, x, y) != ELEVATION_GRID_TILE { return None; }
            self.elevation_grid.as_ref()?.mesh(self.origin_lon, self.origin_lat, self.exaggeration)
        }
    }
}
use terrain::{TerrainElevationGrid, TerrainGridMeshProjection, TerrainSessionCore};
use terrain::overlay::{TerrainOverlays, TerrainOverlayPosition, TerrainOverlayRegion, TerrainOverlayRoute};
#[path = "scene.rs"] mod scene;
use scene::{TerrainSceneRange, TerrainSceneView};

fn check(name: &str, source: TerrainElevationGrid, expected: &[[u32; 3]], queries: &[([f64; 2], Option<f64>)], drapes: &[(f64, &[[f64; 2]])]) {
    let [west, south, east, north] = source.bounds;
    let [width, height] = source.dimensions().unwrap();
    let session = TerrainSessionCore { origin_lon: west, origin_lat: north, exaggeration: 1.0, elevation_grid: Some(source) };
    let source = session.elevation_grid.as_ref().unwrap();
    let mesh = source.mesh(west, north, 1.0).unwrap();
    let compact = source.compact_mesh(west, north, 1.0).unwrap();
    assert_eq!(mesh.indices, expected.iter().flatten().copied().collect::<Vec<_>>(), "{name}");
    let dimensions = (width as usize, height as usize);
    let mut range = TerrainSceneRange::default();
    let mut steps = 0;
    loop {
        steps += 1;
        if range.step(&source.rows_millimetres, dimensions.0, source.missing_samples.as_deref()).unwrap() { break; }
    }
    assert_eq!(steps, height);
    let projection = TerrainGridMeshProjection::from_bounds(source.bounds, [west, north], 1.0, [width, height]);
    let scene = TerrainSceneView { rows: &source.rows_millimetres, dimensions, projection, range: &range, missing: source.missing_samples.as_deref() };
    let count = |column| match scene.node_at_path(&[1, column]).unwrap() { JsonWriteNode::Array(count) => count, _ => panic!("invalid array") };
    let floats = |column| (0..count(column)).map(|index| match scene.node_at_path(&[1, column, index]).unwrap() {
        JsonWriteNode::Float32(value) => value, _ => panic!("invalid scalar"),
    }).collect::<Vec<_>>();
    assert_eq!(floats(0), compact.positions);
    assert_eq!(floats(1), compact.normals);
    if compact.positions.is_empty() {
        assert!(matches!(scene.node_at_path(&[1]).unwrap(), JsonWriteNode::Object(4)));
        assert!(session.terrain_origin_elevation(0, 0, 0).is_none());
        assert!(session.pick_elevation_grid([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]).is_none());
    } else {
        assert_eq!(floats(4), compact.uvs);
    }
    let indices = (0..count(3)).map(|i| match scene.node_at_path(&[1, 3, i]).unwrap() {
        JsonWriteNode::Number(Number::UInt(value)) => value as u32, _ => panic!("invalid index"),
    }).collect::<Vec<_>>();
    assert_eq!(indices, compact.indices);
    let geographic = |x, y| [west + (east - west) * x / f64::from(width - 1), north - (north - south) * y / f64::from(height - 1)];
    let point = |x: f64, y: f64| {
        let (row, column) = (y.floor().min(f64::from(height - 2)), x.floor().min(f64::from(width - 2)));
        let i = 3 * (row as usize * width as usize + column as usize);
        let px = f64::from(mesh.positions[i]) + (x - column) * f64::from(mesh.positions[i + 3] - mesh.positions[i]);
        let py = f64::from(mesh.positions[i + 1]) + (y - row) * f64::from(mesh.positions[i + 3 * width as usize + 1] - mesh.positions[i + 1]);
        [px, py]
    };
    let mut observations = Vec::new();
    for &([x, y], expected) in queries {
        let input = TerrainOverlays { positions: vec![TerrainOverlayPosition {
            id: "point".into(), name: "Point".into(), coordinates: geographic(x, y), elevation: expected.is_none().then_some(100.0),
        }], ..Default::default() };
        let overlays = session.overlay_tile(0, 0, 0, &input).unwrap();
        let [px, py] = point(x, y);
        let hit = session.pick_elevation_grid([px, py, 3_000_000.0], [0.0, 0.0, -1.0]);
        if let Some(expected) = expected {
            let hit = hit.expect("surface hit");
            assert!((hit.position[2] - expected).abs() < 0.01, "{name} {x} {y}: {}", hit.position[2]);
            assert!((f64::from(overlays.positions[0].point[2]) - expected).abs() < 0.01, "{name} {x} {y}");
            assert!(!terrain::missing_samples::is_missing(source.missing_samples.as_deref(), (hit.sample[1] * width + hit.sample[0]) as usize));
            observations.push(format!("[{px},{py},{}]", hit.position[2]));
        } else {
            assert!(hit.is_none() && overlays.positions.is_empty(), "{name} {x} {y}");
            observations.push(format!("[{px},{py},null]"));
        }
    }
    let mut paths = Vec::new();
    for &(y, expected) in drapes {
        let mut forward = Vec::new();
        for reverse in [false, true] {
            let mut coordinates = vec![geographic(0.0, y), geographic(f64::from(width - 1), y)];
            if reverse { coordinates.reverse(); }
            let input = TerrainOverlays { routes: vec![TerrainOverlayRoute { id: "route".into(), name: "Route".into(), coordinates }], ..Default::default() };
            let output = session.overlay_tile(0, 0, 0, &input).unwrap();
            let segments = &output.routes[0].segments;
            let position = |x: f32| f64::from(x) / f64::from(mesh.positions[3 * (width - 1) as usize]) * f64::from(width - 1);
            let mut actual: Vec<[f64; 2]> = segments.iter().map(|[a, b]| {
                let (a, b) = (position(a[0]), position(b[0]));
                [a.min(b), a.max(b)]
            }).collect();
            actual.sort_by(|a, b| a[0].total_cmp(&b[0]));
            let mut merged: Vec<[f64; 2]> = Vec::new();
            for [start, end] in actual {
                if let Some(last) = merged.last_mut().filter(|last| start <= last[1] + 1.0e-6) {
                    last[1] = last[1].max(end);
                } else { merged.push([start, end]); }
            }
            assert_eq!(merged.len(), expected.len(), "{name} {y} {reverse}: {merged:?}");
            for (actual, expected) in merged.iter().flatten().zip(expected.iter().flatten()) {
                assert!((actual - expected).abs() < 1.0e-6, "{name} {y} {reverse}");
            }
            if !reverse { forward = segments.clone(); }
        }
        paths.push(forward);
    }
    if !drapes.is_empty() {
        let ring = vec![geographic(0.0, 0.25), geographic(f64::from(width - 1), 0.25),
            geographic(f64::from(width - 1), 1.75), geographic(0.0, 1.75)];
        let input = TerrainOverlays { regions: vec![TerrainOverlayRegion {
            id: "area".into(), name: "Area".into(), rings: vec![ring],
        }], ..Default::default() };
        let output = session.overlay_tile(0, 0, 0, &input).unwrap();
        assert!(!output.regions[0].segments.is_empty());
        paths.push(output.regions.into_iter().next().unwrap().segments);
    }
    println!("{{\"name\":{name:?},\"positions\":{:?},\"indices\":{:?},\"picks\":[{}],\"drapes\":{paths:?}}}", mesh.positions, mesh.indices, observations.join(","));
}
fn main() {
    @CASES@
}
'''

def rust_numbers(values):
    """🔢 Formats fixture numbers as finite float literals for the native witness."""
    return "[" + ",".join(str(float(value)) for value in values) + "]"

rust_cases = []
for suite in cases:
    source = suite["source"]
    bounds = rust_numbers(source["bounds"])
    rows = "vec![" + ",".join("vec!" + str(row) for row in source["rowsMillimetres"]) + "]"
    mask = 'Some("' + source["missingSamples"] + '".into())' if "missingSamples" in source else "None"
    queries = []
    for query in suite.get("surfaceQueriesNorthFirst", []):
        mm = query["millimetres"]
        height = f"Some({mm / 1000.0})" if mm is not None else "None"
        queries.append(f"({rust_numbers(query['xy'])}, {height})")
    routes = []
    for route in suite.get("horizontalDrapeNorthFirst", []):
        intervals = "[" + ",".join(rust_numbers(interval) for interval in route["intervals"]) + "]"
        routes.append(f"({float(route['y'])}, &{intervals})")
    rust_cases.append(f'''check("{suite['name']}", TerrainElevationGrid {{ bounds: {bounds}, rows_millimetres: {rows}, missing_samples: {mask} }},
        &{suite['facesNorthFirst']}, &[{",".join(queries)}], &[{",".join(routes)}]);''')
(generated / "harness.rs").write_text(harness.replace("@CASES@", "\n    ".join(rust_cases)), encoding="utf-8")
(generated / "terrain").mkdir(exist_ok=True)
for name in ("mask.rs", "topology.rs", "grid.rs", "pick.rs", "overlay.rs"):
    (generated / name).replace(generated / "terrain" / name)
print(f"Staged {len(changes)} surface paths")

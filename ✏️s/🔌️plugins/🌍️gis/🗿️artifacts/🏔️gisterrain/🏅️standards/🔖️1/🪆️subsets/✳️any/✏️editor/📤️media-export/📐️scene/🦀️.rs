use super::TerrainGridMeshProjection;
use crate::schema::snapshot::ELEVATION_MAX_SIDE;
use semio_framework_pack_json::{JsonWriteNode, JsonWriteSource};
use semio_framework_surface::terrain::TerrainGridTopology;
use semio_framework_value::{Number, ValueError, ValueRefusalKind};

#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
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
                    self.projection.normal_present(row, col, |r, c| topology.vertex_has_face(r, c).then(|| self.height(r, c)))[axis]
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

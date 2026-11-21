use super::TerrainGridMeshProjection;
use semio_framework_pack_json::{JsonWriteNode, JsonWriteSource};
use semio_framework_value::{Number, ValueError, ValueRefusalKind};

const SIZE: usize = 33;

/// 📐️ The row frontier for borrowed original terrain scene projection.
#[derive(Clone, Copy, Default)]
pub struct TerrainSceneRange {
    pub geometry: Option<TerrainGridMeshProjection>,
    row: usize,
    minimum: i16,
    maximum: i16,
}

semio_framework_value::artifact_retire_leaf!(TerrainSceneRange);

impl TerrainSceneRange {
    pub fn step(&mut self, rows: &[Vec<i16>]) -> Result<bool, ValueError> {
        if rows.len() != SIZE {
            return Err(invalid_terrain());
        }
        if self.row == rows.len() {
            return Ok(true);
        }
        let row = &rows[self.row];
        if row.len() != SIZE {
            return Err(invalid_terrain());
        }
        let minimum = row.iter().copied().min().unwrap_or(0);
        let maximum = row.iter().copied().max().unwrap_or(0);
        self.minimum = if self.row == 0 { minimum } else { self.minimum.min(minimum) };
        self.maximum = if self.row == 0 { maximum } else { self.maximum.max(maximum) };
        self.row += 1;
        Ok(self.row == rows.len())
    }
}

/// 🏔️ Lends the canonical scalar columns of a scene without first materializing a mesh.
pub struct TerrainSceneView<'a> {
    pub rows: &'a [Vec<i16>],
    pub projection: TerrainGridMeshProjection,
    pub range: TerrainSceneRange,
}

impl TerrainSceneView<'_> {
    fn height(&self, row: u32, column: u32) -> f32 {
        f32::from(self.rows[SIZE - 1 - row as usize][column as usize])
    }

    fn scalar_at(&self, column: usize, index: usize) -> Result<JsonWriteNode<'_>, ValueError> {
        let sample = |index, width| (index / width / SIZE, index / width % SIZE, index % width);
        let floating = match column {
            0 | 1 if index < SIZE * SIZE * 3 => {
                let (row, col, axis) = sample(index, 3);
                if column == 0 {
                    self.projection.position(row as u32, col as u32, self.height(row as u32, col as u32))[axis]
                } else {
                    self.projection.normal(row as u32, col as u32, |r, c| self.height(r, c))[axis]
                }
            }
            3 if index < (SIZE - 1) * (SIZE - 1) * 6 => {
                let cell = index / 6;
                let points = self.projection.triangles((cell / (SIZE - 1)) as u32, (cell % (SIZE - 1)) as u32);
                return Ok(JsonWriteNode::Number(Number::UInt(u64::from(points[index % 6]))));
            }
            4 if index < SIZE * SIZE * 2 => {
                let (row, col, axis) = sample(index, 2);
                TerrainGridMeshProjection::uv(self.height(row as u32, col as u32), f32::from(self.range.minimum), f32::from(self.range.maximum))[axis]
            }
            _ => return Err(absent()),
        };
        Ok(JsonWriteNode::Float32(floating))
    }
}

impl JsonWriteSource for TerrainSceneView<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        Ok(match path {
            [] => JsonWriteNode::Object(2),
            [0] => JsonWriteNode::String("mesh.document"),
            [1] => JsonWriteNode::Object(5),
            [1, 0 | 1] => JsonWriteNode::Array(SIZE * SIZE * 3),
            [1, 2] => JsonWriteNode::Array(0),
            [1, 3] => JsonWriteNode::Array((SIZE - 1) * (SIZE - 1) * 6),
            [1, 4] => JsonWriteNode::Array(SIZE * SIZE * 2),
            [1, column, index] => self.scalar_at(*column, *index)?,
            _ => return Err(absent()),
        })
    }

    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        match path {
            [] => ["schema", "mesh"].get(index),
            [1] => ["positions", "normals", "colors", "indices", "uvs"].get(index),
            _ => None,
        }.copied().ok_or_else(absent)
    }
}

fn absent() -> ValueError {
    ValueError::invariant("the GIS terrain JSON projection path is absent")
}

fn invalid_terrain() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "the authored terrain elevation grid is invalid")
}

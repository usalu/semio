//! 🏷️ Variable actions: the nested `variables` rows of storeys, bridges and towers as load glyphs at the level they act on, with one magnitude grip per row that writes `qkN` (the typed `change-<owner>-variable-qk-n` kind).

use crate::*;
use model::En1998VariableAction;
use scene::projection_kit::handle_kit::load_arrow;

const SPACING: f64 = 1.5;
const SHARE_OF_PERMANENT: f64 = 0.25;
const MIN_PER_METRE: f64 = 1.0e3;

/// 🏷️ One variable action row placed at its owner's level.
#[derive(Clone, Debug, PartialEq)]
pub struct Mark {
    pub id: String,
    pub path: String,
    pub row: String,
    pub category: String,
    pub qk_n: f64,
    pub at: Vec3,
    pub per_metre: f64,
}

/// 🏷️ The marks of `rows` beside each other along `+X` around `anchor`; `owner_id` prefixes the entity ids, `owner_path` is the path of the owning row, and the length scale (one metre of arrow per `per_metre` newton) follows the owner's permanent action so that it does not move while a row is dragged.
pub fn marks(owner_id: &str, owner_path: &str, rows: &[En1998VariableAction], permanent_gk_n: f64, anchor: &Vec3) -> Vec<Mark> {
    let middle = (rows.len() as f64 - 1.0) / 2.0;
    rows.iter()
        .enumerate()
        .map(|(index, row)| Mark {
            id: format!("{owner_id}:var-{}", row.id),
            path: format!("{owner_path}.variables[id={}]", row.id),
            row: row.id.clone(),
            category: format!("{:?}", row.category),
            qk_n: row.qk_n,
            at: Vec3::new(anchor.x + (index as f64 - middle) * SPACING, anchor.y, anchor.z),
            per_metre: (permanent_gk_n * SHARE_OF_PERMANENT).max(MIN_PER_METRE),
        })
        .collect()
}

impl Mark {
    pub fn entity(&self) -> EntityRef {
        EntityRef::new(EntityKind::Load, self.id.as_str(), self.path.as_str())
    }

    /// 🏹 The gravity glyph: a point load pointing down onto the level, its shaft length following the magnitude.
    pub fn glyph(&self) -> Geometry {
        Geometry::Load(LoadGlyph::Point { at: self.at.clone(), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, self.qk_n), case: LoadCaseRef { id: format!("variable-{}", self.category), kind: LoadKind::Variable, psi: None } })
    }

    pub fn name(&self) -> LocalizedCopy {
        loc(&format!("Variable action {} ({}): {:.0} kN", self.row, self.category, self.qk_n / 1000.0), &format!("Ver\u{e4}nderliche Einwirkung {} ({}): {:.0} kN", self.row, self.category, self.qk_n / 1000.0))
    }

    /// 🕹️ The magnitude grip `<entity>:qk`: dragged up (away from the level) it raises `qkN` by `per_metre` per metre.
    pub fn grip(&self) -> Handle {
        load_arrow(&self.entity(), "qk", Vec3::Z, &self.at, format!("{}.qkN", self.path), self.qk_n, self.per_metre, "N", loc(&format!("Variable action Q_k of {}", self.row), &format!("Ver\u{e4}nderliche Einwirkung Q_k von {}", self.row)))
    }
}

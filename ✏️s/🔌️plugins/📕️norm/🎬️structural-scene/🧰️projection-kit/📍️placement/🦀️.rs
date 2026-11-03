//! 📍️ Placement: the persisted pose of one spatial entity, identical across artifacts.

use crate::*;

/// 📍️ A persisted placement (origin in m, yaw about `+Z` in rad); mirrors the snapshot `Placement` the mutations `update-<entity>-placement` write.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub entity_id: String,
    pub origin: Vec3,
    pub yaw: f64,
}

/// 🔎️ The persisted placement of `id`, if any.
pub fn placed<'a>(placements: &'a [Placement], id: &str) -> Option<&'a Placement> {
    placements.iter().find(|p| p.entity_id == id)
}

//#region 🔖️Persisted
pub const PLACEMENTS_FIELD: &str = "placements";

struct Fnv(u64);

impl std::fmt::Write for Fnv {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.0 = text.bytes().fold(self.0, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3));
        Ok(())
    }
}

/// 🔢️ FNV-1a 64 of the `Debug` form of `value`: a deterministic content stamp that equals across processes and platforms. The form is hashed while it is written, so a whole document never becomes a string.
pub fn content_stamp<T: std::fmt::Debug>(value: &T) -> u64 {
    let mut hash = Fnv(0xcbf2_9ce4_8422_2325_u64);
    let _ = std::fmt::Write::write_fmt(&mut hash, format_args!("{value:?}"));
    hash.0
}

/// 🏷️ The revision stamp of one projection: the content stamps of the document, its placements and its report, so the incremental scene diff keys on revisions that change exactly when an input changes.
pub fn revision_of<D: std::fmt::Debug, R: std::fmt::Debug>(document: &D, placements: &[Placement], report: &R) -> SceneRevision {
    SceneRevision { document_seq: content_stamp(&(content_stamp(document), placements)), report_seq: content_stamp(report) }
}
//#endregion 🔖️Persisted

//#region 🔖️Events
/// 🧲️ One entity moved by a placement drag or a typed pose.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementMove {
    pub entity_id: String,
    pub origin: Vec3,
    pub yaw: f64,
}

/// ✍️ One write of a placement commit, addressed like `setFields` (`placements[id=ENTITY].x`).
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementWrite {
    pub path: String,
    pub value: f64,
}

/// 🧾️ A placement row event: the first move of an entity inserts its row, later moves overwrite its leaves, a clear removes the row.
#[derive(Clone, Debug, PartialEq)]
pub enum PlacementOp {
    Insert(Placement),
    Set(Vec<PlacementWrite>),
    Remove(String),
}

/// 🧱️ One drag release as one event: all moves of the gesture in order, so the host raises one mutation batch, i.e. one undo step.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlacementCommit {
    pub ops: Vec<PlacementOp>,
}

/// 🔎️ Splits a `placements[id=ENTITY].leaf` path into entity id and leaf.
pub fn placement_target(path: &str) -> Option<(&str, &str)> {
    path.strip_prefix(PLACEMENTS_FIELD)?.strip_prefix("[id=")?.split_once("].")
}

/// 📐️ The grip base of one placement leaf: the persisted value, else the value of the pose the entity stands in.
pub fn grip_base(current: &[Placement], standing: &Vec3, id: &str, leaf: &str) -> f64 {
    let origin = placed(current, id).map_or(standing, |row| &row.origin);
    match leaf {
        "y" => origin.y,
        "z" => origin.z,
        _ => origin.x,
    }
}

fn leaves(id: &str, origin: &Vec3, yaw: f64) -> Vec<PlacementWrite> {
    [("x", origin.x), ("y", origin.y), ("z", origin.z), ("yaw", yaw)].into_iter().map(|(name, value)| PlacementWrite { path: format!("{PLACEMENTS_FIELD}[id={id}].{name}"), value }).collect()
}

impl PlacementCommit {
    /// 🧲️ The commit of `moves` against the `current` rows; a move onto the present pose writes nothing, so a release at the origin leaves no trace.
    pub fn of(current: &[Placement], moves: &[PlacementMove]) -> Self {
        let ops = moves
            .iter()
            .filter_map(|m| match placed(current, &m.entity_id) {
                Some(old) if old.origin == m.origin && old.yaw == m.yaw => None,
                Some(_) => Some(PlacementOp::Set(leaves(&m.entity_id, &m.origin, m.yaw))),
                None => Some(PlacementOp::Insert(Placement { entity_id: m.entity_id.clone(), origin: m.origin.clone(), yaw: m.yaw })),
            })
            .collect();
        Self { ops }
    }

    /// 🖱️ The commit of one handle drag given as `placements[id=ENTITY].leaf` writes. An entity without a row gets ONE insert carrying the pose it stands in (`standing`, else the zero pose) with the dragged leaves applied, so the first drag of any entity is a single undoable event; an entity with a row gets one set. Writes to other paths are ignored.
    pub fn of_writes(current: &[Placement], standing: &dyn Fn(&str) -> Option<Placement>, writes: &[PlacementWrite]) -> Self {
        let mut moves: Vec<PlacementMove> = Vec::new();
        for write in writes {
            let Some((id, leaf)) = placement_target(&write.path) else { continue };
            if !moves.iter().any(|m| m.entity_id == id) {
                let from = placed(current, id).cloned().or_else(|| standing(id)).unwrap_or(Placement { entity_id: id.to_string(), origin: Vec3::new(0.0, 0.0, 0.0), yaw: 0.0 });
                moves.push(PlacementMove { entity_id: id.to_string(), origin: from.origin, yaw: from.yaw });
            }
            let Some(m) = moves.iter_mut().find(|m| m.entity_id == id) else { continue };
            match leaf {
                "x" => m.origin.x = write.value,
                "y" => m.origin.y = write.value,
                "z" => m.origin.z = write.value,
                "yaw" => m.yaw = write.value,
                _ => {}
            }
        }
        let ops = moves
            .iter()
            .filter_map(|m| match placed(current, &m.entity_id) {
                Some(old) if old.origin == m.origin && old.yaw == m.yaw => None,
                Some(_) => Some(PlacementOp::Set(leaves(&m.entity_id, &m.origin, m.yaw))),
                None => Some(PlacementOp::Insert(Placement { entity_id: m.entity_id.clone(), origin: m.origin.clone(), yaw: m.yaw })),
            })
            .collect();
        Self { ops }
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// 🧮️ Folds the commit into `rows`.
    pub fn apply(&self, rows: &mut Vec<Placement>) {
        for op in &self.ops {
            match op {
                PlacementOp::Insert(row) => rows.push(row.clone()),
                PlacementOp::Remove(id) => rows.retain(|row| &row.entity_id != id),
                PlacementOp::Set(writes) => {
                    for write in writes {
                        let Some((id, leaf)) = write.path.strip_prefix(&format!("{PLACEMENTS_FIELD}[id=")).and_then(|rest| rest.split_once("].")) else { continue };
                        let Some(row) = rows.iter_mut().find(|row| row.entity_id == id) else { continue };
                        match leaf {
                            "x" => row.origin.x = write.value,
                            "y" => row.origin.y = write.value,
                            "z" => row.origin.z = write.value,
                            _ => row.yaw = write.value,
                        }
                    }
                }
            }
        }
    }

    /// ↩️ The commit that undoes this one against the rows it was made from, in reverse order, so that undo is also one step.
    pub fn inverse(&self, before: &[Placement]) -> Self {
        let mut rows = before.to_vec();
        let mut inverse = Vec::new();
        for op in &self.ops {
            let step = Self { ops: vec![op.clone()] };
            match op {
                PlacementOp::Insert(row) => inverse.push(PlacementOp::Remove(row.entity_id.clone())),
                PlacementOp::Remove(id) => inverse.extend(placed(&rows, id).map(|row| PlacementOp::Insert(row.clone()))),
                PlacementOp::Set(writes) => {
                    let id = writes.first().and_then(|w| w.path.strip_prefix(&format!("{PLACEMENTS_FIELD}[id=")).and_then(|rest| rest.split_once("].")).map(|(id, _)| id.to_string()));
                    inverse.extend(id.and_then(|id| placed(&rows, &id).map(|row| PlacementOp::Set(leaves(&id, &row.origin, row.yaw)))));
                }
            }
            step.apply(&mut rows);
        }
        inverse.reverse();
        Self { ops: inverse }
    }
}
//#endregion 🔖️Events

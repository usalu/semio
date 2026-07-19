//! ↔️ Undirected adjacency logic — canonical pairs, matrix view, conflicts, and graph edges.

use crate::kernel::EntityId;
use crate::program::Program;
use crate::registers::{Adjacency, AdjacencyKind, SeparationKind};
use mathematical_graph::{orient_endpoints, Undirected};
use serde::{Deserialize, Serialize};

// #region 🔖PairNormalization
/// @emoji 📐 Canonical undirected endpoint order using `mathematical_graph::orient_endpoints`.
pub fn normalize_pair(a: &EntityId, b: &EntityId) -> (EntityId, EntityId) {
    let (left, right) = orient_endpoints::<&str, Undirected>(&a.0, &b.0);
    (EntityId(left.to_string()), EntityId(right.to_string()))
}
// #endregion

// #region 🔖Mutations
/// @emoji ➕ Upserts an adjacency row with normalized endpoints; replaces same pair if present.
pub fn set_adjacency(program: &mut Program, mut adjacency: Adjacency) {
    let (a, b) = normalize_pair(&adjacency.element_a_id, &adjacency.element_b_id);
    adjacency.element_a_id = a;
    adjacency.element_b_id = b;
    adjacency.normalized = true;
    if let Some(existing) = program
        .adjacencies
        .iter()
        .position(|row| row.element_a_id == adjacency.element_a_id && row.element_b_id == adjacency.element_b_id)
    {
        program.adjacencies[existing] = adjacency;
    } else {
        program.adjacencies.push(adjacency);
    }
}

/// @emoji ➖ Removes an adjacency by id or by normalized element pair.
pub fn clear_adjacency(program: &mut Program, id: &EntityId) {
    if let Some(index) = program.adjacencies.iter().position(|row| &row.header.id == id) {
        program.adjacencies.remove(index);
        return;
    }
    program.adjacencies.retain(|row| &row.element_a_id != id && &row.element_b_id != id);
}
// #endregion

// #region 🔖Views
/// @emoji 🔢 Dense lower-triangle adjacency matrix keyed by element id order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjacencyMatrix {
    pub element_ids: Vec<EntityId>,
    pub cells: Vec<Vec<Option<AdjacencyCell>>>,
}

/// @emoji 🟦 One matrix cell summarizing the undirected link between two elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjacencyCell {
    pub adjacency_id: EntityId,
    pub kind: AdjacencyKind,
    pub weight: f64,
    pub separations: Vec<SeparationKind>,
}

/// @emoji 📊 Builds a lower-triangle matrix view over program elements and adjacencies.
pub fn adjacency_matrix(program: &Program) -> AdjacencyMatrix {
    let mut element_ids: Vec<EntityId> = program.elements.iter().map(|e| e.header.id.clone()).collect();
    element_ids.sort();
    let n = element_ids.len();
    let mut cells = vec![vec![None; n]; n];
    for adjacency in &program.adjacencies {
        let Ok(a) = element_ids.binary_search(&adjacency.element_a_id) else {
            continue;
        };
        let Ok(b) = element_ids.binary_search(&adjacency.element_b_id) else {
            continue;
        };
        let (row, col) = if a > b { (a, b) } else { (b, a) };
        cells[row][col] = Some(AdjacencyCell {
            adjacency_id: adjacency.header.id.clone(),
            kind: adjacency.kind.clone(),
            weight: adjacency.weight,
            separations: adjacency.separations.clone(),
        });
    }
    AdjacencyMatrix { element_ids, cells }
}

/// @emoji 🕸️ Undirected edge list for graph rendering (`a`, `b`, weight).
pub fn undirected_edges(program: &Program) -> Vec<(EntityId, EntityId, f64)> {
    program
        .adjacencies
        .iter()
        .map(|adjacency| (adjacency.element_a_id.clone(), adjacency.element_b_id.clone(), adjacency.weight))
        .collect()
}
// #endregion

// #region 🔖Conflicts
/// @emoji ⚡ Adjacency pair ids that violate required/prohibited or separation rules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjacencyConflict {
    pub adjacency_a_id: EntityId,
    pub adjacency_b_id: EntityId,
    pub message: String,
}

/// @emoji 🔍 Detects duplicate pairs, kind conflicts, separation/distance/level violations.
pub fn detect_adjacency_conflicts(program: &Program) -> Vec<AdjacencyConflict> {
    let mut conflicts = Vec::new();
    for (i, left) in program.adjacencies.iter().enumerate() {
        if let (Some(min), Some(max)) = (left.distance_min_m, left.distance_max_m) {
            if min > max {
                conflicts.push(AdjacencyConflict {
                    adjacency_a_id: left.header.id.clone(),
                    adjacency_b_id: left.header.id.clone(),
                    message: format!("distance_min_m ({min}) exceeds distance_max_m ({max})"),
                });
            }
        }
        for right in program.adjacencies.iter().skip(i + 1) {
            let same_pair = (left.element_a_id == right.element_a_id && left.element_b_id == right.element_b_id)
                || (left.element_a_id == right.element_b_id && left.element_b_id == right.element_a_id);
            if !same_pair {
                continue;
            }
            conflicts.push(AdjacencyConflict {
                adjacency_a_id: left.header.id.clone(),
                adjacency_b_id: right.header.id.clone(),
                message: "duplicate adjacency pair".into(),
            });
            if left.kind == AdjacencyKind::Required && right.kind == AdjacencyKind::Prohibited {
                conflicts.push(AdjacencyConflict {
                    adjacency_a_id: left.header.id.clone(),
                    adjacency_b_id: right.header.id.clone(),
                    message: "required adjacency conflicts with prohibited".into(),
                });
            }
            if let (Some(a), Some(b)) = (&left.level_constraint, &right.level_constraint) {
                if a != b {
                    conflicts.push(AdjacencyConflict {
                        adjacency_a_id: left.header.id.clone(),
                        adjacency_b_id: right.header.id.clone(),
                        message: format!("conflicting level constraints: {a} vs {b}"),
                    });
                }
            }
            if separation_incompatible(&left.separations, &right.separations) {
                conflicts.push(AdjacencyConflict {
                    adjacency_a_id: left.header.id.clone(),
                    adjacency_b_id: right.header.id.clone(),
                    message: "incompatible separation requirements on same pair".into(),
                });
            }
            if let (Some(min_a), Some(max_b)) = (left.distance_min_m, right.distance_max_m) {
                if min_a > max_b {
                    conflicts.push(AdjacencyConflict {
                        adjacency_a_id: left.header.id.clone(),
                        adjacency_b_id: right.header.id.clone(),
                        message: format!("distance min {min_a} exceeds paired max {max_b}"),
                    });
                }
            }
        }
        if left.kind == AdjacencyKind::Required {
            for other in &program.adjacencies {
                if other.header.id == left.header.id {
                    continue;
                }
                if other.element_a_id == left.element_a_id
                    && other.element_b_id == left.element_b_id
                    && other.kind == AdjacencyKind::Prohibited
                {
                    conflicts.push(AdjacencyConflict {
                        adjacency_a_id: left.header.id.clone(),
                        adjacency_b_id: other.header.id.clone(),
                        message: "required adjacency conflicts with prohibited".into(),
                    });
                }
            }
        }
    }
    conflicts
}

fn separation_incompatible(left: &[SeparationKind], right: &[SeparationKind]) -> bool {
    let fire_acoustic = |s: &SeparationKind| {
        matches!(s, SeparationKind::Fire | SeparationKind::Acoustic)
    };
    let has_fire = left.iter().any(fire_acoustic) || right.iter().any(fire_acoustic);
    let has_circulation = left.contains(&SeparationKind::Circulation) || right.contains(&SeparationKind::Circulation);
    has_fire && has_circulation && !(left.contains(&SeparationKind::Fire) && right.contains(&SeparationKind::Fire))
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::sample_program;

    #[test]
    fn normalize_pair_orders_endpoints() {
        let a = EntityId("element-2".into());
        let b = EntityId("element-10".into());
        assert_eq!(normalize_pair(&b, &a), (b, a));
    }

    #[test]
    fn sample_program_matrix_has_one_cell() {
        let program = sample_program();
        let matrix = adjacency_matrix(&program);
        assert_eq!(matrix.element_ids.len(), 2);
        let populated: usize = matrix
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_some())
            .count();
        assert_eq!(populated, 1);
    }

    #[test]
    fn detects_distance_min_max_violation() {
        let mut program = sample_program();
        program.adjacencies[0].distance_min_m = Some(10.0);
        program.adjacencies[0].distance_max_m = Some(5.0);
        let conflicts = detect_adjacency_conflicts(&program);
        assert!(conflicts.iter().any(|c| c.message.contains("distance_min")));
    }
}

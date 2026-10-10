//! 🔗️ `references`: the attach references of the authored walls. A wall points at the roof, slab or ceiling its top is attached to and at the slab its base stands on; those elements point at nothing,
//! so the graph of authored references is read straight from the records, never from an inferred surface.

use crate::{ModelSnapshot, TopConstraint, Wall};

/// 🧭️ The roof, slab or ceiling a top constraint is attached to and its offset above the underside of that element.
pub fn top_target(top: &TopConstraint) -> Option<(&str, f64)> {
    match top {
        TopConstraint::Roof { roof, offset } => Some((roof, *offset)),
        TopConstraint::Slab { slab, offset } => Some((slab, *offset)),
        TopConstraint::Ceiling { ceiling, offset } => Some((ceiling, *offset)),
        _ => None,
    }
}

/// 🔗️ The ids a wall is attached to, the top target first, each once.
pub fn targets_of(wall: &Wall) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    if let Some((id, _)) = top_target(&wall.top) {
        out.push(id);
    }
    if let Some(slab) = wall.base_slab.as_deref() {
        if !out.contains(&slab) {
            out.push(slab);
        }
    }
    out
}

/// 🪜️ The storey of the roof, slab or ceiling `id`, `None` when no such element exists.
pub fn storey_of<'a>(snapshot: &'a ModelSnapshot, id: &str) -> Option<&'a String> {
    snapshot.slabs.get(id).map(|row| &row.storey).or_else(|| snapshot.ceilings.get(id).map(|row| &row.storey)).or_else(|| snapshot.roofs.get(id).map(|row| &row.storey))
}

/// 🔁️ The first loop of the reference graph reachable from `start`, the ids on the loop in visiting order; `edges` names the ids a node points at.
pub fn find_cycle(start: &str, edges: &dyn Fn(&str) -> Vec<String>) -> Option<Vec<String>> {
    fn walk(node: &str, edges: &dyn Fn(&str) -> Vec<String>, path: &mut Vec<String>) -> Option<Vec<String>> {
        if let Some(at) = path.iter().position(|seen| seen == node) {
            return Some(path[at..].to_vec());
        }
        path.push(node.to_string());
        for next in edges(node) {
            if let Some(found) = walk(&next, edges, path) {
                return Some(found);
            }
        }
        path.pop();
        None
    }
    walk(start, edges, &mut Vec::new())
}

/// 🔁️ The attach edges of the authored snapshot: a wall points at its targets; a roof, slab or ceiling points at nothing.
pub fn edges(snapshot: &ModelSnapshot) -> impl Fn(&str) -> Vec<String> + '_ {
    move |id| snapshot.walls.get(id).map(|wall| targets_of(wall).into_iter().map(str::to_string).collect()).unwrap_or_default()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

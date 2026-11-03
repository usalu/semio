//! 🕸️ Entity graph: reverse references derived from entity-reference fields.

use super::*;

/// 🕸️ Reverse reference index: who uses an entity, directly and transitively.
#[derive(Clone, Debug, Default)]
pub struct EntityGraph {
    dependents: HashMap<EntityRef, Vec<EntityRef>>,
}

impl EntityGraph {
    pub fn from_edges(edges: impl IntoIterator<Item = EntityEdge>) -> Self {
        let mut dependents: HashMap<EntityRef, Vec<EntityRef>> = HashMap::new();
        for edge in edges {
            let users = dependents.entry(edge.to).or_default();
            if !users.contains(&edge.from) {
                users.push(edge.from);
            }
        }
        Self { dependents }
    }

    pub fn used_by(&self, entity: &EntityRef) -> &[EntityRef] {
        self.dependents.get(entity).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn closure(&self, seeds: &[EntityRef]) -> HashSet<EntityRef> {
        let mut seen: HashSet<EntityRef> = seeds.iter().cloned().collect();
        let mut queue: VecDeque<EntityRef> = seeds.iter().cloned().collect();
        while let Some(entity) = queue.pop_front() {
            for user in self.used_by(&entity) {
                if seen.insert(user.clone()) {
                    queue.push_back(user.clone());
                }
            }
        }
        seen
    }
}

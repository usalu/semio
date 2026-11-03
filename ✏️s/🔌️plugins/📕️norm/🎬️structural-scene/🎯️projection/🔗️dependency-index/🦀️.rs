//! 🔗️ Dependency index: which entity reads which, with transitive closures in both directions for cache keys and invalidation.

use std::collections::{BTreeMap, BTreeSet};

/// 🕸️ Directed read edges `entity -> entities it depends on`, with the reverse view kept in step.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DependencyIndex {
    forward: BTreeMap<String, BTreeSet<String>>,
    reverse: BTreeMap<String, BTreeSet<String>>,
}

impl DependencyIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, id: &str, deps: impl IntoIterator<Item = String>) {
        self.remove(id);
        let deps: BTreeSet<String> = deps.into_iter().filter(|d| d != id).collect();
        for d in &deps {
            self.reverse.entry(d.clone()).or_default().insert(id.to_owned());
        }
        self.forward.insert(id.to_owned(), deps);
    }

    pub fn remove(&mut self, id: &str) {
        if let Some(old) = self.forward.remove(id) {
            for d in old {
                if let Some(users) = self.reverse.get_mut(&d) {
                    users.remove(id);
                    if users.is_empty() {
                        self.reverse.remove(&d);
                    }
                }
            }
        }
    }

    pub fn direct(&self, id: &str) -> BTreeSet<String> {
        self.forward.get(id).cloned().unwrap_or_default()
    }

    fn walk(edges: &BTreeMap<String, BTreeSet<String>>, start: impl IntoIterator<Item = String>) -> BTreeSet<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut stack: Vec<String> = start.into_iter().collect();
        while let Some(id) = stack.pop() {
            for next in edges.get(&id).into_iter().flatten() {
                if seen.insert(next.clone()) {
                    stack.push(next.clone());
                }
            }
        }
        seen
    }

    /// ⬇️ Everything `id` reads, transitively, excluding itself.
    pub fn dependencies(&self, id: &str) -> BTreeSet<String> {
        let mut all = Self::walk(&self.forward, [id.to_owned()]);
        all.remove(id);
        all
    }

    /// ⬆️ The changed ids plus everything that reads any of them, transitively.
    pub fn dependents(&self, changed: &BTreeSet<String>) -> BTreeSet<String> {
        let mut all = Self::walk(&self.reverse, changed.iter().cloned());
        all.extend(changed.iter().cloned());
        all
    }
}

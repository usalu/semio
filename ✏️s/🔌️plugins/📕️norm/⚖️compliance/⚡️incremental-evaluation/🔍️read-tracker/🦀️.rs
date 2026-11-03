//! 🔍️ Read sets: declared document paths per unit, structural read hashing and the undeclared-read audit.

use super::*;

/// 📖️ Dot-separated document paths a unit may read; `*` matches any key or index, a path covers everything below it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReadSet(Vec<String>);

impl ReadSet {
    pub fn new<I: IntoIterator<Item = P>, P: Into<String>>(paths: I) -> Self {
        Self(paths.into_iter().map(Into::into).collect())
    }

    pub fn with(mut self, path: impl Into<String>) -> Self {
        self.0.push(path.into());
        self
    }

    pub fn paths(&self) -> &[String] {
        &self.0
    }

    pub fn covers(&self, path: &str) -> bool {
        let target: Vec<&str> = path.split('.').collect();
        self.0.iter().any(|read| is_prefix(&read.split('.').collect::<Vec<_>>(), &target))
    }

    pub fn overlaps(&self, path: &str) -> bool {
        let target: Vec<&str> = path.split('.').collect();
        self.0.iter().any(|read| {
            let read: Vec<&str> = read.split('.').collect();
            is_prefix(&read, &target) || is_prefix(&target, &read)
        })
    }
}

fn is_prefix(prefix: &[&str], path: &[&str]) -> bool {
    prefix.len() <= path.len() && prefix.iter().zip(path).all(|(a, b)| *a == "*" || *b == "*" || a == b)
}

/// #️⃣ Structural hash of every document value the read set selects; equal hashes mean the unit's inputs are unchanged.
pub fn read_hash(value: &DslValue, reads: &ReadSet) -> u64 {
    ReadHashes::default().of(value, reads)
}

/// #️⃣ [`read_hash`] of a typed `document`: what a session compares between two revisions to decide whether a unit re-runs.
pub fn read_hash_of<D: ToValue>(document: &D, reads: &ReadSet) -> u64 {
    read_hash(&document.to_value(), reads)
}

/// #️⃣ Read hashing with a per-path cache for ONE document value: units that share declared reads (the shared scalars and collections of every row unit) hash each path once per revision instead of once per unit. Create a new one whenever the value changes.
#[derive(Default)]
pub struct ReadHashes(HashMap<String, u64>);

impl ReadHashes {
    pub fn of(&mut self, value: &DslValue, reads: &ReadSet) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for read in reads.paths() {
            let known = match self.0.get(read.as_str()) {
                Some(known) => *known,
                None => {
                    let mut path = std::collections::hash_map::DefaultHasher::new();
                    walk(value, Some(read), &mut path);
                    let known = path.finish();
                    self.0.insert(read.clone(), known);
                    known
                }
            };
            (read, known).hash(&mut hasher);
        }
        hasher.finish()
    }
}

fn walk(value: &DslValue, path: Option<&str>, hasher: &mut std::collections::hash_map::DefaultHasher) {
    let Some(path) = path else {
        hash_value(value, hasher);
        return;
    };
    let (segment, rest) = match path.split_once('.') {
        Some((segment, rest)) => (segment, Some(rest)),
        None => (path, None),
    };
    match value {
        DslValue::Object(entries) => {
            for (key, child) in entries.iter().filter(|(key, _)| segment == "*" || key == segment) {
                key.hash(hasher);
                walk(child, rest, hasher);
            }
        }
        DslValue::Array(items) if segment == "*" => {
            for (index, child) in items.iter().enumerate() {
                index.hash(hasher);
                walk(child, rest, hasher);
            }
        }
        DslValue::Array(items) => {
            if let Some((index, child)) = segment.parse::<usize>().ok().and_then(|index| items.get(index).map(|child| (index, child))) {
                index.hash(hasher);
                walk(child, rest, hasher);
            }
        }
        _ => {}
    }
}

fn hash_value(value: &DslValue, hasher: &mut impl Hasher) {
    match value {
        DslValue::Null => 0u8.hash(hasher),
        DslValue::Bool(flag) => (1u8, flag).hash(hasher),
        DslValue::Number(Number::UInt(n)) => (2u8, n).hash(hasher),
        DslValue::Number(Number::Int(n)) => (3u8, n).hash(hasher),
        DslValue::Number(Number::Float(n)) => (4u8, n.to_bits()).hash(hasher),
        DslValue::String(text) => (5u8, text).hash(hasher),
        DslValue::Bytes(bytes) => (6u8, bytes).hash(hasher),
        DslValue::Array(items) => {
            (7u8, items.len()).hash(hasher);
            items.iter().for_each(|item| hash_value(item, hasher));
        }
        DslValue::Object(entries) => {
            (8u8, entries.len()).hash(hasher);
            entries.iter().for_each(|(key, item)| {
                key.hash(hasher);
                hash_value(item, hasher);
            });
        }
    }
}

fn leaf_paths(value: &DslValue, path: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
    match value {
        DslValue::Object(entries) if !entries.is_empty() => {
            for (key, child) in entries {
                path.push(key.clone());
                leaf_paths(child, path, out);
                path.pop();
            }
        }
        DslValue::Array(items) if !items.is_empty() => {
            for (index, child) in items.iter().enumerate() {
                path.push(index.to_string());
                leaf_paths(child, path, out);
                path.pop();
            }
        }
        _ => out.push(path.clone()),
    }
}

fn perturb(value: &DslValue, path: &[String]) -> DslValue {
    if let Some((head, rest)) = path.split_first() {
        return match value {
            DslValue::Object(entries) => DslValue::Object(entries.iter().map(|(key, child)| (key.clone(), if key == head { perturb(child, rest) } else { child.clone() })).collect()),
            DslValue::Array(items) => DslValue::Array(items.iter().enumerate().map(|(index, child)| if head.parse::<usize>() == Ok(index) { perturb(child, rest) } else { child.clone() }).collect()),
            other => other.clone(),
        };
    }
    match value {
        DslValue::Null => DslValue::Bool(true),
        DslValue::Bool(flag) => DslValue::Bool(!flag),
        DslValue::Number(Number::UInt(n)) => DslValue::Number(Number::UInt(n.wrapping_add(1))),
        DslValue::Number(Number::Int(n)) => DslValue::Number(Number::Int(n.wrapping_add(1))),
        DslValue::Number(Number::Float(n)) => DslValue::Number(Number::Float(n + 1.0)),
        DslValue::String(text) => DslValue::String(format!("{text}~")),
        DslValue::Bytes(bytes) => DslValue::Bytes(bytes.iter().chain(&[1u8]).copied().collect()),
        DslValue::Array(_) => DslValue::Array(vec![DslValue::Null]),
        DslValue::Object(_) => DslValue::Object(vec![("~".to_owned(), DslValue::Null)]),
    }
}

/// 🕵️ Perturbs every document leaf outside the unit's declared read set and returns the paths whose change altered the unit's output (undeclared reads).
pub fn audit_reads<D: ToValue + FromValue>(unit: &CheckUnit<D>, document: &D) -> Vec<String> {
    let value = document.to_value();
    let baseline = unit.execute(document);
    let mut leaves = Vec::new();
    leaf_paths(&value, &mut Vec::new(), &mut leaves);
    leaves
        .into_iter()
        .map(|leaf| leaf.join("."))
        .filter(|path| !unit.reads.covers(path))
        .filter(|path| {
            let segments: Vec<String> = path.split('.').map(str::to_owned).collect();
            D::from_value(perturb(&value, &segments)).map(|changed| unit.execute(&changed) != baseline).unwrap_or(false)
        })
        .collect()
}

/// 🎲️ Deterministic single-leaf edit of `document` chosen by `seed` (numbers shift, flags flip, text changes); `None` when the edited value is not a valid document.
pub fn leaf_edit<D: ToValue + FromValue>(document: &D, seed: u64) -> Option<D> {
    let value = document.to_value();
    let mut leaves = Vec::new();
    leaf_paths(&value, &mut Vec::new(), &mut leaves);
    let pick = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) >> 33;
    let leaf = leaves.get(pick as usize % leaves.len().max(1))?;
    D::from_value(perturb(&value, leaf)).ok()
}

/// 🧩️ `documents` plus one document that carries every row collection: the document with the most rows, each of its empty top-level collections filled with the first row any other document has; the laws then audit and edit rows of collections no single fixture populates.
pub fn with_every_collection<D: ToValue + FromValue>(mut documents: Vec<D>) -> Vec<D> {
    let values: Vec<DslValue> = documents.iter().map(ToValue::to_value).collect();
    let rows_of = |value: &DslValue| match value {
        DslValue::Object(entries) => entries.iter().map(|(_, child)| if let DslValue::Array(items) = child { items.len() } else { 0 }).sum::<usize>(),
        _ => 0,
    };
    let Some(DslValue::Object(entries)) = values.iter().max_by_key(|value| rows_of(value)) else { return documents };
    let donor = |key: &str| {
        values.iter().find_map(|other| match other {
            DslValue::Object(fields) => fields.iter().find_map(|(name, child)| match child {
                DslValue::Array(items) if name == key => items.first().cloned(),
                _ => None,
            }),
            _ => None,
        })
    };
    let filled = entries
        .iter()
        .map(|(key, child)| match child {
            DslValue::Array(items) if items.is_empty() => (key.clone(), DslValue::Array(donor(key).into_iter().collect())),
            other => (key.clone(), other.clone()),
        })
        .collect();
    if let Ok(document) = D::from_value(DslValue::Object(filled)) {
        documents.push(document);
    }
    documents
}

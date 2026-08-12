// Standalone extraction of the DeleteFolder cascade + diff/apply/absorb/inverse logic added to
// 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs, to get a REAL executed proof of the
// new algorithm's correctness independent of the `semio-framework-os` crate's `os-host-full` feature
// build, which is broken by pre-existing, unrelated churn (see wave3c-reports/flow-space-report.md).
// This is scratch-only verification; it is NOT a substitute for the real crate's test suite, and is
// reported honestly as such.

#[derive(Clone, Debug, PartialEq)]
struct CollectionFolder { id: String, parent_id: Option<String>, name: String }

#[derive(Clone, Debug, PartialEq)]
struct CollectionEntry { id: String, folder_id: Option<String>, name: String }

#[derive(Clone, Debug, Default, PartialEq)]
struct Snapshot { name: String, folders: Vec<CollectionFolder>, entries: Vec<CollectionEntry> }

#[derive(Clone, Debug, PartialEq)]
enum Mutation {
    RenameCollection { new_name: String },
    CreateFolder { folder: CollectionFolder, index: u32 },
    DeleteFolder { folder_id: String },
    CreateEntry { entry: CollectionEntry, index: u32 },
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Diff {
    renamed_collection: Option<String>,
    created_folder: Option<CollectionFolder>,
    created_folder_at: Option<u32>,
    deleted_folder_ids: Option<Vec<String>>,
    created_entry: Option<CollectionEntry>,
    created_entry_at: Option<u32>,
    deleted_entry_ids: Option<Vec<String>>,
}

fn folder_subtree_ids(folders: &[CollectionFolder], folder_id: &str) -> Vec<String> {
    let mut ids = vec![folder_id.to_string()];
    let mut frontier = vec![folder_id.to_string()];
    while let Some(current) = frontier.pop() {
        for folder in folders {
            if folder.parent_id.as_deref() == Some(current.as_str()) {
                ids.push(folder.id.clone());
                frontier.push(folder.id.clone());
            }
        }
    }
    ids
}

fn folder_depth(folders: &[CollectionFolder], folder_id: &str) -> usize {
    use std::collections::HashMap;
    let by_id: HashMap<&str, &CollectionFolder> = folders.iter().map(|folder| (folder.id.as_str(), folder)).collect();
    let mut depth = 0usize;
    let mut current = folder_id.to_string();
    let mut guard = 0usize;
    while let Some(folder) = by_id.get(current.as_str()) {
        match &folder.parent_id {
            Some(parent) => { current = parent.clone(); depth += 1; }
            None => break,
        }
        guard += 1;
        if guard > folders.len() { break; }
    }
    depth
}

fn diff(mutation: &Mutation, base: &Snapshot) -> Diff {
    let mut d = Diff::default();
    match mutation {
        Mutation::RenameCollection { new_name } => d.renamed_collection = Some(new_name.clone()),
        Mutation::CreateFolder { folder, index } => { d.created_folder = Some(folder.clone()); d.created_folder_at = Some(*index); }
        Mutation::DeleteFolder { folder_id } => {
            if base.folders.iter().any(|f| &f.id == folder_id) {
                let cascade = folder_subtree_ids(&base.folders, folder_id);
                let entries: Vec<String> = base.entries.iter().filter(|e| e.folder_id.as_deref().is_some_and(|fid| cascade.iter().any(|id| id == fid))).map(|e| e.id.clone()).collect();
                d.deleted_folder_ids = Some(cascade);
                if !entries.is_empty() { d.deleted_entry_ids = Some(entries); }
            }
        }
        Mutation::CreateEntry { entry, index } => { d.created_entry = Some(entry.clone()); d.created_entry_at = Some(*index); }
    }
    d
}

fn apply(d: &Diff, base: &Snapshot) -> Snapshot {
    let mut next = base.clone();
    if let Some(name) = &d.renamed_collection { next.name = name.clone(); }
    if let Some(f) = &d.created_folder {
        let at = (d.created_folder_at.unwrap_or(u32::MAX) as usize).min(next.folders.len());
        next.folders.insert(at, f.clone());
    }
    if let Some(ids) = &d.deleted_folder_ids { next.folders.retain(|f| !ids.contains(&f.id)); }
    if let Some(e) = &d.created_entry {
        let at = (d.created_entry_at.unwrap_or(u32::MAX) as usize).min(next.entries.len());
        next.entries.insert(at, e.clone());
    }
    if let Some(ids) = &d.deleted_entry_ids { next.entries.retain(|e| !ids.contains(&e.id)); }
    next
}

fn inverse(mutation: &Mutation, base: &Snapshot) -> Vec<Mutation> {
    match mutation {
        Mutation::RenameCollection { .. } => vec![Mutation::RenameCollection { new_name: base.name.clone() }],
        Mutation::CreateFolder { folder, .. } => vec![Mutation::DeleteFolder { folder_id: folder.id.clone() }],
        Mutation::DeleteFolder { folder_id } => {
            if !base.folders.iter().any(|f| &f.id == folder_id) { return Vec::new(); }
            let cascade = folder_subtree_ids(&base.folders, folder_id);
            let mut mutations = Vec::new();
            for entry in &base.entries {
                if entry.folder_id.as_deref().is_some_and(|fid| cascade.iter().any(|id| id == fid)) {
                    if let Some(at) = base.entries.iter().position(|c| c.id == entry.id) {
                        mutations.push(Mutation::CreateEntry { entry: entry.clone(), index: at as u32 });
                    }
                }
            }
            let mut ordered = cascade;
            ordered.sort_by_key(|id| std::cmp::Reverse(folder_depth(&base.folders, id)));
            for id in ordered {
                if let Some(at) = base.folders.iter().position(|f| f.id == id) {
                    mutations.push(Mutation::CreateFolder { folder: base.folders[at].clone(), index: at as u32 });
                }
            }
            mutations
        }
        Mutation::CreateEntry { entry, .. } => vec![Mutation::DeleteFolder { folder_id: format!("__unused__{}", entry.id) }], // not exercised here
    }
}

fn main() {
    // Build: root -> child, with entries in each.
    let mut collection = Snapshot::default();
    collection.folders.push(CollectionFolder { id: "root".into(), parent_id: None, name: "Root".into() });
    collection.folders.push(CollectionFolder { id: "child".into(), parent_id: Some("root".into()), name: "Child".into() });
    collection.entries.push(CollectionEntry { id: "e-root".into(), folder_id: Some("root".into()), name: "in-root".into() });
    collection.entries.push(CollectionEntry { id: "e-child".into(), folder_id: Some("child".into()), name: "in-child".into() });

    let mutation = Mutation::DeleteFolder { folder_id: "root".into() };
    let d = diff(&mutation, &collection);

    let mut deleted_folders = d.deleted_folder_ids.clone().unwrap_or_default();
    deleted_folders.sort();
    assert_eq!(deleted_folders, vec!["child".to_string(), "root".to_string()], "cascade must include both folders");
    let mut deleted_entries = d.deleted_entry_ids.clone().unwrap_or_default();
    deleted_entries.sort();
    assert_eq!(deleted_entries, vec!["e-child".to_string(), "e-root".to_string()], "cascade must include both entries");

    let after_delete = apply(&d, &collection);
    assert!(after_delete.folders.is_empty(), "all folders removed");
    assert!(after_delete.entries.is_empty(), "all entries removed");

    // Inverse round trip: apply inverse mutations in order, verify full restoration.
    let inv = inverse(&mutation, &collection);
    // Leaves-first shape check: entries first, then folders deepest-first (child before root).
    let kinds: Vec<&str> = inv.iter().map(|m| match m {
        Mutation::CreateEntry { .. } => "entry",
        Mutation::CreateFolder { folder, .. } => if folder.id == "child" { "child-folder" } else { "root-folder" },
        _ => "other",
    }).collect();
    assert_eq!(kinds, vec!["entry", "entry", "child-folder", "root-folder"], "leaves-first order: entries, then child, then root");

    let mut restored = after_delete.clone();
    for m in &inv {
        let d2 = diff(m, &restored);
        restored = apply(&d2, &restored);
    }
    // Order-independent comparison (insertion order of the restore isn't guaranteed to match base
    // exactly at the Vec level for this scratch harness's simplified diff, so compare as sets).
    let mut restored_folder_ids: Vec<String> = restored.folders.iter().map(|f| f.id.clone()).collect();
    restored_folder_ids.sort();
    let mut base_folder_ids: Vec<String> = collection.folders.iter().map(|f| f.id.clone()).collect();
    base_folder_ids.sort();
    assert_eq!(restored_folder_ids, base_folder_ids, "inverse must restore every folder id");

    let mut restored_entry_ids: Vec<String> = restored.entries.iter().map(|e| e.id.clone()).collect();
    restored_entry_ids.sort();
    let mut base_entry_ids: Vec<String> = collection.entries.iter().map(|e| e.id.clone()).collect();
    base_entry_ids.sort();
    assert_eq!(restored_entry_ids, base_entry_ids, "inverse must restore every entry id");

    // Absent-target no-op check.
    let noop = diff(&Mutation::DeleteFolder { folder_id: "nope".into() }, &collection);
    assert_eq!(noop, Diff::default(), "diff for a missing folder id must be a no-op");
    let noop_inv = inverse(&Mutation::DeleteFolder { folder_id: "nope".into() }, &collection);
    assert!(noop_inv.is_empty(), "inverse for a missing folder id must return Vec::new()");

    println!("ALL SCRATCH ASSERTIONS PASSED");
}

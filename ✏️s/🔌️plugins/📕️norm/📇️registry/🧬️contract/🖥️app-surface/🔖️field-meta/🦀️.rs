//! 🔖️ Field metadata of the norm app surface: the field choices and the wildcard-aware meta resolution. See [`super`].

use super::*;

//#region 🔖️ValuePath
/// 🏷️ One enum/select option — wire `value` plus localized display labels families MUST supply.
#[derive(Clone, Copy, Debug)]
pub struct NormFieldChoice {
    pub value: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

/// 🏷️ Optional per-path field metadata families may supply for the structured inputs editor.
#[derive(Clone, Copy, Debug)]
pub struct NormFieldMeta {
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,
    pub choices: Option<&'static [NormFieldChoice]>,
}

/// 🏷️ Lookup hook consumed by [`resolve_field_meta`] (families usually wrap [`lookup_norm_field_meta`]).
pub type NormFieldMetaFn = fn(&str) -> Option<NormFieldMeta>;

/// 🏷️ Empty metadata table hook — wire `Some(empty_field_meta)` until a family fills its table.
pub fn empty_field_meta(_path: &str) -> Option<NormFieldMeta> {
    None
}

/// 🧩 Shared adapter for family metadata tables.
///
/// # Table format
///
/// Each entry is `(key, NormFieldMeta)` where `key` is a camelCase value-tree path template:
/// - Exact keys: `annex`, `site.seismicZone`
/// - List wildcards: replace every `[index]` with `[]`, e.g. `buildings[].storeys[].massKg`
/// - Nested list keys may nest wildcards: `footings[].loadCases[].verticalPermanent`
///
/// `NormFieldMeta.choices` is a slice of [`NormFieldChoice`]: wire `value` plus `label_en`/`label_de`.
/// Families MUST supply localized choice labels — raw codes (or humanized codes) as the select label
/// alone are not acceptable.
///
/// Lookup order for a concrete path such as `buildings[0].storeys[id=S2].massKg`:
/// 1. Exact key match on the concrete path
/// 2. Exact key match on the `[]`-normalized template (`[index]` and `[id=…]` both become `[]`)
/// 3. Longest prefix at a `.` segment boundary (trying template form, then concrete), so a table
///    entry `buildings[].storeys[]` still labels `buildings[0].storeys[id=S2].massKg` when the leaf
///    key is absent
pub fn lookup_norm_field_meta(table: &[(&str, NormFieldMeta)], path: &str) -> Option<NormFieldMeta> {
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == path) {
        return Some(*meta);
    }
    let templated = index_wildcards(path);
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == templated.as_str()) {
        return Some(*meta);
    }
    longest_prefix_meta(table, &templated).or_else(|| longest_prefix_meta(table, path))
}

fn index_wildcards(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            for x in chars.by_ref() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn longest_prefix_meta(table: &[(&str, NormFieldMeta)], path: &str) -> Option<NormFieldMeta> {
    let mut best: Option<(usize, NormFieldMeta)> = None;
    for (key, meta) in table {
        if path == *key || path.starts_with(&format!("{key}.")) || path.starts_with(&format!("{key}[")) {
            let rank = key.len();
            if best.as_ref().map(|(len, _)| rank > *len).unwrap_or(true) {
                best = Some((rank, *meta));
            }
        }
    }
    best.map(|(_, meta)| meta)
}

/// 🏷️ Resolves metadata for `path` via the family hook, applying `[]` normalization + longest prefix.
pub fn resolve_field_meta(path: &str, meta_fn: Option<NormFieldMetaFn>) -> Option<NormFieldMeta> {
    let lookup = meta_fn?;
    if let Some(meta) = lookup(path) {
        return Some(meta);
    }
    let templated = index_wildcards(path);
    if templated != path {
        if let Some(meta) = lookup(&templated) {
            return Some(meta);
        }
    }
    let mut prefix = templated.as_str();
    while let Some(cut) = prefix.rfind(['.', '[']) {
        prefix = &prefix[..cut];
        if prefix.is_empty() {
            break;
        }
        if let Some(meta) = lookup(prefix) {
            return Some(meta);
        }
    }
    None
}

pub(super) fn field_label(path: &str, meta: Option<NormFieldMeta>, locale: Locale) -> String {
    if let Some(meta) = meta {
        return chrome(meta.label_en, meta.label_de, locale);
    }
    path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']').to_string()
}
//#endregion 🔖️ValuePath

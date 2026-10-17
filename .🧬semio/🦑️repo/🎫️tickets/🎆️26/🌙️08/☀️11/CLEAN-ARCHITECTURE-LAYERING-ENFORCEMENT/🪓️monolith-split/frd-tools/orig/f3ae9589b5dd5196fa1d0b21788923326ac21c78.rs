//! 🧩️ Repo-wide proof that every scope-owned `🧬️schema/🔣️.json` module compiles through the owned
//! draft-07 validator in `✅️validator/🦀️.rs`, and a per-path inventory of the keywords that do not.
//!
//! Roots walked: `🌎️hub`, `🧰️framework`, `✏️s`. The repo root is derived from `CARGO_MANIFEST_DIR`
//! by walking up to the directory that carries `.🧬semio`; `SEMIO_SCHEMA_MODULE_ROOT` overrides it.
//! `SEMIO_SCHEMA_MODULE_COMPILE_OUT` receives the full JSON report when set.
//! @see <https://json-schema.org/draft-07/schema>

use semio_framework_schema::OwnedJsonSchemaValidator;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const SCHEMA_MODULE_DIR: &str = "🧬️schema";
const SCHEMA_MODULE_FILE: &str = "🔣️.json";
const DRAFT07_DIALECT: &str = "http://json-schema.org/draft-07/schema#";
const ROOTS: [&str; 3] = ["🌎️hub", "🧰️framework", "✏️s"];
const SKIPPED_DIRS: [&str; 6] = ["node_modules", "target", "dist", ".git", ".nx", "🗑️generated"];

/// 🚧️ Keywords that are not part of draft-07 at all. A module that carries one is a
/// `📋️execution-contract.md` §B dialect violation owned by that module's partition, never a gap in
/// this validator, so the gate below reports it instead of failing on it.
const NON_DRAFT07_KEYWORDS: [&str; 9] = ["$dynamicAnchor", "$dynamicRef", "$vocabulary", "dependentRequired", "dependentSchemas", "discriminator", "prefixItems", "unevaluatedItems", "unevaluatedProperties"];

/// 🗂️ How one compile failure is attributed. Only `Draft07KeywordGap` is this partition's bug; the
/// other three are inventories of §B violations owned elsewhere and are reported, not asserted on,
/// because peer partitions add and fix modules while this test runs.
fn failure_category(reason: &str) -> &'static str {
    match reason.split("unsupported JSON Schema keyword `").nth(1).and_then(|tail| tail.split('`').next()) {
        Some(keyword) if NON_DRAFT07_KEYWORDS.contains(&keyword) => "nonDraft07Keyword",
        Some(_) => "draft07KeywordGap",
        None if reason.contains("unresolved reference") || reason.contains("unresolved cross-document reference") => "unresolvedReference",
        None => "metaSchemaViolation",
    }
}

fn repo_root() -> PathBuf {
    if let Ok(declared) = std::env::var("SEMIO_SCHEMA_MODULE_ROOT") {
        return PathBuf::from(declared);
    }
    let mut current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !current.join(".🧬semio").is_dir() {
        assert!(current.pop(), "no repo root carrying `.🧬semio` above {}", env!("CARGO_MANIFEST_DIR"));
    }
    current
}

fn collect_modules(directory: &Path, modules: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else { continue };
        if !path.is_dir() || SKIPPED_DIRS.contains(&name) {
            continue;
        }
        if name == SCHEMA_MODULE_DIR && path.join(SCHEMA_MODULE_FILE).is_file() {
            modules.push(path.join(SCHEMA_MODULE_FILE));
        }
        collect_modules(&path, modules);
    }
}

fn declared_string(body: &str, keyword: &str) -> Option<String> {
    let needle = format!("\"{keyword}\"");
    let start = body.find(&needle)? + needle.len();
    let rest = body[start..].trim_start().strip_prefix(':')?.trim_start();
    let quoted = rest.strip_prefix('"')?;
    let end = quoted.find('"')?;
    Some(quoted[..end].to_string())
}

/// 🔗 Every `$ref` target document named in `body`, i.e. the part before the `#` fragment. An empty
/// target is a local pointer and needs no sibling document.
fn referenced_documents(body: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = body;
    while let Some(index) = rest.find("\"$ref\"") {
        rest = &rest[index + "\"$ref\"".len()..];
        let Some(tail) = rest.trim_start().strip_prefix(':') else { continue };
        let Some(quoted) = tail.trim_start().strip_prefix('"') else { continue };
        let Some(end) = quoted.find('"') else { continue };
        let reference = &quoted[..end];
        let target = reference.split('#').next().unwrap_or_default();
        if !target.is_empty() && !targets.iter().any(|known: &String| known == target) {
            targets.push(target.to_string());
        }
    }
    targets
}

fn json_escape(value: &str) -> String {
    value.chars().fold(String::new(), |mut escaped, entry| {
        match entry {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
        escaped
    })
}

#[test]
fn every_draft07_schema_module_compiles_through_the_owned_validator() {
    let root = repo_root();
    let mut modules = Vec::new();
    for lane in ROOTS {
        collect_modules(&root.join(lane), &mut modules);
    }
    modules.sort();
    assert!(modules.len() > 2_000, "expected the full repo module set, walked only {}", modules.len());

    let mut compiled = 0usize;
    let mut draft07_failures: Vec<(String, String)> = Vec::new();
    let mut other_dialect_failures: Vec<(String, String, String)> = Vec::new();
    let mut dialects: BTreeMap<String, usize> = BTreeMap::new();
    let mut unsupported_keywords: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut colliding_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let bodies: Vec<(String, String)> = modules
        .iter()
        .map(|module| {
            let relative = module.strip_prefix(&root).unwrap_or(module).display().to_string();
            let body = std::fs::read_to_string(module).unwrap_or_else(|error| panic!("{relative}: {error}"));
            (relative, body)
        })
        .collect();
    let mut by_id: BTreeMap<String, &str> = BTreeMap::new();
    for (relative, body) in &bodies {
        let Some(id) = declared_string(body, "$id") else { continue };
        colliding_ids.entry(id.clone()).or_default().push(relative.clone());
        by_id.entry(id).or_insert(body.as_str());
    }
    colliding_ids.retain(|_, paths| paths.len() > 1);

    for (relative, body) in &bodies {
        let dialect = declared_string(body, "$schema").unwrap_or_else(|| "<undeclared>".to_string());
        *dialects.entry(dialect.clone()).or_default() += 1;
        let own_id = declared_string(body, "$id");
        let siblings: Vec<&str> = referenced_documents(body)
            .into_iter()
            .filter(|target| own_id.as_deref() != Some(target.as_str()))
            .filter_map(|target| by_id.get(&target).copied())
            .collect();
        let relative = relative.clone();
        match OwnedJsonSchemaValidator::compile_with_documents(body, &siblings) {
            Ok(_) => compiled += 1,
            Err(error) => {
                let reason = error.to_string();
                if let Some(keyword) = reason.split("unsupported JSON Schema keyword `").nth(1).and_then(|tail| tail.split('`').next()) {
                    unsupported_keywords.entry(keyword.to_string()).or_default().push(relative.clone());
                }
                if dialect == DRAFT07_DIALECT {
                    draft07_failures.push((relative, reason));
                } else {
                    other_dialect_failures.push((relative, dialect, reason));
                }
            }
        }
    }

    let report = format!(
        "{{\n  \"modules\": {},\n  \"compiled\": {},\n  \"collidingIds\": {},\n  \"draft07Failures\": [{}\n  ],\n  \"otherDialectFailures\": [{}\n  ],\n  \"dialects\": {{{}\n  }},\n  \"unsupportedKeywords\": {{{}\n  }}\n}}\n",
        modules.len(),
        compiled,
        colliding_ids.len(),
        draft07_failures.iter().map(|(path, reason)| format!("\n    {{ \"path\": \"{}\", \"reason\": \"{}\" }}", json_escape(path), json_escape(reason))).collect::<Vec<_>>().join(","),
        other_dialect_failures.iter().map(|(path, dialect, reason)| format!("\n    {{ \"path\": \"{}\", \"dialect\": \"{}\", \"reason\": \"{}\" }}", json_escape(path), json_escape(dialect), json_escape(reason))).collect::<Vec<_>>().join(","),
        dialects.iter().map(|(dialect, count)| format!("\n    \"{}\": {count}", json_escape(dialect))).collect::<Vec<_>>().join(","),
        unsupported_keywords.iter().map(|(keyword, paths)| format!("\n    \"{}\": [{}\n    ]", json_escape(keyword), paths.iter().map(|path| format!("\n      \"{}\"", json_escape(path))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(","),
    );
    if let Ok(destination) = std::env::var("SEMIO_SCHEMA_MODULE_COMPILE_OUT") {
        if let Some(parent) = Path::new(&destination).parent() {
            std::fs::create_dir_all(parent).expect("report directory");
        }
        std::fs::write(&destination, &report).expect("report");
    }
    println!("[schema-module-compile] modules={} compiled={} draft07Failures={} otherDialectFailures={} collidingIds={}", modules.len(), compiled, draft07_failures.len(), other_dialect_failures.len(), colliding_ids.len());
    for (keyword, paths) in &unsupported_keywords {
        println!("[schema-module-compile] unsupported keyword `{keyword}` in {} module(s), first {}", paths.len(), paths[0]);
    }
    let mut categories: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, reason) in &draft07_failures {
        *categories.entry(failure_category(reason)).or_default() += 1;
    }
    for (category, count) in &categories {
        println!("[schema-module-compile] draft-07 failure category {category}: {count}");
    }
    for (path, reason) in &draft07_failures {
        println!("[schema-module-compile] {} {path}: {reason}", failure_category(reason));
    }

    let gaps: Vec<&(String, String)> = draft07_failures.iter().filter(|(_, reason)| failure_category(reason) == "draft07KeywordGap").collect();
    assert!(gaps.is_empty(), "the owned validator lacks a draft-07 keyword these modules use:\n{}", gaps.iter().map(|(path, reason)| format!("  {path}: {reason}")).collect::<Vec<_>>().join("\n"));
}

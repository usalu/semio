//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 📐️ Domain model of the `🦑️repo` product: the pure data shapes every other repo module
//! exchanges, plus the slug vocabularies and the derivations that are defined by the shape
//! alone. Nothing here touches the filesystem, a process or a clock.
//!
//! The wire contract is `🧬️schema/🔣️.json` (JSON Schema 2020-12) and the allowed LLM /
//! effort / client tables are `🧬️schema/🔣️allowed-values.json`, embedded below with
//! [`include_str!`] and embedded by the Go twin with `go:embed` — one source of truth.
//!
//! Serialisation is byte-compatible with Go `encoding/json` over the same structs:
//! `omitempty` maps to `skip_serializing_if`, a Go `nil` slice or map maps to `Option::None`
//! (which encodes as `null`, exactly as Go does) while an `omitempty` slice maps to a plain
//! `Vec` (both `nil` and empty vanish), and every map is a [`BTreeMap`] so key order matches
//! Go's sorted map encoding.
//!
//! See also `🧰️framework/🛍️products/🦑️repo/AGENTS.md` §🪪 Identification.

//#region 🔌️Adapters
use std::collections::BTreeMap;
use std::fmt;
use std::sync::OnceLock;

use semio_framework_repo_identity::{emoji_text, entity as entity_emoji, ext_of, extract_entity_emoji, flat};
use serde::{Deserialize, Serialize};
use serde_json::Value;
//#endregion 🔌️Adapters

//#region 🪶️Encoding Helpers

/// 🚫️ Serde predicate for a Go `bool` carrying `omitempty`.
fn is_false(value: &bool) -> bool {
    !*value
}

/// 0️⃣ Serde predicate for a Go `int` carrying `omitempty`.
fn is_zero(value: &i64) -> bool {
    *value == 0
}

//#endregion 🪶️Encoding Helpers

//#region ❌️Errors

/// ❌️ Every failure this module can produce. One variant per error class so a test can assert
/// the class without matching a rendered message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelError {
    /// 🚫️ A value did not resolve against one of the allowed vocabularies.
    NotAllowed {
        /// 🏷️ The vocabulary that rejected the value: `llm`, `effort` or `client`.
        field: &'static str,
        /// 🔤️ The normalised slug that was looked up.
        slug: String,
        /// 📋️ The full allowed vocabulary, in declaration order.
        allowed: Vec<String>,
    },
}

impl ModelError {
    /// 🏷️ The stable error class, for language-agnostic assertions.
    pub fn class(&self) -> &'static str {
        match self {
            ModelError::NotAllowed { .. } => "not-allowed",
        }
    }
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::NotAllowed { field, slug, allowed } => write!(
                f,
                "{field} '{slug}' is not allowed. Please use one of: {}",
                allowed.join(", ")
            ),
        }
    }
}

impl std::error::Error for ModelError {}

//#endregion ❌️Errors

//#region 📋️Allowed Values

/// 📋️ The embedded allowed-value tables, shared verbatim with the Go implementation.
#[derive(Clone, Debug, Deserialize)]
struct AllowedValues {
    llms: Vec<String>,
    efforts: Vec<String>,
    clients: Vec<String>,
}

const ALLOWED_VALUES_JSON: &str = include_str!("../../🧬️schema/🔣️allowed-values.json");

fn allowed_values() -> &'static AllowedValues {
    static CELL: OnceLock<AllowedValues> = OnceLock::new();
    CELL.get_or_init(|| {
        serde_json::from_str(ALLOWED_VALUES_JSON)
            .expect("🧬️schema/🔣️allowed-values.json is not a valid allowed-value table")
    })
}

/// 🤖️ The allowed LLM slugs, in declaration order.
pub fn allowed_llms() -> &'static [String] {
    &allowed_values().llms
}

/// 🏋️ The allowed reasoning-effort slugs, in declaration order.
pub fn allowed_efforts() -> &'static [String] {
    &allowed_values().efforts
}

/// 💻️ The allowed client slugs, in declaration order.
pub fn allowed_clients() -> &'static [String] {
    &allowed_values().clients
}

//#endregion 📋️Allowed Values

//#region 🔤️Slugs

/// 🔠️ Go `strings.ToUpper` — simple (non-special-casing) upper mapping, so `ß` stays `ß`.
fn go_to_upper(text: &str) -> String {
    text.chars()
        .map(|c| {
            let mut upper = c.to_uppercase();
            match (upper.next(), upper.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔡️ Go `strings.ToLower` — simple (non-special-casing) lower mapping.
fn go_to_lower(text: &str) -> String {
    text.chars()
        .map(|c| {
            let mut lower = c.to_lowercase();
            match (lower.next(), lower.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🤝️ Go `strings.EqualFold` over simple case folding.
fn eq_fold(left: &str, right: &str) -> bool {
    go_to_lower(left) == go_to_lower(right)
}

/// 💠️ Splits camel case into `-` boundaries, upper cases, collapses every run outside
/// `[A-Z0-9]` into a single `-` and trims the result. Byte-identical to the repo's `Slugify`.
///
/// It lives here, at the bottom of the dependency DAG, because the slug vocabularies below are
/// defined in terms of it. It is public because `🎫️tickets` derives a ticket folder slug from a
/// title with exactly this rule and may not own a second copy of it.
pub fn slugify(text: &str) -> String {
    let runes: Vec<char> = text.chars().collect();
    let mut buf = String::with_capacity(text.len() + 8);
    for (index, &current) in runes.iter().enumerate() {
        if index > 0 && current.is_ascii_uppercase() {
            let previous = runes[index - 1];
            let leaves_a_word = previous.is_ascii_lowercase();
            let starts_a_word = previous.is_ascii_uppercase() && runes.get(index + 1).is_some_and(char::is_ascii_lowercase);
            if leaves_a_word || starts_a_word {
                buf.push('-');
            }
        }
        buf.push(current);
    }
    let upper = go_to_upper(&buf);
    let mut slug = String::with_capacity(upper.len());
    let mut in_separator = false;
    for character in upper.chars() {
        if character.is_ascii_uppercase() || character.is_ascii_digit() {
            slug.push(character);
            in_separator = false;
        } else if !in_separator {
            slug.push('-');
            in_separator = true;
        }
    }
    slug.trim_matches('-').to_string()
}

/// 🤖️ Canonical form of an LLM slug. Idempotent for already-normalised values.
pub fn normalize_llm_slug(llm: &str) -> String {
    go_to_lower(&slugify(llm))
}

/// 🏋️ Canonical form of a reasoning-effort slug. Idempotent for already-normalised values.
pub fn normalize_effort_slug(effort: &str) -> String {
    go_to_lower(&slugify(effort))
}

/// 💻️ Canonical form of a client slug. Idempotent for already-normalised values.
pub fn normalize_client_slug(client: &str) -> String {
    go_to_lower(&slugify(client))
}

fn resolve_by_containment(field: &'static str, slug: String, allowed: &'static [String]) -> Result<String, ModelError> {
    let mut best = "";
    for candidate in allowed {
        if slug.contains(&go_to_lower(&slugify(candidate))) && candidate.len() > best.len() {
            best = candidate;
        }
    }
    if best.is_empty() {
        return Err(ModelError::NotAllowed { field, slug, allowed: allowed.to_vec() });
    }
    Ok(best.to_string())
}

/// 🤖️ Resolves a free-form LLM name to the longest allowed slug it contains.
pub fn resolve_allowed_llm(llm: &str) -> Result<String, ModelError> {
    resolve_by_containment("llm", normalize_llm_slug(llm), allowed_llms())
}

/// 🏋️ Resolves a free-form reasoning effort to the longest allowed slug it contains. Blank
/// input resolves to a blank effort rather than an error, because effort is optional.
pub fn resolve_allowed_effort(effort: &str) -> Result<String, ModelError> {
    if effort.trim().is_empty() {
        return Ok(String::new());
    }
    resolve_by_containment("effort", normalize_effort_slug(effort), allowed_efforts())
}

/// 💻️ Resolves a free-form client name to the longest allowed slug it contains.
pub fn resolve_allowed_client(client: &str) -> Result<String, ModelError> {
    resolve_by_containment("client", normalize_client_slug(client), allowed_clients())
}

//#endregion 🔤️Slugs

//#region 🏷️Enumerations

/// 📖️ What a definition is, derived from the raw keyword a language plugin reports.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum DefinitionKind {
    #[serde(rename = "implementation")]
    #[default]
    Implementation,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "test")]
    Test,
}

impl DefinitionKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            DefinitionKind::Implementation => "implementation",
            DefinitionKind::Interface => "interface",
            DefinitionKind::Constant => "constant",
            DefinitionKind::Test => "test",
        }
    }
}

impl fmt::Display for DefinitionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📝️ Infers the definition kind from a language's raw declaration keyword. Anything
/// unrecognised is an implementation, so the function is total.
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind {
    match go_to_lower(raw_kind).as_str() {
        "interface" | "type" | "trait" | "abstract" | "extend type" | "extend interface"
        | "extend enum" | "extend union" | "extend input" | "input" | "union" | "scalar"
        | "delegate" | "record" => DefinitionKind::Interface,
        "const" | "constant" | "enum" | "var" | "let" | "static" => DefinitionKind::Constant,
        "test" => DefinitionKind::Test,
        _ => DefinitionKind::Implementation,
    }
}

/// 🎫️ Lifecycle state of a ticket.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TicketStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl TicketStatus {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketStatus::Open => "open",
            TicketStatus::Closed => "closed",
        }
    }
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🎯️ Lifecycle state of a goal.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoalStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl GoalStatus {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            GoalStatus::Open => "open",
            GoalStatus::Closed => "closed",
        }
    }
}

impl fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🔸️ How urgently a breach must be dealt with.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum BreachPriority {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    #[default]
    Low,
}

impl BreachPriority {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BreachPriority::High => "high",
            BreachPriority::Medium => "medium",
            BreachPriority::Low => "low",
        }
    }
}

impl fmt::Display for BreachPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🛠️ Which family a technology belongs to. The wire value is the technology emoji itself.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TechnologyKind {
    #[serde(rename = "👤️")]
    User,
    #[serde(rename = "🧰️")]
    Infrastructure,
    #[serde(rename = "🔬️")]
    Research,
    #[serde(rename = "🌱️")]
    Mono,
}

impl TechnologyKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TechnologyKind::User => "👤️",
            TechnologyKind::Infrastructure => "🧰️",
            TechnologyKind::Research => "🔬️",
            TechnologyKind::Mono => "🌱️",
        }
    }
}

impl fmt::Display for TechnologyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🏷️ Infers the technology kind from a technology name.
pub fn derive_technology_kind(name: &str) -> TechnologyKind {
    match name {
        "compose" => TechnologyKind::User,
        "repo" => TechnologyKind::Infrastructure,
        "coda" => TechnologyKind::Research,
        _ => match name.strip_prefix('@') {
            Some(rest) => derive_technology_kind(rest),
            None => TechnologyKind::User,
        },
    }
}

/// 📦️ What a bundle produces.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum BundleKind {
    #[serde(rename = "library")]
    #[default]
    Library,
    #[serde(rename = "schema")]
    Schema,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "ui")]
    Ui,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "repo")]
    Repo,
}

impl BundleKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BundleKind::Library => "library",
            BundleKind::Schema => "schema",
            BundleKind::Binary => "binary",
            BundleKind::Ui => "ui",
            BundleKind::Site => "site",
            BundleKind::Assets => "assets",
            BundleKind::Repo => "repo",
        }
    }
}

impl fmt::Display for BundleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📁️ Whether a folder is required by a toolchain or purely organisational.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FolderKind {
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "root")]
    Root,
}

impl FolderKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            FolderKind::Organization => "organization",
            FolderKind::Required => "required",
            FolderKind::Root => "root",
        }
    }
}

impl fmt::Display for FolderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📄️ What role a file plays. The Go twin keeps these as bare string constants.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "license")]
    License,
}

impl FileKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            FileKind::Code => "code",
            FileKind::Script => "script",
            FileKind::Config => "config",
            FileKind::Lab => "lab",
            FileKind::Docs => "docs",
            FileKind::Resource => "resource",
            FileKind::Template => "template",
            FileKind::License => "license",
        }
    }
}

impl fmt::Display for FileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// ♻️ How an entity changed between two checkpoints.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticChangeType {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "renamed")]
    Renamed,
}

impl SemanticChangeType {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SemanticChangeType::Added => "added",
            SemanticChangeType::Deleted => "deleted",
            SemanticChangeType::Modified => "modified",
            SemanticChangeType::Renamed => "renamed",
        }
    }
}

impl fmt::Display for SemanticChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

//#endregion 🏷️Enumerations

//#region 🔢️Metrics

/// 💿️ A closed line range inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Range {
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "end")]
    pub end: i64,
}

/// 🟨️ Added and removed line counts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LineMetrics {
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 🟩️ The concrete line numbers a diff touched.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiffLines {
    #[serde(rename = "Added")]
    pub added: Option<Vec<i64>>,
    #[serde(rename = "Removed")]
    pub removed: Option<Vec<i64>>,
}

/// 🟦️ Added, updated and removed entity counts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CountMetrics {
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 🟫️ Breach counts split by priority.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriorityCount {
    #[serde(rename = "high")]
    pub high: i64,
    #[serde(rename = "medium")]
    pub medium: i64,
    #[serde(rename = "low")]
    pub low: i64,
}

/// 🔬️ Aggregate breach statistics of an analyze run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnalyzeMetrics {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "byPriority")]
    pub by_priority: Option<PriorityCount>,
    #[serde(rename = "autofixable")]
    pub autofixable: i64,
}

//#endregion 🔢️Metrics

//#region 🌐️Repo Aggregate

/// 💠️ The repository itself: the root of every artifact id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Repo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "technologies")]
    pub technologies: Option<Vec<Technology>>,
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<Bundle>>,
}

/// 📜️ A technology: the first grouping level under the repo root.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Technology {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "kind")]
    pub kind: TechnologyKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<Bundle>>,
}

/// 🔵️ A bundle: one buildable unit inside a technology.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Bundle {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "sourceRoot", default, skip_serializing_if = "String::is_empty")]
    pub source_root: String,
    #[serde(rename = "technologyName")]
    pub technology_name: String,
    #[serde(rename = "tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "kind")]
    pub kind: BundleKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "packages", default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<Package>,
}

/// 🔴️ One published package produced by a bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Package {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "kind")]
    pub kind: String,
}

/// 🩷️ A directory inside the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Folder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "kind")]
    pub kind: FolderKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[serde(rename = "generated")]
    pub generated: bool,
}

/// 📄️ A file inside the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[serde(rename = "generated")]
    pub generated: bool,
}

/// 💗️ A named region inside a file, possibly nested.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Section {
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path", default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
    #[serde(rename = "children", default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Section>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<Definition>,
}

/// 💕️ A single declaration inside a section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Definition {
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "kind")]
    pub kind: DefinitionKind,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(rename = "sectionPath", default, skip_serializing_if = "String::is_empty")]
    pub section_path: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
}

//#endregion 🌐️Repo Aggregate

//#region 🧑️Contributor Aggregate

/// 🤝️ Rendered avatar links of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorIcons {
    #[serde(rename = "avatar", default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "avatarRound", default, skip_serializing_if = "Option::is_none")]
    pub avatar_round: Option<String>,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
}

/// 🔗️ A named external link of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorLink {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}

/// 🤍️ A ticket a contributor worked on, as stored in the contributor record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorTicket {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
}

/// 🔀️ A checkpoint a contributor authored, as stored in the contributor record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorCheckpoint {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "sha")]
    pub sha: String,
}

/// 🖤️ The persisted contribution index of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorContributionsStorage {
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<String>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(rename = "regions", default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "tickets", default, skip_serializing_if = "Vec::is_empty")]
    pub tickets: Vec<ContributorTicket>,
    #[serde(rename = "checkpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<ContributorCheckpoint>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 🧑️‍💻️ A person or agent that contributes to the repository.
///
/// The four members Go writes unconditionally (`alias`, `github`, `name`, `email`) carry
/// `default` on the way in: Go's `encoding/json` leaves a missing member at its zero value rather
/// than refusing the document, and three of the committed `🧑️‍💻️contributor.json` in this
/// repository predate `email` being written at all. Encoding is unchanged — all four are still
/// emitted, matching the `json:"…"` tags that carry no `omitempty`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contributor {
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "aliases", default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(rename = "emoji", default, skip_serializing_if = "String::is_empty")]
    pub emoji: String,
    #[serde(rename = "github", default)]
    pub github: String,
    #[serde(rename = "githubs", default, skip_serializing_if = "Vec::is_empty")]
    pub githubs: Vec<String>,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "names", default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "links", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,
    #[serde(rename = "fingerprint", default, skip_serializing_if = "String::is_empty")]
    pub fingerprint: String,
    #[serde(rename = "fingerprints", default, skip_serializing_if = "Vec::is_empty")]
    pub fingerprints: Vec<String>,
    #[serde(rename = "contributions", default)]
    pub contributions: ContributorContributionsStorage,
}

/// 🔖️ Line totals a contributor owns inside one definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorDefinition {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
}

/// 🔖️ Line totals a contributor owns inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorSection {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Definitions")]
    pub definitions: Option<Vec<ContributorDefinition>>,
}

/// 🔖️ Line totals a contributor owns inside one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFile {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Sections")]
    pub sections: Option<Vec<ContributorSection>>,
}

/// 🔖️ Line totals a contributor owns inside one folder.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFolder {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Files")]
    pub files: Option<Vec<ContributorFile>>,
}

/// 🔖️ Line totals a contributor owns inside one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorBundle {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Folders")]
    pub folders: Option<Vec<ContributorFolder>>,
}

/// 🌳️ The expanded contribution tree of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorContributionsTree {
    #[serde(rename = "Checkpoints")]
    pub checkpoints: Option<Vec<Checkpoint>>,
    #[serde(rename = "Tickets")]
    pub tickets: Option<Vec<Ticket>>,
    #[serde(rename = "Bundles")]
    pub bundles: Option<Vec<ContributorBundle>>,
}

/// 🔖️ Count metrics of a contributor against one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionBundle {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<CountMetrics>,
}

/// 🔖️ Count metrics of a contributor against one folder.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionFolder {
    #[serde(rename = "folderId")]
    pub folder_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<CountMetrics>,
}

/// 🔖️ Line metrics of a contributor against one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionFile {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ Line metrics of a contributor against one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionSection {
    #[serde(rename = "sectionId")]
    pub section_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ Line metrics of a contributor against one definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionDefinition {
    #[serde(rename = "definitionId")]
    pub definition_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ The contribution index of a contributor, keyed by artifact id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorContributions {
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<ContributionBundle>>,
    #[serde(rename = "folders")]
    pub folders: Option<Vec<ContributionFolder>>,
    #[serde(rename = "files")]
    pub files: Option<Vec<ContributionFile>>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<ContributionSection>>,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<ContributionDefinition>>,
}

//#endregion 🧑️Contributor Aggregate

//#region 🔀️Checkpoint Aggregate

/// ✔️ A committed checkpoint of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Checkpoint {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "authorId", default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "date")]
    pub date: String,
}

/// 💾️ A from/to pair for a renamed entity in a checkpoint diff.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckpointDiffRename {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
}

/// 🆕️ Deleted, renamed, modified and created entities of one diff category.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointDiffStats {
    #[serde(rename = "deleted", default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<String>,
    #[serde(rename = "renamed", default, skip_serializing_if = "Vec::is_empty")]
    pub renamed: Vec<CheckpointDiffRename>,
    #[serde(rename = "modified", default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<String>,
    #[serde(rename = "created", default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<String>,
}

/// 📁️ The full entity diff of a checkpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointDiff {
    #[serde(rename = "technologies", default)]
    pub technologies: CheckpointDiffStats,
    #[serde(rename = "bundles", default)]
    pub bundles: CheckpointDiffStats,
    #[serde(rename = "folders", default)]
    pub folders: CheckpointDiffStats,
    #[serde(rename = "files", default)]
    pub files: CheckpointDiffStats,
    #[serde(rename = "sections", default)]
    pub sections: CheckpointDiffStats,
    #[serde(rename = "definitions", default)]
    pub definitions: CheckpointDiffStats,
}

//#endregion 🔀️Checkpoint Aggregate

//#region 💬️Interaction Aggregate

/// 📄️ A file an interaction touched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InteractionFile {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "uri", default, skip_serializing_if = "String::is_empty")]
    pub uri: String,
}

/// 🟫️ One recorded exchange between a contributor and the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Interaction {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "system")]
    pub system: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "checkpoint")]
    pub checkpoint: String,
    #[serde(rename = "prompt", default, skip_serializing_if = "String::is_empty")]
    pub prompt: String,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<InteractionFile>,
}

/// 🎁️ An interaction flattened together with the artifact it was recorded on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InteractionResource {
    #[serde(flatten)]
    pub interaction: Interaction,
    #[serde(rename = "sourceKind")]
    pub source_kind: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "goalId", default, skip_serializing_if = "String::is_empty")]
    pub goal_id: String,
    #[serde(rename = "ticketId", default, skip_serializing_if = "String::is_empty")]
    pub ticket_id: String,
}

//#endregion 💬️Interaction Aggregate

//#region 🎫️Ticket Aggregate

/// 🧷️ An IDE-native plan or spec file archived into the ticket folder on close.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketPlan {
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(rename = "local", default, skip_serializing_if = "String::is_empty")]
    pub local: String,
}

/// 🎫️ One step of an agent's plan on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketAgentPlanStep {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "status", default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(rename = "ideated", default, skip_serializing_if = "String::is_empty")]
    pub ideated: String,
    #[serde(rename = "started", default, skip_serializing_if = "String::is_empty")]
    pub started: String,
    #[serde(rename = "completed", default, skip_serializing_if = "String::is_empty")]
    pub completed: String,
    #[serde(rename = "abandoned", default, skip_serializing_if = "String::is_empty")]
    pub abandoned: String,
}

/// 💠️ The plan an agent declared on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketAgentPlan {
    #[serde(rename = "steps", default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<TicketAgentPlanStep>,
}

/// 🔳️ One agent session recorded on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketAgent {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "contributor", default, skip_serializing_if = "String::is_empty")]
    pub contributor: String,
    #[serde(rename = "system", default, skip_serializing_if = "String::is_empty")]
    pub system: String,
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "transcript", default, skip_serializing_if = "String::is_empty")]
    pub transcript: String,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketAgentPlan>,
}

/// 🐙️ The management-provider link of a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketManagementData {
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
}

/// 🔖️ A ticket: one unit of work with a lifecycle and a contribution record.
///
/// The Go twin carries a hand-written `UnmarshalJSON` that additionally rewrites goal and
/// contributor compose-ids and absorbs pre-schema field spellings. Both belong to `🪪️identity`
/// and `🎫️tickets`; this crate is the pure shape.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ticket {
    #[serde(skip)]
    pub year: i64,
    #[serde(skip)]
    pub month: i64,
    #[serde(skip)]
    pub day: i64,
    #[serde(skip)]
    pub slug: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "emoji", default, skip_serializing_if = "String::is_empty")]
    pub emoji: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<TicketManagementData>,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(skip)]
    pub parent: String,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketPlan>,
    #[serde(rename = "sessions", default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<String>,
    #[serde(skip)]
    pub interactions: Vec<Interaction>,
    #[serde(skip)]
    pub agents: Vec<TicketAgent>,
    #[serde(skip)]
    pub folder_path: String,
    #[serde(skip)]
    pub json_path: String,
    #[serde(skip)]
    pub important_path: String,
}

/// ◼ The persisted subset of a ticket, without any derived field.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketData {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<TicketManagementData>,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
}

/// 🟪️ Creation and completion timestamps of a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketDate {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "finished", default, skip_serializing_if = "Option::is_none")]
    pub finished: Option<String>,
}

/// ◾ A file a ticket touched, with its line metrics.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFile {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// ◽ A file a ticket renamed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileRenamed {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 🗃️ The four change classes of one entity level.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDiffSet {
    #[serde(rename = "deleted")]
    pub deleted: Option<Vec<TicketFile>>,
    #[serde(rename = "renamed")]
    pub renamed: Option<Vec<TicketFileRenamed>>,
    #[serde(rename = "modified")]
    pub modified: Option<Vec<TicketFile>>,
    #[serde(rename = "added")]
    pub added: Option<Vec<TicketFile>>,
}

/// ◻ The semantic diff of a ticket across every entity level.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDiffs {
    #[serde(rename = "bundles", default)]
    pub bundles: TicketDiffSet,
    #[serde(rename = "folders", default)]
    pub folders: TicketDiffSet,
    #[serde(rename = "files", default)]
    pub files: TicketDiffSet,
    #[serde(rename = "sections", default)]
    pub sections: TicketDiffSet,
    #[serde(rename = "definitions", default)]
    pub definitions: TicketDiffSet,
}

/// 📑️ The metrics a ticket accumulated inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketSectionMetrics {
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// ▫️ A named section a ticket touched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSection {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 📍️ The metrics a ticket accumulated inside one file, keyed by section name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileMetricsEntry {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
    #[serde(rename = "sections", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sections: BTreeMap<String, TicketSectionMetrics>,
}

/// 🎫️ Section metrics of one file, keyed by section name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketFileMetrics {
    #[serde(rename = "sections")]
    pub sections: Option<BTreeMap<String, TicketSectionMetrics>>,
}

/// 📦️ File metrics of one bundle, keyed by file path.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketBundleMetrics {
    #[serde(rename = "files")]
    pub files: Option<BTreeMap<String, TicketFileMetrics>>,
}

/// ⬛️ Bundle metrics of a ticket, keyed by bundle id.
pub type TicketBundles = BTreeMap<String, TicketBundleMetrics>;

/// 🔖️ The definitions and metrics a ticket touched inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSectionContrib {
    #[serde(rename = "sectionId")]
    pub section_id: String,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<String>>,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ The sections a ticket touched inside one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileContrib {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<TicketSectionContrib>>,
}

/// 🔖️ The files a ticket touched inside one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketBundleContrib {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "files")]
    pub files: Option<Vec<TicketFileContrib>>,
}

//#endregion 🎫️Ticket Aggregate

//#region 🎯️Goal Aggregate

/// 🟠️ The dates a goal declares.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalDates {
    #[serde(rename = "due", default, skip_serializing_if = "String::is_empty")]
    pub due: String,
}

/// 🟡️ The management-provider link of a goal.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalManagementData {
    #[serde(rename = "milestone", default, skip_serializing_if = "String::is_empty")]
    pub milestone: String,
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
}

/// 🔵️ A goal: the long-lived intent that tickets are opened against.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Goal {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "status")]
    pub status: GoalStatus,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "dueDate", default, skip_serializing_if = "String::is_empty")]
    pub due_date: String,
    #[serde(rename = "dates", default)]
    pub dates: GoalDates,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<GoalManagementData>,
    #[serde(skip)]
    pub id: String,
    #[serde(skip)]
    pub path: String,
}

//#endregion 🎯️Goal Aggregate

//#region 📝️Draft Aggregate

/// 💿️ A draft: a named set of files staged before a ticket exists.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Draft {
    #[serde(rename = "id")]
    pub id: String,
}

//#endregion 📝️Draft Aggregate

//#region ✅️Todo Aggregate

/// 🔷️ A position inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Location {
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "line")]
    pub line: i64,
    #[serde(rename = "column")]
    pub column: i64,
}

/// ✅️ A todo recorded against an artifact.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Todo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "location", default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

//#endregion ✅️Todo Aggregate

//#region 👮️Statute Aggregate

/// 🟢️ The slash-separated identifier of a statute, for example `code/section/empty`. The
/// closed vocabulary and its metadata belong to `📜️statutes`; the model only carries the id
/// so that every artifact type can reference a statute without depending on that module.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Statute(pub String);

impl fmt::Display for Statute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for Statute {
    fn from(value: &str) -> Self {
        Statute(value.to_string())
    }
}

/// 🟣️ A named grouping of statutes and nested territories.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Territory {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "scopes", default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "groups", default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Territory>,
    #[serde(rename = "kinds", default, skip_serializing_if = "Vec::is_empty")]
    pub kinds: Vec<Statute>,
}

impl Territory {
    /// 🟤️ Every statute of this territory and of every nested territory, depth first.
    pub fn all_kinds(&self) -> Vec<Statute> {
        let mut result = self.kinds.clone();
        for child in &self.groups {
            result.extend(child.all_kinds());
        }
        result
    }
}

/// 🔖️ The declared metadata of one statute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StatuteMeta {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "solution")]
    pub solution: String,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
}

/// 🔖️ A policy: a scoped set of territories and statutes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Policy {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "groups")]
    pub groups: Option<Vec<Territory>>,
    #[serde(rename = "statutes")]
    pub statutes: Option<Vec<StatuteMeta>>,
}

/// 🔶️ One detected violation of a statute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Breach {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "line", default, skip_serializing_if = "is_zero")]
    pub line: i64,
    #[serde(rename = "column", default, skip_serializing_if = "is_zero")]
    pub column: i64,
    #[serde(rename = "excerpt", default, skip_serializing_if = "String::is_empty")]
    pub excerpt: String,
    #[serde(rename = "priority", default, skip_serializing_if = "Option::is_none")]
    pub lint_priority: Option<BreachPriority>,
    #[serde(rename = "autofixable", default, skip_serializing_if = "Option::is_none")]
    pub lint_autofixable: Option<bool>,
    #[serde(rename = "reason", default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(rename = "solution", default, skip_serializing_if = "String::is_empty")]
    pub solution: String,
}

/// 🔖️ The outcome of an analyze run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnalyzeResult {
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<Breach>>,
    #[serde(rename = "metrics")]
    pub metrics: Option<AnalyzeMetrics>,
}

/// 🔖️ The outcome of a fix run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FixResult {
    #[serde(rename = "fixed")]
    pub fixed: i64,
    #[serde(rename = "remaining")]
    pub remaining: i64,
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<Breach>>,
}

/// 🔖️ A single semantic change to one entity between two checkpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SemanticChange {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Status")]
    pub status: SemanticChangeType,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "FromPath")]
    pub from_path: String,
    #[serde(rename = "ToPath")]
    pub to_path: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
}

//#endregion 👮️Statute Aggregate

//#region 🖥️Codebase Aggregate

/// 💿️ Entity counts of one bundle in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleMetricsInternal {
    #[serde(rename = "folders")]
    pub folders: i64,
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📁️ Entity counts of one folder in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FolderMetricsInternal {
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📄️ Entity counts of one file in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileMetricsInternal {
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
}

/// 📑️ Entity counts of one section in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectionMetricsInternal {
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📖️ Entity counts of one definition in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DefinitionMetricsInternal {
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 🔷️ A line and column position.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RangePosition {
    #[serde(rename = "line")]
    pub line: i64,
    #[serde(rename = "column")]
    pub column: i64,
}

/// 🔶️ A start/end position pair inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileRange {
    #[serde(rename = "start")]
    pub start: RangePosition,
    #[serde(rename = "end")]
    pub end: RangePosition,
}

/// 🔹️ The file a codebase breach points at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BreachFile {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<FileRange>,
}

/// 🔸️ The folder a codebase breach points at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BreachFolder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
}

/// 🔺️ A breach as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseBreach {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<BreachFolder>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<BreachFile>,
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// 📦️ A bundle as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseBundle {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "folder")]
    pub folder: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "contributors", default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<String>,
    #[serde(rename = "tickets", default, skip_serializing_if = "Vec::is_empty")]
    pub tickets: Vec<String>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BundleMetricsInternal>,
}

/// 🔻️ A folder as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseFolder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FolderMetricsInternal>,
}

/// 🔗️ The statute a file-level breach reference carries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileBreachRef {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// ⬛️ A file as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseFile {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FileMetricsInternal>,
    #[serde(rename = "breachs", default, skip_serializing_if = "Vec::is_empty")]
    pub breachs: Vec<FileBreachRef>,
}

/// ⬜️ A section as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseSection {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<SectionMetricsInternal>,
}

/// 🟥️ A definition as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseDefinition {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<DefinitionMetricsInternal>,
}

/// 🤝️ A contributor's count metrics against one bundle in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorBundleContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🟧️ A contributor's count metrics against one folder in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFolderContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🟨️ A contributor's line metrics against one file in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFileContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟩️ A contributor's line metrics against one section in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorSectionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟦️ A contributor's line metrics against one definition in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorDefinitionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟪️ The full contribution index of a contributor in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorContributionsInternal {
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<ContributorBundleContrib>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<ContributorFolderContrib>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<ContributorFileContrib>,
    #[serde(rename = "sections", default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<ContributorSectionContrib>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<ContributorDefinitionContrib>,
}

/// 🟫️ Aggregate counts of a contributor in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorMetricsInternal {
    #[serde(rename = "checkpoints")]
    pub checkpoints: i64,
    #[serde(rename = "tickets")]
    pub tickets: i64,
    #[serde(rename = "bundles")]
    pub bundles: i64,
    #[serde(rename = "folders")]
    pub folders: i64,
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
}

/// 💠️ A contributor as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseContributor {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(rename = "icons", default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<ContributorIcons>,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "links", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,
    #[serde(rename = "contributions", default, skip_serializing_if = "Option::is_none")]
    pub contributions: Option<ContributorContributionsInternal>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<ContributorMetricsInternal>,
}

/// 🎫️ The created/finished dates of a ticket in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDateInfo {
    #[serde(rename = "created", default, skip_serializing_if = "String::is_empty")]
    pub created: String,
    #[serde(rename = "finished", default, skip_serializing_if = "String::is_empty")]
    pub finished: String,
}

/// 🔳️ A ticket's count metrics against one bundle in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketBundleContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🔲️ A ticket's count metrics against one folder in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFolderContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ▪️ A ticket's count metrics against one file in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ▫️ A ticket's count metrics against one section in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSectionContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ◾ A ticket's line metrics against one definition in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketDefinitionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🗃️ A ticket as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseTicket {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<TicketDateInfo>,
    #[serde(rename = "checkpoint", default, skip_serializing_if = "String::is_empty")]
    pub checkpoint: String,
    #[serde(rename = "year")]
    pub year: String,
    #[serde(rename = "month")]
    pub month: String,
    #[serde(rename = "day")]
    pub day: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "prompt", default, skip_serializing_if = "String::is_empty")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "author", default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<TicketBundleContribInfo>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<TicketFolderContribInfo>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<TicketFileContribInfo>,
    #[serde(rename = "sections", default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<TicketSectionContribInfo>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<TicketDefinitionContrib>,
}

/// 📜️ The statute a policy-level breach reference carries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PolicyBreachRef {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// ◽ A policy as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebasePolicy {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scopes", default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "breachs", default, skip_serializing_if = "Vec::is_empty")]
    pub breachs: Vec<PolicyBreachRef>,
}

/// 🌳️ The entity level of a codebase tree node.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CbTreeNodeKind {
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "definition")]
    Definition,
}

impl CbTreeNodeKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            CbTreeNodeKind::Repo => "repo",
            CbTreeNodeKind::Bundle => "bundle",
            CbTreeNodeKind::Folder => "folder",
            CbTreeNodeKind::File => "file",
            CbTreeNodeKind::Section => "section",
            CbTreeNodeKind::Definition => "definition",
        }
    }
}

impl fmt::Display for CbTreeNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🌿️ One node of the codebase containment tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CbTreeNode {
    #[serde(rename = "kind")]
    pub kind: CbTreeNodeKind,
    #[serde(rename = "children", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub children: BTreeMap<String, CbTreeNode>,
}

/// ◻ A complete codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Codebase {
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<CodebaseBundle>>,
    #[serde(rename = "folders")]
    pub folders: Option<Vec<CodebaseFolder>>,
    #[serde(rename = "files")]
    pub files: Option<Vec<CodebaseFile>>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<CodebaseSection>>,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<CodebaseDefinition>>,
    #[serde(rename = "contributors")]
    pub contributors: Option<Vec<CodebaseContributor>>,
    #[serde(rename = "tickets")]
    pub tickets: Option<Vec<CodebaseTicket>>,
    #[serde(rename = "policies")]
    pub policies: Option<Vec<CodebasePolicy>>,
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<CodebaseBreach>>,
    #[serde(rename = "tree")]
    pub tree: Option<BTreeMap<String, CbTreeNode>>,
}

//#endregion 🖥️Codebase Aggregate

//#region 🌳️Tree Aggregate

/// 🏷️ Every entity kind the monorepo tree can address.
pub const ENTITY_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "minute", "second", "technology", "bundle", "folder",
    "file", "line", "range", "section", "definition", "goal", "ticket", "draft", "todo", "policy",
    "breach", "contributor", "checkpoint", "interaction", "session",
];

/// 🎁️ The entity kinds that are artifacts, in containment order.
pub const ARTIFACT_KINDS: &[&str] =
    &["repo", "technology", "bundle", "folder", "file", "section", "definition"];

/// 💿️ The entity kinds a checkpoint diff can report on.
pub const DIFFABLE_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "technology", "bundle", "folder", "file", "section",
    "definition", "goal", "ticket", "contributor", "checkpoint", "interaction", "session",
];

/// 📄️ The entity kinds that can be related to a file.
pub const RELATED_TO_FILE_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "minute", "second", "technology", "bundle", "folder",
    "goal", "ticket", "draft", "todo", "policy", "breach", "contributor", "checkpoint",
    "interaction", "session",
];

/// 🌳️ The entity level of a monorepo tree node.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreeNodeKind {
    #[serde(rename = "technology")]
    Technology,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "goal")]
    Goal,
    #[serde(rename = "ticket")]
    Ticket,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "breach")]
    Breach,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "checkpoint")]
    Checkpoint,
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "statute")]
    Statute,
    #[serde(rename = "category")]
    Category,
}

impl TreeNodeKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeNodeKind::Technology => "technology",
            TreeNodeKind::Bundle => "bundle",
            TreeNodeKind::Folder => "folder",
            TreeNodeKind::File => "file",
            TreeNodeKind::Section => "section",
            TreeNodeKind::Definition => "definition",
            TreeNodeKind::Goal => "goal",
            TreeNodeKind::Ticket => "ticket",
            TreeNodeKind::Draft => "draft",
            TreeNodeKind::Todo => "todo",
            TreeNodeKind::Policy => "policy",
            TreeNodeKind::Breach => "breach",
            TreeNodeKind::Contributor => "contributor",
            TreeNodeKind::Checkpoint => "checkpoint",
            TreeNodeKind::Session => "session",
            TreeNodeKind::Statute => "statute",
            TreeNodeKind::Category => "category",
        }
    }
}

impl fmt::Display for TreeNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🌿️ One node of the monorepo tree. The Go twin declares no field tags, so the wire keys are
/// the Go field names themselves.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TreeNode {
    #[serde(rename = "Kind")]
    pub kind: TreeNodeKind,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "SubKind")]
    pub sub_kind: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Summary")]
    pub summary: String,
    #[serde(rename = "Year")]
    pub year: i64,
    #[serde(rename = "Month")]
    pub month: i64,
    #[serde(rename = "Day")]
    pub day: i64,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Contributor")]
    pub contributor: String,
    #[serde(rename = "Data")]
    pub data: Option<BTreeMap<String, Value>>,
    #[serde(rename = "Children")]
    pub children: Option<Vec<TreeNode>>,
}

/// 🧹️ The criteria a monorepo tree query filters by.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeFilter {
    #[serde(rename = "Query")]
    pub query: String,
    #[serde(rename = "OnlyKinds")]
    pub only_kinds: Option<BTreeMap<TreeNodeKind, bool>>,
    #[serde(rename = "ExcludeKinds")]
    pub exclude_kinds: Option<BTreeMap<TreeNodeKind, bool>>,
    #[serde(rename = "OnlySubKinds")]
    pub only_sub_kinds: Option<BTreeMap<TreeNodeKind, Vec<String>>>,
    #[serde(rename = "ExcludeSubKinds")]
    pub exclude_sub_kinds: Option<BTreeMap<TreeNodeKind, Vec<String>>>,
    #[serde(rename = "OnlyYears")]
    pub only_years: Option<Vec<i64>>,
    #[serde(rename = "ExcludeYears")]
    pub exclude_years: Option<Vec<i64>>,
    #[serde(rename = "OnlyMonths")]
    pub only_months: Option<Vec<i64>>,
    #[serde(rename = "ExcludeMonths")]
    pub exclude_months: Option<Vec<i64>>,
    #[serde(rename = "OnlyDays")]
    pub only_days: Option<Vec<i64>>,
    #[serde(rename = "ExcludeDays")]
    pub exclude_days: Option<Vec<i64>>,
    #[serde(rename = "OnlyStatus")]
    pub only_status: String,
    #[serde(rename = "OnlyContributors")]
    pub only_contributors: Option<Vec<String>>,
    #[serde(rename = "ExcludeContributors")]
    pub exclude_contributors: Option<Vec<String>>,
    #[serde(rename = "OnlyPolicies")]
    pub only_policies: Option<Vec<String>>,
    #[serde(rename = "ExcludePolicies")]
    pub exclude_policies: Option<Vec<String>>,
}

impl TreeFilter {
    /// 🏷️ Whether the filter restricts the visible kinds to an explicit allow list.
    pub fn has_only_kinds(&self) -> bool {
        self.only_kinds.as_ref().is_some_and(|kinds| !kinds.is_empty())
    }

    /// 🔷️ Whether a kind survives the filter. A category node is structural and always shown.
    pub fn is_kind_visible(&self, kind: TreeNodeKind) -> bool {
        if kind == TreeNodeKind::Category {
            return true;
        }
        if self.has_only_kinds() {
            return self.only_kinds.as_ref().is_some_and(|kinds| *kinds.get(&kind).unwrap_or(&false));
        }
        !self.exclude_kinds.as_ref().is_some_and(|kinds| *kinds.get(&kind).unwrap_or(&false))
    }

    /// 📥️ Whether a sub kind survives the filter. A blank sub kind always survives.
    pub fn matches_sub_kind(&self, kind: TreeNodeKind, sub_kind: &str) -> bool {
        if sub_kind.is_empty() {
            return true;
        }
        if let Some(only) = self.only_sub_kinds.as_ref().and_then(|map| map.get(&kind)) {
            if !only.is_empty() {
                return only.iter().any(|candidate| eq_fold(candidate, sub_kind));
            }
        }
        if let Some(exclude) = self.exclude_sub_kinds.as_ref().and_then(|map| map.get(&kind)) {
            if exclude.iter().any(|candidate| eq_fold(candidate, sub_kind)) {
                return false;
            }
        }
        true
    }

    /// 🎯️ Whether a year/month/day triple survives the date filters. A zero month or day is
    /// only tested against the exclusion lists.
    pub fn matches_date(&self, year: i64, month: i64, day: i64) -> bool {
        fn survives(only: Option<&Vec<i64>>, exclude: Option<&Vec<i64>>, value: i64, test_only: bool) -> bool {
            if test_only {
                if let Some(only) = only {
                    if !only.is_empty() && !only.contains(&value) {
                        return false;
                    }
                }
            }
            !exclude.is_some_and(|exclude| exclude.contains(&value))
        }
        survives(self.only_years.as_ref(), self.exclude_years.as_ref(), year, true)
            && survives(self.only_months.as_ref(), self.exclude_months.as_ref(), month, month > 0)
            && survives(self.only_days.as_ref(), self.exclude_days.as_ref(), day, day > 0)
    }

    /// 🔶️ Whether a status survives the filter. A blank filter status accepts everything.
    pub fn matches_status(&self, status: &str) -> bool {
        if self.only_status.is_empty() {
            return true;
        }
        eq_fold(&self.only_status, status)
    }

    /// 🤝️ Whether a contributor survives the filter.
    pub fn matches_contributor(&self, contributor: &str) -> bool {
        if let Some(only) = self.only_contributors.as_ref() {
            if !only.is_empty() {
                return only.iter().any(|candidate| eq_fold(candidate, contributor));
            }
        }
        !self
            .exclude_contributors
            .as_ref()
            .is_some_and(|exclude| exclude.iter().any(|candidate| eq_fold(candidate, contributor)))
    }
}

/// 🎫️ A ticket node of the goal/ticket tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Slug")]
    pub slug: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "GoalID")]
    pub goal_id: String,
    #[serde(rename = "ParentID")]
    pub parent_id: String,
    #[serde(rename = "Children")]
    pub children: Option<Vec<TicketNode>>,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Finished")]
    pub finished: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Summary")]
    pub summary: String,
}

/// 💿️ A goal node of the goal/ticket tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GoalNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "DueDate")]
    pub due_date: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Children")]
    pub children: Option<Vec<GoalNode>>,
    #[serde(rename = "Tickets")]
    pub tickets: Option<Vec<TicketNode>>,
}

//#endregion 🌳️Tree Aggregate

//#region 📥️Input Types

/// 💿️ The updated/created/removed file lists a mutation reports.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileListInput {
    #[serde(rename = "updated", default, skip_serializing_if = "Vec::is_empty")]
    pub updated: Vec<String>,
    #[serde(rename = "created", default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<String>,
    #[serde(rename = "removed", default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
}

/// 🎫️ The arguments of a ticket-open mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketOpenInput {
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "noIssue", default, skip_serializing_if = "is_false")]
    pub no_issue: bool,
    #[serde(rename = "draft", default, skip_serializing_if = "String::is_empty")]
    pub draft: String,
    #[serde(rename = "goal")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
    #[serde(rename = "planId", default, skip_serializing_if = "String::is_empty")]
    pub plan_id: String,
    #[serde(rename = "specId", default, skip_serializing_if = "String::is_empty")]
    pub spec_id: String,
}

/// 🆕️ The arguments of a draft-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DraftCreateInput {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
}

/// 📝️ The arguments of a goal-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalCreateInput {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "dueDate")]
    pub due_date: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "milestone", default, skip_serializing_if = "String::is_empty")]
    pub milestone: String,
}

/// ♻️ The arguments of a goal-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalChangeInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "llm", default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(rename = "effort", default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 📪️ The arguments of a goal-close mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalCloseInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🔓️ The arguments of a goal-reopen mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalReopenInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🗑️ The arguments of a goal-delete mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalDeleteInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🔷️ The arguments of a ticket-delete mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDeleteInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🔶️ The arguments of a ticket-close mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketCloseInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "all", default, skip_serializing_if = "is_false")]
    pub all: bool,
}

/// 📬️ The arguments of a ticket-reopen mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketReopenInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "draft", default, skip_serializing_if = "String::is_empty")]
    pub draft: String,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "planId", default, skip_serializing_if = "String::is_empty")]
    pub plan_id: String,
    #[serde(rename = "specId", default, skip_serializing_if = "String::is_empty")]
    pub spec_id: String,
}

/// 🔹️ The arguments of a ticket-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketChangeInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "prompt", default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(rename = "llm", default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(rename = "effort", default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "goal", default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🤝️ The arguments of a contributor-add mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorAddInput {
    #[serde(rename = "github")]
    pub github: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "names", default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "fingerprint", default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "fingerprints", default, skip_serializing_if = "Vec::is_empty")]
    pub fingerprints: Vec<String>,
}

/// 🆕️ The arguments of a todo-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TodoCreateInput {
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
}

/// ♻️ The arguments of a todo-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TodoChangeInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// 🧹️ The criteria a stream or list query filters by.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FilterInput {
    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "regex", default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<bool>,
    #[serde(rename = "matchCase", default, skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
    #[serde(rename = "matchWholeWord", default, skip_serializing_if = "Option::is_none")]
    pub match_whole_word: Option<bool>,
    #[serde(rename = "showIgnored", default, skip_serializing_if = "Option::is_none")]
    pub show_ignored: Option<bool>,
    #[serde(rename = "showGenerated", default, skip_serializing_if = "Option::is_none")]
    pub show_generated: Option<bool>,
    #[serde(rename = "excludeKinds", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_kinds: Vec<String>,
    #[serde(rename = "includeKinds", default, skip_serializing_if = "Vec::is_empty")]
    pub include_kinds: Vec<String>,
}

//#endregion 📥️Input Types

//#region 🏷️File Kinds

/// 📄️ Infers the role a file plays from its name alone.
pub fn derive_file_kind(name: &str) -> FileKind {
    let ext = ext_of(name).to_lowercase();
    let name_lower = name.to_lowercase();
    let name_no_ext = name_lower.strip_suffix(&ext).unwrap_or(&name_lower).to_string();

    if name_lower.contains("license") || name_lower.contains("licence") {
        return FileKind::License;
    }
    for suffix in [".test", ".spec", ".tests", ".requirements", "_test", "_tests", "_spec", "_benchmark", ".benchmark", ".stories", ".story"] {
        if name_no_ext.ends_with(suffix) {
            return FileKind::Lab;
        }
    }
    if name_lower.starts_with("test_") || name_lower.starts_with("test.") || name_lower == "conftest.py" {
        return FileKind::Lab;
    }
    for suffix in [".config", ".conf", ".rc", ".cfg"] {
        if name_no_ext.ends_with(suffix) {
            return FileKind::Config;
        }
    }
    const SCRIPT: &[&str] = &[".sh", ".bash", ".zsh", ".fish", ".bat", ".cmd", ".ps1", ".psm1"];
    const CONFIG: &[&str] = &[
        ".json", ".yaml", ".yml", ".toml", ".xml", ".ini", ".conf", ".config", ".env", ".properties", ".cfg", ".hcl", ".tf", ".tfvars", ".gitignore", ".gitattributes", ".gitmodules",
        ".dockerignore", ".editorconfig", ".eslintignore", ".prettierignore", ".npmignore", ".browserslistrc",
    ];
    const DOCS: &[&str] = &[".md", ".txt", ".rst", ".adoc", ".asciidoc", ".org", ".rdoc"];
    const TEMPLATE: &[&str] = &[
        ".tpl", ".tmpl", ".gotmpl", ".mustache", ".hbs", ".handlebars", ".ejs", ".njk", ".nunjucks", ".jinja", ".jinja2", ".j2", ".liquid", ".pug", ".jade", ".slim", ".haml",
    ];
    const RESOURCE: &[&str] = &[
        ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp", ".bmp", ".tiff", ".tif", ".ttf", ".woff", ".woff2", ".eot", ".otf", ".mp3", ".mp4", ".wav", ".ogg", ".webm", ".avi",
        ".mov", ".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar", ".bin", ".dat", ".db", ".sqlite", ".sqlite3", ".wasm", ".map",
    ];
    const CODE: &[&str] = &[
        ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".mts", ".cts", ".py", ".pyi", ".pyw", ".cs", ".csx", ".go", ".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx", ".rs", ".java",
        ".kt", ".kts", ".scala", ".swift", ".m", ".mm", ".rb", ".erb", ".php", ".lua", ".r", ".jl", ".ex", ".exs", ".hs", ".lhs", ".ml", ".mli", ".clj", ".cljs", ".cljc", ".dart",
        ".v", ".sv", ".vhd", ".vhdl", ".zig", ".nim", ".cr", ".d", ".f", ".f90", ".f95", ".f03", ".pl", ".pm", ".css", ".scss", ".less", ".sass", ".styl", ".html", ".htm", ".vue",
        ".svelte", ".astro", ".sql", ".graphql", ".gql", ".proto", ".thrift", ".avsc", ".g4",
    ];
    if SCRIPT.contains(&ext.as_str()) {
        return FileKind::Script;
    }
    if CONFIG.contains(&ext.as_str()) {
        return FileKind::Config;
    }
    if DOCS.contains(&ext.as_str()) {
        return FileKind::Docs;
    }
    if TEMPLATE.contains(&ext.as_str()) {
        return FileKind::Template;
    }
    if RESOURCE.contains(&ext.as_str()) {
        return FileKind::Resource;
    }
    if CODE.contains(&ext.as_str()) {
        return FileKind::Code;
    }
    const NAMED_CONFIG: &[&str] = &[
        "makefile", "justfile", "rakefile", "gemfile", "vagrantfile", "procfile", "brewfile", "cmakelists.txt", ".babelrc", ".eslintrc", ".prettierrc", ".stylelintrc", ".nycrc",
        ".npmrc", ".nvmrc", ".yarnrc", ".ruby-version", ".python-version", ".node-version", ".tool-versions", ".cursorrules", ".clinerules", "nx.json", "tsconfig.json", "tslint.json",
        "composer.json", "cargo.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock",
    ];
    if name_lower.starts_with("dockerfile") || NAMED_CONFIG.contains(&name_lower.as_str()) {
        return FileKind::Config;
    }
    FileKind::Resource
}

//#endregion 🏷️File Kinds

//#region 🔣️Kind Emojis

/// 📁️ The emoji a folder kind renders as.
pub fn folder_kind_emoji(kind: FolderKind) -> String {
    match kind {
        FolderKind::Organization => emoji_text(entity_emoji("folder-organization")),
        FolderKind::Root => emoji_text(entity_emoji("folder-root")),
        FolderKind::Required => emoji_text(entity_emoji("folder-required")),
    }
}

/// 📄️ The emoji a file kind renders as.
pub fn file_kind_emoji(kind: FileKind) -> String {
    emoji_text(entity_emoji(match kind {
        FileKind::Code => "file-code",
        FileKind::Lab => "file-lab",
        FileKind::Script => "file-script",
        FileKind::Docs => "file-docs",
        FileKind::Config => "file-config",
        FileKind::Resource => "file-resource",
        FileKind::Template => "file-template",
        FileKind::License => "file-license",
    }))
}

/// 🏷️ The emoji a definition kind renders as.
pub fn definition_kind_emoji(kind: DefinitionKind) -> String {
    emoji_text(entity_emoji(match kind {
        DefinitionKind::Interface => "definition-interface",
        DefinitionKind::Constant => "definition-constant",
        DefinitionKind::Test => "definition-test",
        DefinitionKind::Implementation => "definition-impl",
    }))
}

/// 📦️ The emoji a bundle kind renders as, empty for a kind with no vocabulary entry.
pub fn bundle_kind_emoji(bundle: &Bundle) -> String {
    if !bundle.emoji.is_empty() {
        return emoji_text(&bundle.emoji);
    }
    emoji_text(entity_emoji(match bundle.kind {
        BundleKind::Schema => "bundle-schema",
        BundleKind::Binary => "bundle-binary",
        BundleKind::Ui => "bundle-ui",
        BundleKind::Site => "bundle-site",
        BundleKind::Assets => "bundle-assets",
        BundleKind::Library => "bundle-library",
        BundleKind::Repo => "bundle-repo",
    }))
}

/// 🏗️ The emoji a technology renders as, falling back to its kind.
pub fn technology_kind_emoji(technology: &Technology) -> String {
    if !technology.emoji.is_empty() {
        return emoji_text(&technology.emoji);
    }
    emoji_text(match technology.kind {
        TechnologyKind::Infrastructure => entity_emoji("technology-infrastructure"),
        TechnologyKind::Research => entity_emoji("technology-research"),
        TechnologyKind::Mono => entity_emoji("technology-mono"),
        TechnologyKind::User => entity_emoji("technology-user"),
    })
}

//#endregion 🔣️Kind Emojis

//#region 🪪️Section Ids

/// 🔖️ Builds the emoji id of a section inside a file.
pub fn build_section_id(file_id: &str, section_path: &[String]) -> String {
    let segments: Vec<&String> = section_path.iter().filter(|segment| !segment.is_empty()).collect();
    if segments.is_empty() {
        return file_id.to_string();
    }
    let mut result = file_id.to_string();
    for segment in segments {
        let (emoji, name) = extract_entity_emoji(segment);
        let emoji = if emoji.is_empty() { entity_emoji("section").to_string() } else { emoji };
        result = format!("{result}{}{}", emoji_text(&emoji), flat(&name));
    }
    result
}

/// 🧪️ Reports whether a bare declaration name denotes a test by convention.
pub fn is_test_function_name(name: &str) -> bool {
    for prefix in ["Test", "Benchmark", "Fuzz"] {
        if let Some(rest) = name.strip_prefix(prefix) {
            if rest.chars().next().is_some_and(|value| value.is_ascii_uppercase()) {
                return true;
            }
        }
    }
    name.starts_with("test_")
}

/// 🏷️ Builds the emoji id of a definition inside a section.
pub fn build_definition_id(file_id: &str, section_path: &[String], name: &str, kind: DefinitionKind) -> String {
    let mut effective = kind;
    if kind == DefinitionKind::Implementation && file_id.contains(&emoji_text(entity_emoji("file-lab"))) && is_test_function_name(name) {
        effective = DefinitionKind::Test;
    }
    format!("{}{}{}", build_section_id(file_id, section_path), definition_kind_emoji(effective), flat(name))
}

//#endregion 🪪️Section Ids

//#region 🎨️Ansi

/// 🧊️ The ANSI escapes the human renderer paints with, byte-identical to the Go constants.
pub mod ansi {
    /// 🔄️ Restores the default graphic rendition.
    pub const RESET: &str = "\u{1b}[0m";
    /// 🟥️ Red foreground.
    pub const RED: &str = "\u{1b}[31m";
    /// 🟩️ Green foreground.
    pub const GREEN: &str = "\u{1b}[32m";
    /// 🟨️ Yellow foreground.
    pub const YELLOW: &str = "\u{1b}[33m";
    /// 🟦️ Blue foreground.
    pub const BLUE: &str = "\u{1b}[34m";
    /// 🌫️ Dim intensity.
    pub const DIM: &str = "\u{1b}[2m";
    /// 🔆️ Bold intensity.
    pub const BOLD: &str = "\u{1b}[1m";

    /// 🎨️ Resolves a template colour name to its escape, empty for an unknown name.
    pub fn code(name: &str) -> &'static str {
        match name {
            "red" => RED,
            "green" => GREEN,
            "yellow" => YELLOW,
            "blue" => BLUE,
            "dim" => DIM,
            "bold" => BOLD,
            _ => "",
        }
    }

    /// 🖌️ Wraps `text` in `colour` when `enabled`, otherwise returns it verbatim.
    pub fn colorize(text: &str, colour: &str, enabled: bool) -> String {
        if !enabled {
            return text.to_string();
        }
        format!("{}{text}{RESET}", code(colour))
    }

    /// 🌈️ The colour the entity template paints the property at `index` with.
    pub fn prop_color(index: usize) -> &'static str {
        match index {
            0 => "yellow",
            1 => "blue",
            2 => "green",
            _ => "dim",
        }
    }

    /// 📏️ The column budget the human renderer truncates entity lines to.
    pub fn terminal_width() -> usize {
        match std::env::var("COLUMNS").ok().and_then(|value| value.parse::<usize>().ok()) {
            Some(width) if width > 0 => width,
            _ => 120,
        }
    }

    /// ✂️ Truncates to `maximum` visible characters, counting escape sequences as zero width.
    pub fn truncate(text: &str, maximum: usize) -> String {
        if maximum == 0 {
            return text.to_string();
        }
        let bytes = text.as_bytes();
        let mut visible = 0usize;
        let mut index = 0usize;
        while index < bytes.len() {
            if bytes[index] == 0x1b {
                let mut end = index + 1;
                while end < bytes.len() && bytes[end] != b'm' {
                    end += 1;
                }
                index = if end < bytes.len() { end + 1 } else { end };
                continue;
            }
            let width = utf8_width(bytes[index]);
            index += width;
            visible += 1;
            if visible > maximum {
                break;
            }
        }
        if visible <= maximum {
            return text.to_string();
        }
        let limit = maximum.max(3) - 3;
        let mut out = String::new();
        let mut visible = 0usize;
        let mut index = 0usize;
        while index < bytes.len() {
            if bytes[index] == 0x1b {
                let mut end = index + 1;
                while end < bytes.len() && bytes[end] != b'm' {
                    end += 1;
                }
                let end = if end < bytes.len() { end + 1 } else { end };
                out.push_str(&text[index..end]);
                index = end;
                continue;
            }
            if visible >= limit {
                break;
            }
            let width = utf8_width(bytes[index]);
            out.push_str(&text[index..index + width]);
            index += width;
            visible += 1;
        }
        out.push_str("...");
        out
    }

    /// 📐️ The byte length of the UTF-8 sequence a lead byte opens.
    fn utf8_width(lead: u8) -> usize {
        match lead {
            0x00..=0x7f => 1,
            0xc0..=0xdf => 2,
            0xe0..=0xef => 3,
            _ => 4,
        }
    }
}

//#endregion 🎨️Ansi

//#region 🪪️Entity

/// 🪨️ Artifact identities, property projection and the entity templates the renderers print.
pub mod entity {
    use crate::ansi;
    use semio_framework_repo_identity::{collection as collection_emoji, emoji_text, entity as entity_emoji, flat, humanize_seconds};
    use serde_json::{Map, Value};

    /// 🔣️ The presentation-normalised emoji of one entity kind.
    fn e(name: &str) -> String {
        emoji_text(entity_emoji(name))
    }

    /// 🗃️ The presentation-normalised emoji of one collection kind.
    fn c(name: &str) -> String {
        emoji_text(collection_emoji(name))
    }

    /// 📝️ The string member `name`, empty when absent or not a string.
    fn s(data: &Map<String, Value>, name: &str) -> String {
        data.get(name).and_then(Value::as_str).unwrap_or_default().to_string()
    }

    /// 🔢️ The integer member `name`, zero when absent or not a number.
    fn n(data: &Map<String, Value>, name: &str) -> i64 {
        data.get(name).and_then(Value::as_f64).unwrap_or(0.0) as i64
    }

    /// 📁️ The last `/`-separated segment of a path.
    fn base(path: &str) -> String {
        path.rsplit('/').next().unwrap_or(path).to_string()
    }

    /// 🏷️ The trailing extension of a file name, including the dot, empty when there is none.
    fn extension(name: &str) -> String {
        match name.rfind('.') {
            Some(index) if index > 0 => name[index..].to_string(),
            _ => String::new(),
        }
    }

    /// 🛤️ Normalises separators to `/`, matching the workspace path convention.
    pub fn normalize_path(value: &str) -> String {
        value.replace('\\', "/")
    }

    /// 📜️ The emoji a technology artifact id opens with.
    fn technology_kind_emoji(data: &Map<String, Value>) -> String {
        let emoji = s(data, "emoji");
        if !emoji.is_empty() {
            return emoji_text(&emoji);
        }
        match s(data, "kind").as_str() {
            "infrastructure" => return e("technology-infrastructure"),
            "research" => return e("technology-research"),
            "mono" => return e("technology-mono"),
            "user" => return e("technology-user"),
            _ => {}
        }
        let name = s(data, "name");
        if name.contains("repo") {
            return e("technology-infrastructure");
        }
        if name.starts_with("coda") {
            return e("technology-research");
        }
        e("technology-user")
    }

    /// 🔵️ The emoji a bundle artifact id carries after its technology segment.
    fn bundle_kind_emoji(data: &Map<String, Value>) -> String {
        let emoji = s(data, "emoji");
        if !emoji.is_empty() {
            return emoji_text(&emoji);
        }
        match s(data, "kind").as_str() {
            "schema" => e("bundle-schema"),
            "binary" => e("bundle-binary"),
            "ui" => e("bundle-ui"),
            "site" => e("bundle-site"),
            "assets" => e("bundle-assets"),
            "example" => e("bundle-example"),
            _ => e("bundle-library"),
        }
    }

    /// 📄️ The emoji a file artifact id carries, empty for an unknown kind.
    fn file_kind_emoji(data: &Map<String, Value>) -> String {
        match s(data, "kind").as_str() {
            "code" => e("file-code"),
            "lab" => e("file-lab"),
            "script" => e("file-script"),
            "docs" => e("file-docs"),
            "config" => e("file-config"),
            "resource" => e("file-resource"),
            "template" => e("file-template"),
            "license" => e("file-license"),
            _ => String::new(),
        }
    }

    /// 📁️ The emoji a folder artifact id carries.
    fn folder_kind_emoji(data: &Map<String, Value>) -> String {
        match s(data, "kind").as_str() {
            "organization" => e("folder-organization"),
            "root" => e("folder-root"),
            _ => e("folder-required"),
        }
    }

    /// 💕️ The emoji a definition artifact id carries.
    fn definition_kind_emoji(data: &Map<String, Value>) -> String {
        match s(data, "kind").to_ascii_lowercase().as_str() {
            "interface" => e("definition-interface"),
            "constant" => e("definition-constant"),
            "test" => e("definition-test"),
            _ => e("definition-impl"),
        }
    }

    /// 💬️ The emoji an interaction artifact id ends with.
    fn interaction_kind_emoji(data: &Map<String, Value>) -> String {
        match s(data, "kind").as_str() {
            "edited" => e("interaction-edited"),
            "finished" => e("interaction-finished"),
            "restarted" => e("interaction-restarted"),
            "deleted" => e("interaction-deleted"),
            _ => e("interaction-started"),
        }
    }

    /// 🎯️ The emoji artifact id of a `/`-separated goal path.
    pub fn goal_artifact_id(raw: &str) -> String {
        raw.split('/').map(|segment| format!("{}{}", e("goal"), flat(segment))).collect()
    }

    /// 📄️ The emoji artifact id of a file path, resolved without a bundle registry.
    ///
    /// The reference implementation consults the process-global bundle table here; this crate
    /// holds no such global, so the id is built from the path alone. Callers that own a
    /// `🗂️codebase` handle use its own `build_file_id`, which resolves the owning bundle first.
    pub fn build_file_id(path: &str) -> String {
        let normalized = normalize_path(path);
        let name = base(&normalized);
        let kind = crate::derive_file_kind(&name);
        let ext = extension(&name);
        let stem = name.strip_suffix(&ext).unwrap_or(&name).to_string();
        let mut data = Map::new();
        data.insert("kind".to_string(), Value::String(kind.as_str().to_string()));
        format!("{}{}", file_kind_emoji(&data), flat(&stem))
    }

    /// 📑️ The emoji artifact id of a section path below a file id.
    pub fn build_section_id(file_id: &str, section_path: &[String]) -> String {
        crate::build_section_id(file_id, section_path)
    }

    /// 🪪️ The emoji artifact id of one entity, mirroring the reference `GetArtifactID`.
    pub fn artifact_id(kind: &str, data: &Map<String, Value>) -> String {
        let parent = s(data, "parentId");
        match kind {
            "root" | "statutes" => String::new(),
            "years" => parent + &c("years"),
            "year" => parent + &e("year") + &s(data, "yy"),
            "months" => parent + &c("months"),
            "month" => parent + &e("month") + &s(data, "mm"),
            "days" => parent + &c("days"),
            "day" => parent + &e("day") + &s(data, "dd"),
            "hours" => parent + &c("hours"),
            "hour" => parent + &e("hour") + &s(data, "hh"),
            "minutes" => parent + &c("minutes"),
            "minute" => parent + &e("minute") + &s(data, "mm"),
            "seconds" => parent + &c("seconds"),
            "second" => parent + &e("second") + &s(data, "ss"),
            "codebase" => parent + &c("codebase"),
            "technologies" => parent + &c("technologies"),
            "technology" => technology_kind_emoji(data) + &flat(&s(data, "name")),
            "bundles" => parent + &c("bundles"),
            "bundle" => {
                let name = s(data, "name");
                let (technology_code, bundle_code) = match name.split_once('/') {
                    Some((left, right)) => (left.to_string(), right.to_string()),
                    None => (name.clone(), name.clone()),
                };
                let mut technology_data = Map::new();
                technology_data.insert("name".to_string(), Value::String(technology_code.clone()));
                technology_kind_emoji(&technology_data) + &flat(&technology_code) + &bundle_kind_emoji(data) + &flat(&bundle_code)
            }
            "folders" => parent + &c("folders"),
            "folder" => parent + &folder_kind_emoji(data) + &flat(&base(&s(data, "path"))),
            "files" => parent + &c("files"),
            "file" => {
                let mut path = s(data, "path");
                if path.is_empty() {
                    path = s(data, "id");
                }
                let name = base(&path);
                let ext = extension(&name);
                let stem = name.strip_suffix(&ext).unwrap_or(&name);
                parent + &file_kind_emoji(data) + &flat(stem)
            }
            "sections" => parent + &c("sections"),
            "section" => section_artifact_id(data, &parent),
            "definitions" => parent + &c("definitions"),
            "definition" => definition_artifact_id(data, &parent),
            "tickets" => parent + &c("tickets"),
            "ticket" => {
                let mut parent = parent;
                if parent.is_empty() {
                    let goal = s(data, "goalId");
                    if !goal.is_empty() {
                        parent = goal_artifact_id(&goal);
                    }
                }
                let mut slug = s(data, "slug");
                if slug.is_empty() {
                    slug = s(data, "title");
                }
                if slug.is_empty() {
                    let id = s(data, "id");
                    if !id.is_empty() {
                        return id;
                    }
                }
                parent + &e("ticket") + &flat(&slug)
            }
            "goals" => parent + &c("goals"),
            "goal" => {
                let id = s(data, "id");
                let name = id.rsplit('/').next().unwrap_or(&id).to_string();
                parent + &e("goal") + &flat(&name)
            }
            "drafts" => parent + &c("drafts"),
            "draft" => {
                let mut slug = s(data, "slug");
                if slug.is_empty() {
                    slug = s(data, "id");
                }
                parent + &e("draft") + &flat(&slug)
            }
            "todos" => parent + &c("todos"),
            "todo" => parent + &e("todo") + &flat(&s(data, "id")),
            "policies" => parent + &c("policies"),
            "policy" => parent + &e("policy") + &flat(s(data, "id").trim_start_matches('/')),
            "statute" => flat(&s(data, "id")),
            "contributors" => parent + &c("contributors"),
            "contributor" => {
                let mut alias = s(data, "alias");
                if alias.is_empty() {
                    alias = s(data, "github");
                }
                e("contributor") + &flat(&alias)
            }
            "checkpoints" => parent + &c("checkpoints"),
            "checkpoint" => {
                let mut contributor = s(data, "contributorId");
                if contributor.is_empty() {
                    let author = s(data, "authorId");
                    if !author.is_empty() {
                        contributor = e("contributor") + &flat(&author);
                    }
                }
                contributor + &e("checkpoint") + &s(data, "sha")
            }
            "line" => format!("{parent}{}{}", e("line"), n(data, "line")),
            "range" => format!("{parent}{}{}{}{}", e("line"), n(data, "startLine"), e("line"), n(data, "endLine")),
            "breach" => parent + &e("breach") + &s(data, "affected") + &e("breach-scope") + &s(data, "lineId") + &s(data, "secondId"),
            "breaches" => parent + &c("breaches"),
            "interactions" => parent + &c("interactions"),
            "interaction" => s(data, "secondId") + &s(data, "contributorId") + &s(data, "entityId") + &interaction_kind_emoji(data),
            "sessions" => parent + &c("sessions"),
            "session" => {
                let mut uuid = s(data, "uuid");
                if uuid.is_empty() {
                    uuid = s(data, "id");
                }
                parent + &e("session") + &flat(&uuid)
            }
            _ => String::new(),
        }
    }

    /// 📑️ The section branch of [`artifact_id`], kept apart because it is the deepest one.
    fn section_artifact_id(data: &Map<String, Value>, parent: &str) -> String {
        let mut path = s(data, "path");
        if path.is_empty() {
            let file_path = s(data, "filePath");
            let name = s(data, "name");
            if !file_path.is_empty() && !name.is_empty() {
                path = format!("{file_path}#{name}");
            } else if !name.is_empty() {
                path = name;
            }
        }
        if !parent.is_empty() {
            let section_name = match path.rfind('#') {
                Some(index) => path[index + 1..].to_string(),
                None => path.clone(),
            };
            return format!("{parent}{}{}", e("section"), flat(&section_name));
        }
        if let Some(index) = path.find('#') {
            let file_path = normalize_path(&path[..index]);
            let segments: Vec<String> = path[index + 1..].split('#').map(str::to_string).collect();
            return build_section_id(&build_file_id(&file_path), &segments);
        }
        let name = s(data, "name");
        if !name.is_empty() {
            return e("section") + &flat(&name);
        }
        e("section") + &flat(&path)
    }

    /// 💕️ The definition branch of [`artifact_id`].
    fn definition_artifact_id(data: &Map<String, Value>, parent: &str) -> String {
        let marker = definition_kind_emoji(data);
        let mut name = s(data, "name");
        if name.is_empty() {
            let id = s(data, "id");
            if !id.is_empty() {
                name = match id.rfind('§') {
                    Some(index) => id[index + '§'.len_utf8()..].to_string(),
                    None => match id.rfind('#') {
                        Some(index) => id[index + 1..].to_string(),
                        None => id.clone(),
                    },
                };
            }
        }
        if !parent.is_empty() {
            return format!("{parent}{marker}{}", flat(&name));
        }
        let mut section_path = s(data, "sectionPath");
        let mut file_path = s(data, "filePath");
        if file_path.is_empty() {
            let id = s(data, "id");
            if !id.is_empty() {
                if let Some(paragraph) = id.rfind('§') {
                    name = id[paragraph + '§'.len_utf8()..].to_string();
                    match id[..paragraph].find('#') {
                        Some(hash) => {
                            file_path = id[..hash].to_string();
                            section_path = id[hash + 1..paragraph].replace('#', "/");
                        }
                        None => file_path = id[..paragraph].to_string(),
                    }
                } else if let Some(hash) = id.rfind('#') {
                    name = id[hash + 1..].to_string();
                    file_path = id[..hash].to_string();
                }
            }
        }
        if !file_path.is_empty() {
            let file_id = build_file_id(&file_path);
            let section_id = if section_path.is_empty() {
                file_id
            } else {
                let segments: Vec<String> = section_path.replace('#', "/").split('/').map(str::to_string).collect();
                build_section_id(&file_id, &segments)
            };
            return format!("{section_id}{marker}{}", flat(&name));
        }
        format!("{marker}{}", flat(&name))
    }

    /// 🔗️ The `repo://` uri of one entity.
    pub fn artifact_uri(kind: &str, data: &Map<String, Value>) -> String {
        let id = artifact_id(kind, data);
        if id.is_empty() {
            return format!("repo://{kind}");
        }
        format!("repo://{kind}/{id}")
    }

    /// 🧼️ Collapses newlines and runs of spaces and swaps backticks for apostrophes.
    pub fn sanitize_prop(value: &str) -> String {
        let mut value = value.replace("\r\n", " ").replace(['\n', '\r'], " ").replace('`', "'");
        while value.contains("  ") {
            value = value.replace("  ", " ");
        }
        value.trim().to_string()
    }

    /// 🧽️ Collapses every newline form to a space, leaving the rest untouched.
    pub fn sanitize_single_line(value: &str) -> String {
        value.replace("\r\n", " ").replace(['\n', '\r'], " ")
    }

    /// ⏳️ A relative timestamp for one of the flexible time layouts, or the raw text.
    fn relative_time(raw: &str) -> String {
        match parse_flexible_epoch(raw) {
            Some(epoch) => humanize_seconds(now_epoch() - epoch),
            None => raw.to_string(),
        }
    }

    /// 🕰️ The host wall clock in whole seconds since the Unix epoch.
    fn now_epoch() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|value| value.as_secs() as i64).unwrap_or_default()
    }

    /// ⏰️ Decodes RFC 3339, `YYYY-MM-DD` and `YYYY-MM-DD HH:MM:SS` to a Unix timestamp.
    pub fn parse_flexible_epoch(raw: &str) -> Option<i64> {
        let bytes = raw.as_bytes();
        if bytes.len() < 10 {
            return None;
        }
        let number = |from: usize, to: usize| raw.get(from..to).and_then(|slice| slice.parse::<i64>().ok());
        let year = number(0, 4)?;
        let month = number(5, 7)?;
        let day = number(8, 10)?;
        if bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let (hour, minute, second) = if bytes.len() >= 19 && (bytes[10] == b'T' || bytes[10] == b' ') {
            (number(11, 13)?, number(14, 16)?, number(17, 19)?)
        } else if bytes.len() == 10 {
            (0, 0, 0)
        } else {
            return None;
        };
        Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
    }

    /// 📅️ Howard Hinnant's civil-date to days-since-epoch conversion.
    fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
        let year = year - i64::from(month <= 2);
        let era = if year >= 0 { year } else { year - 399 } / 400;
        let year_of_era = year - era * 400;
        let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// 🗓️ The creation timestamp of an entity, from either the flat or the nested member.
    fn created_of(data: &Map<String, Value>) -> String {
        let direct = s(data, "created");
        if !direct.is_empty() {
            return direct;
        }
        for wrapper in ["dates", "date"] {
            if let Some(Value::Object(nested)) = data.get(wrapper) {
                let value = s(nested, "created");
                if !value.is_empty() {
                    return value;
                }
            }
        }
        String::new()
    }

    /// 🏁️ The completion timestamp of an entity, from either the flat or the nested member.
    fn finished_of(data: &Map<String, Value>) -> String {
        let direct = s(data, "finished");
        if !direct.is_empty() {
            return direct;
        }
        for wrapper in ["dates", "date"] {
            if let Some(Value::Object(nested)) = data.get(wrapper) {
                let value = s(nested, "finished");
                if !value.is_empty() {
                    return value;
                }
            }
        }
        String::new()
    }

    /// ✂️ Truncates to 80 characters with an ellipsis when the caller asked for it.
    fn clip(value: &str, truncate: bool) -> String {
        if truncate && value.len() > 80 {
            let mut end = 80;
            while end > 0 && !value.is_char_boundary(end) {
                end -= 1;
            }
            return format!("{}...", &value[..end]);
        }
        value.to_string()
    }

    /// 🧾️ The rendered property list of one entity kind.
    pub fn collect_props(kind: &str, data: &Map<String, Value>, truncate: bool) -> Vec<String> {
        let mut props = Vec::new();
        let push = |value: &str, props: &mut Vec<String>| {
            if value.is_empty() {
                return;
            }
            let cleaned = sanitize_prop(value);
            if !cleaned.is_empty() {
                props.push(cleaned);
            }
        };
        match kind {
            "goal" => {
                let created = created_of(data);
                let created_display = if created.is_empty() { String::new() } else { format!("created {}", relative_time(&created)) };
                let mut due = s(data, "dueDate");
                if due.is_empty() {
                    if let Some(Value::Object(dates)) = data.get("dates") {
                        due = s(dates, "due");
                    }
                }
                let due_display = if due.is_empty() { String::new() } else { relative_time(&due) };
                push(&s(data, "title"), &mut props);
                push(&s(data, "status"), &mut props);
                push(&created_display, &mut props);
                push(&due_display, &mut props);
                push(&clip(&s(data, "description"), truncate), &mut props);
            }
            "ticket" => {
                let status = s(data, "status");
                let open = status == "open" || status == "OPEN";
                let created = created_of(data);
                let finished = finished_of(data);
                let date_display = if open {
                    if created.is_empty() { String::new() } else { format!("opened {}", relative_time(&created)) }
                } else if !finished.is_empty() {
                    format!("closed {}", relative_time(&finished))
                } else if !created.is_empty() {
                    relative_time(&created)
                } else {
                    String::new()
                };
                let content = if open { s(data, "prompt") } else { s(data, "summary") };
                push(&s(data, "title"), &mut props);
                push(&status, &mut props);
                push(&date_display, &mut props);
                push(&clip(&content, truncate), &mut props);
            }
            "bundle" => {
                let mut project_type = s(data, "projectType");
                if project_type.is_empty() {
                    project_type = s(data, "type");
                }
                push(&s(data, "root"), &mut props);
                push(&project_type, &mut props);
            }
            "section" => {
                let start = n(data, "startLine");
                let end = n(data, "endLine");
                if start > 0 || end > 0 {
                    push(&format!(":{start}-{end}"), &mut props);
                }
            }
            "definition" => {
                push(&s(data, "name"), &mut props);
                let start = n(data, "startLine");
                let end = n(data, "endLine");
                if start > 0 || end > 0 {
                    push(&format!(":{start}-{end}"), &mut props);
                }
            }
            "contributor" | "todo" | "root" => push(&s(data, "name"), &mut props),
            "policy" | "statute" | "technology" => push(&s(data, "description"), &mut props),
            "checkpoint" => push(&s(data, "message"), &mut props),
            _ => {}
        }
        props
    }

    /// 🖥️ One entity as the human renderer prints it (`text/entity`).
    pub fn render_human(kind: &str, data: &Map<String, Value>, is_tty: bool) -> String {
        let id = artifact_id(kind, data);
        let props = collect_props(kind, data, false);
        let mut out = String::new();
        if !id.is_empty() {
            out.push_str(&ansi::colorize(&sanitize_prop(&id), "bold", is_tty));
        }
        for (index, prop) in props.iter().enumerate() {
            if !id.is_empty() || index > 0 {
                out.push(' ');
            }
            out.push_str(&ansi::colorize(prop, ansi::prop_color(index), is_tty));
        }
        sanitize_single_line(&out)
    }

    /// 🔗️ One entity as a markdown link (`md/entity_link`).
    pub fn render_markdown_link(kind: &str, data: &Map<String, Value>) -> String {
        let id = artifact_id(kind, data);
        let uri = artifact_uri(kind, data);
        let props = collect_props(kind, data, false);
        let mut out = format!("[{}]({uri})", sanitize_prop(&id));
        for prop in &props {
            out.push_str(&format!(" - `{prop}`"));
        }
        sanitize_single_line(&out)
    }

    /// 📋️ One entity as a markdown list item (`md/entity_item`).
    pub fn render_markdown(kind: &str, data: &Map<String, Value>) -> String {
        format!("- {}", render_markdown_link(kind, data))
    }

    /// 🧭️ The entity kind a result member name projects to, empty when it names none.
    pub fn infer_kind(key: &str) -> &'static str {
        let key = key.to_ascii_lowercase();
        const PREFIXES: &[(&str, &str)] = &[
            ("ticket", "ticket"),
            ("goal", "goal"),
            ("file", "file"),
            ("folder", "folder"),
            ("section", "section"),
            ("definition", "definition"),
            ("contributor", "contributor"),
            ("todo", "todo"),
            ("draft", "draft"),
            ("policy", "policy"),
            ("breachkind", "statute"),
            ("bundle", "bundle"),
            ("technology", "technology"),
            ("checkpoint", "checkpoint"),
            ("repo", "root"),
            ("syncgithub", "root"),
            ("syncmanagement", "root"),
            ("integrate", "file"),
            ("extract", "file"),
            ("fix", "root"),
        ];
        for (prefix, kind) in PREFIXES {
            if key.starts_with(prefix) {
                return kind;
            }
        }
        ""
    }

    /// 🌐️ Uri path form of a path or identifier: every segment carries its blanks percent-encoded,
    /// so the link a markdown renderer emits is reversible by the matching decoder.
    pub fn path_to_uri_path(value: &str) -> String {
        value.split('/').map(|segment| segment.replace(' ', "%20")).collect::<Vec<String>>().join("/")
    }

    /// 🌐️ The inverse of [`path_to_uri_path`].
    pub fn path_from_uri_path(value: &str) -> String {
        value.split('/').map(|segment| segment.replace("%20", " ")).collect::<Vec<String>>().join("/")
    }
}

//#endregion 🪪️Entity

//#region 🔁️Wire Round Trip

macro_rules! wire_types {
    ($($name:literal => $ty:ty),+ $(,)?) => {
        /// 📦️ Every domain type this crate can decode and re-encode by name, in declaration order.
        pub fn wire_type_names() -> &'static [&'static str] {
            &[$($name),+]
        }

        /// 🔁️ Decodes one JSON document into the named domain type and encodes it again.
        ///
        /// The wire contract is what other modules depend on, so it is exercised through this
        /// owned entry point rather than by re-exporting a serialisation crate: nothing outside
        /// this codebase appears in the signature.
        pub fn round_trip_json(type_name: &str, text: &str) -> Result<String, String> {
            match type_name {
                $($name => {
                    let value: $ty = serde_json::from_str(text).map_err(|error| error.to_string())?;
                    serde_json::to_string(&value).map_err(|error| error.to_string())
                })+
                other => Err(format!("unknown domain type {other}")),
            }
        }
    };
}

wire_types! {
    "Statute" => Statute,
    "Range" => Range,
    "LineMetrics" => LineMetrics,
    "CountMetrics" => CountMetrics,
    "PriorityCount" => PriorityCount,
    "AnalyzeMetrics" => AnalyzeMetrics,
    "Package" => Package,
    "Bundle" => Bundle,
    "Technology" => Technology,
    "Repo" => Repo,
    "Folder" => Folder,
    "File" => File,
    "Definition" => Definition,
    "Section" => Section,
    "ContributorIcons" => ContributorIcons,
    "ContributorLink" => ContributorLink,
    "ContributorTicket" => ContributorTicket,
    "ContributorCheckpoint" => ContributorCheckpoint,
    "ContributorContributionsStorage" => ContributorContributionsStorage,
    "Contributor" => Contributor,
    "Checkpoint" => Checkpoint,
    "CheckpointDiffRename" => CheckpointDiffRename,
    "CheckpointDiffStats" => CheckpointDiffStats,
    "InteractionFile" => InteractionFile,
    "TicketPlan" => TicketPlan,
    "TicketAgentPlanStep" => TicketAgentPlanStep,
    "TicketAgent" => TicketAgent,
    "TicketManagementData" => TicketManagementData,
    "Ticket" => Ticket,
    "TicketData" => TicketData,
    "TicketFile" => TicketFile,
    "TicketFileRenamed" => TicketFileRenamed,
    "TicketDiffSet" => TicketDiffSet,
    "TicketSectionMetrics" => TicketSectionMetrics,
    "TicketFileMetricsEntry" => TicketFileMetricsEntry,
    "GoalDates" => GoalDates,
    "GoalManagementData" => GoalManagementData,
    "Draft" => Draft,
    "Location" => Location,
    "Todo" => Todo,
    "Territory" => Territory,
    "StatuteMeta" => StatuteMeta,
    "Breach" => Breach,
    "RangePosition" => RangePosition,
    "FileRange" => FileRange,
    "BreachFile" => BreachFile,
    "BreachFolder" => BreachFolder,
    "CodebaseBreach" => CodebaseBreach,
    "BundleMetricsInternal" => BundleMetricsInternal,
    "CodebaseFile" => CodebaseFile,
    "CbTreeNode" => CbTreeNode,
    "Codebase" => Codebase,
    "TreeNode" => TreeNode,
    "TicketNode" => TicketNode,
    "GoalNode" => GoalNode,
    "SemanticChange" => SemanticChange,
    "DiffLines" => DiffLines,
    "FileListInput" => FileListInput,
    "TicketOpenInput" => TicketOpenInput,
    "TicketCloseInput" => TicketCloseInput,
    "FilterInput" => FilterInput,
}

//#endregion 🔁️Wire Round Trip

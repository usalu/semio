//! 🧱️ Tree model: vocabulary constants, the four ports, the loaded records and the node helpers every builder shares.
//!
//! Holds the `🏷️Vocabulary`, `🔌️Ports`, `💿️Records`, `🌿️NodeHelpers` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🏷️Vocabulary

/// 🌿️ The connector drawn in front of a non-last child in the text tree.
pub const TEXT_BRANCH_CONNECTOR: &str = "├️─️─️ ";

/// 🌿️ The connector drawn in front of the last child in the text tree.
pub const TEXT_LAST_CONNECTOR: &str = "└️─️─️ ";

/// 🌿️ The indent carried down a subtree that still has following siblings.
pub const TEXT_PIPE_INDENT: &str = "│️   ";

/// 🌿️ The indent carried down the last subtree of a level.
pub const TEXT_BLANK_INDENT: &str = "    ";

/// 🟢️ The icon of a low-priority statute.
pub const PRIORITY_ICON_LOW: &str = "🟢️";

/// 🟡️ The icon of a medium-priority statute.
pub const PRIORITY_ICON_MEDIUM: &str = "🟡️";

/// 🔴️ The icon of a high-priority statute.
pub const PRIORITY_ICON_HIGH: &str = "🔴️";

/// 🎫️ The label of the synthetic goal that collects every ticket without a goal.
pub const NO_GOAL_LABEL: &str = "No Goal";

/// 📌️ The schema version of the tree cache envelope; a mismatch invalidates the cache.
pub const TREE_CACHE_SCHEMA_VERSION: i64 = 3;

//#endregion 🏷️Vocabulary

//#region 🔌️Ports

/// 🪪️ Mints the artifact id of one entity from its tree-node data, as `🪪️identity` defines it.
pub trait ArtifactIdentifier {
    /// 🆔️ Returns the artifact id of an entity of `entity_kind` described by `data`.
    fn artifact_id(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
}

/// 🎨️ Renders one entity as a single line, as the CLI renderers define it.
pub trait EntityRenderer {
    /// 🖥️ Renders the human, single-line form.
    fn human(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
    /// 📰️ Renders the markdown list-item form.
    fn markdown(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
    /// 🔗️ Renders the markdown link form used inside a goal tree line.
    fn markdown_link(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
}

/// 📜️ Resolves statute metadata, ids, uris and labels, as `📜️statutes` defines them.
pub trait StatuteCatalog {
    /// 🔖️ Returns the declared metadata of a statute.
    fn info(&self, statute: &Statute) -> Option<StatuteMeta>;
    /// 🆔️ Returns the artifact id of a statute.
    fn statute_id(&self, statute: &Statute) -> String;
    /// 🔗️ Returns the `repo://` uri of a statute.
    fn statute_uri(&self, statute: &Statute) -> String;
    /// 🏷️ Returns the display label of a statute path, without the priority icon.
    fn statute_label(&self, statute: &Statute) -> String;
    /// 🧱️ Returns the entity kind a statute applies to.
    fn entity_kind(&self, statute: &Statute) -> String;
    /// 🆔️ Returns the artifact id of a territory.
    fn territory_id(&self, territory: &Territory) -> String;
}

/// 🗂️ Supplies the already-loaded records the monorepo tree projects.
pub trait TreeSource {
    /// 🧪️ The technologies with their bundles, in load order.
    fn technologies(&self) -> Vec<TechnologyRecord>;
    /// 📁️ Every folder of the repository.
    fn folders(&self) -> Vec<FolderRecord>;
    /// 📄️ Every file of the repository.
    fn files(&self) -> Vec<FileRecord>;
    /// 🎯️ Every goal.
    fn goals(&self) -> Vec<GoalRecord>;
    /// 🎫️ Every ticket.
    fn tickets(&self) -> Vec<TicketRecord>;
    /// ✍️ Every draft.
    fn drafts(&self) -> Vec<DraftRecord>;
    /// 🛡️ Every policy.
    fn policies(&self) -> Vec<PolicyRecord>;
    /// 🧑️ Every contributor.
    fn contributors(&self) -> Vec<ContributorRecord>;
    /// 🔀️ Every checkpoint.
    fn checkpoints(&self) -> Vec<CheckpointRecord>;
    /// ⚪️ Every session.
    fn sessions(&self) -> Vec<SessionRecord>;
    /// 📑️ The parsed sections of one file, read only when sections are requested.
    fn sections(&self, file_path: &str) -> Vec<SectionRecord>;
}

/// 🪪️ Mints artifact ids through `📐️model`'s own id builder, the repository's one grammar.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultArtifactIdentifier;

impl ArtifactIdentifier for DefaultArtifactIdentifier {
    fn artifact_id(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::artifact_id(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }
}

/// 🎨️ Renders entities through `📐️model`'s own entity templates, without terminal colour.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultEntityRenderer;

impl EntityRenderer for DefaultEntityRenderer {
    fn human(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_human(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect(), false)
    }

    fn markdown(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_markdown(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }

    fn markdown_link(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_markdown_link(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }
}

//#endregion 🔌️Ports

//#region 💿️Records

/// 🧪️ A technology with its bundles.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TechnologyRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub bundles: Vec<BundleRecord>,
}

/// 📦️ One bundle of a technology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub root: String,
    #[serde(default, rename = "sourceRoot")]
    pub source_root: String,
}

/// 📁️ One folder of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FolderRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    /// 🪜️ The identity of the folder this one sits in, empty at the repository root.
    #[serde(default)]
    pub parent_id: String,
}

/// 📄️ One file of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    /// 🗃️ The identity of the folder this file sits in, empty at the repository root.
    #[serde(default)]
    pub parent_id: String,
}

/// 📑️ One section of a file, with its definitions and nested sections.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectionRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    #[serde(default, rename = "startLine")]
    pub start_line: i64,
    #[serde(default, rename = "endLine")]
    pub end_line: i64,
    #[serde(default)]
    pub definitions: Vec<DefinitionRecord>,
    #[serde(default)]
    pub children: Vec<SectionRecord>,
}

/// 🏷️ One definition inside a section.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DefinitionRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default, rename = "filePath")]
    pub file_path: String,
    #[serde(default, rename = "startLine")]
    pub start_line: i64,
    #[serde(default, rename = "endLine")]
    pub end_line: i64,
}

/// 🎯️ One goal.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalRecord {
    pub id: String,
    pub title: String,
    pub uri: String,
    #[serde(default, rename = "artifactId")]
    pub artifact_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "dueDate")]
    pub due_date: String,
    #[serde(default, rename = "createdAt")]
    pub created_at: String,
    #[serde(default)]
    pub description: String,
}

/// 🎫️ One ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketRecord {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub started: String,
    #[serde(default)]
    pub finished: String,
}

/// ✍️ One draft.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DraftRecord {
    pub id: String,
    pub uri: String,
    #[serde(default, rename = "artifactId")]
    pub artifact_id: String,
}

/// 🛡️ One policy with its territories.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub groups: Vec<Territory>,
}

/// 🧑️ One contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorRecord {
    pub id: String,
    pub alias: String,
    pub uri: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub githubs: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub emails: Vec<String>,
}

/// 🔀️ One checkpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointRecord {
    pub id: String,
    pub sha: String,
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default, rename = "authorId")]
    pub author_id: String,
}

/// ⚪️ One agent session.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SessionRecord {
    pub id: String,
    pub uuid: String,
    pub uri: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default, rename = "kindEmoji")]
    pub kind_emoji: String,
    #[serde(default)]
    pub client: String,
    #[serde(default, rename = "startedAt")]
    pub started_at: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub checkpoint: String,
}

/// 🗄️ A [`TreeSource`] backed entirely by already-decoded records, the shape the fixtures carry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MemoryTreeSource {
    #[serde(default)]
    pub technologies: Vec<TechnologyRecord>,
    #[serde(default)]
    pub folders: Vec<FolderRecord>,
    #[serde(default)]
    pub files: Vec<FileRecord>,
    #[serde(default)]
    pub goals: Vec<GoalRecord>,
    #[serde(default)]
    pub tickets: Vec<TicketRecord>,
    #[serde(default)]
    pub drafts: Vec<DraftRecord>,
    #[serde(default)]
    pub policies: Vec<PolicyRecord>,
    #[serde(default)]
    pub contributors: Vec<ContributorRecord>,
    #[serde(default)]
    pub checkpoints: Vec<CheckpointRecord>,
    #[serde(default)]
    pub sessions: Vec<SessionRecord>,
    #[serde(default)]
    pub sections: BTreeMap<String, Vec<SectionRecord>>,
}

impl MemoryTreeSource {
    /// 📥️ Decodes a record set from its JSON document.
    pub fn from_json(document: &str) -> Result<MemoryTreeSource, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }
}

impl TreeSource for MemoryTreeSource {
    fn technologies(&self) -> Vec<TechnologyRecord> {
        self.technologies.clone()
    }
    fn folders(&self) -> Vec<FolderRecord> {
        self.folders.clone()
    }
    fn files(&self) -> Vec<FileRecord> {
        self.files.clone()
    }
    fn goals(&self) -> Vec<GoalRecord> {
        self.goals.clone()
    }
    fn tickets(&self) -> Vec<TicketRecord> {
        self.tickets.clone()
    }
    fn drafts(&self) -> Vec<DraftRecord> {
        self.drafts.clone()
    }
    fn policies(&self) -> Vec<PolicyRecord> {
        self.policies.clone()
    }
    fn contributors(&self) -> Vec<ContributorRecord> {
        self.contributors.clone()
    }
    fn checkpoints(&self) -> Vec<CheckpointRecord> {
        self.checkpoints.clone()
    }
    fn sessions(&self) -> Vec<SessionRecord> {
        self.sessions.clone()
    }
    fn sections(&self, file_path: &str) -> Vec<SectionRecord> {
        self.sections.get(file_path).cloned().unwrap_or_default()
    }
}

/// 🗄️ A [`StatuteCatalog`] backed by declared entries, the shape the fixtures carry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MemoryStatuteCatalog {
    #[serde(default)]
    pub statutes: Vec<StatuteCatalogEntry>,
    #[serde(default)]
    pub territories: BTreeMap<String, String>,
}

/// 📜️ One declared statute of a [`MemoryStatuteCatalog`].
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatuteCatalogEntry {
    pub id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub uri: String,
    pub label: String,
    #[serde(rename = "entityKind")]
    pub entity_kind: String,
    pub meta: StatuteMetaRecord,
}

/// 🔖️ The declared metadata of one catalog entry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatuteMetaRecord {
    #[serde(default, rename = "policyId")]
    pub policy_id: String,
    pub priority: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub solution: String,
    #[serde(default)]
    pub autofixable: bool,
}

impl MemoryStatuteCatalog {
    /// 📥️ Decodes a catalog from its JSON document.
    pub fn from_json(document: &str) -> Result<MemoryStatuteCatalog, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🔎️ Returns the declared entry of a statute.
    pub(super) fn entry(&self, statute: &Statute) -> Option<&StatuteCatalogEntry> {
        self.statutes.iter().find(|entry| entry.id == statute.0)
    }
}

impl StatuteCatalog for MemoryStatuteCatalog {
    fn info(&self, statute: &Statute) -> Option<StatuteMeta> {
        self.entry(statute).map(|entry| StatuteMeta {
            kind: statute.clone(),
            policy_id: entry.meta.policy_id.clone(),
            priority: parse_breach_priority(&entry.meta.priority),
            reason: entry.meta.reason.clone(),
            solution: entry.meta.solution.clone(),
            autofixable: entry.meta.autofixable,
        })
    }
    fn statute_id(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.artifact_id.clone()).unwrap_or_default()
    }
    fn statute_uri(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.uri.clone()).unwrap_or_default()
    }
    fn statute_label(&self, statute: &Statute) -> String {
        self.entry(statute).map_or_else(|| statute.0.clone(), |entry| entry.label.clone())
    }
    fn entity_kind(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.entity_kind.clone()).unwrap_or_default()
    }
    fn territory_id(&self, territory: &Territory) -> String {
        self.territories.get(&territory.name).cloned().unwrap_or_else(|| territory.name.clone())
    }
}

/// 🌿️ A compact tree-node literal: every field defaults, so a fixture states only what matters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeNodeSpec {
    pub kind: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub uri: String,
    #[serde(default, rename = "subKind")]
    pub sub_kind: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub contributor: String,
    #[serde(default)]
    pub data: Option<BTreeMap<String, Value>>,
    #[serde(default)]
    pub children: Vec<TreeNodeSpec>,
}

impl TreeNodeSpec {
    /// 📥️ Decodes a node literal from its JSON document.
    pub fn from_json(document: &str) -> Result<TreeNodeSpec, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🌿️ Expands the literal into a real tree node.
    pub fn to_tree_node(&self) -> TreeNode {
        let mut node = tree_node(parse_tree_node_kind(&self.kind), &self.id, &self.label, &self.uri);
        node.sub_kind = self.sub_kind.clone();
        node.description = self.description.clone();
        node.summary = self.summary.clone();
        node.year = self.year;
        node.month = self.month;
        node.day = self.day;
        node.status = self.status.clone();
        node.contributor = self.contributor.clone();
        node.data = self.data.clone();
        if !self.children.is_empty() {
            node.children = Some(self.children.iter().map(TreeNodeSpec::to_tree_node).collect());
        }
        node
    }
}

/// 🧹️ A compact tree-filter literal: every field defaults, so a fixture states only what matters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeFilterSpec {
    #[serde(default)]
    pub query: String,
    #[serde(default, rename = "onlyKinds")]
    pub only_kinds: Vec<String>,
    #[serde(default, rename = "excludeKinds")]
    pub exclude_kinds: Vec<String>,
    #[serde(default, rename = "onlySubKinds")]
    pub only_sub_kinds: BTreeMap<String, Vec<String>>,
    #[serde(default, rename = "excludeSubKinds")]
    pub exclude_sub_kinds: BTreeMap<String, Vec<String>>,
    #[serde(default, rename = "onlyYears")]
    pub only_years: Vec<i64>,
    #[serde(default, rename = "excludeYears")]
    pub exclude_years: Vec<i64>,
    #[serde(default, rename = "onlyMonths")]
    pub only_months: Vec<i64>,
    #[serde(default, rename = "excludeMonths")]
    pub exclude_months: Vec<i64>,
    #[serde(default, rename = "onlyDays")]
    pub only_days: Vec<i64>,
    #[serde(default, rename = "excludeDays")]
    pub exclude_days: Vec<i64>,
    #[serde(default, rename = "onlyStatus")]
    pub only_status: String,
    #[serde(default, rename = "onlyContributors")]
    pub only_contributors: Vec<String>,
    #[serde(default, rename = "excludeContributors")]
    pub exclude_contributors: Vec<String>,
    #[serde(default, rename = "onlyPolicies")]
    pub only_policies: Vec<String>,
    #[serde(default, rename = "excludePolicies")]
    pub exclude_policies: Vec<String>,
}

impl TreeFilterSpec {
    /// 📥️ Decodes a filter literal from its JSON document.
    pub fn from_json(document: &str) -> Result<TreeFilterSpec, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🧹️ Expands the literal into a real tree filter.
    pub fn to_filter(&self) -> TreeFilter {
        fn kind_set(kinds: &[String]) -> BTreeMap<TreeNodeKind, bool> {
            kinds.iter().map(|kind| (parse_tree_node_kind(kind), true)).collect()
        }
        fn sub_kind_map(entries: &BTreeMap<String, Vec<String>>) -> BTreeMap<TreeNodeKind, Vec<String>> {
            entries.iter().map(|(kind, values)| (parse_tree_node_kind(kind), values.clone())).collect()
        }
        TreeFilter {
            query: self.query.clone(),
            only_kinds: Some(kind_set(&self.only_kinds)),
            exclude_kinds: Some(kind_set(&self.exclude_kinds)),
            only_sub_kinds: Some(sub_kind_map(&self.only_sub_kinds)),
            exclude_sub_kinds: Some(sub_kind_map(&self.exclude_sub_kinds)),
            only_years: Some(self.only_years.clone()),
            exclude_years: Some(self.exclude_years.clone()),
            only_months: Some(self.only_months.clone()),
            exclude_months: Some(self.exclude_months.clone()),
            only_days: Some(self.only_days.clone()),
            exclude_days: Some(self.exclude_days.clone()),
            only_status: self.only_status.clone(),
            only_contributors: Some(self.only_contributors.clone()),
            exclude_contributors: Some(self.exclude_contributors.clone()),
            only_policies: Some(self.only_policies.clone()),
            exclude_policies: Some(self.exclude_policies.clone()),
        }
    }
}

/// 📥️ Decodes a territory forest from its JSON document.
pub fn decode_territories(document: &str) -> Result<Vec<Territory>, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 📥️ Decodes a statute list from its JSON document.
pub fn decode_statutes(document: &str) -> Result<Vec<Statute>, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 🔤️ Decodes a tree node kind slug, defaulting to the structural category kind.
pub fn parse_tree_node_kind(value: &str) -> TreeNodeKind {
    match value {
        "technology" => TreeNodeKind::Technology,
        "bundle" => TreeNodeKind::Bundle,
        "folder" => TreeNodeKind::Folder,
        "file" => TreeNodeKind::File,
        "section" => TreeNodeKind::Section,
        "definition" => TreeNodeKind::Definition,
        "goal" => TreeNodeKind::Goal,
        "ticket" => TreeNodeKind::Ticket,
        "draft" => TreeNodeKind::Draft,
        "todo" => TreeNodeKind::Todo,
        "policy" => TreeNodeKind::Policy,
        "breach" => TreeNodeKind::Breach,
        "contributor" => TreeNodeKind::Contributor,
        "checkpoint" => TreeNodeKind::Checkpoint,
        "session" => TreeNodeKind::Session,
        "statute" => TreeNodeKind::Statute,
        _ => TreeNodeKind::Category,
    }
}

/// 🔤️ Decodes a priority slug, defaulting to the low priority the icon table also defaults to.
pub fn parse_breach_priority(value: &str) -> BreachPriority {
    match value {
        "high" => BreachPriority::High,
        "medium" => BreachPriority::Medium,
        _ => BreachPriority::Low,
    }
}

//#endregion 💿️Records

//#region 🌿️NodeHelpers

/// 🌿️ Builds an empty node of one kind; every projection fills the fields it owns.
pub fn tree_node(kind: TreeNodeKind, id: &str, label: &str, uri: &str) -> TreeNode {
    TreeNode {
        kind,
        id: id.to_string(),
        label: label.to_string(),
        uri: uri.to_string(),
        sub_kind: String::new(),
        description: String::new(),
        summary: String::new(),
        year: 0,
        month: 0,
        day: 0,
        status: String::new(),
        contributor: String::new(),
        data: None,
        children: None,
    }
}

/// 🧒️ Returns the children of a node as a slice, treating the absent list as empty.
pub fn children_of(node: &TreeNode) -> &[TreeNode] {
    node.children.as_deref().unwrap_or(&[])
}

/// ➕️ Appends one child, creating the child list on first use.
pub fn push_child(node: &mut TreeNode, child: TreeNode) {
    node.children.get_or_insert_with(Vec::new).push(child);
}

/// 🗃️ Builds a data map from string pairs.
pub(super) fn data_of(entries: Vec<(&str, Value)>) -> BTreeMap<String, Value> {
    entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect()
}

/// 🔤️ Wraps a string as a JSON value.
pub(super) fn text(value: &str) -> Value {
    Value::String(value.to_string())
}

/// 🔢️ Wraps a number as a JSON value, matching the Go encoder that widens every count to float64.
pub(super) fn number(value: i64) -> Value {
    Value::from(value as f64)
}

/// 📃️ Wraps a string list as a JSON value.
pub(super) fn list(values: &[String]) -> Value {
    Value::Array(values.iter().map(|value| text(value)).collect())
}

/// 📁️ Returns the parent path of a slash-separated path, matching Go's `filepath.Dir` on the
/// normalised paths the record set carries: no separator yields `.`.
pub fn parent_path(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => path[..index].to_string(),
        None => ".".to_string(),
    }
}

/// 📄️ Returns the last segment of a slash-separated path.
pub fn base_name(path: &str) -> String {
    match path.rfind('/') {
        Some(index) => path[index + 1..].to_string(),
        None => path.to_string(),
    }
}

/// 🪜️ Nests items under the item carrying their parent id, preserving input order at every level.
/// An item whose parent is absent from the set, or that is its own parent, becomes a root.
pub(super) fn nest_ordered<T>(items: Vec<(String, String, T)>, attach: &dyn Fn(&mut T, T)) -> Vec<T> {
    let positions: BTreeMap<String, usize> = items.iter().enumerate().map(|(index, (id, _, _))| (id.clone(), index)).collect();
    let mut roots: Vec<usize> = Vec::new();
    let mut children: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, (id, parent, _)) in items.iter().enumerate() {
        match positions.get(parent) {
            Some(&parent_index) if parent != id && parent_index != index => children.entry(parent_index).or_default().push(index),
            _ => roots.push(index),
        }
    }
    let mut slots: Vec<Option<T>> = items.into_iter().map(|(_, _, value)| Some(value)).collect();
    roots.into_iter().filter_map(|index| assemble_nested(index, &mut slots, &children, attach)).collect()
}

/// 🪜️ Assembles one item of [`nest_ordered`] together with everything below it.
fn assemble_nested<T>(index: usize, slots: &mut Vec<Option<T>>, children: &BTreeMap<usize, Vec<usize>>, attach: &dyn Fn(&mut T, T)) -> Option<T> {
    let mut node = slots[index].take()?;
    if let Some(kids) = children.get(&index).cloned() {
        for kid in kids {
            if let Some(child) = assemble_nested(kid, slots, children, attach) {
                attach(&mut node, child);
            }
        }
    }
    Some(node)
}

/// 🎨️ Returns the icon of a priority.
pub fn priority_icon(priority: BreachPriority) -> &'static str {
    match priority {
        BreachPriority::High => PRIORITY_ICON_HIGH,
        BreachPriority::Medium => PRIORITY_ICON_MEDIUM,
        _ => PRIORITY_ICON_LOW,
    }
}

//#endregion 🌿️NodeHelpers

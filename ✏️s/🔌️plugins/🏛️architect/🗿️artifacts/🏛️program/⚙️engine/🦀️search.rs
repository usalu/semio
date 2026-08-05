//! ⚙️ Architect program artifact engine — the `search` topic.

//! 🔍️ Program search — keyword and structured filters across registers.

use crate::artifacts::program::kernel::{EntityHeader, EntityId, LifecycleStatus, Priority};
use crate::artifacts::program::Program;
use crate::artifacts::program::registers::SearchFilter;
use serde::{Deserialize, Serialize};

// #region 🔖️SearchQuery
/// @emoji 🎯️ Ad-hoc search query with optional structured filters.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub owner_ids: Vec<EntityId>,
    #[serde(default)]
    pub statuses: Vec<LifecycleStatus>,
    #[serde(default)]
    pub priorities: Vec<Priority>,
    #[serde(default)]
    pub entity_kinds: Vec<String>,
    #[serde(default)]
    pub tag_filters: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
}

/// @emoji 📌️ One search hit with register kind and display name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub register: String,
    pub entity_id: EntityId,
    pub name: String,
    pub score: f64,
}
// #endregion

// #region 🔖️SearchProgram
/// @emoji 🔎️ Searches all registers; uses `filter` when provided; records query in `search_history`.
pub fn search_plugin(program: &Program, query: &SearchQuery, filter: Option<&SearchFilter>, search_history: Option<&mut Vec<SearchQuery>>) -> Vec<SearchHit> {
    let effective = merge_query(query, filter);
    if let Some(history) = search_history {
        history.push(effective.clone());
    }
    let mut hits = Vec::new();
    macro_rules! search_register {
        ($register:literal, $collection:expr) => {
            for item in $collection {
                push_if_match(&mut hits, $register, &item.header, &effective);
            }
        };
    }
    search_register!("stakeholders", &program.stakeholders);
    search_register!("users", &program.users);
    search_register!("activities", &program.activities);
    search_register!("functions", &program.functions);
    search_register!("elements", &program.elements);
    search_register!("quantities", &program.quantities);
    search_register!("relationships", &program.relationships);
    search_register!("adjacencies", &program.adjacencies);
    search_register!("processes", &program.processes);
    search_register!("flows", &program.flows);
    search_register!("access_rules", &program.access_rules);
    search_register!("operations", &program.operations);
    search_register!("equipment", &program.equipment);
    search_register!("resources", &program.resources);
    search_register!("storage", &program.storage);
    search_register!("environmental", &program.environmental);
    search_register!("human_factors", &program.human_factors);
    search_register!("accessibility", &program.accessibility);
    search_register!("privacy", &program.privacy);
    search_register!("safety", &program.safety);
    search_register!("security", &program.security);
    search_register!("regulatory", &program.regulatory);
    search_register!("site_context", &program.site_context);
    search_register!("organizational", &program.organizational);
    search_register!("services", &program.services);
    search_register!("infrastructure", &program.infrastructure);
    search_register!("information", &program.information);
    search_register!("communication", &program.communication);
    search_register!("wayfinding", &program.wayfinding);
    search_register!("schedules", &program.schedules);
    search_register!("flexibility", &program.flexibility);
    search_register!("growth", &program.growth);
    search_register!("sustainability", &program.sustainability);
    search_register!("resilience", &program.resilience);
    search_register!("costs", &program.costs);
    search_register!("delivery", &program.delivery);
    search_register!("risks", &program.risks);
    search_register!("conflicts", &program.conflicts);
    search_register!("requirements", &program.requirements);
    search_register!("priorities", &program.priorities);
    search_register!("scenarios", &program.scenarios);
    search_register!("options", &program.options);
    search_register!("decisions", &program.decisions);
    search_register!("validations", &program.validations);
    search_register!("performance", &program.performance);
    search_register!("quality", &program.quality);
    search_register!("documents", &program.documents);
    search_register!("changes", &program.changes);
    search_register!("collaboration", &program.collaboration);
    search_register!("analyses", &program.analyses);
    search_register!("reports", &program.reports);
    search_register!("search_filters", &program.search_filters);
    search_register!("status_records", &program.status_records);
    search_register!("workshops", &program.workshops);
    search_register!("surveys", &program.surveys);
    search_register!("issues", &program.issues);
    search_register!("audit_events", &program.audit_events);
    search_register!("templates", &program.templates);
    search_register!("knowledge", &program.knowledge);
    search_register!("benchmarks", &program.benchmarks);
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits
}

fn merge_query(query: &SearchQuery, filter: Option<&SearchFilter>) -> SearchQuery {
    let Some(filter) = filter else {
        return query.clone();
    };
    SearchQuery {
        keywords: if filter.keywords.is_empty() { query.keywords.clone() } else { filter.keywords.clone() },
        categories: if filter.categories.is_empty() { query.categories.clone() } else { filter.categories.clone() },
        owner_ids: if filter.owner_ids.is_empty() { query.owner_ids.clone() } else { filter.owner_ids.clone() },
        statuses: if filter.statuses.is_empty() { query.statuses.clone() } else { filter.statuses.clone() },
        priorities: if filter.priorities.is_empty() { query.priorities.clone() } else { filter.priorities.clone() },
        entity_kinds: if filter.entity_kinds.is_empty() { query.entity_kinds.clone() } else { filter.entity_kinds.clone() },
        tag_filters: if filter.tag_filters.is_empty() { query.tag_filters.clone() } else { filter.tag_filters.clone() },
        sources: if filter.sources.is_empty() { query.sources.clone() } else { filter.sources.clone() },
        date_from: filter.date_from.clone().or(query.date_from.clone()),
        date_to: filter.date_to.clone().or(query.date_to.clone()),
    }
}

fn push_if_match(hits: &mut Vec<SearchHit>, register: &str, header: &EntityHeader, query: &SearchQuery) {
    if !query.statuses.is_empty() && !query.statuses.contains(&header.status) {
        return;
    }
    if !query.priorities.is_empty() && !query.priorities.contains(&header.priority) {
        return;
    }
    if let Some(owner) = &header.ownership.owner_id {
        if !query.owner_ids.is_empty() && !query.owner_ids.contains(owner) {
            return;
        }
    }
    if !query.entity_kinds.is_empty() && !query.entity_kinds.iter().any(|k| k == register) {
        return;
    }
    if !query.tag_filters.is_empty() && !query.tag_filters.iter().any(|t| header.tags.contains(t)) {
        return;
    }
    if !query.categories.is_empty() && !query.categories.iter().any(|c| header.tags.contains(c) || header.name.contains(c)) {
        return;
    }
    if let Some(from) = &query.date_from {
        if header.timestamps.updated < *from {
            return;
        }
    }
    if let Some(to) = &query.date_to {
        if header.timestamps.updated > *to {
            return;
        }
    }
    if !query.sources.is_empty() {
        let source_match = header.notes.iter().any(|n| query.sources.iter().any(|s| n.tag.contains(s) || n.text.contains(s))) || header.tags.iter().any(|t| query.sources.contains(t));
        if !source_match {
            return;
        }
    }
    let mut score = 0.0;
    let haystack = format!("{} {} {:?}", header.name, header.description.as_ref().map_or("", |d| d.text.as_str()), header.tags).to_lowercase();
    for keyword in &query.keywords {
        if haystack.contains(&keyword.to_lowercase()) {
            score += 1.0;
        }
    }
    if query.keywords.is_empty() || score > 0.0 {
        hits.push(SearchHit { register: register.into(), entity_id: header.id.clone(), name: header.name.clone(), score: if score == 0.0 { 0.1 } else { score } });
    }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn search_finds_reception_element() {
        let hits = search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Reception".into()], ..Default::default() }, None, None);
        assert!(hits.iter().any(|h| h.name == "Reception"));
    }

    #[test]
    fn search_history_records_query() {
        let mut history = Vec::new();
        search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Waiting".into()], ..Default::default() }, None, Some(&mut history));
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].keywords, vec!["Waiting".to_string()]);
    }

    #[test]
    fn entity_kind_filter_limits_registers() {
        let hits = search_plugin(&sample_plugin(), &SearchQuery { entity_kinds: vec!["elements".into()], ..Default::default() }, None, None);
        assert!(hits.iter().all(|h| h.register == "elements"));
    }
}
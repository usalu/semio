//! 🔍️ Architect play app command — keyword search across every register, selecting the top hits and
//! appending the query to the config's search history.

pub mod query {
    use crate::apps::architect::config::{parse_search_history, snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::engine::search::{search_plugin, SearchQuery};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::Program;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "search")]
    pub struct Search {
        pub query: String,
    }

    pub fn handle(payload: &Search, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let base_config = cfg.projection;
        let mut history = parse_search_history(base_config);
        let hits = search_plugin(doc.projection, &SearchQuery { keywords: payload.query.split_whitespace().map(str::to_string).collect(), ..SearchQuery::default() }, None, Some(&mut history));
        let mut next = base_config.clone();
        next.search_query = payload.query.clone();
        next.selected_ids = hits.iter().take(8).map(|hit| hit.entity_id.to_string()).collect();
        next.search_history_json = serde_json::to_string(&history).unwrap_or_else(|_| "[]".into());
        next.last_result_json = serde_json::to_string_pretty(&hits).unwrap_or_else(|_| "[]".into());
        Ok(Emit::config(snapshot(next)))
    }
}

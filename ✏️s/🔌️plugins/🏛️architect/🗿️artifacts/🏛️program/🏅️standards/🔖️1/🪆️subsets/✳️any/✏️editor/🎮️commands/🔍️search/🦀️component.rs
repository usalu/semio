//! 🔍️ Architect play app command — keyword search across every register, selecting the top hits and
//! appending the query to the config's search history.

pub mod query {
    use crate::editor::architect::config::{parse_search_history, snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::{search_plugin, SearchQuery};
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "search")]
    pub struct Search {
        pub query: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the top hits used to also
    /// become the selection here — selection is framework-owned `InteractionState` now, only ever
    /// mutated by the framework's own injected `interactionSelect` handling, never by an app
    /// command's `Emit` (mirrors note's `add-block`); hits still land in `last_result_json`.
    pub fn handle(payload: &Search, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let base_config = cfg.snapshot;
        let mut history = parse_search_history(base_config);
        let hits = search_plugin(doc.snapshot, &SearchQuery { keywords: payload.query.split_whitespace().map(str::to_string).collect(), ..SearchQuery::default() }, None, Some(&mut history));
        let mut next = base_config.clone();
        next.search_query = payload.query.clone();
        next.search_history_json = serde_json::to_string(&history).unwrap_or_else(|_| "[]".into());
        next.last_result_json = serde_json::to_string_pretty(&hits).unwrap_or_else(|_| "[]".into());
        Ok(Emit::config(snapshot(next)))
    }
}

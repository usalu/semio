//! ⚙️ Trinity Jack app — headless compute (constitutional: engine).
//!
//! 📌 Deviation from the constitutional-split recipe: the query-language compute itself
//! (`run_jack_query` and friends) lives in `trinity_jack` (the shared Jack query-language crate,
//! used by both `jack`'s UI and `trinity_rewrite`'s `apply_rule`) — see the ticket report for why
//! it stays there rather than moving here. This crate holds the one document-level pure helper.

use trinity_ram::GraphFixture;

/// 📦 An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> GraphFixture {
    trinity_ram::empty_trinity_graph_fixture()
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_jack_document_has_no_nodes_or_edges() {
        let fixture = empty_jack_document();
        assert!(fixture.nodes.is_empty());
        assert!(fixture.edges.is_empty());
    }
}
//#endregion 🧪Tests

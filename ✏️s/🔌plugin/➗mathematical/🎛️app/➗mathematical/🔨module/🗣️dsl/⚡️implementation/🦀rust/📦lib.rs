//! 📜 Mathematical app — textual document grammar surface + laws (constitutional: dsl).
//!
//! 🧭 No external `.mathematical` fixture file has ever shipped for this app, so these laws stay
//! proven purely against inline-constructed fixtures (mirrors the original flattened `🔖DslTests`).

use mathematical::MathProjection;

/// 📖 Parses `.mathematical` DSL text into a `MathProjection`.
pub fn parse_dsl(text: &str) -> Result<MathProjection, store::TextError> {
    <MathProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MathProjection` back to `.mathematical` DSL text.
pub fn print_dsl(projection: &MathProjection) -> String {
    store::DocumentDsl::print_dsl(projection)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::{MathGeometry, MathGraph};

    #[test]
    fn math_projection_dsl_round_trips_default() {
        store::test_support::assert_dsl_round_trip(&MathProjection::default());
    }

    #[test]
    fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        store::test_support::assert_dsl_round_trip(&projection);
    }
}
//#endregion 🧪Tests

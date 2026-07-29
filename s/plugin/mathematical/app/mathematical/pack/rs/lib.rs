//! 📦 Mathematical app — binary document surface + laws (constitutional: pack).

use mathematical::MathProjection;
use store::PackError;

/// 📦 Encodes a `MathProjection` to its binary pack form.
pub fn encode(projection: &MathProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖 Decodes a `MathProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<MathProjection, PackError> {
    <MathProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::{MathGeometry, MathGraph};

    #[test]
    fn math_projection_dsl_pack_equivalence_default() {
        store::test_support::assert_dsl_pack_equivalence(&MathProjection::default());
    }

    #[test]
    fn math_projection_dsl_pack_equivalence_with_seed_and_empty_collections() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }
}
//#endregion 🧪Tests

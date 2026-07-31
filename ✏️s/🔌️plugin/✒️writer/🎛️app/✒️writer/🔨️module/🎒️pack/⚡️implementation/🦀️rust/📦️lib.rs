//! 📦️ Writer app — binary document surface + laws (constitutional: pack).

use store::PackError;
use writer::WriterProjection;

/// 📦️ Encodes a `WriterProjection` to its binary pack form.
pub fn encode(projection: &WriterProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `WriterProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WriterProjection, PackError> {
    <WriterProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// ✍️ Hand-built representative document — verbatim from the original file's `🔖️DslAndOpText`
    /// test region (duplicated per-crate since each constitutional crate's tests compile independently).
    fn jack_projection() -> WriterProjection {
        WriterProjection {
            schema: "writer.document".into(),
            id: "jack".into(),
            language_id: "jack".into(),
            uri: "writer://jack".into(),
            text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
        }
    }

    #[test]
    fn writer_projection_dsl_pack_equivalence() {
        let empty = writer_engine::empty_writer_projection();
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let bytes = encode(&empty);
        assert_eq!(decode(&bytes).expect("decode"), empty);

        let jack = jack_projection();
        store::test_support::assert_dsl_pack_equivalence(&jack);
        let bytes = encode(&jack);
        assert_eq!(decode(&bytes).expect("decode"), jack);
    }
}
//#endregion 🧪️Tests

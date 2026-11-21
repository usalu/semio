//! 🧪️ Reopens fixed compressed source owners from the actual PDF COS canonical model.

use crate::native::png_source::{
    PNG_GRAPH_CHUNK_BYTES, PngGraphExtent, PngGraphSource, PngPlane, PngSourceError, is_png_graph_source, append_hex,
};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfIndirectObject, PdfObject};
use semio_framework_value::{FromValue, paged::ByteReadSource};

const SOURCE: &[u8] = include_bytes!("🗑️generated/large-picked/large-picked.png");
const IDENTITY: &[u8; 32] = include_bytes!("🗑️generated/raw-png-graph/identity");

fn object(index: usize, value: PdfObject) -> PdfIndirectObject {
    PdfIndirectObject { id: ObjRef { num: u32::try_from(index + 1).unwrap(), gen: 0 }, value }
}

#[test]
fn exact_large_png_source_reopens_from_canonical_cos_and_rejects_partial_custody() {
    assert_eq!(SOURCE.len(), 473_683);
    let extent = PngGraphExtent { digest: *IDENTITY, length: SOURCE.len() };
    let color = PngPlane::Rgb.reference(extent.digest);
    let alpha = PngPlane::Alpha.reference(extent.digest);
    assert_ne!(color, alpha);
    let mut graph: Vec<_> = SOURCE.chunks(PNG_GRAPH_CHUNK_BYTES).enumerate().map(|(index, bytes)| {
        let mut hex = String::with_capacity(bytes.len() * 2);
        append_hex(&mut hex, bytes);
        let value = extent.part(index, hex).unwrap();
        let encoded = semio_framework_pack_json::to_json_string(&value);
        assert!(encoded.len() < 128 * 1024);
        let parsed = semio_framework_pack_json::parse_bytes(encoded.as_bytes(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let restored = PdfObject::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
        assert_eq!(restored, value);
        object(index, restored)
    }).collect();
    assert_eq!(graph.len(), 15);
    assert!(graph.iter().all(|chunk| is_png_graph_source(&chunk.value)));
    graph.reverse();
    for (reference, role) in [(&color, PngPlane::Rgb), (&alpha, PngPlane::Alpha)] {
        let (actual_role, reopened) = PngGraphSource::resolve(&graph, reference).unwrap();
        assert_eq!(actual_role, role);
        assert_eq!(reopened.byte_len(), SOURCE.len());
        let mut bytes = [0; 4096];
        for (index, source) in SOURCE.chunks(bytes.len()).enumerate() {
            assert_eq!(reopened.read_bytes(index * bytes.len(), &mut bytes), source.len());
            assert_eq!(&bytes[..source.len()], source);
        }
    }
    #[cfg(graph_consumer)]
    {
        use crate::native::{NativePdfArtifactResources, PdfArtifactResourcePort};
        let owner = NativePdfArtifactResources::from_objects(&graph);
        for (reference, expected) in [
            (&color, include_bytes!("🗑️generated/raw-png-graph/rgb-sha")),
            (&alpha, include_bytes!("🗑️generated/raw-png-graph/alpha-sha")),
        ] {
            let stream = owner.resolve(reference).unwrap();
            assert_eq!(stream.dict_get("Width").and_then(PdfObject::as_i64), Some(521));
            assert_eq!(stream.dict_get("Height").and_then(PdfObject::as_i64), Some(257));
            let PdfObject::Stream { data, .. } = stream else { panic!("decoded role is a native stream"); };
            let mut hash = semio_framework_hash::Sha256::new();
            hash.update(&data);
            assert_eq!(&hash.finalize(), expected);
        }
    }
    let removed = graph.pop().unwrap();
    assert_eq!(PngGraphSource::resolve(&graph, &color).err(), Some(PngSourceError::Missing));
    graph.push(removed.clone());
    graph.push(removed);
    assert_eq!(PngGraphSource::resolve(&graph, &alpha).err(), Some(PngSourceError::Duplicate));
    graph.pop();
    let PdfObject::Dict(fields) = &mut graph[0].value else { unreachable!() };
    let PdfObject::Text(data) = &mut fields.last_mut().unwrap().value else { unreachable!() };
    data.replace_range(0..1, if &data[..1] == "0" { "1" } else { "0" });
    assert_eq!(PngGraphSource::resolve(&graph, &color).err(), Some(PngSourceError::Digest));
    let PdfObject::Dict(fields) = &mut graph[0].value else { unreachable!() };
    let PdfObject::Text(data) = &mut fields.last_mut().unwrap().value else { unreachable!() };
    data.replace_range(0..1, "G");
    assert_eq!(PngGraphSource::resolve(&graph, &color).err(), Some(PngSourceError::Part));
}

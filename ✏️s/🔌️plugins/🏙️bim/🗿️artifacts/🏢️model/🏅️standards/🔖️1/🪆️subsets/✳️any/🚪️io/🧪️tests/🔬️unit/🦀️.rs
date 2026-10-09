use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, testkit::house, IFC_DIALECT};
use semio_framework_os_kernel::io::io_mechanism::IoEntryDirection;
use semio_framework::io_schema::IoPayload;

fn entry(direction: IoEntryDirection) -> &'static semio_framework_os_kernel::io::io_mechanism::IoEntry {
    io().entries.iter().find(|entry| entry.direction == direction && (entry.from == IFC_DIALECT || entry.into == IFC_DIALECT)).expect("the IFC entry")
}

#[test]
fn the_declaration_lists_the_ifc_export_and_import_between_the_bim_and_the_ifc_dialect() {
    let (export, import) = (entry(IoEntryDirection::Export), entry(IoEntryDirection::Import));
    assert_eq!((export.from, export.into), (crate::BIM_MODEL_DIALECT, IFC_DIALECT));
    assert_eq!((import.from, import.into), (IFC_DIALECT, crate::BIM_MODEL_DIALECT));
    assert!(import.sniff.is_some());
}

#[test]
fn the_export_entry_turns_a_packed_model_into_the_ifc_bytes() {
    let model = house();
    let produced = (entry(IoEntryDirection::Export).run)(&IoPayload::Binary(pack::encode(&model))).expect("the entry runs");
    assert_eq!(produced.value, IoPayload::Binary(export_ifc2x3(&model).expect("direct export").0));
}

#[test]
fn the_import_entry_turns_ifc_bytes_into_a_packed_model() {
    let model = house();
    let ifc = IoPayload::Binary(export_ifc2x3(&model).expect("direct export").0);
    let produced = (entry(IoEntryDirection::Import).run)(&ifc).expect("the entry runs");
    let IoPayload::Binary(bytes) = produced.value else { panic!("a packed model") };
    let imported = pack::decode(&bytes).expect("the pack decodes");
    assert!(imported.walls.contains_key("w-south") && imported.storeys.len() == 4);
}

#[test]
fn the_io_declaration_lists_the_native_pack_and_text_codec_and_every_foreign_hop() {
    use crate::standards::v1::subsets::any::io::export::{gltf::ModelIntoGlb, ifc::ModelIntoIfc2x3, svg::ModelIntoSvg};
    use crate::standards::v1::subsets::any::io::import::ifc::IfcIntoModel;
    use crate::standards::v1::subsets::any::io::text::snapshot as text;
    use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let declaration = io();
    assert_eq!(declaration.native.codec.schema, crate::BIM_MODEL_DOCUMENT_SCHEMA);
    let model = house();
    assert_eq!(pack::decode(&pack::encode(&model)).expect("the pack carrier decodes"), model);
    assert_eq!(text::parse_dsl(&text::print_dsl(&model)).expect("the text carrier parses"), model);
    let hops: Vec<_> = declaration.entries.iter().map(|entry| (entry.direction, entry.from.artifact_kind, entry.into.artifact_kind, entry.into.standard.0)).collect();
    assert_eq!(
        hops,
        [
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.ifc", "2x3"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.gltf", "2.0"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.svg", "1.1"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.csv", "rfc4180"),
            (IoEntryDirection::Import, "s.stdio.ifc", "s.bim.model", "1"),
        ]
    );
    assert_eq!([<ModelIntoIfc2x3 as Serializer<crate::ModelSnapshot>>::INTO, <ModelIntoGlb as Serializer<crate::ModelSnapshot>>::INTO, <ModelIntoSvg as Serializer<crate::ModelSnapshot>>::INTO].map(|dialect| dialect.artifact_kind), ["s.stdio.ifc", "s.stdio.gltf", "s.stdio.svg"]);
    assert_eq!(<IfcIntoModel as Deserializer<crate::ModelSnapshot>>::FROM, IFC_DIALECT);
}

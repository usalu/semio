use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, testkit::house, IFC_DIALECT};
use semio_framework_os_kernel::io::io_mechanism::IoEntryDirection;
use semio_framework::io_schema::IoPayload;

#[path = "../🪶️sqlite/🦀️.rs"]
mod sqlite;

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
    use crate::standards::v1::subsets::any::io::export::{gltf::ModelIntoGlb, ifc::ifc4::ModelIntoIfc4, ifc::ModelIntoIfc2x3, svg::ModelIntoSvg};
    use crate::standards::v1::subsets::any::io::import::ifc::{Ifc2x3IntoModel, Ifc4IntoModel};
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
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.ifc", "4"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.gltf", "2.0"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.svg", "1.1"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.csv", "rfc4180"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.json", "rfc8259"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.pdf", "1.7"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.xml", "1.0"),
            (IoEntryDirection::Export, "s.bim.model", "s.energy.model", "1"),
            (IoEntryDirection::Export, "s.bim.model", "s.stdio.bcf", "2.1"),
            (IoEntryDirection::Import, "s.stdio.ifc", "s.bim.model", "1"),
            (IoEntryDirection::Import, "s.stdio.ifc", "s.bim.model", "1"),
            (IoEntryDirection::Import, "s.stdio.bcf", "s.bim.model", "1"),
        ]
    );
    assert_eq!(<ModelIntoIfc4 as Serializer<crate::ModelSnapshot>>::INTO.standard.0, "4");
    assert_eq!([<ModelIntoIfc2x3 as Serializer<crate::ModelSnapshot>>::INTO, <ModelIntoGlb as Serializer<crate::ModelSnapshot>>::INTO, <ModelIntoSvg as Serializer<crate::ModelSnapshot>>::INTO].map(|dialect| dialect.artifact_kind), ["s.stdio.ifc", "s.stdio.gltf", "s.stdio.svg"]);
    assert_eq!(<Ifc2x3IntoModel as Deserializer<crate::ModelSnapshot>>::FROM, IFC_DIALECT);
    assert_eq!(<Ifc4IntoModel as Deserializer<crate::ModelSnapshot>>::FROM, crate::standards::v1::subsets::any::io::export::ifc::ifc4::IFC4_DIALECT);
}

#[test]
fn the_ifc4_entry_turns_a_packed_model_into_the_ifc4_bytes() {
    use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc4, ifc4::IFC4_DIALECT};
    let model = house();
    let library = io().entries.iter().find(|entry| entry.direction == IoEntryDirection::Export && entry.into == IFC4_DIALECT).expect("the ifc4 entry");
    let produced = (library.run)(&IoPayload::Binary(pack::encode(&model))).expect("the entry runs");
    assert_eq!(produced.value, IoPayload::Binary(export_ifc4(&model).expect("direct export").0));
}

#[test]
fn sweeps_attached_walls_and_reveals_survive_the_pack_and_the_text_carrier_and_the_ifc_round_trip() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::attic;
    use crate::standards::v1::subsets::any::io::import::ifc::attach::tests::with_depth;
    use crate::standards::v1::subsets::any::io::text::snapshot as text;
    use crate::TopConstraint;
    let model = attic();
    assert!(!model.wall_sweeps.is_empty() && model.openings.values().any(|opening| opening.reveal_depth.is_some() && opening.reveal_material.is_some()));
    assert!(model.walls.values().any(|wall| matches!(wall.top, TopConstraint::Roof { .. })) && model.walls.values().any(|wall| wall.base_slab.is_some()));
    assert_eq!(pack::decode(&pack::encode(&model)).expect("the pack carrier decodes"), model);
    assert_eq!(text::parse_dsl(&text::print_dsl(&model)).expect("the text carrier parses"), model);
    let rooted = with_depth();
    assert!(rooted.walls.values().any(|wall| matches!(wall.top, TopConstraint::Ceiling { .. })) && rooted.walls.values().any(|wall| wall.base_slab.is_some()));
    assert_eq!(pack::decode(&pack::encode(&rooted)).expect("the pack carrier decodes"), rooted);
    assert_eq!(text::parse_dsl(&text::print_dsl(&rooted)).expect("the text carrier parses"), rooted);
}

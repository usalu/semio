from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

orphan_start = text.index("    macro_rules! __gap_surface_family_names_orphan_delete_marker {")
orphan_end = text.index(
    "    #[macro_export]\n    macro_rules! register_gap_surface_family_connections {"
)
text = text[:orphan_start] + text[orphan_end:]

with_family = """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            gap_surface_families!();
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            register_gap_surface_family_connections!($builder, gap_surface_family_name_list!())
        };
    }

"""
old_with_family = text[
    text.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names {") : text.index(
        "    #[macro_export]\n    macro_rules! register_gap_surface_family_connections {"
    )
]
text = text.replace(old_with_family, with_family, 1)

relay_list = """    macro_rules! gap_surface_existing_relay_name_list {
        () => {
        AddedAttributeToConceptInput,
        AddedAttributeToPortInput,
        AddedAttributeToQualityInput,
        AddedAttributeToTagInput,
        AddedAttributesToConceptInput,
        AddedAttributesToPortInput,
        AddedAttributesToQualityInput,
        AddedAttributesToTagInput,
        AlternativeCommand,
        ChangedDescription,
        CreatedConceptInput,
        CreatedConceptsInput,
        CreatedFixedPiece,
        CreatedPortInput,
        CreatedPortsInput,
        CreatedQualitiesInput,
        CreatedQualityInput,
        CreatedTagInput,
        CreatedTagsInput,
        DraggedPiece,
        FileBackbone,
        FileBackboneCommand,
        FixedPiece,
        Graph,
        Kit,
        LocalProviderCommand,
        Place,
        RemoteProviderCommand,
        RenamedConceptInput,
        RenamedKit,
        RenamedPortInput,
        RenamedQualityInput,
        RenamedTagInput,
        Session,
        SessionCommand,
        Side,
        StoreCommand,
        TheKit,
        UnsavedChangeCommand,
        UpdatedConceptDescriptionInput,
        UpdatedConceptIconInput,
        UpdatedPortDescriptionInput,
        UpdatedPortIconInput,
        UpdatedQualityDescriptionInput,
        UpdatedQualityIconInput,
        UpdatedTagDescriptionInput,
        UpdatedTagIconInput,
        VersionCommand,
        WebsocketBackbone,
        WebsocketBackboneCommand
        };
    }

"""

insert_at = text.index("    macro_rules! gap_surface_existing_relays {")
text = text[:insert_at] + relay_list + text[insert_at:]

with_relay = """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            gap_surface_existing_relays!();
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            register_gap_surface_existing_relay_connections!($builder, gap_surface_existing_relay_name_list!())
        };
    }

"""
old_with_relay_start = text.index(
    "    #[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names {"
)
old_with_relay_end = text.index(
    "    #[macro_export]\n    macro_rules! register_gap_surface_existing_relay_connections {"
)
text = text[:old_with_relay_start] + with_relay + text[old_with_relay_end:]

p.write_text(text, encoding="utf-8")
print("fixed", len(text))

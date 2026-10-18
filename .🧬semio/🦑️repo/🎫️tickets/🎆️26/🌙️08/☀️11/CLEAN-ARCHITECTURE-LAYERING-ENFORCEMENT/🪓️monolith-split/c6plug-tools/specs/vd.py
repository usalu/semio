from specs.lib import *
CH = [
    ch('🔤', 'localized-text-and-units', 'localized_text_and_units', [(131, 227)],
       'Localized text and units of the VDI 3805 snapshot: the bilingual text builders, the local quantity kind tag with its conversion from the compliance quantity kind and the unit record.'),
    ch('🏷', 'product-identity-and-links', 'product_identity_and_links', [(229, 288)],
       'Product identity and links of the VDI 3805 snapshot: the extension bag, the product identity, the manufacturer file and the accessory and composition links.'),
    ch('🛡', 'limits-and-code-tables', 'limits_and_code_tables', [(290, 375)],
       'Limits and code tables of the VDI 3805 snapshot: the security limits with their defaults and the connection, pressure, filter and type code tables with the sheet numeric bounds.'),
    ch('🗂', 'sheet-registry', 'sheet_registry', [(377, 620)],
       'Sheet registry of the VDI 3805 snapshot: the sheet and edition identifiers with their DSL field bindings, the schema status, the domain and the sheet entry table.'),
    ch('🧾', 'record-families', 'record_families', [(622, 883)],
       'Record families of the VDI 3805 snapshot: the building system number and the record family identifier with its DSL field bindings and the family tables.'),
    ch('🧮', 'sheet-attributes', 'sheet_attributes', [(45, 117), (885, 1102)],
       'Sheet attributes of the VDI 3805 snapshot: the attribute assembly from native records and the native record, the heating attribute sets, the generic attributes and the sheet attribute accessors.'),
    ch('📚', 'catalog-model', 'catalog_model', [(1104, 1323)],
       'Catalog model of the VDI 3805 snapshot: the configuration, the catalogue product, the manufacturer catalog, the geometry and characteristic curve records and the catalog index.'),
    ch('🩺', 'diagnostics-and-edition-profile', 'diagnostics_and_edition_profile', [(1325, 1370)],
       'Diagnostics and edition profile of the VDI 3805 snapshot: the diagnostic record with its severity and the edition profile choice.'),
    ch('🧪', 'reference-fixture', 'reference_dataset', [(1372, 1529)],
       'Reference fixture of the VDI 3805 snapshot: the reference snapshot and the conforming valve dataset.'),
]
SPEC = dict(orig='vd.rs', file='✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs', children=CH, external=[], leaf_concat=True, source_concat=True)

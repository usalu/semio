# Furnishing Exchange Review R83

Read-only independent source/diff/schema/quantity review of published528b3ffeab26e4c038b9eb603e7ae1e9bc52c66a1f068b3e799de8fcd66ccc8f. No product/test edits.

## Exact Metadata And Numerical Lineage

Compared retained pre-publication1bad source object with current source: deleting predefined_type from exactly25IfcFurnishingElement occurrences and changing only classes.IfcFurnishingElement.adds to[] reproduces current parsed source exactly, including order. Thus every geometry,GUID,class,type,material and other field is unchanged. This independently corroborates publication transition fingerprint5b9c780c4069c4784cd7170dfa73c60723a8b107561437e2ca6cf50bac900192. Main API whole-function byte comparison cdfd7872→e9afe657 shows exactly `_by_class` and `information` changed: missing predefined_type selection uses get; information raises explicit ValueError when asking an inapplicable PredefinedType. All numerical geometry function bytes unchanged. Whole-code byte stability remains false; numerical lineage is established across this narrow metadata/API change.

## IFC Taxonomy, Schema And Quantities

Retained official buildingSMART evidence in furnishing-native-audit-r77.md establishes generic IfcFurnishingElement has no added explicit attributes; eight inherited occurrence attributes end atTag. It is distinct from concrete IfcFurniture with its own PredefinedType. Generic furnishing export is therefore coherent without inventing a subtype or enum for pedagogical fixture geometry. Current schema explicitly forbids predefined_type for this class and requires it on other existing branches; registry adds[] matches that chain. New API inapplicability error preservesGlobalId and other applicable information.

Body dispatch uses actual solid_geometry native occupied parts; per-part material fallback is explicit, validates referenced IfcMaterial, then produces material constituents/association and spatial containment. NetVolume is serialized as IfcQuantityVolume with5attributes; its Unit references explicit IfcSIUnit(*,.VOLUMEUNIT.,$,.CUBIC_METRE.). IfcElementQuantity has6attributes and is linked via IfcRelDefinesByProperties to the furnishing. `FurnishingOwnedQuantities` is a custom quantity set, not a falsely claimed standardized Qto_* template. Quantity volume comes from occupied-union model quantity rather than a label or enclosing box. Geometry/material constituents and quantity may have different aggregation semantics when same-owner source components overlap; do not call sum of component volumes union without proof.

Official IFC4.3 entity/ElementQuantity/QuantityVolume pages returned403 during this reviewer’s direct verification attempt. Retained primary links and the explicit inspected entity argument chains support this scope; this is not a fresh successful full official schema validation or native SDK decode.

## New Contracts And Inventory

Inspected3new permanent NativeFurnishingContract methods: schema/attribute applicability with negative fabricated-field case; mixed rectangle+metal+section native parts/material/unit/quantity contract using independentSciPy ConvexHull volume2.9; source laundry14parts/0.470136m³ and guestbasin5parts/0.045536m³, both languages. Recorded3PASS/0failures/0errors13.4328743s on stable528bsource. These tests independently measure volume withSciPy/Shapely but parse STEP through the repository parser; they do not themselves establish third-party native IFC decoder acceptance. Owning native decoder proof should be reported separately.

Fresh lightweight collection:502standard/502unique,355hook/355unique, no duplicates or loader errors. Exact planned portable standard scope excludes the same3native gates, hence499unique planned portable identities; this is not499PASS. Net four identities added since498/351 inventory:3furnishing methods plus the registered wash-fixture clearance method. The salon radiator method was also renamed: one old identity removed and one replacement identity added, so raw ID-set comparison shows5added/1removed. Do not claim the rename as an extra contract. Generated verification-collection-r83.json preserves exact IDs.

Exact five added IDs sinceR76 (includes one renamed method):

- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪑furnishings.NativeFurnishingContract.test_mixed_owned_solids_have_native_materials_and_explicit_volume_units`
- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪑furnishings.NativeFurnishingContract.test_schema_and_information_have_no_fabricated_predefined_attribute`
- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪑furnishings.NativeFurnishingContract.test_source_hollow_furnishings_keep_real_native_parts_and_volume`
- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel.test_salon_radiator_containment_and_retired_pipe_absence`
- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️details.test_registered_wash_fixtures_keep_material_and_body_clearances`

Exact removed/replaced old ID:

- `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel.test_salon_radiator_and_terminal_pipe_are_in_salon`

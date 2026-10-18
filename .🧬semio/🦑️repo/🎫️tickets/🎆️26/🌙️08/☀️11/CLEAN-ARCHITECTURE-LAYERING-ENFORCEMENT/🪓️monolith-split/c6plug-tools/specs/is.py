from specs.lib import *
CH = [
    ch('🧱', 'catalogue-core', 'catalogue_core', [(80, 229)],
       'Catalogue core of the ISO 16757 snapshot: the catalogue identifier, dictionary reference, names, dimension signature, unit, value, null state, cardinality, extension bag and lifecycle.'),
    ch('📚', 'part-1-catalogue', 'part_1', [(231, 545)], 'Part 1 of the ISO 16757 snapshot: the product catalogue with manufacturers, groups, classes, series, properties, variants, products, relationships, selection and BIM embedding.', inline='part_1'),
    ch('📐', 'part-2-geometry', 'part_2', [(547, 728)], 'Part 2 of the ISO 16757 snapshot: the space classification, ports, bounding box, geometry nodes and objects, surfaces and the geometry catalogue.', inline='part_2'),
    ch('📖', 'part-4-dictionary', 'part_4', [(730, 866)], 'Part 4 of the ISO 16757 snapshot: the dictionary subjects, relationships, properties, controlled value lists, constraints and ISO 12006 mapping.', inline='part_4'),
    ch('🔄', 'part-5-exchange', 'part_5', [(868, 991)], 'Part 5 of the ISO 16757 snapshot: the exchange process, part number rule, external media, IFC catalogue and the script limits, result and error.', inline='part_5'),
    ch('🧾', 'artifact-definition', 'artifact_definition', [(72, 78), (993, 1036)],
       'Artifact definition of the ISO 16757 snapshot: the language-neutral package declaration, the artifact kind, the dialect coordinate, the document schema and the capability rows of the definition.'),
]
SPEC = dict(orig='is.rs', file='✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs', children=CH, external=[], leaf_concat=True, source_concat=True)

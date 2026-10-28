# Course Assignment Ownership Integration

The approved Parametrization worksheet is owned by the Architecture Parametric topic. No duplicate topic navigation was added. Its content and geometry were moved without visual changes; main files now use canonical local input names.

## Authored Paths

- Canonical leaf: 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment
- Canonical sources: en.tex, de.tex, en.content.tex, de.content.tex, geometry.tex, icons.json
- Relocated domain tests: 🧪️tests/🟦️.ts, 🥒️.feature, 🧫️cases.json, 🧬️cases.schema.json, 🔮️svg-provenance.py
- Course registry: 🎓️teaching/🏛️architecture/📚️course/🔣️.json
- Test dependency input retained in the ticket: oracle-requirements.txt

All six old parametrization-prefixed sources and the old domain-test folder were removed from the print product through moves. Sixteen generated resource PDFs were removed from authored source ownership. The icon plan retains unchanged canonical Metabolism SVG paths and SHA256 provenance; the engine creates 2048-pixel PNG resources during each build through its SVG rendering interface.

The registry uses namespace course, identifiers architecture-parametric-parametrization-en/de, producers @teaching/architecture-course:build-parametrization-en/de, and matching identifier.pdf filenames.

## Validation

- Red: built-in Node assertion failed because the canonical course assignment leaf was absent before moving sources.
- Green: repo tools exec --light bun -- -e importing the relocated adapter and calling verifyParametrizationAssignmentContentV1 passed. This includes Ajv schema validation and negative ownership mutations, mathjs and d3-array independent semantic comparisons, all existing worksheet content assertions, canonical input resolution, no authored PDFs, registry agreement, and exactly one Parametric topic.
- Green: repo tools exec --light python -- -X utf8 with the separate lxml oracle verified all sixteen canonical SVGs against SHA256, viewBox, path count and original vector XML.
- Authored path audit: all assignment source/test paths remain below 256 characters.

Python initially lacked lxml. Installed pinned test-only lxml 6.0.2 into ticket-generated scratch, ran the oracle, and removed that scratch. No runtime library or source asset was added. Native pip logging emitted a cp1252 warning while installing into emoji paths; the oracle itself ran successfully under explicit UTF-8.

## Coordination

Root owns generic course schema/validation. Build integration owns shared engine/catalog/publication/course producers and normal test registration. Those files were deliberately left to their owners. Domain integration has no remaining blockers. Generated scratch created by this subtask has been removed.

Resource staging follow-up: icons.json now declares svg-to-raster-png, per-resource width 2048 and .png output names. Geometry and language-neutral schema/cases use matching image references. Worksheet dimensions and all visual metadata remain unchanged. The complete Bun adapter passed again after this transport change.

Institutional footer ownership follow-up: both canonical content preambles now explicitly select the existing LUH footer asset through footerlogos. Fixture and schema declare the same selection in each language; the domain feature and adapter reject missing or changed footer assets. The new assertion failed before adding the declarations and the complete Bun adapter passed afterward. Shared footer rendering and visual dimensions were unchanged; root owns removal of the former global default.

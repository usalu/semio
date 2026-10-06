# Showcase Local Key Tables

Nine explicit native local tables now cover 110 controls in namespace, figure, grammar, encoding, scale, shape, layout-algorithm, transform-data and analytical. The scale family table replaces its incomplete three-row table; eight other tables replace the incorrect no-options placeholders. Registry references and all other source spans are preserved.

Before API SHA-256 `47427AD9E4FC9F9D6D8C8991D43CB7CFF266F158713A71D5BA9E56E68E9EFAA0`; after `9D414ED08E8FC69DE70B1C5E30A4D53EFF78B73667501181A8F5AC53C97D405C`. Full before/after API inputs, selected before spans, authored bilingual rows and source/metadata collisions are retained under `📥️authored-inputs/showcase-local-key-tables`. The comprehensive semantic RED64318 and source-owned contract RED42870 preceded this authored edit.

| Family | Local Keys |
|---|---:|
| namespace | 7 |
| figure | 13 |
| grammar | 12 |
| encoding | 17 |
| scale | 19 |
| shape | 9 |
| layout-algorithm | 11 |
| analytical | 11 |
| transform-data | 11 |

## Native Authority

The eight showcase tables were read against actual key declarations and respective keys_reset functions in existing semio-viz-showcase.sty. Analytical was read against its actual local keys, common reset and family reset in semio-viz-domain.sty. Token-list and comma-list controls remain string values, native bool/int/fp controls use boolean/integer/number. Geometric units are millimetres, angular span and analytical angle use degrees, and shape padding angle uses radians. Native variant defaults are local rather than the registry's empty generic value. Lists use separate Key tokens with inter-item spaces so long defaults can wrap in their cells.

## Metadata Conflicts

The existing metadata snapshot has 74 source type/default conflicts for this slice, recorded with exact before and actual source-owned values in native-metadata-collisions.json. The local tables preserve actual source semantics and do not coerce numeric or boolean options into string rows to conceal those collisions. These must be repaired by the existing source/schema owner and followed by the registered semantic checks. No compiler or semantic GREEN is claimed for this candidate.

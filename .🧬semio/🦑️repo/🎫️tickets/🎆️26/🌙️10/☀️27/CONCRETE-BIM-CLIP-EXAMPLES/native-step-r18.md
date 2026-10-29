# Native Filling STEP R18

The canonical unit enum now includes dimensionless `1`. Full Draft202012Validator.check_schema and validation of the 374-element source passed using cached official Pyodide jsonschema 4.23.0.

Source geometry is unchanged by this work. The language-agnostic fixture covers RW-S, D-SON, D-ENTRY, GARAGE-SLIDING and CW-1 in English/German. The first native contracts failed on missing body providers; six contracts now pass, including full-source schema validation and the public geometry interface. They check native occurrence/type attribute positions, independent GEOS volume reconstruction, exact material/aspect solid coverage, and fragments without spatial context. They are registered in standard load_tests.

## Primary IFC References

[IfcWindow](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWindow.htm), [IfcDoor](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcDoor.htm), [IfcWindowType](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWindowType.htm), [IfcDoorType](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcDoorType.htm), [IfcCurtainWall](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcCurtainWall.htm). Native dimensions describe rough apertures. Occurrence layout/predefined fields are omitted when a native type owns them. Unmeasured window partitions and door handedness stay NOTDEFINED.

[IfcMaterialConstituent](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcMaterialConstituent.htm) and [IfcShapeAspect](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcShapeAspect.htm) provide named material-to-component addressing. Ordinary windows use Lining and Glazing. Multi-material opaque Framing names are material-qualified, retaining Framing as Category to distinguish wood battens from plywood skins. Air has no solid constituent.

## Independent Native IFC Validation

The test-only [web-ifc](https://github.com/ThatOpen/engine_web-ifc) engine 0.0.78 came from its official npm package. Its archive passed the published integrity check `sha512-LGKMNYFVNyz9vVERN0iz+zvPQJqT7MEGBxslzcuuhwDDOA116LmJLKkg8sZ5SEvQJSc5NRVMWgpRNkDmuoQoJg==`. No package scripts ran. Its Node entry, WASM and package metadata were extracted only under ticket-generated output. Retained input adapter native-ifc-r18.mjs accepts plain paths/data and hides all external engine classes.

All 10 case/language combinations passed native product/type reads, localized names, dimensions, exact material/aspect coverage mesh-part counts, and world-space placement/bounds. Triangulated volumes agreed within 1e-6 absolute or 1e-5 relative. Native mesh volumes: RW-S 0.1617944065m³; D-SON 0.0355439976m³; entrance 0.0285500016m³; garage 0.8351700088m³; curtain 0.1729308052m³.

## Scope and Limits

This is a selected-product STEP fragment, not a complete native model export. It supplies the selected product, occupied multipart solids, type, component materials/aspects, selected common properties and optional storey containment. Host wall/opening relationships, project/unit hierarchy, all model entities and full MVD/EXPRESS global-rule validation are outside this fragment. The independent reader proves schema-aware consumption and geometry agreement, not complete building exchange compliance. Historical profile/module and hardware uncertainties remain as recorded in facade-r18.md.

## Final Checks and Files

A fresh interpreter passed all six native contracts in 10.668 seconds, including the full Draft202012Validator regression. The existing public column mesh smoke passed. Two existing STEP baseline methods (all original entity arities and independent slab/wall volume reconstruction) passed in 44.741 seconds. All 10 native IFC engine cases passed again with world bounds checked as well as volume.

The public filling_geometry(key, model) API dispatches the existing shared physical constructors. The exporter uses only public core consumer calls and preserves the public column_geometry rename. Source geometry and material dimensions were not changed by the STEP task.

Product files: model/🐍️.py (public interface); model/🔌️exchange/🐍️.py; model/🧬️schema/🔣️.schema.json (canonical dimensionless unit); model/🔌️exchange/tests/🪟️fillings.json and 🪟️fillings.py; main tests/🐍️.py registration. Retained ticket inputs: native-ifc-r18.mjs, native-ifc-setup-r18.mjs, native-step-samples-r18.py. Generated engine/archive/sample outputs stay below the ticket generated directory for final parent cleanup.

The pinned setup helper recreates the independent IFC engine from an integrity-checked archive using portable Node APIs and executes no package installation scripts. Engine-specific objects never cross the test adapter public boundary.

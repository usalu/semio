# Graph and OS Record Canonical Cut Audit

A canonical trait identity shared by Graph and OS can resolve the four direct Jack container collisions, but an OS re-export plus deletion of primitive implementations is insufficient. Three existing OS implementations then violate the foreign-trait/foreign-type boundary: ArtifactRef, Viewport2d and Viewport3dOrbit. Their actual owners must own their genuine Record bindings. No adapter or replacement umbrella API is required.

The GUI authority metadata audit is separately retained in CURRENT-NATIVE-SOURCE4-OWNED-GUI-METADATA-AUDIT.md. Its bounded refresh delegates unrelated configurations only; both owned ingress configurations, unique commands and top-level fields remain exact. It changes witness refresh metadata, not runtime guards.

## Canonical algebra and direct ownership

Current semio-framework-dsl-record is the four-normal-dependency package under neutral DSL/schema, distinct from the older Pack Record proposal. It already exports Record algebra, producer controls and binding traits through its root and binding module. Value imports are intentionally private. Graph should import Value APIs directly from semio-framework-value, Record APIs directly from semio-framework-dsl-record, and diagnostics directly from their owner. Remove its OS dependency and dsl_core alias; do not redirect the alias or export Value through Record.

The OS physical DSL component owns another DslField/DslVariants/Wire identity and its physical schema owns another Shape/FieldValue/RecordSpec/RecordValue algebra. Retire those duplicate declarations and generic implementations together. OS product implementations can privately import the canonical owners. A permanent public OS generic forwarding facade would retain the ownership ambiguity the user asked to remove.

OS component direct DslField implementations cover Box, bool, floats, String, Wire, Vec, BTreeMap, arrays and DslValue; its integer macro contributes ten integer implementations. Neutral Record already owns these bindings. After adopting its trait, copies for standard-library and Value types cannot remain in OS. Graph's own PropertyDef/PropertyValue/PropertyBag/PropertyKind/PortDirection targets are Graph-local and can legally implement the neutral trait.

## Mandatory owner closure

The OS DSL reference module imports ArtifactRef from the separately packaged neutral IO schema. Move that genuine binding to IO schema and add its direct Record dependency, then remove the OS implementation. The OS viewport module similarly implements its local trait for the external UI viewport's Viewport2d and Viewport3dOrbit. Move those bindings to their actual UI viewport owner and add its Record dependency. The captured Record dependency closure contains no IO/viewport dependency; this is static dependency evidence, not a compiled cycle verdict. OS-local Store types such as OwnerRef/LinkPin/ArtifactLink/ArtifactChild remain legal implementors of the foreign canonical trait.

Four direct Jack declarations establish the nominal collision: Edge carries Graph PropertyBag; NodeKindDef, EdgeKindDef and PortKindDef carry Vec<Graph PropertyDef>, with PortKindDef also carrying Graph PortDirection. They currently derive the OS dsl::DslRecord trait identity. JackManifest and JackSnapshot propagate the same container requirements. Jack's Camera is a Jack-local type, not the UI viewport type. Jack currently lacks direct neutral Record and Record Derive dependencies; its existing Value/Diagnostic dependencies already cover those generated paths.

## Macro and test closure

Neutral Record Derive exports DslRecord/DslScalar/DslEnum and emits direct semio_framework_dsl_record, semio_framework_value and semio_framework_diagnostic paths. The OS derive exports eight macros. Its generic derives emit ::dsl algebra and ::dsl::__rt::field_error; the latter helper is absent from neutral Record. Blindly changing one alias therefore does not establish macro closure. Move generic derive call sites to the existing canonical derives and rewrite remaining product macros' generic emitted paths directly to the canonical owners.

Do not replace DslArtifact indiscriminately with DslRecord: it also emits envelope ID/extension constants and product Store errors. DslDiff emits product DiffCodec/ProtocolError/pack runtime; MutationLeaf/Mutations have product metadata semantics. Those product macros need a separate bounded emitted-path review, rather than deleting all eight as a group. Compiler expansion counts remain unknown.

Test-first source cohort: canonical Record public binding/producer/refusal laws; IO ArtifactRef and UI viewport owned binding laws; Graph's previously retained 12-row direct-owner retirement cohort; Jack's four container declarations plus manifest/snapshot propagation and direct Cargo edges; OS duplicate component/schema/producer/decoding/encoding removal and its product macro paths. Preserve product-specific codec tests in OS. OS boxed-field, checked-integer, protocol-record and hygienic-binding tests must use the actual canonical generic owners when their generic API disappears. Neutral public-field refusal and controlled producer laws provide retained owner tests. No named owner gate should skip missing sources, and generic enforcement must continue using synthetic owner fixtures.

Full current sources and SHA-256 inputs are retained in graph-os-record-canonical-cut-inputs/full-current-owner-cohort-1.json (48 bounded files). This is an ownership/orphan and source-dependency audit only. No production source changed, no compiler ran, and no native/runtime success is claimed. Exact mounted source cohort and macro-expansion closure require test-first publication and ordinary registered compiler/runtime proof.

## Parsed declaration evidence


Read-only current source AST audit; no source/native writes. All compiler/macro-expansion unresolved counts remain unknown. Current Record manifest owns four normal dependencies and independent three neutral derives; it is distinct from Pack Record owner.

```json
[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "trait_item",
        "name": "DslField"
      },
      {
        "kind": "struct_item",
        "name": "Wire"
      },
      {
        "kind": "trait_item",
        "name": "DslVariants"
      }
    ],
    "fieldTraitImpls": [
      {
        "trait": "DslField",
        "target": "Box<T>"
      },
      {
        "trait": "DslField",
        "target": "bool"
      },
      {
        "trait": "DslField",
        "target": "f32"
      },
      {
        "trait": "DslField",
        "target": "f64"
      },
      {
        "trait": "DslField",
        "target": "String"
      },
      {
        "trait": "DslField",
        "target": "Wire"
      },
      {
        "trait": "DslField",
        "target": "Vec<T>"
      },
      {
        "trait": "DslField",
        "target": "std::collections::BTreeMap<String, T>"
      },
      {
        "trait": "DslField",
        "target": "[T; N]"
      },
      {
        "trait": "DslField",
        "target": "DslValue"
      }
    ],
    "jackGraphContainers": []
  },
  {
    "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "trait_item",
        "name": "DslField"
      },
      {
        "kind": "struct_item",
        "name": "Wire"
      },
      {
        "kind": "trait_item",
        "name": "DslVariants"
      }
    ],
    "fieldTraitImpls": [
      {
        "trait": "DslField",
        "target": "Box<T>"
      },
      {
        "trait": "DslField",
        "target": "bool"
      },
      {
        "trait": "DslField",
        "target": "f32"
      },
      {
        "trait": "DslField",
        "target": "f64"
      },
      {
        "trait": "DslField",
        "target": "String"
      },
      {
        "trait": "DslField",
        "target": "Wire"
      },
      {
        "trait": "DslField",
        "target": "Vec<T>"
      },
      {
        "trait": "DslField",
        "target": "std::collections::BTreeMap<String, T>"
      },
      {
        "trait": "DslField",
        "target": "[T; N]"
      },
      {
        "trait": "DslField",
        "target": "DslValue"
      }
    ],
    "jackGraphContainers": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "enum_item",
        "name": "RecordLayout"
      },
      {
        "kind": "enum_item",
        "name": "Shape"
      },
      {
        "kind": "struct_item",
        "name": "FieldSpec"
      },
      {
        "kind": "struct_item",
        "name": "RecordSpec"
      },
      {
        "kind": "struct_item",
        "name": "GrammarSpec"
      },
      {
        "kind": "struct_item",
        "name": "WireNode"
      },
      {
        "kind": "struct_item",
        "name": "WireEdgeLabel"
      },
      {
        "kind": "struct_item",
        "name": "WireValue"
      },
      {
        "kind": "enum_item",
        "name": "FieldValue"
      },
      {
        "kind": "struct_item",
        "name": "RecordValue"
      },
      {
        "kind": "type_item",
        "name": "Cst"
      },
      {
        "kind": "enum_item",
        "name": "ExprOp"
      },
      {
        "kind": "enum_item",
        "name": "ExprValue"
      },
      {
        "kind": "enum_item",
        "name": "SourceMode"
      },
      {
        "kind": "struct_item",
        "name": "ParseOptions"
      },
      {
        "kind": "enum_item",
        "name": "JoinMode"
      },
      {
        "kind": "struct_item",
        "name": "Writer"
      },
      {
        "kind": "struct_item",
        "name": "LanguageService"
      },
      {
        "kind": "struct_item",
        "name": "CompletionItem"
      }
    ],
    "fieldTraitImpls": [],
    "jackGraphContainers": []
  },
  {
    "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "enum_item",
        "name": "RecordLayout"
      },
      {
        "kind": "enum_item",
        "name": "Shape"
      },
      {
        "kind": "struct_item",
        "name": "FieldSpec"
      },
      {
        "kind": "struct_item",
        "name": "RecordSpec"
      },
      {
        "kind": "struct_item",
        "name": "GrammarSpec"
      },
      {
        "kind": "struct_item",
        "name": "WireNode"
      },
      {
        "kind": "struct_item",
        "name": "WireEdgeLabel"
      },
      {
        "kind": "struct_item",
        "name": "WireValue"
      },
      {
        "kind": "enum_item",
        "name": "FieldValue"
      },
      {
        "kind": "struct_item",
        "name": "RecordValue"
      },
      {
        "kind": "type_item",
        "name": "Cst"
      },
      {
        "kind": "enum_item",
        "name": "ExprOp"
      },
      {
        "kind": "enum_item",
        "name": "ExprValue"
      },
      {
        "kind": "enum_item",
        "name": "SourceMode"
      },
      {
        "kind": "struct_item",
        "name": "ParseOptions"
      },
      {
        "kind": "enum_item",
        "name": "JoinMode"
      },
      {
        "kind": "struct_item",
        "name": "Writer"
      },
      {
        "kind": "struct_item",
        "name": "LanguageService"
      },
      {
        "kind": "struct_item",
        "name": "CompletionItem"
      }
    ],
    "fieldTraitImpls": [],
    "jackGraphContainers": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔗️reference/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [],
    "fieldTraitImpls": [
      {
        "trait": "DslField",
        "target": "ArtifactRef"
      }
    ],
    "jackGraphContainers": []
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🪟️viewport/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [],
    "fieldTraitImpls": [
      {
        "trait": "DslField",
        "target": "Viewport2d"
      },
      {
        "trait": "DslField",
        "target": "Viewport3dOrbit"
      }
    ],
    "jackGraphContainers": []
  },
  {
    "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "enum_item",
        "name": "TrinityRamError"
      },
      {
        "kind": "struct_item",
        "name": "Port"
      },
      {
        "kind": "struct_item",
        "name": "Node"
      },
      {
        "kind": "struct_item",
        "name": "Edge"
      },
      {
        "kind": "struct_item",
        "name": "Camera"
      },
      {
        "kind": "struct_item",
        "name": "Graph"
      },
      {
        "kind": "enum_item",
        "name": "EntityRef"
      },
      {
        "kind": "trait_item",
        "name": "ArtifactApps"
      }
    ],
    "fieldTraitImpls": [],
    "jackGraphContainers": [
      {
        "name": "Edge",
        "fullStruct": "pub struct Edge {\n    pub id: String,\n    pub kind: String,\n    pub source: String,\n    pub target: String,\n    #[value(default)]\n    pub properties: PropertyBag,\n}",
        "attributes": "#[value(rename_all = \"camelCase\")]"
      }
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest/🦀️.rs",
    "parserError": false,
    "publicDeclarations": [
      {
        "kind": "struct_item",
        "name": "Manifest"
      },
      {
        "kind": "struct_item",
        "name": "NodeKindDef"
      },
      {
        "kind": "struct_item",
        "name": "EdgeKindDef"
      },
      {
        "kind": "struct_item",
        "name": "PortKindDef"
      }
    ],
    "fieldTraitImpls": [],
    "jackGraphContainers": [
      {
        "name": "NodeKindDef",
        "fullStruct": "pub struct NodeKindDef {\n    pub name: String,\n    #[value(default)]\n    pub properties: Vec<PropertyDef>,\n    #[value(default, rename = \"portKinds\")]\n    pub port_kinds: Vec<String>,\n}",
        "attributes": "#[value(rename_all = \"camelCase\")]"
      },
      {
        "name": "EdgeKindDef",
        "fullStruct": "pub struct EdgeKindDef {\n    pub name: String,\n    #[value(default)]\n    pub properties: Vec<PropertyDef>,\n}",
        "attributes": "#[value(rename_all = \"camelCase\")]"
      },
      {
        "name": "PortKindDef",
        "fullStruct": "pub struct PortKindDef {\n    pub name: String,\n    pub direction: PortDirection,\n    #[value(default)]\n    pub properties: Vec<PropertyDef>,\n}",
        "attributes": "#[value(rename_all = \"camelCase\")]"
      }
    ]
  }
]
```

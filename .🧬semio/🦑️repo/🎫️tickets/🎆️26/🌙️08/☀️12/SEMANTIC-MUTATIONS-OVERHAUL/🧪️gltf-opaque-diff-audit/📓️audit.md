# glTF Opaque Diff Audit

Read-only audit of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations`. No source edits, Cargo, or Nx runs. Exact current leaf and recoverable typed-inverse mappings are retained in `🔣️audit.json`. The shared STDIO source freeze remains in force.

## Executed Counts

```json
{
  "counts": {
    "leaves": 120,
    "arbitraryRestore": 120,
    "inversesBuildRestore": 120,
    "directTextCodecs": 0,
    "directBinaryCodecs": 0,
    "nullTextOpcodes": 120,
    "nullBinaryTags": 120,
    "payloadSchemasWithoutPhase": 120,
    "declaredDetect": 120,
    "explicitDetectMethods": 0,
    "retainedTestFiles": 130,
    "testPathMounts": 0,
    "originalInverseFiles": 116,
    "originalArbitraryDiffTypes": 45,
    "originalStaleGuards": 10
  },
  "verbs": {
    "add": 2,
    "bind": 10,
    "change": 23,
    "create": 15,
    "delete": 15,
    "move": 22,
    "remove": 2,
    "reorder": 21,
    "unbind": 10
  }
}
```

## Confirmed Escape Hatch

All 120 direct Rust mutation types are public serde `Apply(Payload) | Restore(GltfDiff)` enums. Every inverse builds its own semantic variant around `Self::Restore(inverse)`. The Restore branch checks only whether the supplied whole-document diff applies; it does not constrain it to that leaf's semantic target. For example, the change-node-name type can carry source-form, asset, collection, buffer, and document-extra changes despite its name and descriptor. This is not an explicit semantic inverse and must not remain a public wire carrier.

Representative: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️component.rs:20` (payload) and line 50 (inverse construction). Runtime execution was not attempted during the source freeze; the conclusion follows directly from the public serde enum and branch body.

## Codec and Schema Closure

- Canonical executing codecs are outside the schema mutation root, under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🧬️mutations`. Text prints `gltf-mutation payload=<hex(JSON aggregate)>`; binary writes format byte, marker `0x47`, varint JSON byte length, and JSON bytes. Binary encode/decode and text decode enforce a 64 KiB payload bound; text encode does not. Neither codec dispatches through operation-specific direct facets.
- All 120 descriptors declare null textOpcode/binaryTag and omit text/binary required-language surfaces even though the aggregate is publicly serializable. There are 0 direct Rust text facets and 0 binary facets. The structural gate therefore does not prove leaf codec ownership.
- All 120 direct payload schemas describe the underlying raw typed payload, not the Rust phase/value wrapper. The root Rust aggregate uses camelCase serde discriminators while the root JSON schema uses kebab-case semantic identities. TypeScript also exposes raw payloads.
- Ajv2020 independently confirmed the representative schema accepts the direct raw payload and rejects both Rust Apply and Restore wire payloads:

```json
[
  {
    "name": "declared direct payload",
    "value": {
      "node": 0,
      "value": "Renamed"
    },
    "schemaAccepts": true,
    "errors": null
  },
  {
    "name": "Rust Apply wire payload",
    "value": {
      "phase": "apply",
      "value": {
        "node": 0,
        "value": "Renamed"
      }
    },
    "schemaAccepts": false,
    "errors": [
      {
        "instancePath": "",
        "schemaPath": "#/required",
        "keyword": "required",
        "params": {
          "missingProperty": "node"
        },
        "message": "must have required property 'node'"
      },
      {
        "instancePath": "",
        "schemaPath": "#/additionalProperties",
        "keyword": "additionalProperties",
        "params": {
          "additionalProperty": "phase"
        },
        "message": "must NOT have additional properties"
      },
      {
        "instancePath": "/value",
        "schemaPath": "#/properties/value/type",
        "keyword": "type",
        "params": {
          "type": [
            "string",
            "null"
          ]
        },
        "message": "must be string,null"
      }
    ]
  },
  {
    "name": "Rust arbitrary Restore wire payload",
    "value": {
      "phase": "restore",
      "value": {
        "sourceForm": "glb"
      }
    },
    "schemaAccepts": false,
    "errors": [
      {
        "instancePath": "",
        "schemaPath": "#/required",
        "keyword": "required",
        "params": {
          "missingProperty": "node"
        },
        "message": "must have required property 'node'"
      },
      {
        "instancePath": "",
        "schemaPath": "#/additionalProperties",
        "keyword": "additionalProperties",
        "params": {
          "additionalProperty": "phase"
        },
        "message": "must NOT have additional properties"
      },
      {
        "instancePath": "/value",
        "schemaPath": "#/properties/value/type",
        "keyword": "type",
        "params": {
          "type": [
            "string",
            "null"
          ]
        },
        "message": "must be string,null"
      }
    ]
  }
]
```

## Dormant Laws and Detect Claims

There are 130 retained per-leaf Rust contract/scenario files, but 0 current leaf source mounts to those files. Many still import the removed `diff`, `inverse`, `mutation`, and `DESCRIPTOR` triad APIs; current sources run only a small inline identity assertion. Shared glue contains no mounts for these old per-leaf tests. Thus the earlier compiled library check was not proof that these retained semantic/inverse laws executed. All 120 descriptors advertise detect, while no current direct leaf implements an explicit detect method; planner ownership must be audited independently instead of inferred from the descriptor.

## Reusable Typed Inverse Sources

The scoped read-only Git diff identifies 116 deleted typed inverse Rust sources, all readable from HEAD at the exact paths below. 45 mention arbitrary GltfDiff; 10 contain stale-inverse guards. Their computations and preconditions can be moved into direct owners, without restoring nested modules or compatibility aliases. They are design evidence, not automatically-correct drop-ins: generic multi-operation inverse enums and collection/reference restoration need narrowing and tests.

- Change-node-name previously stored only node, before name, after name, and exact touched path; it rejected stale after-values. Reuse that precondition with a normal change-node-name payload restoring the old optional name.
- Create-scene previously stored position/default-scene before and expected after-state, checked the created scene and exact scene sequence, then called the still-existing `mutation_support/create-scene::remove_created_scene`. Reuse those guards but emit semantic delete-scene plus an explicit default-scene bind/unbind when required.
- Top-level collection inverse sources carry typed Insert/Delete/Move/Reorder operations, e.g. delete-node stores the removed GltfNode and its position. Reuse item/index/permutation calculations, not the overbroad four-operation enum under each semantic leaf. Deletion repair discards incoming optional/list references, so insertion alone is insufficient: restore affected scene roots, node children, skin joints/skeleton, animation targets, and other family-specific relations through explicit typed semantic operations.
- The current `schema/🔺️diff/🦀️component.rs` retains `inverse_indexed_collection`, `IndexedCollectionDiff::inverse`, typed item diffs, and `GltfDiff::inverse`. They are useful internal algebra/oracles; do not serialize their output as any mutation payload.

## Dependency-First Semantic Inverse Plan

The 116 recoverable inverse sources are not uniformly typed restoration records: 45 return a whole `GltfDiff` (usually populated for one family) and therefore cannot be exposed as mutation payloads. The remaining 71 provide narrower typed record or operation evidence. Even these require review: `HEAD:.../bind-node-mesh/↩️inverse/🦀️component.rs` contains `next.document.nodes[diff.operation.node]` inside `apply(..., inverse)`, an undeclared-variable bug. Preserve the before-value/index intent, not the broken source or its old module API.

The root GraphQL file also starts its roster with a literal `+# add-required-extension` line; that is not a GraphQL comment and requires a real parser gate in remediation. No GraphQL parser dependency was available in this workspace lookup, so this audit reports the lexical defect without a GraphQL test-pass claim.

1. Remove phase/Restore carriers and use each direct typed payload as the aggregate wrapper. Correct schema, TS, GraphQL, protobuf, descriptor tags/opcodes, and direct codec ownership together. Keep generic transport framing outside mutation behavior.
2. Finish scalar/optional-field and transform leaves first: inverse restores the exact prior value with the same semantic mutation and preserves stale-target guards from the old typed inverse. Matrix/TRS exclusivity and optional absence must round-trip.
3. Finish bind/unbind pairs and ordered relation lists: recover previous binding, original position, and map ordering; use normal bind/unbind/move/reorder leaves, not arbitrary collections. Bind over an existing target may invert to another bind, not always unbind.
4. Finish move/reorder leaves: invert positions/permutations while reusing typed reference repair. Test index rebasing at both sides and inverse-of-inverse.
5. Finish create/delete families: create inverts to deletion of the known new typed item; deletion must restore a fully typed item plus its affected references. Current create payloads often accept only a position and create a default item, so add field-specific semantic setters or dedicated typed insert-item operations before claiming deletion invertible. Do not smuggle a complete GltfSnapshot or GltfDiff through any new operation.
6. Restore the dormant language-neutral scenario/contract laws against the new public trait API, add cross-semantic Restore rejection fixtures, and run forward+inverse through both direct codecs. Verify untouched sibling fields and references as well as final equality.

## Exact Current Leaf Types

| Semantic Kind | Direct Rust Type | Underlying Payload | Restore Line |
| --- | --- | --- | ---: |
| add-required-extension | AddRequiredExtensionMutation | GltfRequireExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️🧩️add-required-extension/🦀️component.rs:20) |
| add-used-extension | AddUsedExtensionMutation | GltfDeclareUsedExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📣️🧩️add-used-extension/🦀️component.rs:20) |
| bind-default-scene | BindDefaultSceneMutation | GltfBindDefaultScenePayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-default-scene/🦀️component.rs:20) |
| bind-morph-target-attribute | BindMorphTargetAttributeMutation | GltfBindMorphTargetAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🧬️bind-morph-target-attribute/🦀️component.rs:21) |
| bind-node-camera | BindNodeCameraMutation | GltfBindNodeCameraPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-camera/🦀️component.rs:22) |
| bind-node-child | BindNodeChildMutation | GltfBindNodeChildPayload | [50](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-child/🦀️component.rs:50) |
| bind-node-mesh | BindNodeMeshMutation | GltfBindNodeMeshPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-mesh/🦀️component.rs:22) |
| bind-node-skin | BindNodeSkinMutation | GltfBindNodeSkinPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔘️bind-node-skin/🦀️component.rs:22) |
| bind-primitive-attribute | BindPrimitiveAttributeMutation | GltfBindPrimitiveAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-attribute/🦀️component.rs:21) |
| bind-primitive-indices | BindPrimitiveIndicesMutation | GltfBindPrimitiveIndicesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-indices/🦀️component.rs:21) |
| bind-primitive-material | BindPrimitiveMaterialMutation | GltfBindPrimitiveMaterialPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-material/🦀️component.rs:21) |
| bind-scene-root-node | BindSceneRootNodeMutation | GltfBindSceneRootNodePayload | [39](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🎬️bind-scene-root-node/🦀️component.rs:39) |
| change-asset-descriptive-metadata | ChangeAssetDescriptiveMetadataMutation | GltfChangeAssetDescriptiveMetadataPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-descriptive-metadata/🦀️component.rs:20) |
| change-asset-extension-data | ChangeAssetExtensionDataMutation | GltfChangeAssetExtensionDataPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extension-data/🦀️component.rs:21) |
| change-asset-extra-data | ChangeAssetExtraDataMutation | GltfChangeAssetExtraDataPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extra-data/🦀️component.rs:21) |
| change-asset-version | ChangeAssetVersionMutation | GltfChangeAssetVersionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-version/🦀️component.rs:20) |
| change-document-extension-data | ChangeDocumentExtensionDataMutation | GltfChangeDocumentExtensionDataPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extension-data/🦀️component.rs:21) |
| change-document-extra-data | ChangeDocumentExtraDataMutation | GltfChangeDocumentExtraDataPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extra-data/🦀️component.rs:21) |
| change-material-alpha-mode | ChangeMaterialAlphaModeMutation | GltfChangeMaterialAlphaModePayload | [64](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-alpha-mode/🦀️component.rs:64) |
| change-material-double-sided | ChangeMaterialDoubleSidedMutation | GltfChangeMaterialDoubleSidedPayload | [62](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-double-sided/🦀️component.rs:62) |
| change-mesh-extension-data | ChangeMeshExtensionDataMutation | GltfChangeMeshExtensionDataPayload | [24](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extension-data/🦀️component.rs:24) |
| change-mesh-extra-data | ChangeMeshExtraDataMutation | GltfChangeMeshExtraDataPayload | [24](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extra-data/🦀️component.rs:24) |
| change-mesh-morph-weights | ChangeMeshMorphWeightsMutation | GltfChangeMeshMorphWeightsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-morph-weights/🦀️component.rs:21) |
| change-mesh-name | ChangeMeshNameMutation | GltfChangeMeshNamePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-name/🦀️component.rs:21) |
| change-node-extension-data | ChangeNodeExtensionDataMutation | GltfChangeNodeExtensionDataPayload | [25](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extension-data/🦀️component.rs:25) |
| change-node-extra-data | ChangeNodeExtraDataMutation | GltfChangeNodeExtraDataPayload | [24](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-extra-data/🦀️component.rs:24) |
| change-node-morph-weights | ChangeNodeMorphWeightsMutation | GltfChangeNodeMorphWeightsPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-morph-weights/🦀️component.rs:22) |
| change-node-name | ChangeNodeNameMutation | GltfChangeNodeNamePayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️component.rs:20) |
| change-node-transform | ChangeNodeTransformMutation | GltfTransformNodePayload | [25](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️🔘️change-node-transform/🦀️component.rs:25) |
| change-primitive-extension-data | ChangePrimitiveExtensionDataMutation | GltfChangePrimitiveExtensionDataPayload | [24](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extension-data/🦀️component.rs:24) |
| change-primitive-extra-data | ChangePrimitiveExtraDataMutation | GltfChangePrimitiveExtraDataPayload | [24](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extra-data/🦀️component.rs:24) |
| change-primitive-topology-mode | ChangePrimitiveTopologyModeMutation | GltfChangePrimitiveTopologyModePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-topology-mode/🦀️component.rs:21) |
| change-scene-extension-data | ChangeSceneExtensionDataMutation | GltfChangeSceneExtensionDataPayload | [25](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extension-data/🦀️component.rs:25) |
| change-scene-extra-data | ChangeSceneExtraDataMutation | GltfChangeSceneExtraDataPayload | [25](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-extra-data/🦀️component.rs:25) |
| change-scene-name | ChangeSceneNameMutation | GltfChangeSceneNamePayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🎬️change-scene-name/🦀️component.rs:22) |
| create-accessor | CreateAccessorMutation | GltfCreateAccessorPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️📐️create-accessor/🦀️component.rs:21) |
| create-animation | CreateAnimationMutation | GltfCreateAnimationPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎞️create-animation/🦀️component.rs:21) |
| create-buffer | CreateBufferMutation | GltfCreateBufferPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️💾️create-buffer/🦀️component.rs:21) |
| create-buffer-view | CreateBufferViewMutation | GltfCreateBufferViewPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️👁️create-buffer-view/🦀️component.rs:21) |
| create-camera | CreateCameraMutation | GltfCreateCameraPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎥️create-camera/🦀️component.rs:21) |
| create-image | CreateImageMutation | GltfCreateImagePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🖼️create-image/🦀️component.rs:21) |
| create-material | CreateMaterialMutation | GltfCreateMaterialPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️💎️create-material/🦀️component.rs:21) |
| create-mesh | CreateMeshMutation | GltfCreateMeshPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🕸️create-mesh/🦀️component.rs:21) |
| create-morph-target | CreateMorphTargetMutation | GltfCreateMorphTargetPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🧬️create-morph-target/🦀️component.rs:21) |
| create-node | CreateNodeMutation | GltfCreateNodePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🔘️create-node/🦀️component.rs:21) |
| create-primitive | CreatePrimitiveMutation | GltfCreatePrimitivePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🔺️create-primitive/🦀️component.rs:21) |
| create-sampler | CreateSamplerMutation | GltfCreateSamplerPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎛️create-sampler/🦀️component.rs:21) |
| create-scene | CreateSceneMutation | GltfCreateScenePayload | [33](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎬️create-scene/🦀️component.rs:33) |
| create-skin | CreateSkinMutation | GltfCreateSkinPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🧥️create-skin/🦀️component.rs:21) |
| create-texture | CreateTextureMutation | GltfCreateTexturePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️🎨️create-texture/🦀️component.rs:21) |
| delete-accessor | DeleteAccessorMutation | GltfDeleteAccessorPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️📐️delete-accessor/🦀️component.rs:21) |
| delete-animation | DeleteAnimationMutation | GltfDeleteAnimationPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎞️delete-animation/🦀️component.rs:21) |
| delete-buffer | DeleteBufferMutation | GltfDeleteBufferPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️💾️delete-buffer/🦀️component.rs:21) |
| delete-buffer-view | DeleteBufferViewMutation | GltfDeleteBufferViewPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️👁️delete-buffer-view/🦀️component.rs:21) |
| delete-camera | DeleteCameraMutation | GltfDeleteCameraPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎥️delete-camera/🦀️component.rs:21) |
| delete-image | DeleteImageMutation | GltfDeleteImagePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🖼️delete-image/🦀️component.rs:21) |
| delete-material | DeleteMaterialMutation | GltfDeleteMaterialPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️💎️delete-material/🦀️component.rs:21) |
| delete-mesh | DeleteMeshMutation | GltfDeleteMeshPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🕸️delete-mesh/🦀️component.rs:21) |
| delete-morph-target | DeleteMorphTargetMutation | GltfDeleteMorphTargetPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🧬️delete-morph-target/🦀️component.rs:21) |
| delete-node | DeleteNodeMutation | GltfDeleteNodePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🔘️delete-node/🦀️component.rs:21) |
| delete-primitive | DeletePrimitiveMutation | GltfDeletePrimitivePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🔺️delete-primitive/🦀️component.rs:21) |
| delete-sampler | DeleteSamplerMutation | GltfDeleteSamplerPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎛️delete-sampler/🦀️component.rs:21) |
| delete-scene | DeleteSceneMutation | GltfDeleteScenePayload | [34](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎬️delete-scene/🦀️component.rs:34) |
| delete-skin | DeleteSkinMutation | GltfDeleteSkinPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🧥️delete-skin/🦀️component.rs:21) |
| delete-texture | DeleteTextureMutation | GltfDeleteTexturePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️🎨️delete-texture/🦀️component.rs:21) |
| move-accessor | MoveAccessorMutation | GltfMoveAccessorPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️📐️move-accessor/🦀️component.rs:21) |
| move-animation | MoveAnimationMutation | GltfMoveAnimationPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎞️move-animation/🦀️component.rs:21) |
| move-buffer | MoveBufferMutation | GltfMoveBufferPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️💾️move-buffer/🦀️component.rs:21) |
| move-buffer-view | MoveBufferViewMutation | GltfMoveBufferViewPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️👁️move-buffer-view/🦀️component.rs:21) |
| move-camera | MoveCameraMutation | GltfMoveCameraPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎥️move-camera/🦀️component.rs:21) |
| move-image | MoveImageMutation | GltfMoveImagePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🖼️move-image/🦀️component.rs:21) |
| move-material | MoveMaterialMutation | GltfMoveMaterialPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️💎️move-material/🦀️component.rs:21) |
| move-mesh | MoveMeshMutation | GltfMoveMeshPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🕸️move-mesh/🦀️component.rs:21) |
| move-morph-target | MoveMorphTargetMutation | GltfMoveMorphTargetPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target/🦀️component.rs:21) |
| move-morph-target-attribute | MoveMorphTargetAttributeMutation | GltfMoveMorphTargetAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target-attribute/🦀️component.rs:21) |
| move-node | MoveNodeMutation | GltfMoveNodePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node/🦀️component.rs:21) |
| move-node-child | MoveNodeChildMutation | GltfMoveNodeChildPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔘️move-node-child/🦀️component.rs:22) |
| move-node-parent | MoveNodeParentMutation | GltfReparentNodePayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👪️🔘️move-node-parent/🦀️component.rs:22) |
| move-primitive | MovePrimitiveMutation | GltfMovePrimitivePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive/🦀️component.rs:21) |
| move-primitive-attribute | MovePrimitiveAttributeMutation | GltfMovePrimitiveAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive-attribute/🦀️component.rs:21) |
| move-required-extension | MoveRequiredExtensionMutation | GltfMoveRequiredExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-required-extension/🦀️component.rs:20) |
| move-sampler | MoveSamplerMutation | GltfMoveSamplerPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎛️move-sampler/🦀️component.rs:21) |
| move-scene | MoveSceneMutation | GltfMoveScenePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene/🦀️component.rs:21) |
| move-scene-root-node | MoveSceneRootNodeMutation | GltfMoveSceneRootNodePayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎬️move-scene-root-node/🦀️component.rs:22) |
| move-skin | MoveSkinMutation | GltfMoveSkinPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧥️move-skin/🦀️component.rs:21) |
| move-texture | MoveTextureMutation | GltfMoveTexturePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🎨️move-texture/🦀️component.rs:21) |
| move-used-extension | MoveUsedExtensionMutation | GltfMoveUsedExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-used-extension/🦀️component.rs:20) |
| remove-required-extension | RemoveRequiredExtensionMutation | GltfUnrequireExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️🧩️remove-required-extension/🦀️component.rs:20) |
| remove-used-extension | RemoveUsedExtensionMutation | GltfWithdrawUsedExtensionPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔙️🧩️remove-used-extension/🦀️component.rs:20) |
| reorder-accessors | ReorderAccessorsMutation | GltfReorderAccessorsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️📐️reorder-accessors/🦀️component.rs:21) |
| reorder-animations | ReorderAnimationsMutation | GltfReorderAnimationsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎞️reorder-animations/🦀️component.rs:21) |
| reorder-buffer-views | ReorderBufferViewsMutation | GltfReorderBufferViewsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️👁️reorder-buffer-views/🦀️component.rs:21) |
| reorder-buffers | ReorderBuffersMutation | GltfReorderBuffersPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️💾️reorder-buffers/🦀️component.rs:21) |
| reorder-cameras | ReorderCamerasMutation | GltfReorderCamerasPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎥️reorder-cameras/🦀️component.rs:21) |
| reorder-images | ReorderImagesMutation | GltfReorderImagesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🖼️reorder-images/🦀️component.rs:21) |
| reorder-materials | ReorderMaterialsMutation | GltfReorderMaterialsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️💎️reorder-materials/🦀️component.rs:21) |
| reorder-meshs | ReorderMeshsMutation | GltfReorderMeshsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🕸️reorder-meshs/🦀️component.rs:21) |
| reorder-morph-target-attributes | ReorderMorphTargetAttributesMutation | GltfReorderMorphTargetAttributesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-target-attributes/🦀️component.rs:21) |
| reorder-morph-targets | ReorderMorphTargetsMutation | GltfReorderMorphTargetsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-targets/🦀️component.rs:21) |
| reorder-node-children | ReorderNodeChildrenMutation | GltfReorderNodeChildrenPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-node-children/🦀️component.rs:22) |
| reorder-nodes | ReorderNodesMutation | GltfReorderNodesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔘️reorder-nodes/🦀️component.rs:21) |
| reorder-primitive-attributes | ReorderPrimitiveAttributesMutation | GltfReorderPrimitiveAttributesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitive-attributes/🦀️component.rs:21) |
| reorder-primitives | ReorderPrimitivesMutation | GltfReorderPrimitivesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitives/🦀️component.rs:21) |
| reorder-required-extensions | ReorderRequiredExtensionsMutation | GltfReorderRequiredExtensionsPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-required-extensions/🦀️component.rs:20) |
| reorder-samplers | ReorderSamplersMutation | GltfReorderSamplersPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎛️reorder-samplers/🦀️component.rs:21) |
| reorder-scene-root-nodes | ReorderSceneRootNodesMutation | GltfReorderSceneRootNodesPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scene-root-nodes/🦀️component.rs:22) |
| reorder-scenes | ReorderScenesMutation | GltfReorderScenesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎬️reorder-scenes/🦀️component.rs:21) |
| reorder-skins | ReorderSkinsMutation | GltfReorderSkinsPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧥️reorder-skins/🦀️component.rs:21) |
| reorder-textures | ReorderTexturesMutation | GltfReorderTexturesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🎨️reorder-textures/🦀️component.rs:21) |
| reorder-used-extensions | ReorderUsedExtensionsMutation | GltfReorderUsedExtensionsPayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-used-extensions/🦀️component.rs:20) |
| unbind-default-scene | UnbindDefaultSceneMutation | GltfUnbindDefaultScenePayload | [20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-default-scene/🦀️component.rs:20) |
| unbind-morph-target-attribute | UnbindMorphTargetAttributeMutation | GltfUnbindMorphTargetAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🧬️unbind-morph-target-attribute/🦀️component.rs:21) |
| unbind-node-camera | UnbindNodeCameraMutation | GltfUnbindNodeCameraPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-camera/🦀️component.rs:22) |
| unbind-node-child | UnbindNodeChildMutation | GltfUnbindNodeChildPayload | [37](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-child/🦀️component.rs:37) |
| unbind-node-mesh | UnbindNodeMeshMutation | GltfUnbindNodeMeshPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-mesh/🦀️component.rs:22) |
| unbind-node-skin | UnbindNodeSkinMutation | GltfUnbindNodeSkinPayload | [22](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔘️unbind-node-skin/🦀️component.rs:22) |
| unbind-primitive-attribute | UnbindPrimitiveAttributeMutation | GltfUnbindPrimitiveAttributePayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-attribute/🦀️component.rs:21) |
| unbind-primitive-indices | UnbindPrimitiveIndicesMutation | GltfUnbindPrimitiveIndicesPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-indices/🦀️component.rs:21) |
| unbind-primitive-material | UnbindPrimitiveMaterialMutation | GltfUnbindPrimitiveMaterialPayload | [21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-material/🦀️component.rs:21) |
| unbind-scene-root-node | UnbindSceneRootNodeMutation | GltfUnbindSceneRootNodePayload | [37](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️🎬️unbind-scene-root-node/🦀️component.rs:37) |

## Exact Recoverable Inverse Files

- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-default-scene/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-morph-target-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-node-camera/↩️inverse/🦀️component.rs`: `GltfBindNodeCameraInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-node-child/↩️inverse/🦀️component.rs`: `GltfBindNodeChildInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-node-mesh/↩️inverse/🦀️component.rs`: `GltfBindNodeMeshInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-node-skin/↩️inverse/🦀️component.rs`: `GltfBindNodeSkinInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-primitive-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-primitive-indices/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-primitive-material/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/bind-scene-root-node/↩️inverse/🦀️component.rs`: `GltfBindSceneRootNodeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-asset-descriptive-metadata/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-asset-extension-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-asset-extra-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-asset-version/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-document-extension-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-document-extra-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-material-alpha-mode/↩️inverse/🦀️component.rs`: `GltfChangeMaterialAlphaModeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-material-double-sided/↩️inverse/🦀️component.rs`: `GltfChangeMaterialDoubleSidedInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-mesh-extension-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-mesh-extra-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-mesh-morph-weights/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-mesh-name/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-node-extension-data/↩️inverse/🦀️component.rs`: `GltfChangeNodeExtensionDataInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-node-extra-data/↩️inverse/🦀️component.rs`: `GltfChangeNodeExtraDataInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-node-morph-weights/↩️inverse/🦀️component.rs`: `GltfChangeNodeMorphWeightsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-node-name/↩️inverse/🦀️component.rs`: `GltfChangeNodeNameInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-primitive-extension-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-primitive-extra-data/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-primitive-topology-mode/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-scene-extension-data/↩️inverse/🦀️component.rs`: `GltfChangeSceneExtensionDataInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-scene-extra-data/↩️inverse/🦀️component.rs`: `GltfChangeSceneExtraDataInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/change-scene-name/↩️inverse/🦀️component.rs`: `GltfChangeSceneNameInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-accessor/↩️inverse/🦀️component.rs`: `GltfCreateAccessorInverseOperation`, `GltfCreateAccessorInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-animation/↩️inverse/🦀️component.rs`: `GltfCreateAnimationInverseOperation`, `GltfCreateAnimationInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-buffer-view/↩️inverse/🦀️component.rs`: `GltfCreateBufferViewInverseOperation`, `GltfCreateBufferViewInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-buffer/↩️inverse/🦀️component.rs`: `GltfCreateBufferInverseOperation`, `GltfCreateBufferInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-camera/↩️inverse/🦀️component.rs`: `GltfCreateCameraInverseOperation`, `GltfCreateCameraInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-image/↩️inverse/🦀️component.rs`: `GltfCreateImageInverseOperation`, `GltfCreateImageInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-material/↩️inverse/🦀️component.rs`: `GltfCreateMaterialInverseOperation`, `GltfCreateMaterialInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-mesh/↩️inverse/🦀️component.rs`: `GltfCreateMeshInverseOperation`, `GltfCreateMeshInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-morph-target/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-node/↩️inverse/🦀️component.rs`: `GltfCreateNodeInverseOperation`, `GltfCreateNodeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-primitive/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-sampler/↩️inverse/🦀️component.rs`: `GltfCreateSamplerInverseOperation`, `GltfCreateSamplerInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/↩️inverse/🦀️component.rs`: `GltfCreateSceneInversePhase`, `GltfCreateSceneInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-skin/↩️inverse/🦀️component.rs`: `GltfCreateSkinInverseOperation`, `GltfCreateSkinInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-texture/↩️inverse/🦀️component.rs`: `GltfCreateTextureInverseOperation`, `GltfCreateTextureInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/declare-used-extension/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-accessor/↩️inverse/🦀️component.rs`: `GltfDeleteAccessorInverseOperation`, `GltfDeleteAccessorInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-animation/↩️inverse/🦀️component.rs`: `GltfDeleteAnimationInverseOperation`, `GltfDeleteAnimationInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-buffer-view/↩️inverse/🦀️component.rs`: `GltfDeleteBufferViewInverseOperation`, `GltfDeleteBufferViewInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-buffer/↩️inverse/🦀️component.rs`: `GltfDeleteBufferInverseOperation`, `GltfDeleteBufferInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-camera/↩️inverse/🦀️component.rs`: `GltfDeleteCameraInverseOperation`, `GltfDeleteCameraInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-image/↩️inverse/🦀️component.rs`: `GltfDeleteImageInverseOperation`, `GltfDeleteImageInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-material/↩️inverse/🦀️component.rs`: `GltfDeleteMaterialInverseOperation`, `GltfDeleteMaterialInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-mesh/↩️inverse/🦀️component.rs`: `GltfDeleteMeshInverseOperation`, `GltfDeleteMeshInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-morph-target/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-node/↩️inverse/🦀️component.rs`: `GltfDeleteNodeInverseOperation`, `GltfDeleteNodeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-primitive/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-sampler/↩️inverse/🦀️component.rs`: `GltfDeleteSamplerInverseOperation`, `GltfDeleteSamplerInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-scene/↩️inverse/🦀️component.rs`: `GltfDeleteSceneInversePhase`, `GltfDeleteSceneInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-skin/↩️inverse/🦀️component.rs`: `GltfDeleteSkinInverseOperation`, `GltfDeleteSkinInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/delete-texture/↩️inverse/🦀️component.rs`: `GltfDeleteTextureInverseOperation`, `GltfDeleteTextureInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-accessor/↩️inverse/🦀️component.rs`: `GltfMoveAccessorInverseOperation`, `GltfMoveAccessorInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-animation/↩️inverse/🦀️component.rs`: `GltfMoveAnimationInverseOperation`, `GltfMoveAnimationInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-buffer-view/↩️inverse/🦀️component.rs`: `GltfMoveBufferViewInverseOperation`, `GltfMoveBufferViewInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-buffer/↩️inverse/🦀️component.rs`: `GltfMoveBufferInverseOperation`, `GltfMoveBufferInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-camera/↩️inverse/🦀️component.rs`: `GltfMoveCameraInverseOperation`, `GltfMoveCameraInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-image/↩️inverse/🦀️component.rs`: `GltfMoveImageInverseOperation`, `GltfMoveImageInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-material/↩️inverse/🦀️component.rs`: `GltfMoveMaterialInverseOperation`, `GltfMoveMaterialInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-mesh/↩️inverse/🦀️component.rs`: `GltfMoveMeshInverseOperation`, `GltfMoveMeshInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-morph-target-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-morph-target/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-node/↩️inverse/🦀️component.rs`: `GltfMoveNodeInverseOperation`, `GltfMoveNodeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-primitive-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-primitive/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-required-extension/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-sampler/↩️inverse/🦀️component.rs`: `GltfMoveSamplerInverseOperation`, `GltfMoveSamplerInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-scene/↩️inverse/🦀️component.rs`: `GltfMoveSceneInverseOperation`, `GltfMoveSceneInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-skin/↩️inverse/🦀️component.rs`: `GltfMoveSkinInverseOperation`, `GltfMoveSkinInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-texture/↩️inverse/🦀️component.rs`: `GltfMoveTextureInverseOperation`, `GltfMoveTextureInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/move-used-extension/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-accessors/↩️inverse/🦀️component.rs`: `GltfReorderAccessorsInverseOperation`, `GltfReorderAccessorsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-animations/↩️inverse/🦀️component.rs`: `GltfReorderAnimationsInverseOperation`, `GltfReorderAnimationsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-buffer-views/↩️inverse/🦀️component.rs`: `GltfReorderBufferViewsInverseOperation`, `GltfReorderBufferViewsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-buffers/↩️inverse/🦀️component.rs`: `GltfReorderBuffersInverseOperation`, `GltfReorderBuffersInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-cameras/↩️inverse/🦀️component.rs`: `GltfReorderCamerasInverseOperation`, `GltfReorderCamerasInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-images/↩️inverse/🦀️component.rs`: `GltfReorderImagesInverseOperation`, `GltfReorderImagesInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-materials/↩️inverse/🦀️component.rs`: `GltfReorderMaterialsInverseOperation`, `GltfReorderMaterialsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-meshs/↩️inverse/🦀️component.rs`: `GltfReorderMeshsInverseOperation`, `GltfReorderMeshsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-morph-target-attributes/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-morph-targets/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-nodes/↩️inverse/🦀️component.rs`: `GltfReorderNodesInverseOperation`, `GltfReorderNodesInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-primitive-attributes/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-primitives/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-required-extensions/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-samplers/↩️inverse/🦀️component.rs`: `GltfReorderSamplersInverseOperation`, `GltfReorderSamplersInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-scenes/↩️inverse/🦀️component.rs`: `GltfReorderScenesInverseOperation`, `GltfReorderScenesInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-skins/↩️inverse/🦀️component.rs`: `GltfReorderSkinsInverseOperation`, `GltfReorderSkinsInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-textures/↩️inverse/🦀️component.rs`: `GltfReorderTexturesInverseOperation`, `GltfReorderTexturesInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reorder-used-extensions/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/reparent-node/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/require-extension/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/transform-node/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-default-scene/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-morph-target-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-node-camera/↩️inverse/🦀️component.rs`: `GltfUnbindNodeCameraInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-node-child/↩️inverse/🦀️component.rs`: `GltfUnbindNodeChildInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-node-mesh/↩️inverse/🦀️component.rs`: `GltfUnbindNodeMeshInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-node-skin/↩️inverse/🦀️component.rs`: `GltfUnbindNodeSkinInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-primitive-attribute/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-primitive-indices/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-primitive-material/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unbind-scene-root-node/↩️inverse/🦀️component.rs`: `GltfUnbindSceneRootNodeInverse`.
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/unrequire-extension/↩️inverse/🦀️component.rs`: .
- `HEAD:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/withdraw-used-extension/↩️inverse/🦀️component.rs`: .

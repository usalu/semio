# glTF Authoritative Mutation Coverage Matrix

Ticket: `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`  
Scope: `GltfSnapshot` and every persisted `GltfDocument` subshape in `📸️snapshot/🦀️component.rs`

## Coverage rule

The old 28 commands cover only seven top-level collections and a few node relationships. They are not the domain boundary. Every command below has a stable `s.stdio.gltf.mutation.<slug>.v1` identity, a single cohesive payload, a direct sparse diff, an inverse, exact touched paths, and reference repair. `change-*` never carries a whole document, a whole object record, or an unrelated option bag: each payload names one semantic field group.

`schema` is a fixed artifact discriminator and cannot be mutated. `source_form` records the last codec dialect and cannot be changed by a semantic command. `GltfSnapshot.buffers[index]` is the byte payload of `document.buffers[index]`; it is changed only by `update-buffer-bytes` and is created/deleted/moved/reordered atomically with its owning buffer.

`create-*` payloads contain only a target position and the required construction invariants of an otherwise empty value (for example accessor layout, animation-sampler input/output, buffer-view backing buffer/layout, or camera projection). They never carry a cloned pre-existing record. All optional data is supplied by the named commands below.

## Document and top-level collections

| Authoritative shape | Required command families | Cohesive subshape coverage |
| --- | --- | --- |
| `asset` | `change-asset-version`, `change-asset-descriptive-metadata`, `change-asset-extension-data`, `change-asset-extra-data` | spec version, the coherent descriptive group (`generator`, `copyright`, `minVersion`), extension data, and extra data |
| default `scene` | `bind-default-scene`, `unbind-default-scene` | one explicit scene reference; deletion repairs or rejects it deterministically |
| `scenes` | `create-scene`, `delete-scene`, `move-scene`, `reorder-scenes` | scene metadata plus root-node relationship commands below |
| `nodes` | `create-node`, `delete-node`, `move-node`, `reorder-nodes` | node metadata, transform, weights, and typed links below |
| `meshes` | `create-mesh`, `delete-mesh`, `move-mesh`, `reorder-meshes` | mesh metadata, morph weights, primitives below |
| `accessors` | `create-accessor`, `delete-accessor`, `move-accessor`, `reorder-accessors` | accessor layout, bounds, sparse storage, metadata below |
| `bufferViews` | `create-buffer-view`, `delete-buffer-view`, `move-buffer-view`, `reorder-buffer-views` | buffer binding, layout, target, metadata below |
| `buffers` and aligned bytes | `create-buffer`, `delete-buffer`, `move-buffer`, `reorder-buffers` | descriptor fields and raw byte payload commands appear in the buffer section below |
| `materials` | `create-material`, `delete-material`, `move-material`, `reorder-materials` | PBR, texture links, render state, metadata below |
| `textures` | `create-texture`, `delete-texture`, `move-texture`, `reorder-textures` | sampler/image links and metadata below |
| `images` | `create-image`, `delete-image`, `move-image`, `reorder-images` | mutually-exclusive URI vs buffer-view source, MIME type, metadata below |
| `samplers` | `create-sampler`, `delete-sampler`, `move-sampler`, `reorder-samplers` | filters, wrapping, metadata below |
| `skins` | `create-skin`, `delete-skin`, `move-skin`, `reorder-skins` | inverse-bind/skeleton links, joints, metadata below |
| `animations` | `create-animation`, `delete-animation`, `move-animation`, `reorder-animations` | samplers, channels, metadata below |
| `cameras` | `create-camera`, `delete-camera`, `move-camera`, `reorder-cameras` | perspective or orthographic projection and metadata below |
| `extensionsUsed` | `declare-used-extension`, `withdraw-used-extension`, `move-used-extension`, `reorder-used-extensions` | declaration identity and stable order |
| `extensionsRequired` | `require-extension`, `unrequire-extension`, `move-required-extension`, `reorder-required-extensions` | requirement identity and stable order; `require` validates it is used |
| document extension/extra data | `change-document-extension-data`, `change-document-extra-data` | each is a single local `GltfJson` field, never a document replacement |

## Scene, node, and hierarchy

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| `scenes[i].nodes` | `bind-scene-root-node`, `unbind-scene-root-node`, `move-scene-root-node`, `reorder-scene-root-nodes` | no duplicate root in one scene; node deletion repairs/rejects every scene-root reference |
| `nodes[i].children` | `bind-node-child`, `unbind-node-child`, `move-node-child`, `reorder-node-children`, `reparent-node` | no duplicate child in one parent, no self-parent/cycle, deterministic position and reverse-link repair |
| node transform | `transform-node` | matrix and TRS exclusivity, finite scalar validation, only transform paths touched |
| `nodes[i].mesh` | `bind-node-mesh`, `unbind-node-mesh` | mesh index validation and deletion repair/rejection |
| `nodes[i].camera` | `bind-node-camera`, `unbind-node-camera` | camera index validation and deletion repair/rejection |
| `nodes[i].skin` | `bind-node-skin`, `unbind-node-skin` | skin index validation and deletion repair/rejection |
| `nodes[i].weights` | `change-node-morph-weights` | finite values; arity checked against bound mesh targets when present |
| node metadata | `change-node-name`, `change-node-extension-data`, `change-node-extra-data` | each field has independent diff/inverse |
| scene metadata | `change-scene-name`, `change-scene-extension-data`, `change-scene-extra-data` | each field has independent diff/inverse |

## Meshes, primitives, and morph targets

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| `meshes[i].primitives` | `create-primitive`, `delete-primitive`, `move-primitive`, `reorder-primitives` | indexed primitive range checks; mesh deletion owns nested removal |
| `meshes[i].weights` | `change-mesh-morph-weights` | finite values and target-count coherence |
| mesh metadata | `change-mesh-name`, `change-mesh-extension-data`, `change-mesh-extra-data` | independent field diffs |
| `primitive.attributes` | `bind-primitive-attribute`, `unbind-primitive-attribute`, `move-primitive-attribute`, `reorder-primitive-attributes` | semantic-key uniqueness and accessor reference repair |
| `primitive.indices` | `bind-primitive-indices`, `unbind-primitive-indices` | accessor type/index validation |
| `primitive.material` | `bind-primitive-material`, `unbind-primitive-material` | material reference repair on deletion |
| `primitive.mode` | `change-primitive-topology-mode` | glTF primitive-mode domain validation |
| `primitive.targets` | `create-morph-target`, `delete-morph-target`, `move-morph-target`, `reorder-morph-targets` | target position repair and target-count coherence |
| `morphTarget.attributes` | `bind-morph-target-attribute`, `unbind-morph-target-attribute`, `move-morph-target-attribute`, `reorder-morph-target-attributes` | semantic-key uniqueness and accessor repair |
| primitive metadata | `change-primitive-extension-data`, `change-primitive-extra-data` | independent field diffs |

## Accessors, buffers, and buffer views

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| accessor structural fields | `change-accessor-layout` | `bufferView`, `byteOffset`, component type, normalization, count, and accessor kind form one storage-layout value object |
| accessor bounds | `change-accessor-bounds` | min/max arity, finite values, and component-kind coherence |
| accessor sparse presence | `enable-accessor-sparse-storage`, `disable-accessor-sparse-storage`, `change-accessor-sparse-count` | enable owns the required sparse construction invariants; count and constituent references remain independently mutable |
| sparse index storage | `change-accessor-sparse-indices` | component/index constraints and buffer-view reference repair |
| sparse value storage | `change-accessor-sparse-values` | buffer-view reference repair |
| accessor metadata | `change-accessor-name`, `change-accessor-extension-data`, `change-accessor-extra-data` | independent field diffs |
| `bufferViews[i].buffer` | `bind-buffer-view-buffer` | this required relationship may be changed but never unbound; buffer deletion rejects live views |
| buffer-view layout | `change-buffer-view-layout` | offset/length/stride range and backing-buffer coherence |
| buffer-view target | `change-buffer-view-target` | target enum domain validation |
| buffer-view metadata | `change-buffer-view-name`, `change-buffer-view-extension-data`, `change-buffer-view-extra-data` | independent field diffs |
| buffer descriptor | `change-buffer-uri`, `change-buffer-name`, `change-buffer-extension-data`, `change-buffer-extra-data` | `byteLength` is derived/validated from bytes, never independently falsified |
| raw aligned bytes | `update-buffer-bytes` | byte length atomically maintained, direct bytes diff, inverse retains prior bytes |

## Materials, textures, images, and samplers

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| optional PBR value object | `enable-material-pbr-metallic-roughness`, `disable-material-pbr-metallic-roughness`, `change-material-pbr-factors`, `change-material-pbr-extension-data`, `change-material-pbr-extra-data` | the optional PBR object has explicit existence operations before its factor/data commands |
| PBR textures | `bind/unbind-material-base-color-texture`, `bind/unbind-material-metallic-roughness-texture` | each bind payload owns the cohesive texture-info field: texture index, texture coordinate, extension data, and extra data |
| normal texture | `bind/unbind-material-normal-texture`, `change-material-normal-texture-scale` | bind owns texture index, coordinate, extension data, and extra data; scale is independently finite |
| occlusion texture | `bind/unbind-material-occlusion-texture`, `change-material-occlusion-texture-strength` | bind owns texture index, coordinate, extension data, and extra data; strength is independently finite |
| emissive | `bind/unbind-material-emissive-texture`, `change-material-emissive-factor` | bind owns texture index, coordinate, extension data, and extra data; factor is independently finite |
| material render state | `change-material-alpha-mode`, `change-material-alpha-cutoff`, `change-material-double-sided` | alpha-mode domain and cutoff validation |
| material metadata | `change-material-name`, `change-material-extension-data`, `change-material-extra-data` | independent field diffs |
| `textures[i].sampler` | `bind-texture-sampler`, `unbind-texture-sampler` | sampler reference repair |
| `textures[i].source` | `bind-texture-image`, `unbind-texture-image` | image reference repair |
| texture metadata | `change-texture-name`, `change-texture-extension-data`, `change-texture-extra-data` | independent field diffs |
| image source | `bind-image-uri`, `bind-image-buffer-view`, `unbind-image-source` | URI/buffer-view mutual exclusion; a buffer-view bind carries its required MIME type; reference repair |
| image metadata | `change-image-mime-type`, `change-image-name`, `change-image-extension-data`, `change-image-extra-data` | independent field diffs |
| sampler filter | `change-sampler-filters` | min/mag filter domain validation |
| sampler wrap | `change-sampler-wrapping` | wrap enum domain validation |
| sampler metadata | `change-sampler-name`, `change-sampler-extension-data`, `change-sampler-extra-data` | independent field diffs |

## Skins, animations, and cameras

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| skin references | `bind/unbind-skin-inverse-bind-matrices`, `bind/unbind-skin-skeleton` | accessor/node reference repair |
| `skins[i].joints` | `bind-skin-joint`, `unbind-skin-joint`, `move-skin-joint`, `reorder-skin-joints` | node uniqueness/index validation and deletion repair |
| skin metadata | `change-skin-name`, `change-skin-extension-data`, `change-skin-extra-data` | independent field diffs |
| `animations[i].samplers` | `create-animation-sampler`, `delete-animation-sampler`, `move-animation-sampler`, `reorder-animation-samplers` | sampler indices repaired with every local move/delete |
| animation sampler fields | `change-animation-sampler-input`, `change-animation-sampler-output`, `change-animation-sampler-interpolation`, `change-animation-sampler-extension-data`, `change-animation-sampler-extra-data` | accessor references and interpolation domain validation |
| `animations[i].channels` | `create-animation-channel`, `delete-animation-channel`, `move-animation-channel`, `reorder-animation-channels` | channel sampler references repaired with local sampler changes |
| channel sampler link | `bind-animation-channel-sampler` | this required relationship may be changed but never unbound; sampler index validation and local reference repair |
| channel target | `bind/unbind-animation-channel-target-node`, `change-animation-channel-target-path`, `change-animation-channel-target-extension-data`, `change-animation-channel-target-extra-data` | node reference repair and target-path domain validation |
| channel metadata | `change-animation-channel-extension-data`, `change-animation-channel-extra-data` | independent field diffs |
| animation metadata | `change-animation-name`, `change-animation-extension-data`, `change-animation-extra-data` | independent field diffs |
| camera projection | `change-camera-perspective-projection`, `change-camera-orthographic-projection` | exactly one projection variant; each cohesive projection payload includes its own extension and extra data plus finite frustum constraints |
| camera metadata | `change-camera-name`, `change-camera-extension-data`, `change-camera-extra-data` | independent field diffs |

## Nested extension and extra-data audit

Every persisted `extensions` and `extras` field is covered without smuggling it through a record replacement. The direct pairs below each update exactly one `Option<GltfJson>` field: asset (except that `version` and descriptive metadata remain separate), scene, node, primitive, mesh, accessor, buffer view, buffer, material, texture, image, sampler, skin, animation sampler, animation channel target, animation channel, animation, camera, and document.

The remaining nested JSON-bearing values are covered by cohesive owners rather than generic data commands:

| Nested shape | Semantic owner | Data coverage |
| --- | --- | --- |
| `pbrMetallicRoughness` | `change-material-pbr-extension-data`, `change-material-pbr-extra-data` | its own two optional JSON fields |
| base-color, metallic-roughness, and emissive `GltfTextureInfo` | their respective `bind-material-*-texture` commands | initial texture index, coordinate, extensions, and extras form one relationship value; re-binding replaces only that relationship |
| `GltfNormalTextureInfo` | `bind-material-normal-texture`, `change-material-normal-texture-scale` | bind owns index, coordinate, extensions, extras; scale is isolated |
| `GltfOcclusionTextureInfo` | `bind-material-occlusion-texture`, `change-material-occlusion-texture-strength` | bind owns index, coordinate, extensions, extras; strength is isolated |
| `GltfPerspective`, `GltfOrthographic` | `change-camera-perspective-projection`, `change-camera-orthographic-projection` | each command atomically selects its projection variant and owns only that projection’s frustum fields, extensions, and extras |

## Payload boundary matrix

| Command shape | Payload boundary | Direct-diff boundary |
| --- | --- | --- |
| `create-*` | target collection and position plus only required construction invariants; no optional record clone | new element path and required aligned buffer bytes, if any |
| `delete-*` | target element only, with explicit repair policy selected by the command contract | deleted path plus every repaired typed reference |
| `move-*` | one target identity and one destination position | the source and destination ordering spans; all typed indices rebased deterministically |
| `reorder-*` | one collection/relation owner and a complete permutation of its current identities | that collection’s ordering only; all typed indices rebased deterministically |
| `bind-*` / `unbind-*` | one optional typed relationship, or one required relationship for bind-only links | exactly the relation path; unbind is unavailable where the underlying field is required |
| `transform-node` | exactly one discriminated transform value: matrix, or TRS | only matrix or TRS paths, clearing the mutually-exclusive representation |
| layout/value-object changes | one named cohesive value object: accessor layout/bounds, buffer-view layout, sparse indices/values, PBR factors, sampler filters/wrapping, or camera projection | only the constituent paths of that named value object |
| scalar and metadata changes | one named field or declared descriptive group; never an object plus unrelated `Option` fields | one field path or the declared descriptive group paths |
| `update-buffer-bytes` | one buffer identity and raw byte payload | its aligned `GltfSnapshot.buffers` entry and derived `byteLength` |

This boundary forbids payloads such as `GltfNode`, `GltfMaterial`, `GltfDocument`, or `GltfSnapshot` inside any `change-*` command. It also forbids a generic collection-position payload from being reused across unrelated entity families.

## Typed-reference repair matrix

All top-level `move-*`, `reorder-*`, and `delete-*` commands consult this matrix. A command either rebases every listed index in its sparse diff or returns a typed rejection naming the offending path; it never leaves a dangling or silently retargeted reference.

| Target collection | Typed reference paths to rebase, repair, or reject |
| --- | --- |
| scenes | `document.scene` |
| nodes | `scenes[*].nodes[*]`, `nodes[*].children[*]`, `skins[*].skeleton`, `skins[*].joints[*]`, `animations[*].channels[*].target.node` |
| meshes | `nodes[*].mesh` |
| accessors | `primitives[*].attributes[*]`, `primitives[*].indices`, `morphTargets[*].attributes[*]`, `skins[*].inverseBindMatrices`, `animations[*].samplers[*].input`, `animations[*].samplers[*].output` |
| buffer views | `accessors[*].bufferView`, `accessors[*].sparse.indices.bufferView`, `accessors[*].sparse.values.bufferView`, `images[*].bufferView` |
| buffers | `bufferViews[*].buffer`, plus aligned `GltfSnapshot.buffers[*]` |
| materials | `meshes[*].primitives[*].material` |
| textures | material base-color, metallic-roughness, normal, occlusion, and emissive texture-info `index` fields |
| images | `textures[*].source` |
| samplers | `textures[*].sampler` |
| skins | `nodes[*].skin` |
| animation samplers (nested per animation) | `animations[*].channels[*].sampler` in the same animation |

## Acceptance boundary

This matrix is the domain-shape coverage prerequisite for the canonical leaf inventory. Expanding every compound cell into separate stable `.v1` descriptors, then deduplicating the buffer detail rows that are cross-referenced from the top-level table, produces **222 canonical command leaves**. The prior 221 total was an arithmetic error: the literal animation rows contain 24 commands, so skins (11), animations (24), and cameras (5) total 40. No named semantic command is excluded to preserve a stale count.

| Domain group | Leaf commands |
| --- | ---: |
| document, top-level collections, extension declarations, and document data | 68 |
| scene roots, nodes, and hierarchy | 23 |
| meshes, primitives, and morph targets | 27 |
| accessors, sparse storage, buffer views, and buffers | 21 |
| materials, textures, images, and samplers | 43 |
| skins, animations, and cameras | 40 |
| **Total** | **222** |

The old 28-variant enum is neither a target count nor a compatibility constraint.

Every create/delete/move/reorder command repairs all typed index references or rejects with a typed path. Every bind/unbind command owns the exact relationship it changes. Every `change-*` command is limited to the named cohesive value object or one declared metadata field. The command root only assembles these descriptors and wire tags; it never examines payload fields or applies a diff.

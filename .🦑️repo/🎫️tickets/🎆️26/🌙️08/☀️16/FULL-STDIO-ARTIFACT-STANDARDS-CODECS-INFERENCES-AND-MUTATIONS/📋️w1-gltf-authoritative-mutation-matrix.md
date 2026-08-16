# glTF Authoritative Mutation Coverage Matrix

Ticket: `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`  
Scope: `GltfSnapshot` and every persisted `GltfDocument` subshape in `📸️snapshot/🦀️component.rs`

## Coverage rule

The old 28 commands cover only seven top-level collections and a few node relationships. They are not the domain boundary. Every command below has a stable `s.stdio.gltf.mutation.<slug>.v1` identity, a single cohesive payload, a direct sparse diff, an inverse, exact touched paths, and reference repair. `change-*` never carries a whole document, a whole object record, or an unrelated option bag: each payload names one semantic field group.

`schema` is a fixed artifact discriminator and cannot be mutated. `source_form` records the last codec dialect and cannot be changed by a semantic command. `GltfSnapshot.buffers[index]` is the byte payload of `document.buffers[index]`; it is changed only by `update-buffer-bytes` and is inserted/deleted/moved/reordered atomically with its owning buffer.

## Document and top-level collections

| Authoritative shape | Required command families | Cohesive subshape coverage |
| --- | --- | --- |
| `asset` | `change-asset-metadata` | `version`, `generator`, `copyright`, `minVersion`, extension data, extra data |
| default `scene` | `bind-default-scene`, `unbind-default-scene` | one explicit scene reference; deletion repairs or rejects it deterministically |
| `scenes` | `create/delete/move/reorder-scene` | scene metadata plus root-node relationship commands below |
| `nodes` | `create/delete/move/reorder-node` | node metadata, transform, weights, and typed links below |
| `meshes` | `create/delete/move/reorder-mesh` | mesh metadata, morph weights, primitives below |
| `accessors` | `create/delete/move/reorder-accessor` | accessor layout, bounds, sparse storage, metadata below |
| `bufferViews` | `create/delete/move/reorder-buffer-view` | buffer binding, layout, target, metadata below |
| `buffers` and aligned bytes | `create/delete/move/reorder-buffer`, `change-buffer-descriptor`, `update-buffer-bytes` | descriptor and raw byte payload are intentionally distinct |
| `materials` | `create/delete/move/reorder-material` | PBR, texture links, render state, metadata below |
| `textures` | `create/delete/move/reorder-texture` | sampler/image links and metadata below |
| `images` | `create/delete/move/reorder-image` | mutually-exclusive URI vs buffer-view source, MIME type, metadata below |
| `samplers` | `create/delete/move/reorder-sampler` | filters, wrapping, metadata below |
| `skins` | `create/delete/move/reorder-skin` | inverse-bind/skeleton links, joints, metadata below |
| `animations` | `create/delete/move/reorder-animation` | samplers, channels, metadata below |
| `cameras` | `create/delete/move/reorder-camera` | perspective or orthographic projection and metadata below |
| `extensionsUsed` | `declare/withdraw/move/reorder-used-extension` | declaration identity and stable order |
| `extensionsRequired` | `require/unrequire/move/reorder-required-extension` | requirement identity and stable order; `require` validates it is used |
| document extension/extra data | `change-document-extension-data`, `change-document-extra-data` | each is a single local `GltfJson` field, never a document replacement |

## Scene, node, and hierarchy

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| `scenes[i].nodes` | `bind-scene-root-node`, `unbind-scene-root-node`, `move-scene-root-node`, `reorder-scene-root-nodes` | one owner per node; no duplicate root; node deletion repairs/rejects every scene-root reference |
| `nodes[i].children` | `bind-node-child`, `unbind-node-child`, `move-node-child`, `reorder-node-children`, `reparent-node` | parent ownership, no self-parent/cycle, deterministic position and reverse-link repair |
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
| `meshes[i].primitives` | `create/delete/move/reorder-primitive` | indexed primitive range checks; mesh deletion owns nested removal |
| `meshes[i].weights` | `change-mesh-morph-weights` | finite values and target-count coherence |
| mesh metadata | `change-mesh-name`, `change-mesh-extension-data`, `change-mesh-extra-data` | independent field diffs |
| `primitive.attributes` | `bind-primitive-attribute`, `unbind-primitive-attribute`, `move-primitive-attribute`, `reorder-primitive-attributes` | semantic-key uniqueness and accessor reference repair |
| `primitive.indices` | `bind-primitive-indices`, `unbind-primitive-indices` | accessor type/index validation |
| `primitive.material` | `bind-primitive-material`, `unbind-primitive-material` | material reference repair on deletion |
| `primitive.mode` | `change-primitive-topology-mode` | glTF primitive-mode domain validation |
| `primitive.targets` | `create/delete/move/reorder-morph-target` | target position repair and target-count coherence |
| `morphTarget.attributes` | `bind-morph-target-attribute`, `unbind-morph-target-attribute`, `move-morph-target-attribute`, `reorder-morph-target-attributes` | semantic-key uniqueness and accessor repair |
| primitive metadata | `change-primitive-extension-data`, `change-primitive-extra-data` | independent field diffs |

## Accessors, buffers, and buffer views

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| accessor structural fields | `change-accessor-layout` | `bufferView`, `byteOffset`, component type, normalization, count, and accessor kind form one storage-layout value object |
| accessor bounds | `change-accessor-bounds` | min/max arity, finite values, and component-kind coherence |
| accessor sparse presence | `bind-accessor-sparse-storage`, `unbind-accessor-sparse-storage` | sparse count and buffer-view references |
| sparse index storage | `change-accessor-sparse-indices` | component/index constraints and buffer-view reference repair |
| sparse value storage | `change-accessor-sparse-values` | buffer-view reference repair |
| accessor metadata | `change-accessor-name`, `change-accessor-extension-data`, `change-accessor-extra-data` | independent field diffs |
| `bufferViews[i].buffer` | `bind-buffer-view-buffer`, `unbind-buffer-view-buffer` | buffer reference validation; unbind rejects invalid glTF layouts |
| buffer-view layout | `change-buffer-view-layout` | offset/length/stride range and backing-buffer coherence |
| buffer-view target | `change-buffer-view-target` | target enum domain validation |
| buffer-view metadata | `change-buffer-view-name`, `change-buffer-view-extension-data`, `change-buffer-view-extra-data` | independent field diffs |
| buffer descriptor | `change-buffer-uri`, `change-buffer-name`, `change-buffer-extension-data`, `change-buffer-extra-data` | `byteLength` is derived/validated from bytes, never independently falsified |
| raw aligned bytes | `update-buffer-bytes` | byte length atomically maintained, direct bytes diff, inverse retains prior bytes |

## Materials, textures, images, and samplers

| Authoritative shape | Required commands | Validation and repair responsibility |
| --- | --- | --- |
| PBR factors | `change-material-pbr-factors` | base color, metallic, roughness are one PBR factor value object; finite/range validation |
| PBR textures | `bind/unbind-material-base-color-texture`, `bind/unbind-material-metallic-roughness-texture` | texture index and texture-info validation |
| normal texture | `bind/unbind-material-normal-texture`, `change-material-normal-texture-scale` | texture reference and finite scale |
| occlusion texture | `bind/unbind-material-occlusion-texture`, `change-material-occlusion-texture-strength` | texture reference and finite strength |
| emissive | `bind/unbind-material-emissive-texture`, `change-material-emissive-factor` | texture reference and finite factor |
| material render state | `change-material-alpha-mode`, `change-material-alpha-cutoff`, `change-material-double-sided` | alpha-mode domain and cutoff validation |
| material metadata | `change-material-name`, `change-material-extension-data`, `change-material-extra-data` | independent field diffs |
| `textures[i].sampler` | `bind-texture-sampler`, `unbind-texture-sampler` | sampler reference repair |
| `textures[i].source` | `bind-texture-image`, `unbind-texture-image` | image reference repair |
| texture metadata | `change-texture-name`, `change-texture-extension-data`, `change-texture-extra-data` | independent field diffs |
| image source | `bind-image-uri`, `bind-image-buffer-view`, `unbind-image-source` | URI/buffer-view mutual exclusion, MIME requirement for buffer view, reference repair |
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
| `animations[i].samplers` | `create/delete/move/reorder-animation-sampler` | sampler indices repaired with every local move/delete |
| animation sampler fields | `change-animation-sampler-input`, `change-animation-sampler-output`, `change-animation-sampler-interpolation`, `change-animation-sampler-extension-data`, `change-animation-sampler-extra-data` | accessor references and interpolation domain validation |
| `animations[i].channels` | `create/delete/move/reorder-animation-channel` | channel sampler references repaired with local sampler changes |
| channel sampler link | `bind/unbind-animation-channel-sampler` | sampler index validation; unbind rejected if the channel would be invalid |
| channel target | `bind/unbind-animation-channel-target-node`, `change-animation-channel-target-path`, `change-animation-channel-target-extension-data`, `change-animation-channel-target-extra-data` | node reference repair and target-path domain validation |
| channel metadata | `change-animation-channel-extension-data`, `change-animation-channel-extra-data` | independent field diffs |
| animation metadata | `change-animation-name`, `change-animation-extension-data`, `change-animation-extra-data` | independent field diffs |
| camera projection | `change-camera-perspective-projection`, `change-camera-orthographic-projection` | exactly one projection variant; finite frustum constraints |
| camera metadata | `change-camera-name`, `change-camera-extension-data`, `change-camera-extra-data` | independent field diffs |

## Count and acceptance boundary

The matrix contains 13 top-level object collections with four collection commands each (52), two buffer descriptor/byte commands, 13 document-level commands, and 153 named nested/field commands: **220 canonical commands**. This is the authoritative surface to implement in bounded groups; the old 28-variant enum is neither a target count nor a compatibility constraint.

Every create/delete/move/reorder command repairs all typed index references or rejects with a typed path. Every bind/unbind command owns the exact relationship it changes. Every `change-*` command is limited to the named cohesive value object or one declared metadata field. The command root only assembles these descriptors and wire tags; it never examines payload fields or applies a diff.

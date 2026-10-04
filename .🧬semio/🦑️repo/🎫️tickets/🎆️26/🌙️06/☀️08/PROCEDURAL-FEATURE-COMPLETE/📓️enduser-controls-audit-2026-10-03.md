# End-User Controls Audit — 2026-10-03

Read-only Light source audit. Read widget-plan and current-integration reports. No tests, browser, heavy commands, production edits or git mutations. Source reachability does not prove assembled runtime success. Ticket remains open. Paths are repository-relative; aliases below identify exact owners.

- **I**: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- **C**: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🦀️.rs`
- **L**: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- **S**: `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`
- **B**: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`
- **F**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- **D**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`

## Material and Texture Authoring: Confirmed Gap, Existing Authority

I:253–263 admits only vertices/faces/attributes. C:148 rejects materials/textures paths. S:73–85 already owns PolygonMeshSource with both tables; S:90–115 parses and validates the complete canonical payload. C:120–140 resolves Construct Mesh data or its connected InputNote and publishes one existing absolute change_widget_input Text mutation. Extend these owners; no second model, state, evaluator or module is needed.

S:124–149 validates baseColor, metallic, roughness, alphaCutoff, occlusionStrength, normalScale, emissive, alphaMode and doubleSided; role-specific textureCoordinates and textureSamplers already exist. Five roles are baseColorTexture, metallicRoughnessTexture, normalTexture, occlusionTexture and emissiveTexture. Sampler constants already validate wrapS/wrapT, magFilter/minFilter. This is stronger than arbitrary opaque material JSON and should drive structured controls.

High slice: add bounded material/texture map rows in I with canonical named add/remove/rename operations in C. Add missing optional members explicitly through existing meshSource facet, because mesh_field_mut C:169–176 resolves only existing members. Material defaults and reference rename/removal must preserve face material assignments and every role reference atomically; validation must refuse dangling assets. Provide labeled coefficient/tuple controls and selects for roles, UV set, alphaMode and sampler enums. Keep texture MIME, byte length and import/replace action visible; never descend into individual image bytes as editable JSON rows. Texture byte admission should reuse the existing document IO/file invocation owner, then publish the same retained canonical source mutation. Root owns rich glTF and native IO coordination.

I:205–218 already labels controls, commits numeric/text edits on blur and renders checkbox changes. Reuse these and the existing export select pattern I:222–232. Extend L with all EN/DE labels; L:282/285 demonstrates localized custom attribute actions. Browser must verify accessible names, keyboard commit, error presentation, cancellation, undo/redo and downstream rendering. Source labels alone are not accessibility proof.

## Custom Attributes: Existing Feature, Domain Usability Caveat

I:277 offers number/text/boolean/vector alongside UV/normal/color. C:185–194 creates custom numeric scalar and three-component vector samples with linear interpolation; text/boolean use nearest. I:289–320 recursively exposes declared samples, domain, semantic, interpolation and indices. Renaming and add/remove/reorder exist. Do not claim custom scalar/vector attributes are absent.

Creation defaults custom channels to vertex domain; simply changing domain can fail cardinality unless counts coincide. A domain-aware add preset/selector in existing C/I should construct correctly sized face/edge/corner channels atomically. Add a material assignment preset using existing semantic validation rather than asking users to manipulate semantic strings and lengths manually. Avoid introducing generic raw JSON editing.

## Analysis Output: Existing Preview Is Real; Selected Inspection Still Missing

I:103–152 iterates only neuron inputs. I:161–163 reflects evaluation only for Variable. Selecting Analyze Mesh, Inspect Face or a BRep measurement therefore does not itself show calculated outputs in this panel.

Existing OutputPreview is sufficient machinery to display individual analysis results: F:1674–1687 copies connected evaluated output into preview and synchronizes DAG display; D:923–955 renders number/text/image or a structured Tree fallback. D:629 declares its sink port and D:295–297 records connected preview source. Scalar, vector, topology lists and report text are consequently representable without new analysis computation. This audit does not verify its current keyboard/TreeWindow expansion behavior in the assembled browser, and multiple measurement outputs require explicit preview wiring.

Recommended High slice: project the selected neuron's declared evaluated output ports into bounded read-only TreeWindow rows in I using the already supplied eval_json, localized port names and unavailable/error status. Display scalar/vector/list results without reparsing or recomputing geometry. Keep OutputPreview usable for graph-authored analysis. Test one multi-output analyze result, vector, topology list, unavailable and stale evaluation cases. If current root browser proves existing previews meet the complete requested inspection UX, record that evidence before treating this enhancement as mandatory.

## BRep Creation and Queries: Broad Roster, Distinguish Discovery from Capability

B:25–109 maps existing operators to kernel methods: primitives; line/circle/arc/ellipse/polyline/interpolation/approximation; planar wire faces, NURBS grid and Coons surfaces; extrude/revolve/loft/sweep/pipe; fuse/cut/intersect; transforms; selected-edge fillet/chamfer; section/split; curve/surface evaluations; mass/topology queries. B:1236–1246 declares point-grid inputs and boundary curve lists. B:1556–1569 exposes geometry plus selected edge lists. B:833–862 computes volume/area/center and validation through current kernel authority. The catalogue owner `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`:90–101 windows actual registered sections, rather than a primitive-only hardcoded palette.

I:127–139 edits typed collection and point/vector constructor inputs. Arbitrary point profiles can therefore be authored through existing polyline/interpolate and downstream planarFaceWire/sweeps; this is not limited to canned examples. Geometry-valued channels remain connection-only I:148–149, which is appropriate graph ownership but needs clear actionable port descriptions/examples. Prefer curated existing graph examples for profiles, surface boundaries, booleans, topology list extraction and mass measurements, with metadata parity tests; no new kernel is justified by discovery gaps.

No explicit trim operator appears in B's registered method roster. Do not claim arbitrary independent surface trim editing from successful boolean/section or kernel p-curve validation alone. Existing `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🦀️.rs`:1,41–45,92–98 resolves evaluated mesh component ids and topology, not an analytic BRep subshape editing contract. BRep topology decomposition/query list ports exist, but direct viewport BRep component selection feeding fillet/face operations needs assembled reachability evidence from the owning selected-BRep lane. Mesh selection success cannot serve as that evidence.

## Execution Handoff

Priority is existing-owner material/texture/assignment authoring and selected output projection, followed by domain-aware custom channels and discovery/selected-BRep runtime receipts. Keep GPU/mips with surface_execute, rich glTF/native IO with root, and retained jobs/tangents/provenance with mesh_jobs_continue. Add portable fixtures and third-party test oracle coverage before implementation, then run owned Bun/Nx and root's sole browser 6018 gates. No feature/ticket/goal completion is established by this source audit.

# Current Native Service Boundary Audit

Read-only production-source inspection on 2026-10-08. No native compilation or runtime tests were run during the shared kernel compile hold. Production files were inspected through `rg` and direct reads; candidate discovery excluded dedicated test folders and inspection separated inline test modules from production. This is a bounded source audit, not a claim that all native boundaries or runtime laws pass.

## Verified Artifact Dependencies Outside Pending Procedural Work

### Lowpoly Default Document Admission Still Lives In Schema

`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:83` embeds the authored `.mesh.json` source. Production `concrete_forest_left_owned_document` at line 94 calls `HalfedgeMesh::from_json` and line 96 calls `crate::snapshot_from_mesh_json`. `default_owned_document`, `default_snapshot`, and `default_mesh_workspace` invoke that constructor at lines 105–116. This is an indirect first-party physical codec dependency, even though the schema source contains no `serde_json::from_str`.

The target is verified: `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs:1352` implements `HalfedgeMesh::from_json` with first-party `pack_json::from_json_str(...Reject)`. Lowpoly `schema/📸️snapshot/🦀️.rs:40` builds a snapshot from physical JSON text, passes it to `mesh_child_handle`, and retains it as `mesh_content`. The pure construction from an already decoded mesh can remain semantic; authored JSON admission and source-retaining default assembly should be owned by snapshot text IO. Relocating mesh-engine codecs does not remove this distinct mesh-kernel JSON boundary.

### Terrain Inference Admits Bundled JSON Through An IO-Owned Inherent Method

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:66` calls `crate::schema::ImportedMap::from_json(include_str!(...scenery.json))` inside production `fixture_descriptor`. The implementation is correctly physically owned at `🚪️io/📝️text/📸️snapshot/🦀️.rs:231`, where it calls first-party JSON `parse`, converts to `DslValue`, and validates the map. The call still makes semantic inference dependent on physical admission. Its following origin and pin projections are semantic. Supply an already admitted typed bundled map to the projection, or place bundled media admission at IO assembly and retain the projection in inference. The inferred method receiver hides this dependency from searches restricted to explicit `io::` imports.

### Map Viewer Config Schema Performs Host JSON Projection

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🧬️schema/🦀️.rs:31–32` defines `GisMapViewerCamera::scene_camera_json` through `pack_json::to_json_string`. This is explicitly a host projection, with production callers in window `map/🦀️.rs:84` and viewer `🦀️.rs:118`. It is not evidence of document parsing or mutation authority. However its implementation resides in schema, so ownership is still wrong: the serializer should live in host/IO projection and consume the typed camera; `is_valid` belongs in schema.

## Pending Peer Work, Not Additional Ownership Claims

Procedural generation3d geometry inference currently imports mesh-engine physical IO: `💾️brep-interchange/🦀️.rs:9–10` imports OBJ and STL codecs; `📼️mesh-interchange/🦀️.rs:10–11` imports GLB/STL codecs and `ObjSourceCursor`; line 58 directly calls `encode_polygon_mesh_source`; `🥽️mesh-primitive/🦀️.rs:9` imports `PolygonSourcePreparation`. All paths share `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/`. HighPDF is assigned compute-context injection and interchange relocation, so these references are evidence for that active repair rather than a duplicate assignment.

Parent is relocating mesh-engine physical implementations to IO. The Lowpoly mesh-kernel inherent JSON call above is a separate target and is not covered automatically by that move. Replication fold grant custody is assigned to HighHost and was not re-audited here.

## Native Framework Service Boundaries Still In Schema

These are service/config wire boundaries rather than artifact interchange and may require a separate ownership pass. No blanket schema exemption is warranted.

- OS `🎚️config/🧬️schema/🦀️.rs:472–482,522–532` defines JSON admission/emission directly through first-party `pack_json`.
- OS config mutation leaves `🪪️sign-in/🦀️.rs:158–169`, `📥️admit-local-document/🦀️.rs:168–178`, `📎️attach-local-folder/🦀️.rs:166–176`, and `🛡️change-merge-policy/🦀️.rs:103–113` define physical JSON decoder/encoder functions beside semantic mutations. Their parent mutation module reexports these physical names. Common prefix: `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/`.
- OS directory `🔨️modules/📇️directory/🧬️schema/🦀️.rs` canonicalizes/parses native service wire payloads directly: event page lines 399–400; request lines 628,647; command receipt lines 710–722; administration page lines 1328–1340. Typed validation can remain semantic; exact JSON emission, byte limits on emitted JSON, and parse-plus-canonical-text comparison need service IO ownership.
- Directory schema leaves `🪪️session-authority-v1/🦀️.rs:62–72`, `🌱️space-artifact-creation-v1/🦀️.rs:111–121,155–156,282–283`, and `📌️document-check-in-v1/🦀️.rs:43–52,148–158` also own physical JSON admission/emission. Directory access policy `🛡️access-policy/🧬️schema/🦀️.rs:108` directly parses JSON.
- Space-history SQLite native snapshot `🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs:270` physically admits JSON with controlled first-party decoder. Its parent `🪶️sqlite/🦀️.rs:23–24` registers native snapshot codec assembly from schema. This is a concrete representation adapter currently under schema, not a semantic record conversion.

All service paths above are under `🧰️framework/🛍️products/💻️os/`. They were source-verified; no runtime assertion follows from this audit.

## Scope And Exclusions

Inline test helper JSON calls in Lowpoly mutation tests and FEM mutation tests are not production findings. Neutral `FromValue`/`ToValue`, `DslValue` binding helpers, native value retirement, and semantic validation do not become physical codecs merely because functions contain `decode` or `encode`. Dedicated IO mutation codecs are legitimate physical owners and were excluded from schema violation census.

The audit did not execute an AST-based exhaustive call graph. Direct production imports and the two verified inherent-method dependencies above establish concrete findings; absence from this bounded candidate list is not proof of complete separation. A follow-up ownership law should specifically detect schema calls into physical inherent methods and JSON host projection methods rather than rely only on `serde_json` token matching.

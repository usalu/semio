# Native307 Diagnostics

The full 63-package native all-target check completed with nonzero status. This run started before the map/graph/catalog edits and combined old dependency metadata with newer callers; those sites need a fresh build. Missing store imports in GIS/VCS integration tests were corrected in pass335. Other diagnostics remain under review.

{"error:E0425":1,"error:E0422":1,"failure-note:none":6,"error:E0433":10,"warning:unused_imports":2,"error:E0308":23}

```text
error[E0425]: cannot find type `DagPointerSnapshot` in module `dag`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️.rs:495:90
    |
495 |     pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerSnapshot, dag::DagInteractionPla...
    |                                                                                          ^^^^^^^^^^^^^^^^^^
    |
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2136:0
    |
    = note: similarly named struct `DagPointerIntent` defined here
help: a struct with a similar name exists
    |
495 -     pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerSnapshot, dag::DagInteractionPlanFault> {
495 +     pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerIntent, dag::DagInteractionPlanFault> {
    |

```

```text
error[E0422]: cannot find struct, variant or union type `DagPointerSnapshot` in module `dag`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️.rs:504:17
    |
504 | ...   Ok(dag::DagPointerSnapshot { node_ids: self.dag.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered...
    |               ^^^^^^^^^^^^^^^^^^
    |
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2136:0
    |
    = note: similarly named struct `DagPointerIntent` defined here
help: a struct with a similar name exists
    |
504 -         Ok(dag::DagPointerSnapshot { node_ids: self.dag.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered_id: self.dag.projection_hovered_id_ref(projection).map(str::to_owned), camera: projection.camera() })
504 +         Ok(dag::DagPointerIntent { node_ids: self.dag.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered_id: self.dag.projection_hovered_id_ref(projection).map(str::to_owned), camera: projection.camera() })
    |

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:36:24
   |
36 | ...   let declared = store::ArtifactCodec::of::<semio_s_plugin_vcs::artifacts::vcs::VcsSnapshot, semio_s_plugin_vcs::artifacts::vcs:...
   |                      ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`
help: consider importing one of these items
   |
 3 + use semio_framework::io::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::os_io::ArtifactCodec;
   |
 3 + use semio_framework_plugin::io::ArtifactCodec;
   |
help: if you import `ArtifactCodec`, refer to it directly
   |
36 -         let declared = store::ArtifactCodec::of::<semio_s_plugin_vcs::artifacts::vcs::VcsSnapshot, semio_s_plugin_vcs::artifacts::vcs::VcsDemoMutation>(identity.schema);
36 +         let declared = ArtifactCodec::of::<semio_s_plugin_vcs::artifacts::vcs::VcsSnapshot, semio_s_plugin_vcs::artifacts::vcs::VcsDemoMutation>(identity.schema);
   |

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:46:22
   |
46 | ...   let parsed = store::parse_document_pack::<semio_s_plugin_vcs::artifacts::vcs::VcsSnapshot, semio_s_plugin_vcs::artifacts::vcs:...
   |                    ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:29:25
   |
29 |     let (envelope, _) = store::semio_format::unwrap_binary(&pack).unwrap();
   |                         ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`
help: consider importing one of these items
   |
 3 + use framework_surface::tiled_map::os_store::semio_format;
   |
 3 + use semio_framework_os_kernel::os_store::semio_format;
   |
 3 + use semio_framework_os_kernel::semio_format;
   |
help: if you import `semio_format`, refer to it directly
   |
29 -     let (envelope, _) = store::semio_format::unwrap_binary(&pack).unwrap();
29 +     let (envelope, _) = semio_format::unwrap_binary(&pack).unwrap();
   |

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:52:26
   |
52 | ...   "gis.map" => store::ArtifactCodec::of::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMutation>(identity.schema),
   |                    ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`
help: consider importing one of these items
   |
 3 + use framework_surface::tiled_map::os_store::ArtifactCodec;
   |
 3 + use semio_framework::io::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::os_io::ArtifactCodec;
   |
   = and 1 other candidate
help: if you import `ArtifactCodec`, refer to it directly
   |
52 -             "gis.map" => store::ArtifactCodec::of::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMutation>(identity.schema),
52 +             "gis.map" => ArtifactCodec::of::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMutation>(identity.schema),
   |

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:53:30
   |
53 | ...   "gis.terrain" => store::ArtifactCodec::of::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis:...
   |                        ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`
help: consider importing one of these items
   |
 3 + use framework_surface::tiled_map::os_store::ArtifactCodec;
   |
 3 + use semio_framework::io::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::ArtifactCodec;
   |
 3 + use semio_framework_os_kernel::os_io::ArtifactCodec;
   |
   = and 1 other candidate
help: if you import `ArtifactCodec`, refer to it directly
   |
53 -             "gis.terrain" => store::ArtifactCodec::of::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis::artifacts::gisterrain::GisTerrainMutation>(identity.schema),
53 +             "gis.terrain" => ArtifactCodec::of::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis::artifacts::gisterrain::GisTerrainMutation>(identity.schema),
   |

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:70:46
   |
70 | ...   "gis.map" => assert_zero_history(store::parse_document_pack::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMut...
   |                                        ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:72:17
   |
72 | ...   store::parse_document_pack::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis::artifacts::gis...
   |       ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0433]: cannot find module or crate `store` in this scope
 --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:9:38
  |
9 | fn assert_zero_history<P, M>(parsed: store::ParsedDocumentText<P, M>, document_id: &str, dialect: &semio_framework::ArtifactDialect) {
  |                                      ^^^^^ use of unresolved module or unlinked crate `store`
  |
  = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:30:59
   |
30 |     assert_eq!(envelope.envelope_id(), <GisMapSnapshot as store::ArtifactDsl>::envelope_id());
   |                                                           ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0433]: cannot find module or crate `store` in this scope
  --> ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/../../📇️native-codecs/🧪️tests/🦀️.rs:31:59
   |
31 |     assert_ne!(envelope.envelope_id(), <GisMapSnapshot as store::ArtifactDsl>::EXTENSION);
   |                                                           ^^^^^ use of unresolved module or unlinked crate `store`
   |
   = help: if you wanted to use a crate named `store`, use `cargo add store` to add it to your `Cargo.toml`

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:100:38
    |
100 |     if list_os_space_catalog_entries(&port).map_or(true, |entries| entries.is_empty()) {
    |        ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |        |
    |        arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
100 -     if list_os_space_catalog_entries(&port).map_or(true, |entries| entries.is_empty()) {
100 +     if list_os_space_catalog_entries(port).map_or(true, |entries| entries.is_empty()) {
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:124:54
    |
124 |         let _ = seed_os_space_catalog_if_empty(seed, &port);
    |                 ------------------------------       ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1190:11
help: consider removing the borrow
    |
124 -         let _ = seed_os_space_catalog_if_empty(seed, &port);
124 +         let _ = seed_os_space_catalog_if_empty(seed, port);
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:259:64
    |
259 |         if let Ok(document) = load_os_space_document(space_id, &port) {
    |                               ----------------------           ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                               |
    |                               arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1180:11
help: consider removing the borrow
    |
259 -         if let Ok(document) = load_os_space_document(space_id, &port) {
259 +         if let Ok(document) = load_os_space_document(space_id, port) {
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:414:57
    |
414 |         if let Ok(rows) = list_os_space_catalog_entries(&port) {
    |                           ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                           |
    |                           arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
414 -         if let Ok(rows) = list_os_space_catalog_entries(&port) {
414 +         if let Ok(rows) = list_os_space_catalog_entries(port) {
    |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:25:92
   |
25 |     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, &port)?;
   |                 --------------- arguments to this function are incorrect                   ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1083:11
help: consider removing the borrow
   |
25 -     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, &port)?;
25 +     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, port)?;
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️delete-virtual-file-system-node/🦀️.rs:26:47
   |
26 |             let _ = delete_os_space(space_id, &semio_framework_plugin::resolve_ready(crate::catalog_port()));
   |                     ---------------           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                     |
   |                     arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1102:11
help: consider removing the borrow
   |
26 -             let _ = delete_os_space(space_id, &semio_framework_plugin::resolve_ready(crate::catalog_port()));
26 +             let _ = delete_os_space(space_id, semio_framework_plugin::resolve_ready(crate::catalog_port()));
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-space/🦀️.rs:21:46
   |
21 |             if import_os_space_from_dsl(dsl, &semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
   |                ------------------------      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                |
   |                arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1131:11
help: consider removing the borrow
   |
21 -             if import_os_space_from_dsl(dsl, &semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
21 +             if import_os_space_from_dsl(dsl, semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../⚙️engine/🪐️space/🎮️commands/🧳️import-space-pack-payload/🦀️.rs:24:63
   |
24 |         let _ = import_os_space_from_pack(&bytes, &empty_spr, &port);
   |                 -------------------------                     ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1150:11
help: consider removing the borrow
   |
24 -         let _ = import_os_space_from_pack(&bytes, &empty_spr, &port);
24 +         let _ = import_os_space_from_pack(&bytes, &empty_spr, port);
   |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:100:38
    |
100 |     if list_os_space_catalog_entries(&port).map_or(true, |entries| entries.is_empty()) {
    |        ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |        |
    |        arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
100 -     if list_os_space_catalog_entries(&port).map_or(true, |entries| entries.is_empty()) {
100 +     if list_os_space_catalog_entries(port).map_or(true, |entries| entries.is_empty()) {
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:124:54
    |
124 |         let _ = seed_os_space_catalog_if_empty(seed, &port);
    |                 ------------------------------       ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1190:11
help: consider removing the borrow
    |
124 -         let _ = seed_os_space_catalog_if_empty(seed, &port);
124 +         let _ = seed_os_space_catalog_if_empty(seed, port);
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:259:64
    |
259 |         if let Ok(document) = load_os_space_document(space_id, &port) {
    |                               ----------------------           ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                               |
    |                               arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1180:11
help: consider removing the borrow
    |
259 -         if let Ok(document) = load_os_space_document(space_id, &port) {
259 +         if let Ok(document) = load_os_space_document(space_id, port) {
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/../../🦀️.rs:414:57
    |
414 |         if let Ok(rows) = list_os_space_catalog_entries(&port) {
    |                           ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                           |
    |                           arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
414 -         if let Ok(rows) = list_os_space_catalog_entries(&port) {
414 +         if let Ok(rows) = list_os_space_catalog_entries(port) {
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:751:54
    |
751 |         let _ = seed_os_space_catalog_if_empty(demo, &port).expect("seed");
    |                 ------------------------------       ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1190:11
help: consider removing the borrow
    |
751 -         let _ = seed_os_space_catalog_if_empty(demo, &port).expect("seed");
751 +         let _ = seed_os_space_catalog_if_empty(demo, port).expect("seed");
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:752:61
    |
752 |         let loaded = load_os_space_document("persist-test", &port).expect("load");
    |                      ----------------------                 ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                      |
    |                      arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1180:11
help: consider removing the borrow
    |
752 -         let loaded = load_os_space_document("persist-test", &port).expect("load");
752 +         let loaded = load_os_space_document("persist-test", port).expect("load");
    |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:25:92
   |
25 |     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, &port)?;
   |                 --------------- arguments to this function are incorrect                   ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1083:11
help: consider removing the borrow
   |
25 -     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, &port)?;
25 +     let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, port)?;
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:90:52
   |
90 |         let before = list_os_space_catalog_entries(&port).expect("list").len();
   |                      ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                      |
   |                      arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
   |
90 -         let before = list_os_space_catalog_entries(&port).expect("list").len();
90 +         let before = list_os_space_catalog_entries(port).expect("list").len();
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:93:51
   |
93 |         let after = list_os_space_catalog_entries(&port).expect("list").len();
   |                     ----------------------------- ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                     |
   |                     arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
   |
93 -         let after = list_os_space_catalog_entries(&port).expect("list").len();
93 +         let after = list_os_space_catalog_entries(port).expect("list").len();
   |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:107:56
    |
107 |         let persistent = list_os_space_catalog_entries(&crate::catalog_port().await).expect("list");
    |                          ----------------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                          |
    |                          arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
107 -         let persistent = list_os_space_catalog_entries(&crate::catalog_port().await).expect("list");
107 +         let persistent = list_os_space_catalog_entries(crate::catalog_port().await).expect("list");
    |

```

```text
error[E0308]: mismatched types
   --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:109:63
    |
109 |         let ephemeral_catalog = list_os_space_catalog_entries(&crate::temp_catalog_port().await).unwrap_or_default();
    |                                 ----------------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
    |                                 |
    |                                 arguments to this function are incorrect
    |
    = note: expected struct `std::sync::Arc<_>`
            found reference `&std::sync::Arc<_>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1059:11
help: consider removing the borrow
    |
109 -         let ephemeral_catalog = list_os_space_catalog_entries(&crate::temp_catalog_port().await).unwrap_or_default();
109 +         let ephemeral_catalog = list_os_space_catalog_entries(crate::temp_catalog_port().await).unwrap_or_default();
    |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️delete-virtual-file-system-node/🦀️.rs:26:47
   |
26 |             let _ = delete_os_space(space_id, &semio_framework_plugin::resolve_ready(crate::catalog_port()));
   |                     ---------------           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                     |
   |                     arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1102:11
help: consider removing the borrow
   |
26 -             let _ = delete_os_space(space_id, &semio_framework_plugin::resolve_ready(crate::catalog_port()));
26 +             let _ = delete_os_space(space_id, semio_framework_plugin::resolve_ready(crate::catalog_port()));
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-space/🦀️.rs:21:46
   |
21 |             if import_os_space_from_dsl(dsl, &semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
   |                ------------------------      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                |
   |                arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1131:11
help: consider removing the borrow
   |
21 -             if import_os_space_from_dsl(dsl, &semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
21 +             if import_os_space_from_dsl(dsl, semio_framework_plugin::resolve_ready(crate::catalog_port())).is_ok() {
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../⚙️engine/🪐️space/🎮️commands/🧳️import-space-pack-payload/🦀️.rs:24:63
   |
24 |         let _ = import_os_space_from_pack(&bytes, &empty_spr, &port);
   |                 -------------------------                     ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1150:11
help: consider removing the borrow
   |
24 -         let _ = import_os_space_from_pack(&bytes, &empty_spr, &port);
24 +         let _ = import_os_space_from_pack(&bytes, &empty_spr, port);
   |

```

```text
error[E0308]: mismatched types
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🦀️.rs:46:106
   |
46 |         let entry = create_os_space("Opened Empty", SpaceKind::Atelier, SpaceVisibility::Private, owner, &port).expect("create");
   |                     --------------- arguments to this function are incorrect                             ^^^^^ expected `Arc<OsBackbonePorts>`, found `&Arc<OsBackbonePorts>`
   |
   = note: expected struct `std::sync::Arc<_>`
           found reference `&std::sync::Arc<_>`
note: function defined here
  --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️.rs:1083:11
help: consider removing the borrow
   |
46 -         let entry = create_os_space("Opened Empty", SpaceKind::Atelier, SpaceVisibility::Private, owner, &port).expect("create");
46 +         let entry = create_os_space("Opened Empty", SpaceKind::Atelier, SpaceVisibility::Private, owner, port).expect("create");
   |

```

```text
warning: unused import: `HistoryView`
  --> ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/././../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:11:245
   |
11 | ...onfigView, EditorApp, Emit, Fault, FaultCode, FaultOrigin, HistoryView};
   |                                                               ^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

```

```text
warning: unused import: `HistoryView`
  --> ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/././../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:11:245
   |
11 | ...onfigView, EditorApp, Emit, Fault, FaultCode, FaultOrigin, HistoryView};
   |                                                               ^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

```

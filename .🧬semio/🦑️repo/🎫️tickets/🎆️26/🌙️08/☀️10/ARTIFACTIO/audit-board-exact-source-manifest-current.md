# Exact Current Board Boundary Audit Manifest

Read-only source audit; no runtime execution. Inventory positions checked against present signatures. Lexical caller rows retained as evidence, never counted as resolved calls.

## 1. apply_force_graph_layout_to_board_snapshot_value
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:932`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `3107823242009c0fdf2a7f4b23e346d71e6b89455e0ed4f23c4803fdc8596928`.
- Current signature: `pub fn apply_force_graph_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:946:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1713:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:343:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:398:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`

## 2. apply_force_graph_layout_to_board_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:943`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `cfcc8cadb0558d865a8dfb61e303364af62f8c867ec24e9c20ef203414c350c4`.
- Current signature: `pub fn apply_force_graph_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:37:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:77:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:179:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:189:    let err = apply_force_graph_layout_to_board_snapshot_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:231:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:285:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:286:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:316:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:317:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:346:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚛️force-layout/🦀️.rs:8:    let Ok(layout_json) = crate::editor::puzzle2d::engine::apply_force_graph_layout_to_board_snapshot_json(&ctx.scene.board_snapshot.to_string(), r#"{"mode":"force-graph"}"#) else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:36:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:64:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`

## 3. apply_hierarchical_tree_layout_to_board_snapshot_value
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1311`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `fcc94ceaef1cf7a8a42fc1a0b67e6536d619cadb20b64c1a7b9748bf03ad0f80`.
- Current signature: `pub fn apply_hierarchical_tree_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &HierarchicalTreeLayoutOptions) -> Result<(), String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1728:                apply_hierarchical_tree_layout_to_board_snapshot_value(&mut snapshot, &hierarchical_opts)?;`

## 4. apply_edge_handle_snap_to_board_snapshot_value
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1570`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `49ce431d561b6ae752a2d5b6be8bdcecea0b17645783651e6217e1be5d94777e`.
- Current signature: `pub fn apply_edge_handle_snap_to_board_snapshot_value(snapshot: &mut Value) -> Result<(), String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1661:        apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1733:            apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;`

## 5. apply_edge_handle_snap_to_board_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1659`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `26a97cf4130a7ef222e2e6974e7ec4f6a86df1601288b4cb7dc562f39e59057e`.
- Current signature: `pub fn apply_edge_handle_snap_to_board_snapshot_json(snapshot_json: &str) -> Result<String, String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:419:    let out = apply_edge_handle_snap_to_board_snapshot_json(&snapshot.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:71:    apply_edge_handle_snap_to_board_snapshot_json(snapshot_json).map_err(|e| JsValue::from_str(&e))`

## 6. apply_redraw_layout_to_board_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1693`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `f9e541990dc275d8dac15efe1b888c0ac9a4c12c82bf463df7bc22b6be959dac`.
- Current signature: `pub fn apply_redraw_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, String> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- None in inventory.

## 7. puzzle_2d_lod_scale_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:197`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `8d154e73d938a7ef62b15c9673b23ba3fdd5ea541ff57b8b633169c2e1943202`.
- Current signature: `pub fn puzzle_2d_lod_scale_json() -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️lod-scale-json/🦀️.rs:9:    let _ = crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs:11:    semio_framework_pack_json::from_json_str::<Vec<Value>>(&crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:327:        puzzle_2d_lod_scale_json()`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs:12:    semio_framework_pack_json::from_json_str::<Vec<Value>>(&semio_framework_os_infinite::puzzle_2d_lod_scale_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned LOD options admit").into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()`

## 8. payload_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1869`; receiver: `BoardOwnedEvent`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `aad1214393ffe95f4ac4f2a521686a56c345e9c4c8d85056306e857a81cfa75c`.
- Current signature: `pub fn payload_json(&self) -> &str {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs:161:    let document = payload_json(semio_framework_pack_json::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5405:    serde_json::from_str::<Value>(event.payload_json()).ok()?.get("gestureId")?.as_str().map(str::to_string)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5456:    let camera = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).filter_map(|event| engine_camera_from_json(event.payload_json())).last().map(|(x, y, zoom)| [x, y, zoom]);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1885:            output.push_str(self.payload_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3587:            self.output_raw(event.payload_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3601:                self.output_raw(event.payload_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:315:            assert!(event.payload_json().contains(&format!("node-{index}")));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:335:        let payload: serde_json::Value = serde_json::from_str(event.payload_json()).unwrap();`

## 9. write_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1881`; receiver: `BoardOwnedEvent`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `e26f799a90e412f45a30bfac720ddba8471b0e3fe0678b88dd4b22c4547801e9`.
- Current signature: `pub fn write_json(&self, output: &mut String) {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1239:        event.write_json(&mut output);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:329:            event.write_json(&mut row);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5438:    event.write_json(output);`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️.rs:1875:        let text = write_json(&self.to_json().await).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:97:    let written = write_json(&value).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:160:    let written = write_json(&value).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:163:    assert_eq!(write_json(&JsonValue::Num(3.0)).await, "3");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8174:                event.write_json(&mut out);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8180:                event.write_json(&mut out);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:230:                    Ok(crate::io_schema::IoPayload::Text(write_json(value.get(), &mut c)?))`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:1234:                    self.write_json(&edge, Process3dTimelinePhase::Edges(index + 1));`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:1248:                    self.write_json(&node, Process3dTimelinePhase::Nodes { index: index + 1, tool: tool + usize::from(has_tool) });`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:108:    write_json(&read_json(bytes)?)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:206:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:218:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:232:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:245:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:255:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:257:        "restore-snapshot" => write_json(&library_from_wire(params.get("snapshot").and_then(|snapshot| snapshot.get("value")).unwrap_or(&Json::Null))?),`

## 10. events_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3361`; receiver: `BoardPointerPublication`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `70a4ab83543c8dcc45a3696a092e435c9030f94f832ce45ca6102d11674eca9f`.
- Current signature: `pub fn events_json(&self) -> &str {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5707:    let dispatch = board_page_dispatch_rows(plan.events_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5836:        let Some(events_json) = board_page_dispatch_rows(publication.events_json())? else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:170:        let value: serde_json::Value = serde_json::from_str(escaped.expect("escaped release plan").events_json()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:208:        let rows: serde_json::Value = serde_json::from_str(publication.events_json()).expect("publication rows");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:236:        assert!(preview.events_json().contains("preselect"));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:249:        assert!(commit.events_json().contains("select"));`

## 11. events_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3607`; receiver: `BoardPointerPlan`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `ffc1d5dfa24b608494085a64ee82f2e13cf4158b66194b22800f5509eb578b3e`.
- Current signature: `pub fn events_json(&self) -> &str {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5707:    let dispatch = board_page_dispatch_rows(plan.events_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5836:        let Some(events_json) = board_page_dispatch_rows(publication.events_json())? else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:170:        let value: serde_json::Value = serde_json::from_str(escaped.expect("escaped release plan").events_json()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:208:        let rows: serde_json::Value = serde_json::from_str(publication.events_json()).expect("publication rows");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:236:        assert!(preview.events_json().contains("preselect"));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:249:        assert!(commit.events_json().contains("select"));`

## 12. overlay_paint_state_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3757`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `a4c3f99fde2efcd783236a40bfbed4b4a9098a4dc8086b943effbb714c671dc9`.
- Current signature: `pub fn overlay_paint_state_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:407:    let raw: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&h.overlay_paint_state_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("overlay paint state json");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:241:    let overlay: Value = semio_framework_pack_json::from_json_str(&host.overlay_paint_state_json(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(Value::Null);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:307:        self.state.borrow().host.overlay_paint_state_json()`

## 13. set_external_link_preview_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4335`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `928b88c174e2ff6c40da93704ee2361c9ab8fb16dac40c8694b9ff65a2af7700`.
- Current signature: `pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:451:        self.state.borrow_mut().host.set_external_link_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`

## 14. set_handle_link_compat_from_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4423`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `25fe493dedb06b58980a76ae1ead774b898273887d6f70102fb96548bb898061`.
- Current signature: `pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1166:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1214:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1260:        h.set_handle_link_compat_from_json(&compat_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1342:        h.set_handle_link_compat_from_json(&compat_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1453:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:15:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:35:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:90:        h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:381:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:489:        h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:511:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:545:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:560:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:587:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:616:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:635:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:649:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:668:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:759:        h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:818:        h.set_handle_link_compat_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:847:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:884:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1152:            let _ = host.set_handle_link_compat_from_json(&json);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:317:        self.state.borrow_mut().host.set_handle_link_compat_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3129:        let _ = host.set_handle_link_compat_from_json(&board.placement_compatibility_json);`

## 15. set_board_kind_catalogs_from_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4460`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `464dfb2fdd55875ab11848f36b40b4c1ac2e9fda62e84b7d5477d94812bc4de3`.
- Current signature: `pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:286:        host.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:323:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:397:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:443:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:511:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:545:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:576:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:615:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:667:        h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({ "handleKinds": [{ "id": "child", "name": "Child", "color": "#888888" }], "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "child", "angle": 0.0 }] }] }).to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1094:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1122:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1167:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1215:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1261:        h.set_board_kind_catalogs_from_json(&catalogs_json_from_manifest_id("nakagin")).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1353:        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1421:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1454:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1530:        host.set_board_kind_catalogs_from_json(catalogs_json).expect("catalog push must be accepted by the engine");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1558:        BoardHost::new().set_board_kind_catalogs_from_json(&catalogs).expect("manifest catalogs must satisfy the engine contract");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1607:        catalog_host.set_board_kind_catalogs_from_json(&catalogs).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:79:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:242:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:727:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:751:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:780:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:801:        let err = h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:810:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:338:    host.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1148:        let _ = host.set_board_kind_catalogs_from_json(&json);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:206:        self.state.borrow_mut().host.set_board_kind_catalogs_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3124:        let _ = host.set_board_kind_catalogs_from_json(&board.glyph_catalogs_json);`

## 16. set_brush_session_mirror_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7525`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `f4c2cb00629133186ec96dab282be4e9fe9100e14d87887e2f9b97cb050f4f76`.
- Current signature: `pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1152:        h.set_brush_session_mirror_json(&session.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:431:        self.state.borrow_mut().host.set_brush_session_mirror_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:436:        let _ = self.state.borrow_mut().host.set_brush_session_mirror_json("");`

## 17. set_drop_preview_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7906`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `45258b80e5be5a8e4e09902b49a7a41e97683d04c20c368a18a9fea2a62e3110`.
- Current signature: `pub fn set_drop_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1080:        h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1084:        h.set_drop_preview_json("").unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1108:        h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1111:        h.set_drop_preview_json("").unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:441:        self.state.borrow_mut().host.set_drop_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:446:        let _ = self.state.borrow_mut().host.set_drop_preview_json("");`

## 18. drain_events_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8166`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `87bb87b3f603da67e1e65bca628a40559dd53ded1d04b19bcaeb4607f577b12d`.
- Current signature: `pub fn drain_events_json(&mut self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:370:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:373:        let ev2 = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:413:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:416:        let fastened = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:419:        assert!(!h.drain_events_json().contains("brushPlace"), "an empty page must place nothing");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:422:        let free = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:487:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:495:        let ev_commit = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:498:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:500:        let ev_cancel = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:527:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:530:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:534:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:561:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:564:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:566:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1081:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1135:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1153:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1193:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1196:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1234:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1237:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1396:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1401:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1437:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1440:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1470:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1476:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1532:        let _ = host.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1536:        let events: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&host.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("board events json");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:17:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:24:        let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:37:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:44:        let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:55:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:65:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:172:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:184:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:195:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:199:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:209:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:361:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:371:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:405:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:412:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:458:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:468:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:492:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:502:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:514:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:524:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:547:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:562:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:575:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:589:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:604:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:637:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:670:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:676:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:684:        let ev_end = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:762:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:772:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:827:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:837:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:849:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:859:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:871:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:876:        assert!(!h.drain_events_json().contains("edgeCreate"));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:886:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:900:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:903:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:915:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:921:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:347:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:352:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:362:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:368:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:372:    let end: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:458:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:461:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:465:    let ev2 = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:490:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:492:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:497:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:500:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:654:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:660:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:693:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:700:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:736:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:743:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:786:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:793:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:857:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:934:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:982:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:988:    let mid = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:996:    let fin = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1063:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1104:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1106:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1115:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1120:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1127:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1130:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1165:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1172:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1178:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1227:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1231:    let _ = h.drain_events_json();`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs:4077:        self.state.borrow_mut().host.drain_events_json()`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs:563:    let events: Vec<serde_json::Value> = serde_json::from_str(&host.drain_events_json()).expect("events");`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs:1090:    assert_eq!(direct.drain_events_json(), planned.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:900:        let events = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:952:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:958:    let during = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:962:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:982:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:989:        frames += board_event_names(&host.drain_events_json()).len();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:994:    assert_eq!(board_event_names(&host.drain_events_json()), vec!["gesture".to_string()], "and the whole gesture commits once");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1005:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1054:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1062:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1075:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1084:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1113:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1119:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1130:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1142:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1152:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1162:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1296:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1300:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a paint frame announces nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1303:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1320:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1323:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1331:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a rectangle under the extent floor commits nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1341:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1343:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "the press stages its selection and publishes nothing yet");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1345:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a region drag frame announces nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1347:    let moved = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1357:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1362:    let resized = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1371:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1374:    assert_eq!(board_event_names(&host.drain_events_json()), vec!["select".to_string()], "a release that moved nothing publishes only the selection");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1378:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1383:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "and publishes nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1395:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1399:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1406:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1410:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1420:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1424:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1427:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1462:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1466:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1574:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1581:    serde_json::from_str::<Vec<serde_json::Value>>(&host.drain_events_json()).expect("events parse").into_iter().filter(|row| row["name"] != "hover").collect()`

## 19. interaction_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8966`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `01efa66d101da21f7c9698c6957159f7f94078cdd4742ba8803ea8c302055121`.
- Current signature: `pub fn interaction_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:379:        self.state.borrow().host.interaction_json()`

## 20. handle_positions_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:9025`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `dd83afb4c0ad58ffc7d67cecdd6326ed8e2344f04dffd7edffad871dadc2ae2d`.
- Current signature: `pub fn handle_positions_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:386:        self.state.borrow().host.handle_positions_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1201:    let json = host.handle_positions_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1208:    let row = host.handle_positions_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1212:    let away = host.handle_positions_json();`

## 21. pick_targets_at_screen_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:9863`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `f4e845c21f3db777cfc693f53b2d3b5511fd5f67c116c846036fb0314c641ace`.
- Current signature: `pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:251:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🦀️.rs:1423:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:668:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:681:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:145:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| host.pick_targets_at_screen_json(0.0, 0.0))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4199:        NodeGraphEngine::Flow(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4200:        NodeGraphEngine::Dag(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5369:        let json = host.pick_targets_at_screen_json(sx, sy);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6191:        let hits: Vec<Value> = serde_json::from_str(&host.pick_targets_at_screen_json(sx, sy)).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6255:    let hits = with_board_host(surface_id, |host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6267:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6294:            Some((pick_target_hits(&host.pick_targets_at_screen_json(sx, sy)), Some(text)))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:544:        self.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:865:            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:564:    let json = host.pick_targets_at_screen_json(200.0, 200.0);`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:1267:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:116:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(300.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:134:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(200.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:142:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(0.0, 0.0)).expect("json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1442:    let targets = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-260.0, 0.0)).x, host.world_to_screen(Point::new(-260.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1444:    let visible = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-50.0, 0.0)).x, host.world_to_screen(Point::new(-50.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4527:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.pick_targets_at_screen_json(number(args, "sx")?, number(args, "sy")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1797:        self.dag.pick_targets_at_screen_json(sx, sy)`

## 22. set_node_positions_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:10259`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `8d349fb403e725eae3912b483285f5756984ac4a90cf7aacd2f0c2c8a1618dbc`.
- Current signature: `pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:391:    h.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:201:        self.state.borrow_mut().host.set_node_positions_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`

## 23. highlighted_ids_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:11201`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `238cacf8af5902748adf33976263e2c9b684b9f9c5630634ac07da289801004f`.
- Current signature: `pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:111:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:115:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#, "a new preview keeps what the draft references");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:118:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), "[]", "a closed draft highlights nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1716:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["node-a","node-b","node-locked"]"#, "a preview repaint keeps what the draft references");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1719:    assert_eq!((style(&host, "node-a"), host.highlighted_ids_json().expect("highlighted ids")), (BoardElementStyleKind::Neutral, "[]".to_string()), "the empty set clears it");`

## 24. transform_gumball_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:13180`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `6da46939d221faa3dfc5405f8d62502c73e9ccb5c2c68d96f1e72db8f87270c1`.
- Current signature: `pub fn transform_gumball_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:208:    let gumball: Value = semio_framework_pack_json::from_json_str(&host.transform_gumball_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("gumball vitals parse");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:360:        self.state.borrow().host.transform_gumball_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1176:    assert!(host.transform_gumball_json().contains("\"ringVisible\":false"), "and the vitals say so: {}", host.transform_gumball_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1188:    assert!(host.transform_gumball_json().contains("\"rotate\":true"), "the vitals name the composed handles: {}", host.transform_gumball_json());`

## 25. target_regions_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:13372`; receiver: `BoardHost`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: Board/🚪️io/📝️text; typed Board/🧬️schema fact record + pure host method.
- Inventoried body SHA-256: `1bb291d9a55415adc6f5cb87068998d5c831cf7d094752d08df1ca5402d680ca`.
- Current signature: `pub fn target_regions_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:373:        self.state.borrow().host.target_regions_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1277:    let published = host.target_regions_json();`

## 26. apply_dag_layout_to_host_snapshot_v1_value
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1175`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `542e0328ead91019e0a40284bfc04926102877ae39e5718d39bec51557a90ee8`.
- Current signature: `pub fn apply_dag_layout_to_host_snapshot_v1_value(snapshot: &mut Value, opts: &DagLayoutOptions) -> Result<(), DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4837:        apply_dag_layout_to_host_snapshot_v1_value(&mut snapshot_value, opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4861:            let _ = apply_dag_layout_to_host_snapshot_v1_value(&mut snapshot_value, &DagLayoutOptions::default());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:165:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:175:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &opts).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:184:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:187:    apply_dag_layout_to_host_snapshot_v1_value(&mut wide, &DagLayoutOptions { layer_spacing: 240.0, sibling_gap: 80.0, ..DagLayoutOptions::default() }).unwrap();`

## 27. dag_lod_scale_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1505`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `9d392fc17398f70e0025d3af5b1f0ecb4fd7889e753ad012f120a2b4457113e2`.
- Current signature: `pub fn dag_lod_scale_json() -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:78:            SEQUENCE_OPERATION_LOD_SCALE => Ok(infinite_board_port_directed_dag::board::ports::directed_dag::dag_lod_scale_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:829:            dag::dag_lod_scale_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7028:            dag_lod_scale_json()`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🦀️.rs:14:    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:3907:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(dag::dag_lod_scale_json().into_bytes()) };`

## 28. dag_graph_edit_rows_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2207`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `0c8befbb47b1ddceb92ca2d68021ecd310bae3fca4abe33fb1cc526c10c9a1e8`.
- Current signature: `pub fn dag_graph_edit_rows_json(edits: &[DagGraphEdit], refused: Option<DagJournalRefusal>) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts:169:    expect(answerBody).toContain("dag::dag_graph_edit_rows_json(&edits, refusal)");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:513:        dag::dag_graph_edit_rows_json(&edits, refusal)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:68:    let rows: Value = serde_json::from_str(&dag_graph_edit_rows_json(edits, None)).expect("journal rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:45:    let encoded: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:47:    assert_eq!(dag_graph_edit_rows_json(&[], None), r#"{"operations":[]}"#, "an empty journal is no row");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:116:    let answer: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:127:    let refused: Value = serde_json::from_str(&dag_graph_edit_rows_json(&[], Some(refusal))).expect("refusal json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1328:        dag::dag_graph_edit_rows_json(&edits, refusal)`

## 29. selection_domains_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3617`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `31cb70df65219d28c218472e23a4a9711738d7355a1b037319d4bc4e325b5579`.
- Current signature: `pub fn selection_domains_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1867:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_domains_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1787:        self.dag.selection_domains_json()`

## 30. set_selection_domains_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3655`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `6cd811dc8279f077d5f66bd51672b4f8c54671ffbc0c850762a74d700f422246`.
- Current signature: `pub fn set_selection_domains_json(&mut self, json: &str) {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:43:    host.set_selection_domains_json(&format!("{{\"nodes\":[],\"edges\":[{edge:?}],\"handles\":[]}}"));`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2618:    host.dag.set_selection_domains_json(&json.to_string());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1818:        self.dag.set_selection_domains_json(json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1828:        self.dag.set_selection_domains_json(&json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:2179:    host.dag.set_selection_domains_json(r#"{"nodes":[],"edges":["s1"],"handles":[]}"#);`

## 31. selected_channels_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3722`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `1cef0608e058c46b5c8719e2243238e1e61022b2e925299dce6948a54f52a0f8`.
- Current signature: `pub fn selected_channels_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1984:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selected_channels_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1813:        self.dag.selected_channels_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1964:    let selected: Vec<dag::DagChannelRef> = semio_framework_pack_json::from_json_str(&host.selected_channels_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`

## 32. wire_type_refusal_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3732`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `6b9b52e7a4862a2d8f28ffb46400924808d43e7fe2af282a838b5c53b365257a`.
- Current signature: `pub fn wire_type_refusal_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3753:        let refusal = self.wire_type_refusal_json();`

## 33. hovered_channel_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3752`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `11512d1d101d7ba18667a3a07371658e7ed306fa5256fdaa2cda0f37c83c0595`.
- Current signature: `pub fn hovered_channel_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:390:        self.dag.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:814:            self.state.borrow().host.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:319:    assert_eq!(host.hovered_channel_json(), "null");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1945:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.hovered_channel_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1808:        self.dag.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1959:    let hovered: dag::DagChannelRef = semio_framework_pack_json::from_json_str(&host.hovered_channel_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`

## 34. screen_geometry_census_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3773`; receiver: `Iterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `16eb851e963c8152e170ec96b9b8ae26f86f508785aaaeabc2f315c5647654b0`.
- Current signature: `pub fn screen_geometry_census_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4657:            NodeGraphEngine::Flow(host) => host.dag.screen_geometry_census_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4658:            NodeGraphEngine::Dag(host) => host.dag.screen_geometry_census_json(),`

## 35. pick_targets_at_screen_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3829`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `e0c00b6723e31d2978c298fb1ba0c6aa01675f0a6ba636247db0a880c939ca93`.
- Current signature: `pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:251:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🦀️.rs:1423:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:668:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:681:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:145:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| host.pick_targets_at_screen_json(0.0, 0.0))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4199:        NodeGraphEngine::Flow(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4200:        NodeGraphEngine::Dag(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5369:        let json = host.pick_targets_at_screen_json(sx, sy);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6191:        let hits: Vec<Value> = serde_json::from_str(&host.pick_targets_at_screen_json(sx, sy)).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6255:    let hits = with_board_host(surface_id, |host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6267:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6294:            Some((pick_target_hits(&host.pick_targets_at_screen_json(sx, sy)), Some(text)))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:544:        self.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:865:            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:564:    let json = host.pick_targets_at_screen_json(200.0, 200.0);`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:1267:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:116:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(300.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:134:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(200.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:142:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(0.0, 0.0)).expect("json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1442:    let targets = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-260.0, 0.0)).x, host.world_to_screen(Point::new(-260.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1444:    let visible = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-50.0, 0.0)).x, host.world_to_screen(Point::new(-50.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4527:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.pick_targets_at_screen_json(number(args, "sx")?, number(args, "sy")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1797:        self.dag.pick_targets_at_screen_json(sx, sy)`

## 36. selection_preview_points_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3878`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `f8f82cfa9ebc714a3e016be6538b80ab5df2775d290b5464a2074ef4a24303aa`.
- Current signature: `pub fn selection_preview_points_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:103:            SEQUENCE_OPERATION_SELECTION_PREVIEW_POINTS => Ok(self.host.dag.selection_preview_points_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4908:                preview_points_json: host.selection_preview_points_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4914:                preview_points_json: host.dag.selection_preview_points_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:789:            self.state.borrow().host.dag.selection_preview_points_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:806:    let preview_points: Vec<[f64; 2]> = semio_framework_pack_json::from_json_str(&host.selection_preview_points_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4735:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_preview_points_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1336:        self.dag.selection_preview_points_json()`

## 37. selection_union_bounds_screen_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4110`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `04705c374216bbb6f2b78ca83b13df6a51ed2daf48fb9498ae50432cf26d3b3c`.
- Current signature: `pub fn selection_union_bounds_screen_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:106:            SEQUENCE_OPERATION_SELECTION_BOUNDS => Ok(self.host.dag.selection_union_bounds_screen_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4911:                selection_bounds_json: host.selection_union_bounds_screen_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4917:                selection_bounds_json: host.dag.selection_union_bounds_screen_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:784:            self.state.borrow().host.dag.selection_union_bounds_screen_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:845:    let json = host.selection_union_bounds_screen_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4852:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_union_bounds_screen_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1833:        self.dag.selection_union_bounds_screen_json()`

## 38. entity_screen_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4150`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `94e4d960d4fe0dd6a987704144af40926e1145244377d0c222061aa95074c97d`.
- Current signature: `pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:99:            serde_json::from_str::<Value>(&host.entity_screen_json(domain, id)).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2311:        NodeGraphEngine::Dag(host) => host.entity_screen_json(domain, entity),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2312:        NodeGraphEngine::Flow(host) => host.entity_screen_json(domain, entity),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:550:        self.dag.entity_screen_json(domain, id)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:870:            self.state.borrow().host.dag.entity_screen_json(domain, id)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:574:    let json = host.entity_screen_json("node", "a");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:581:    let json = host.entity_screen_json("node", "missing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3781:            rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), self.entity_screen_json("node", &node.id)));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3784:                rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), self.entity_screen_json("handle", &channel)));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3796:        let value = semio_framework_pack_json::parse(&self.entity_screen_json(domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:73:/// `entity_screen_json("handle", …)` hands a script, a demonstration or an assistive caller. Its`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:76:    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:237:/// The defect this pins: `entity_screen_json("handle", …)` published the port ROW rect while`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:357:    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:362:    let back: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:856:    let json = host.entity_screen_json("node", "scale");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:862:    let wildcard: Value = semio_framework_pack_json::parse(&host.entity_screen_json("node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:872:    let input_json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale@in"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:874:    let output_json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "combine@b"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:876:    let malformed: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:878:    let missing_port: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale@nope"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:886:    let json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("edge", "e1"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:898:        let json: Value = semio_framework_pack_json::parse(&host.entity_screen_json(domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:905:    let empty: Value = semio_framework_pack_json::parse(&DagHost::from_host_snapshot(empty_fixture).entity_screen_json("node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1670:/// at every zoom (`entity_screen_json("handle", …)`), so every tier but the silhouette must accept`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:558:    /// rect through `entity_screen_json("handle", …)` at every zoom — so a press on one wires at every`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4566:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.entity_screen_json(text(args, "domain")?, text(args, "id")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:206:        let geometry: Value = serde_json::from_str(&adapter.host.entity_screen_json("node", id)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:261:        let geometry: Value = serde_json::from_str(&host.entity_screen_json("node", &id)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1803:        self.dag.entity_screen_json(domain, id)`

## 39. set_selected_channels_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4453`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `bc35f5f07ac9ca1815b8cb893e524adc902a354db6e965d5723d3a4868404ebd`.
- Current signature: `pub fn set_selected_channels_json(&mut self, json: &str) {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:2208:                        domain.host.set_selected_channels_json(text(args, "json")?);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1863:        self.dag.set_selected_channels_json(json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1963:    host.set_selected_channels_json(r#"[{"widgetId":"add","port":"a","direction":"in"}]"#);`

## 40. set_node_statuses_from_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4503`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `db3a4191617b650eb7f49da898baff017ede4e5b094bf786f22b265ceb78e622`.
- Current signature: `pub fn set_node_statuses_from_json(&mut self, json: &str) {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2869:                NodeGraphEngine::Flow(host) => host.set_node_statuses_from_json(json),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2870:                NodeGraphEngine::Dag(host) => host.dag.set_node_statuses_from_json(json),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:331:            self.dag.set_node_statuses_from_json(status_json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1638:                        domain.host.set_node_statuses_from_json(text(args, "json")?);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:667:        self.dag.set_node_statuses_from_json(json);`

## 41. load_host_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4822`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `c536f9173434d9b32582fc2a82c8ab6b89087e52a66cdffd8bea3a47428926a9`.
- Current signature: `pub fn load_host_snapshot_json(json: &str) -> Result<Self, DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6958:            let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:74:    let mut host = DagHost::load_host_snapshot_json(&semio_framework_pack_json::to_json_string(&fixture)).map_err(|error| layout_fault("dag.layout-run.layered-load", error.to_string()))?;`

## 42. host_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4830`; receiver: `Fn`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `d452cceb345786e275a83b7c2b5769ac8ef81197e70582d23fe4412f823282f1`.
- Current signature: `pub fn host_snapshot_json(&self) -> Result<String, DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:567:        Ok(self.dag.host_snapshot_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:906:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:611:    let json = host.host_snapshot_json().expect("fixture json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6965:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5468:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.host_snapshot_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5712:        let fixture = self.host.host_snapshot_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1080:    let json = host.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️rule-application/🦀️.rs:81:    let snapshot_json = g.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:242:    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json()?, query: result }))`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs:245:    let json = g.host_snapshot_json().expect("fixture json");`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:76:    let layered: DagHostSnapshot = host.host_snapshot_json().ok().and_then(|json| semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).ok_or_else(|| layout_fault("dag.layout-run.layered-result", "the layered host returned no decodable fixture".into()))?;`

## 43. screen_hit_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5186`; receiver: `IntoIterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `e8574604e519058cd5b4369efaaa5b17ac62e359c6cefae872f84d4d1c9184fc`.
- Current signature: `pub fn screen_hit_json(&self, sx: f64, sy: f64) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4297:        let trace = dag.screen_hit_json(sx, sy);`

## 44. node_overlays_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5872`; receiver: `IntoIterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `b9b776b2859d6ef5d3482c538b076ca57a60e7045ee101d07743db3ae3a5bf78`.
- Current signature: `pub fn node_overlays_json(&self) -> Result<String, DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6970:            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:776:    let json = host.node_overlays_json().unwrap();`

## 45. label_overlay_rows_for_node_spec
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5989`; receiver: `IntoIterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `867c960b3038612448f50f5a147579670ad97ac5a360676a0dfe46271d12ed7e`.
- Current signature: `pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1872:    let ghost_overlay_rows = host.dag.label_overlay_rows_for_node_spec(ghost, true);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1889:    let placed_rows = host.dag.label_overlay_rows_for_node_spec(&placed_node, false);`

## 46. slider_overlay_state_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6136`; receiver: `IntoIterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `64951bc422ee7a5f05e44cf320ec174c1cb7fc4e324c920deef645bea479907d`.
- Current signature: `pub fn slider_overlay_state_json(&self) -> Result<String, DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:201:        let state: Value = serde_json::from_str(&host.slider_overlay_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2295:            NodeGraphEngine::Dag(host) => host.dag.slider_overlay_state_json().ok()?,`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2296:            NodeGraphEngine::Flow(host) => host.slider_overlay_state_json().ok()?,`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:779:            self.state.borrow().host.dag.slider_overlay_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:622:        let json = host.slider_overlay_state_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:656:    let raw: Value = semio_framework_pack_json::parse(&host.slider_overlay_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:2577:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.slider_overlay_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1998:        Ok(self.dag.slider_overlay_state_json()?)`

## 47. label_overlay_paint_state_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6169`; receiver: `IntoIterator`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `8550688d1893a8cf8ca2f5c92343ba968a967c47719cafd3fd63652bdfaa0680`.
- Current signature: `pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:95:            SEQUENCE_OPERATION_LABEL_OVERLAY => self.host.dag.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error),`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:182:        let labels = self.host.dag.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:365:            serde_json::from_str(&host.label_overlay_paint_state_json().ok()?).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4869:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4873:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:394:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:774:            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:620:    let json = host.label_overlay_paint_state_json().expect("labels");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:636:    let state: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label state")).expect("independent JSON oracle");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6975:            self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:563:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:592:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:693:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:744:        let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1723:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1734:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1824:    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:2017:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4117:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5713:        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:329:        let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:361:    let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2173:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1145:    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1874:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1922:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs:220:        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`

## 48. load_host_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6957`; receiver: `DagSession`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `a9db502ba671f78bcad57886c76bfb8ad1872b82e5c87ab96b58ebf0a51ee490`.
- Current signature: `pub fn load_host_snapshot_json(&self, json: &str) -> Result<(), JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:12:    loadHostSnapshotJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:12:    loadHostSnapshotJson(json: string): void;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:26:    loadHostSnapshotJson(json: string): void;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:74:    let mut host = DagHost::load_host_snapshot_json(&semio_framework_pack_json::to_json_string(&fixture)).map_err(|error| layout_fault("dag.layout-run.layered-load", error.to_string()))?;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:26:    loadHostSnapshotJson(json: string): void;`

## 49. host_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6964`; receiver: `DagSession`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `6674ed3dea9c4b344037369b97580e2562895a8a86436c32208358a2e0539908`.
- Current signature: `pub fn host_snapshot_json(&self) -> Result<String, JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:567:        Ok(self.dag.host_snapshot_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:906:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:611:    let json = host.host_snapshot_json().expect("fixture json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:10:    hostSnapshotJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5468:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.host_snapshot_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5712:        let fixture = self.host.host_snapshot_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1080:    let json = host.host_snapshot_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:10:    hostSnapshotJson(): string;`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️rule-application/🦀️.rs:81:    let snapshot_json = g.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:242:    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json()?, query: result }))`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs:245:    let json = g.host_snapshot_json().expect("fixture json");`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:24:    hostSnapshotJson(): string;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:76:    let layered: DagHostSnapshot = host.host_snapshot_json().ok().and_then(|json| semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).ok_or_else(|| layout_fault("dag.layout-run.layered-result", "the layered host returned no decodable fixture".into()))?;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:24:    hostSnapshotJson(): string;`

## 50. node_overlays_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6969`; receiver: `DagSession`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `b7ab231585030058db3a72325b046dab4b1afbd1d21b4dd921486632e19d606a`.
- Current signature: `pub fn node_overlays_json(&self) -> Result<String, JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:776:    let json = host.node_overlays_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:15:    nodeOverlaysJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:15:    nodeOverlaysJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:29:    nodeOverlaysJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:29:    nodeOverlaysJson(): string;`

## 51. label_overlay_paint_state_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6974`; receiver: `DagSession`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `a9a0556d977d16d3d3e621f06b7dcda01d0e5876d6c9688f7687160f18851edf`.
- Current signature: `pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:95:            SEQUENCE_OPERATION_LABEL_OVERLAY => self.host.dag.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error),`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:182:        let labels = self.host.dag.label_overlay_paint_state_json().map_err(domain_error)?;`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:69:  labelOverlayPaintStateJson(): SequenceTask<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:365:            serde_json::from_str(&host.label_overlay_paint_state_json().ok()?).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4869:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4873:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:106:  labelOverlayPaintStateJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:844:      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1974:      const cameraJson = read("labels", session.labelOverlayPaintStateJson());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1991:        const cameraJson = read("labels", session.labelOverlayPaintStateJson());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:394:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:774:            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:620:    let json = host.label_overlay_paint_state_json().expect("labels");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:636:    let state: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label state")).expect("independent JSON oracle");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:563:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:592:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:693:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:744:        let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1723:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1734:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1824:    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:2017:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:11:    labelOverlayPaintStateJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4117:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5713:        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:329:        let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:361:    let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2173:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1145:    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1874:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1922:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:11:    labelOverlayPaintStateJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:25:    labelOverlayPaintStateJson(): string;`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs:220:        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:25:    labelOverlayPaintStateJson(): string;`

## 52. lod_scale_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7027`; receiver: `DagSession`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `f9bcb430f33ac63d149cc112c57d6412310c59ab75876bee733eed76f8fec3be`.
- Current signature: `pub fn lod_scale_json(&self) -> String {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2577:            "lodScaleJson" => lod_scale_json::lod_scale_json(ctx),`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:138:export class DagSession { lodScaleJson() { return dagLodScaleJson(); } }`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:139:export class BoardSession { lodScaleJson() { return dagLodScaleJson(); } }`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:59:  lodScaleJson(): SequenceTask<string>;`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:23:    lodScaleJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:13:    lodScaleJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:13:    lodScaleJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:27:    lodScaleJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:27:    lodScaleJson(): string;`

## 53. set_canvas_theme_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7082`; receiver: `DagSession`.
- Classification: Physical browser bridge directly reaches canonical controlled IO (accepted source ownership).
- Canonical ownership: DAG/🚪️io/📝️text; DAG/🧬️schema projection/input record + pure DagHost method.
- Inventoried body SHA-256: `9af99d36d93defd9b7581c7a9e74566d00ed9085568eab6d55e12f91b75d23f0`.
- Current signature: `pub fn set_canvas_theme_json(&mut self, json: &str) {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts:573:      setCanvasThemeJson(json: string) {`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts:232:        setCanvasThemeJson(json: string) {`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:570:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:578:    session.setCanvasThemeJson(serializeCanvasThemeJson());`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:63:  setCanvasThemeJson(json: string): SequenceTask<Uint8Array>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:2671:  observeFlowTask(session, "setCanvasThemeJson", session.setCanvasThemeJson(serializeCanvasThemeJson()));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:94:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:150:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:237:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:83:    setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:24:    setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:24:    setCanvasThemeJson(json: string): void;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:38:    setCanvasThemeJson(json: string): void;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:38:    setCanvasThemeJson(json: string): void;`

## 54. snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7154`; receiver: `DagSnapshotVcs`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: VCS explicit text IO; typed Store snapshot/envelope receiving.
- Inventoried body SHA-256: `872e9b7afc310b80f93931653fbcfe3a3a7975adcb006c1e9c9690e7aefd6feb`.
- Current signature: `pub async fn snapshot_json(&self) -> Result<String, JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:76:    let json = snapshot_json(node);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:106:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:124:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:238:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:283:    let json = snapshot_json(opened);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:311:    let json = snapshot_json(node);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧪️tests/🙈️ignore-integration/🦀️.rs:36:            .snapshot_json(VECTORS)?`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧪️tests/🪪️artifact-id-builders/🦀️.rs:76:            .snapshot_json(VECTORS)?`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/🧬️definition-kind-derivation/🦀️.rs:18:            .snapshot_json(VECTORS)?`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:19:  snapshotJson(): SequenceTask<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8522:    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8534:    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:43:    snapshotJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:79:    snapshotJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:43:    snapshotJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:79:    snapshotJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:57:    snapshotJson(): Promise<string>;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:93:    snapshotJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:57:    snapshotJson(): Promise<string>;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:93:    snapshotJson(): string;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:345:        let projection = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:358:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:365:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:379:        let applied = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:386:        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:402:            ("document".to_string(), snapshot_json(&derived)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:449:            ("graph".to_string(), snapshot_json(&graph)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:181:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:220:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:232:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:239:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:252:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:277:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:401:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:411:        let projection = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:423:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:430:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:445:        let applied = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:452:        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:494:            ("building".to_string(), snapshot_json(&building)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:427:            format!("masters=[{masters}] layouts=[{layouts}] slides=[{slides}] digest={}", digest(snapshot_json(deck).to_string().as_bytes()))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:437:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:448:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:455:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:473:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:511:                    ("document".to_string(), snapshot_json(&parsed)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:157:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:190:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:202:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:209:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:225:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:250:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:989:function snapshotJson(ctx: AdapterContext, uri: string): unknown {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1007:  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1014:  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1025:  const before = snapshotJson(ctx, uris[0]!) as Snapshot;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1026:  const mutation = snapshotJson(ctx, uris[1]!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1027:  const expected = snapshotJson(ctx, uris[2]!) as Snapshot;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:212:        let document = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:225:        let mutated = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:232:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:245:        Ok(Outcome::projection(snapshot_json(&current)?))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:280:        let document = snapshot_json(&parsed)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:231:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:267:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:279:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:286:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:299:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:327:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:174:        let document = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:187:        let mutated = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:194:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:207:        Ok(Outcome::projection(snapshot_json(&current)?))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:242:        let document = snapshot_json(&parsed)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:250:        Ok(Outcome::projection(Json::Object(vec![("document".to_string(), snapshot_json(&current)), ("raster".to_string(), raster_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:261:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:268:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:287:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:323:            ("document".to_string(), snapshot_json(&parsed)),`

## 55. envelope_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7160`; receiver: `DagSnapshotVcs`.
- Classification: Physical browser bridge; semantic delegate still violates ownership.
- Canonical ownership: VCS explicit text IO; typed Store snapshot/envelope receiving.
- Inventoried body SHA-256: `5b3f9de12b265115fdac807328b5e05aa5547eb2586932e0957f4060385eaeb9`.
- Current signature: `pub async fn envelope_json(&self) -> Result<String, JsValue> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:41:    envelopeJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:41:    envelopeJson(): Promise<string>;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:55:    envelopeJson(): Promise<string>;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:55:    envelopeJson(): Promise<string>;`

## 56. apply_force_graph_layout_to_board_snapshot_value
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:217`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `5309f4e4c83544a0e3b42a1eae901a6bf7f08b43f0fc676c394f0f8824eb0716`.
- Current signature: `pub fn apply_force_graph_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), UndirectedGraphError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:946:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1713:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:343:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:398:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`

## 57. apply_force_graph_layout_to_board_snapshot_value_resolved
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:222`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `c88dee91437a633e5ce686e71eae93544ac17ac48d3005d5c964088f31dfc89e`.
- Current signature: `pub fn apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot: &mut Value, opts: &ForceGraphLayoutOptions, resolve_node_id: impl Fn(&str, &HashMap<String, usize>) -> Option<String>) -> Result<(), UndirectedGraphError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:935:        infinite::board::normal::undirected::apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, |endpoint, id_to_index| {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:218:        apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, resolve_node_id_endpoint)`

## 58. apply_force_graph_layout_to_board_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:340`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `b37264fcf423e64b8d71e139e33e8fa12a059cff6272b89291d427c47fd992f7`.
- Current signature: `pub fn apply_force_graph_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, UndirectedGraphError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:37:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:77:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:179:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:189:    let err = apply_force_graph_layout_to_board_snapshot_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:231:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:285:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:286:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:316:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:317:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:346:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚛️force-layout/🦀️.rs:8:    let Ok(layout_json) = crate::editor::puzzle2d::engine::apply_force_graph_layout_to_board_snapshot_json(&ctx.scene.board_snapshot.to_string(), r#"{"mode":"force-graph"}"#) else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:36:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:64:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`

## 59. apply_redraw_layout_to_board_snapshot_json
- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:375`; receiver: `free function / associated constructor`.
- Classification: Semantic owner contains physical interpretation/publication.
- Canonical ownership: layout typed positions/options operators; snapshot/options codec under Board/🚪️io/📝️text/📐️layout.
- Inventoried body SHA-256: `d75f780ba89d64adbf18a891b117269494adee3eaf8b802dae284040f89528a1`.
- Current signature: `pub fn apply_redraw_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, UndirectedGraphError> {`

Direct lexical call evidence (each receiver must be attributed by its local imports/type; common names below are not all Board):
- None in inventory.

# Exact 76 Foreign Camera Files

For each path below, camera token families were compared with read-only Git HEAD. HEAD is an independent pre-existing contract witness, not a before-correction snapshot. It cannot prove unrelated edits were preserved.

- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs` — current `CameraJsonIn, navigatorFitCameraJson`; HEAD `CameraJsonIn, navigatorFitCameraJson`; same token family.
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJsonIn`; HEAD `CameraJsonIn`; same token family.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎚️mutate-gis-gisterrain-1-config/🦀️.rs` — current `baseCameraJson`; HEAD `baseCameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — current `parseCameraJson`; HEAD `parseCameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` — current `InkCameraJson`; HEAD `InkCameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️update-camera/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` — current `CameraJsonDsl`; HEAD `CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` — current `CameraJson, CameraJsonDsl`; HEAD `CameraJson, CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` — current `CameraJson, CameraJsonDsl`; HEAD `CameraJson, CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🧰️owned/🦀️.rs` — current `CameraJson, CameraJsonDsl`; HEAD `CameraJson, CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧬️fields/🦀️.rs` — current `CameraJson`; HEAD `none`; new/untracked or changed owner; requires direct owner verification.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🕸️set-camera/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧪️tests/🌿️vcs/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/📝️text/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` — current `CameraJsonDsl`; HEAD `CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs` — current `CameraJson, CameraJsonDsl`; HEAD `CameraJson, CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` — current `CameraJsonDsl`; HEAD `CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs` — current `CameraJson, CameraJsonDsl`; HEAD `CameraJson, CameraJsonDsl`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/📥️reconstruction/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native/🌱️value/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `none`; new/untracked or changed owner; requires direct owner verification.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — current `CameraJson`; HEAD `CameraJson`; same token family.

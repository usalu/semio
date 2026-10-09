# Probe DslValue Storage Migration

Read-only source; no tests. Exact constructor/mutation callers below include tests and declarations, not an owner denominator.

Production owner193/diff261/mutation300 each currently stores serde_json::Value. Transparent ToValue251/287 and FromValue256/292 plus mutation354/364 build foreign-to-firstparty trees; replacing actual storage with DslValue permits owned FromValue move and genuine supported retirement. Borrowed ToValue still needs actual controlled clone path where productive ownership is required; plain .clone is not native receipt qualification. Diff inverse265/apply275 and Mutation diff335/inverse340 currently clone whole trees; these must use real original operation authority or remain separately unqualified, not hidden uncontrolled conversions. merge279 already moves actual diff.

Gateway ensure_probe_artifact4557 takes serde initial, dispatch4586 and set-value4609 accept foreign JSON; undo4628/redo4640 return .0 as serde JSON. Production boundary methods must accept/return firstparty DslValue or explicitly controlled host conversion under caller-owned receiving authority. Do not retain public serde API as an invisible unquoted adapter. Mutation text373–378/binary384–389 are original raw JSON representations; preserve SetValue external tagging for ToValue/from_value and existing protocol fixtures. Quick tests407–409 construct all three from serde oracle values; fixtures should compare explicit controlled conversions or original JSON bytes, not make production hold foreign trees for test convenience.

Actual SQL schema is one semantic `probe_node` table (`🪶️sqlite/🗄️.sql`), with eight columns including id: parent identity, array position, member_name, node_type, bool, number TEXT, string. Earlier eight-table wording was incorrect. Keep node entities and genuine IDs/preorder traversal; arrays sort/validate contiguous position66–71; objects reject duplicate member_name77–80. serde Map currently gives its configured key ordering, whereas DslValue Object is an ordered Vec: choose schema-first canonical key policy preserving existing emitted node order and reject duplicates, rather than accidentally alter row identity through arbitrary insertion order.

Number TEXT currently uses serde Number display25 and parse63. DslValue Number carries exact UInt(u64)/Int(i64)/Float(f64), Value Rust63–67. Never route integers through as_f64: >2^53 loses exact words. Author canonical number formatting/parsing via firstparty JSON number parser/writer, preserving finite values, negative zero and integer/float distinctions with independent serde oracle. Lexical formatting may canonicalize original JSON already; test numeric semantics and exact SQL canonical words explicitly.

Genuine direct firstparty parser exists: JsonGrammarCursor<DslValue>; sealed JsonParsedValue implementation at JSON `📥️decode/🫳️borrowed/🦀️.rs:135`, so no intermediate foreign JSON tree or JsonValueProjection is needed. JSON main889 owns pending/result/frames/lexemes; normal_step_demands903, step922 takes original DecodeControl and explicit grant, normal_step_progress901 records actual turn. Retain this actual cursor in same original receive frame before stepping and debit its accepted receipts into BodyWallet, including errors. Do not use from_json_str_controlled returning whole value as proof of partial custody.

Genuine borrowed writer JsonWriteSource for DslValue1808 and JsonBorrowedWriteCursor1821/normal demands1831/step1853 accept actual borrowed tree, original EncodeControl and explicit grant. Retain its actual cursor/output and accumulate accepted normal_step_progress; no uncontrolled owned source clone. Real supported DslValue retirement476 and cursor declarations are original owner paths. Original raw binary JSON is UTF8 bytes of accepted writer output; move owned String backing when possible only through authentic controlled ownership API, not extra unquoted copy.

## Exact Framework Constructor/Mutation Source Lines

```text
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs:29:    let snapshot = NativeSocketProbeSnapshot(value.clone());
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs:30:    let diff = NativeSocketProbeDiff(value.clone());
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19233:struct NativeSocketProbeSnapshot(String);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19310:struct NativeSocketProbeDiff(String);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19315:        NativeSocketProbeDiff(base.0.clone())
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19326:        Ok(NativeSocketProbeSnapshot(self.0.clone()))
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19388:        store::os_spr::command::MutationOutcome::new(NativeSocketProbeDiff(value.clone()))
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🧪️tests/🦀️.rs:102: let f=fixture();let mut callback=|_|true;let original=NativeSocketProbeSnapshot("probe".into()).to_sqlite_database(&mut Control::new(&mut callback,Limits::default())).unwrap();
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🧪️tests/🦀️.rs:104: let large=f["large"]["unit"].as_str().unwrap().repeat(f["large"]["repetitions"].as_u64().unwrap()as usize);let snapshot=NativeSocketProbeSnapshot(large.clone());let database=snapshot.to_sqlite_database(&mut Control::new(&mut callback,Limits::default())).unwrap();
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🧪️tests/🦀️.rs:118: for(text,_)in samples(&f){let snapshot=NativeSocketProbeSnapshot(text);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🪶️sqlite/🧪️tests/🦀️.rs:133:  let snapshot=NativeSocketProbeSnapshot("original λ🙂\0".into());
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:193:pub struct ProbeSnapshot(pub serde_json::Value);
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:256:        Ok(ProbeSnapshot(serde_json::Value::from(value)))
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:261:pub struct ProbeDiff(pub serde_json::Value);
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:265:        ProbeDiff(base.0.clone())
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:275:        Ok(ProbeSnapshot(self.0.clone()))
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:292:        Ok(ProbeDiff(serde_json::Value::from(value)))
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:334:        let ProbeMutation::SetValue(value) = self;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:335:        store::MutationOutcome::new(ProbeDiff(value.clone()))
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:340:        vec![ProbeMutation::SetValue(base.0.clone())]
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:346:/// 🌉️ Hand-written — `ProbeMutation::SetValue` wraps a foreign `serde_json::Value` field, same gap
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:354:        let ProbeMutation::SetValue(value) = self;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:364:            Ok([(key, payload)]) if key == PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant => Ok(ProbeMutation::SetValue(serde_json::Value::from(payload))),
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:373:        let ProbeMutation::SetValue(value) = self;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:378:        serde_json::from_str(line).map(ProbeMutation::SetValue).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(0, 0)))
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:384:        let ProbeMutation::SetValue(value) = self;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:389:        serde_json::from_slice(bytes).map(ProbeMutation::SetValue).map_err(|error| store::ProtocolError::Malformed { what: "probe-op", offset: 0, detail: error.to_string() })
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:4586:                .dispatch(store::ArtifactCommand::Apply { mutations: vec![ProbeMutation::SetValue(initial)], transaction: None })
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:4609:        let dispatched = probe_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![ProbeMutation::SetValue(value)], transaction: None }).await;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs:407:    let snapshot = ProbeSnapshot(value.clone());
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs:408:    let diff = ProbeDiff(value.clone());
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs:409:    let mutation = ProbeMutation::SetValue(value.clone());
```

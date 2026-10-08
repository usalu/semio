# Record10 and General Pack Bridge Owner Audit

Read-only sealed artifact/source audit, no executions or production edits.

Record owning10 admission full-key join agrees. Actual Cargo0/harness0, all30 source posts and producer/input exact. All-owner execution flag true; raw process/lease/stdout/stderr closed, captured508841bytes. Actual target summaries:

- test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s
- test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
- test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
- test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

This closes the selected current Record package targets, not whole dependency closure or atomic snapshot. Earlier Record9 failures cannot be promoted into this result.

## Canonical defining API

General Pack owns the one-field DslValue Record bridge, with explicitly distinct document-container and containerless-wire decode/encode, typed Pack refusal/errors, options, and mandatory controlled/cancellation ports. Move all bridge construction together, not only two Surface-facing functions. OS store currently value_bridge_spec6800/encode_pack_value6805/decode_pack_value6814/encode_wire_value6823 establish that semantic identity. Numeric carrier variants, bytes, duplicate/ordered members and exact record-list boundaries must survive extraction.

General Value/JSON owns DslValue JSON projection. Existing store dsl_value_to_json simply delegates serde_json::Value::from. Do not export third-party JSON types from the new canonical API: emit first-party owned JSON/value or controlled first-party JSON text; use Serde as independent oracle. Preserve ordinary raw scene strings exactly rather than parse/reprint them.

General Pack presentation owns exact pk: envelope recognition, first-party base64 decode, document decode, and canonical JSON-text projection, with explicit input/output limits. Current scene_field_json_text checks starts_with(pk:), otherwise returns unchanged string; malformed pk: never becomes ordinary JSON. Current Surface binary474 explicitly tries wire then document; make that accepted framing policy explicit in a defining input-format contract rather than silently guessing all inputs globally. Keep document/wire distinction for callers who require one only. No legacy Store facade or default bridge.

## Existing neutral evidence and missing cases

UI retained fixture `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/🧳️scene-pack-field/🔣️.json` provides10 valid base64 vectors (unpadded, whitespace, split padding, unused-low-bits),13 invalid vectors (prefix case/leading space, alphabet, padding, non-ASCII whitespace), large16384, cancellation prefixes and maxchunk256. These qualify base64 envelope semantics only: its empty-binary and arbitrary octet vectors are not valid Pack payload proofs.

Store unit8944+ has actual pack/wire round trips, carrier UInt3 versus positive Int7, malformed/redundant varints. Preserve those as binary bridge laws with fixtures/oracles. Kernel return-content dialect tests and manifest integer carriers exercise consumers independently.

Add owner schema outside test/fixture ancestors specifying plain arbitrary JSON text byte-preservation, actual pk: document payload+JSON oracle, malformed prefix/base64, valid base64 invalid document, wire-only/document-only/refused ambiguous/truncated framing, all Number variants, bytes and nested duplicate keys, cancellation partial outputs and exact allocation/release. Source fixture classification is not an executed new-law success.

## Exact caller roster

The bounded source search covers framework and plugin source, and captures direct qualified/unqualified calls below. It does not prove dynamic symbol resolution or generated callers. Active consumers include Print binary diff, Workflow and test artifact codecs, renderer ProgramBridge/EngineCanvas, plugin reactor/descriptor paths, General Surface, General Manifest and Kernel tests. Update actual consumer dependencies and first-party errors at their owners; do not assume removing Surface calls completes the extraction.

```text
🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔢️integer-carriers/🦀️.rs:66:    let value = dsl::pack_rt::decode_wire_value(&unhex(&fixture.pack_hex)).expect("the fixture pack decodes");
🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🗣️return-content-dialects/🦀️.rs:33:    assert!(semio_framework_os_kernel::pack_rt::decode_wire_value(&payload).is_err());
🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🗣️return-content-dialects/🦀️.rs:43:    let decoded = semio_framework_os_kernel::pack_rt::decode_wire_value(&encoded).unwrap();
🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs:103:    let decoded = ChartTextOutput::from_value(protocol::pack_rt::decode_wire_value(&first.canonical_payload).unwrap()).unwrap();
🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs:113:    let decoded=ChartTextOutput::from_value(protocol::pack_rt::decode_wire_value(&invalid.canonical_payload).unwrap()).unwrap();
🧰️framework/🛍️products/📓️print/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = protocol::pack_rt::decode_wire_value(bytes).map_err(|error|malformed(error.to_string()))?;
🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧬️intrinsic-bytes/🦀️.rs:11:    for case in input["cases"].as_array().unwrap(){let bytes=case["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let value=DslValue::Bytes(bytes.clone());let source=format!("data=bytes64(\"{}\")",case["base64"].as_str().unwrap());let record=parse(&source,&spec,&ParseOptions::default()).unwrap();assert_eq!(record.get(1),Some(&FieldValue::Value(value.clone())));assert_eq!(print(&record,&spec,JoinMode::Inline),source);let wire=crate::store::pack_rt::encode_wire_value(&value);assert_eq!(crate::store::pack_rt::decode_wire_value(&wire).unwrap(),value);assert_eq!(serde_json::Value::from(&value),case["octets"]);assert_eq!(semio_framework_value::bytes::from_value(value).unwrap(),bytes);assert_eq!(semio_framework_value::bytes::from_value(DslValue::from(&case["octets"])).unwrap(),bytes);}
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:255:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:258:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:261:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:264:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:267:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:270:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:273:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:276:        *json = store::pack_rt::scene_field_json_text(json)?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:474:        let dsl = store::pack_rt::decode_wire_value(bytes).or_else(|_| store::pack_rt::decode_pack_value(bytes))?;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:475:        let value = store::pack_rt::dsl_value_to_json(dsl);
🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:13:        let value = protocol::pack_rt::decode_wire_value(bytes).map_err(|error|malformed(error.to_string()))?;
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🎞️intrinsic/🦀️.rs:56: let descriptor=MediaArtifactDescriptor::from_value(store::pack_rt::decode_wire_value(&descriptor_bytes).expect("descriptor wire input")).expect("typed descriptor");
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:141:    store::pack_rt::decode_wire_value(bytes).map_err(wire_err)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs:136:    let value = match store::pack_rt::decode_wire_value(payload) {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs:162:    let value = store::pack_rt::decode_wire_value(payload).map_err(|_| "plugin.document-backbone.receipt-codec".to_string())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs:108:    let keys = |bytes: &[u8]| match store::pack_rt::decode_wire_value(bytes).expect("the control decodes") {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🦀️.rs:97:    let value = semio_framework_os_kernel::pack_rt::decode_wire_value(bytes).map_err(|error| error.into_value_error())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🦀️.rs:131:        let value = semio_framework_os_kernel::pack_rt::decode_wire_value(&packed).unwrap();
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:14:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:9:        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "bitmap-diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:91:        let value = pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:96:        let decoded = pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:112:        let Ok(value) = pack_rt::decode_wire_value(report) else { return String::new() };
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:124:        let decoded = pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:1795:    let value = store::pack_rt::decode_wire_value(&bytes).map_err(|_| "hub execution-target descriptor is not a canonical pack".to_string())?;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1852:/// (`semio_framework_os_kernel::os_store::pack_rt::scene_field_json_text`, React's `parseSceneJsonField`) and read; a JSON `null` is no value.
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1854:    let json = semio_framework_os_kernel::os_store::pack_rt::scene_field_json_text(raw).map_err(|error| format!("{member}: {error}"))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:578:    let read = semio_framework_os_kernel::pack_rt::decode_wire_value(&sent).expect("the sent value decodes");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4972:        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13824:            let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("transaction.child-groups-malformed"), format!("owned-child op groups did not decode: {error}")))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37533:                store::pack_rt::decode_wire_value(request).map_err(|error| plugin_sdk_fault(error.to_string())).and_then(|value| semio_framework_value::FromValue::from_value(value).map_err(|error| plugin_sdk_fault(error.to_string())))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:42714:        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:42877:        let value = store::pack_rt::decode_wire_value(bytes).ok()?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:45654:            let Ok(semio_framework_value::DslValue::Object(mut entries)) = store::pack_rt::decode_wire_value(output) else { continue };
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs:37:    let restored = <crate::sqlite_wire::SnapshotRejection as semio_framework_value::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2451:        let value = store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:6043:        let value = store::pack_rt::decode_wire_value(&packed).expect("leftover pack decode");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:8142:        let merged = store::pack_rt::decode_wire_value(output).expect("merged output decodes");
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:2512:    let decoded = store::pack_rt::decode_wire_value(bytes).ok();
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:2713:                        let value = store::pack_rt::decode_wire_value(&history_patch).map_err(|error| Self::not_wired("decoding HistoryPatch", error))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:526:                    let descriptor_value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(&descriptor_bytes)
🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧬️intrinsic-bytes/🦀️.rs:11:    for case in input["cases"].as_array().unwrap(){let bytes=case["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let value=DslValue::Bytes(bytes.clone());let source=format!("data=bytes64(\"{}\")",case["base64"].as_str().unwrap());let record=parse(&source,&spec,&ParseOptions::default()).unwrap();assert_eq!(record.get(1),Some(&FieldValue::Value(value.clone())));assert_eq!(print(&record,&spec,JoinMode::Inline),source);let wire=crate::store::pack_rt::encode_wire_value(&value);assert_eq!(crate::store::pack_rt::decode_wire_value(&wire).unwrap(),value);assert_eq!(serde_json::Value::from(&value),case["octets"]);assert_eq!(protocol::bytes::from_value(value).unwrap(),bytes);assert_eq!(protocol::bytes::from_value(DslValue::from(&case["octets"])).unwrap(),bytes);}
🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:416:        let payload = decode_wire_value(&bytes[offset..]).map_err(|error| malformed("op payload", offset as u64, error.to_string()))?;
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:46:    match store::pack_rt::decode_wire_value(bytes) {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🧪️tests/🔬️unit/🦀️.rs:23:    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🧪️tests/🔬️unit/🦀️.rs:52:    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🧪️tests/🔬️unit/🦀️.rs:67:    let initial = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("native descriptor");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs:44:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| format!("{field}: {error}"))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:411:    store::pack_rt::decode_wire_value(bytes).ok()
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:3157:                    let Ok(value) = store::pack_rt::decode_wire_value(&entry.update) else { continue };
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:3438:    store::pack_rt::decode_wire_value(bytes).ok()
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6221:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6227:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6236:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:7311:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️.rs:45:    let value = store::pack_rt::decode_wire_value(input).map_err(|error| super::fault("job.mutation-plan.decode", format!("invalid {} input: {error}", super::JOB_KIND_MUTATION_PLAN)))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs:52:            let value = store::pack_rt::decode_wire_value(&bytes).expect("wire value decodes");
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:756:        Ok(bytes) => match store::pack_rt::decode_wire_value(bytes) {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1422:        store::pack_rt::decode_wire_value(bytes).ok().and_then(|value| semio_framework_value::FromValue::from_value(value).ok()).unwrap_or_default()
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs:10:    let decoded: semio_framework::Fault = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:922:        let dsl = crate::os_store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:938:        let dsl = crate::os_store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:13:        let value = protocol::pack_rt::decode_wire_value(bytes).map_err(|error|malformed(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:885:                if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:2078:            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:2151:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:2201:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs:107:                    let decoded = store::pack_rt::decode_wire_value(bytes).expect("the fault arm is a pack");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6814:    pub fn decode_pack_value(bytes: &[u8]) -> Result<DslValue, PackError> {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6831:    pub fn decode_wire_value(bytes: &[u8]) -> Result<DslValue, PackRefusal> {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6868:        decode_pack_value(bytes).map(dsl_value_to_json)
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6891:            decode_pack_value(&pack_value_from_base64(encoded)?)
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6898:    pub fn scene_field_json_text(field: &str) -> Result<String, PackError> {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6900:            let dsl = decode_pack_value(&pack_value_from_base64(field)?)?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6901:            Ok(serde_json::to_string(&dsl_value_to_json(dsl)).unwrap_or_else(|_| "null".into()))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6910:        dsl_value_to_json(renormalize_whole_number_floats(json_value_to_dsl(&value)))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:6919:    pub fn dsl_value_to_json(value: DslValue) -> serde_json::Value {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11818:        pack_rt::decode_pack_value(bytes)
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:28451:        let value = pack_rt::decode_pack_value(bytes).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error.to_string() })?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:28473:        let value = pack_rt::decode_pack_value(bytes)?;
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:530:    let decoded = store::pack_rt::decode_wire_value(&descriptor_bytes).map_err(|error| DescribeError(format!("decoding describe() output as a pack: {error}")))?;
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️unit/🦀️.rs:75:            <standards::v1::subsets::any::schema::inferences::GisMapInference as semio_framework_value::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&first.canonical_payload).expect("canonical inference payload"))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8916:/// capture the printed `name -> hex` lines; also asserts `decode_pack_value(encode_pack_value(v))
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8924:        let decoded = pack_rt::decode_pack_value(&bytes).expect("decode_pack_value");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8950:        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8984:        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8989:    assert!(matches!(pack_rt::decode_wire_value(&dynamic_integer_fixture_bytes("000101110403")).expect("uint 3"), DslValue::Number(semio_framework_value::Number::UInt(3))));
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8990:    assert!(matches!(pack_rt::decode_wire_value(&pack_rt::encode_wire_value(&DslValue::int(7))).expect("int 7"), DslValue::Number(semio_framework_value::Number::Int(7))), "a positive Int keeps TAG_INT rather than collapsing onto TAG_UINT");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:9000:                assert!(pack_rt::decode_wire_value(&bytes).is_err(), "{id}: a truncated or overlong varint must fail the reader");
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:9003:                let decoded = pack_rt::decode_wire_value(&bytes).expect("the shared permissive varint reader admits redundant encodings");
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:349:    let value = store::pack_rt::decode_wire_value(descriptor).map_err(|error| RunError::Host(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:452:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| RunError::Host(error.to_string()))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:457:    let decoded = store::pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:475:    let Ok(value) = store::pack_rt::decode_wire_value(report) else { return String::new() };
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1844:        let decoded = store::pack_rt::decode_wire_value(&bytes).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: decoding `{}` as a pack: {error}", path.display())))?;
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧪️tests/🔬️unit/🦀️.rs:790:    let wire: semio_framework_plugin::app::MediaArtifactDescriptor = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(&wire_descriptor).expect("descriptor")).expect("typed descriptor");
✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:8:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:13:        let value = protocol::pack_rt::decode_wire_value(bytes).map_err(|error|malformed(error.to_string()))?;
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:12:        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "lowpoly-diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs:59:    let value = store::pack_rt::decode_wire_value(&bytes).expect("first-party wire decoder");
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:10:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/🦀️.rs:316:                let value = store::pack_rt::decode_wire_value(&bytes).map_err(|e| malformed("op payload", reader.position(), e.to_string()))?;
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:10:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:10:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🔬️unit/🦀️.rs:119:    let decoded_value = pack_rt::decode_wire_value(&execution.canonical_payload).expect("summary decodes");
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs:36:    let value = store::pack_rt::decode_wire_value(&bytes[1..]).map_err(|error| malformed("diff body", 1, error.to_string()))?;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs:25:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs:26:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:11:        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs:23:    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;

```

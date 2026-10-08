# Value Cow Refusal Consumer and Schema Audit 6

Read-only current physical source audit. No edits, compilation or tests.

Current Value refusal owner defines message Cow static str, dynamic new using Cow Owned, literal using Cow Borrowed and consuming into_message using into_owned. This accurately distinguishes allocation-free static budget refusals from owned dynamic diagnostics. Borrowing Display, contains/as_bytes/len and control.copy_text borrow work with Cow; explicitly owned String result/field boundaries must consume into_message or into_owned, not assume the public field remains String.

Exact compile-sensitive boundary: Record derive schema/✨️derive/🦀️.rs889 emits DslField::from_value returning Result Self,String but map_err e.message now yields Cow. This generated code must use the consuming owned-message operation. Generic constructor accepting Into String can consume Cow, whereas branch expressions mixing error.message.clone with format String need an explicit common owned type. Do not globally modify similarly named message fields: many belong to other error types. The lexical roster below includes files naming ValueError but individual variable types still require local confirmation/compiler evidence.

## Current Lexical Consumer Roster

```text
🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs
42:             Err(error) => MutationOutcome::error(error.code, error.message, error.target),

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
228:         Err(error) => Err(messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect()),

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
1271:                 let message = rejected.fault.message.clone();
1816:             .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation2d.eval-session-owner", error.message))?

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
2283:         cad_import_object_emit(doc, CadPaneId::Shape, "import-media", imported).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs
233:     let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&drawing_to_svg_io_key(), std::slice::from_ref(&source))).map_err(|error| error.message)?;

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
25:  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<Generation2dSnapshot as store::ArtifactPack>::encode_pack(&source)),SnapshotEncoding::Text=>store:
32:  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<Generation2dSnapshot as store::ArtifactPack>::encode_pack(&source)),SnapshotEncoding::Text=>store:

✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
1833:                 .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner", error.message))?
1854:                 .map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner",error.message))?.unwrap_or_default(),

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🦀️.rs
74: pub(crate) fn invalid(message: &'static str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs
253:     let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&key, std::slice::from_ref(&source))).map_err(|e| format!("layout->semio/drawing->svg: {}", e.message))?;

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
1272:                     4=>if let Some(export)=self.envelope.as_mut().unwrap().step(1,&mut control).map_err(|error|semio_framework_value::ValueError::new(error.kind,error.message))?{
2209:         }).map_err(|error| MediaError::Payload(port.into(), error.message))?.map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
2641:             .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation3d.eval-session-owner", error.message))?

✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs
39:         cause: semio_framework_value::ValueError::new(error.kind,format!("JsonIntoDraw: {}",error.message)),
42:             message: error.message, expected: error.expected.map(|token|ExpectedSet {tokens:vec![token],keywords:Vec::new(),keys:Vec::new()}), scope:FaultScope::default(),

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
1597:         tree.map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation3d-view.eval-session-owner", error.message))?

✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
664:         let spec = playbook_composed_spec(doc.snapshot, &doc.children).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("{}: {}", fault.code.0, fault.message)))?;
682:         let refused = |fault: Fault| MediaError::Payload(port.to_string(), format!("{}: {}", fault.code.0, fault.message));
698:         let spec = || playbook_composed_spec(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new(fault.code.0, fault.message));

✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
106:                 let spec = crate::playbook_composed_spec(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new(fault.code.0, fault.message))?;

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
749:         Err(error) => Err(messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect()),

✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs
503:             .explanation(copy(format!("Part 1 structural error at {}: {}", err.field, err.message), format!("Teil-1-Strukturfehler bei {}: {}", err.field, err.message)));
555:                     copy(format!("Fix Part 1 field `{}`: {}.", err.field, err.message), format!("Teil-1-Feld `{}` korrigieren: {}.", err.field, err.message)),

✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
34:  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<Generation3dSnapshot as store::ArtifactPack>::encode_pack(&source)),SnapshotEncoding::Text=>store:

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
848:                 let state = crate::content::composed(doc.snapshot, &doc.children).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))?;
915:         let composed = || crate::content::composed(state, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.rewriting.working-graph", fault.message));
964:  let composed=crate::content::composed(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;let state=&composed;

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/💰️backing/🗂️indexes/🦀️.rs
6: pub(super) fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs
1181:         let snapshot = <space::SpaceSnapshot as store::ArtifactDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
1450:                                     let snapshot = <space::SpaceSnapshot as store::ArtifactDsl>::parse_dsl(&text_files.dsl).map_err(|error| VcsError::Deserialize(error.message))?;
3788:             let message = AbiMessageBytes::try_new(failure.message.clone()).unwrap_or_else(|_| AbiMessageBytes::from_text("bounded OS host codec failure").expect("fixed fallback is bounded"));
3927:             let failure_empty = self.terminal_failure.as_ref().is_none_or(|failure| failure.message.is_empty());
3947:                 if failure.message.pop().is_some() {

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
1038:         let content = || crate::jack_content_from_children(snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.jack.content", fault.message));
1097:  let raw=crate::jack_content_from_children(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
103:                 let content = crate::jack_content_from_children(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.jack.content", fault.message))?;

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs
333:     let error=query_ownership_preparation_rejection(&node_source);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message,"query entity exceeds its byte grant");
338:     let error=query_ownership_preparation_rejection(&edge_source);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message,"query entity exceeds its byte grant");
372:     assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message, "query result exceeds its retained ownership grant");

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs
122:             Self::Manifest(error) => write!(formatter, "{}: {}", error.path, error.message),

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🦀️.rs
1498:                         self.fail(code.message.as_bytes());

✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs
208:     assert!(err.message.contains("unknown mutation line"));
231:         match self.message.take() {
238:         self.message.is_none()

✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
73:  let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(128,&mut accepted);let error=<crate::CatalogueValue as DslField>::from_value_controlled(&input,&mut control).unwrap_err();assert!(error.message.len()<f["maximum

✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
754:                 let message = rejected.fault.message.clone();

🧰️framework/🔨️modules/⚠️diagnostic/🚧️text-error/🧪️tests/🦀️.rs
45:     let script = "import Ajv from'ajv/dist/2020.js';import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const a=new Ajv({strict:true}).addSchema(x.valueSchema).addSchema(x.diagnosticSchema);const v=a.getSchema(x.diagnosticSche

🧰️framework/🔨️modules/⚠️diagnostic/🎛️controlled/🦀️.rs
75:     record(c, 3 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "kind", &value.kind.as_str(), c)?; push(fields, "message", &value.message, c)?; push(fields, "span", &value.span, c)?; if let Some(value) = &value.expected { push(fie
81:     let message = text(required(message, "missing TextError.message")?, "expected a string for TextError.message", c)?;
114: pub(super) fn encode_cause(value: &FaultCause, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { record(c, 1 + usize::from(value.code.is_some()), |fields, c| { push(fields, "message", &value.message, c)?; if let Some(value) = &value.
117:     Ok(FaultCause { message: text(required(message, "missing FaultCause.message")?, "expected a string for FaultCause.message", c)?, code: optional(code, c)? })
120:     record(c, 5 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "code", &value.code, c)?; push(fields, "severity", &value.severity, c)?; push(fields, "span", &value.span, c)?; push(fields, "message", &value.message, c)?; if let So
127:     let message = text(required(message, "missing Diagnostic.message")?, "expected a string for Diagnostic.message", c)?.guard_decoded();
155:     record(c, 6 + usize::from(value.span.is_some()) + usize::from(!value.causes.is_empty()) + usize::from(params.is_some()), |fields, c| { push(fields, "origin", &value.origin, c)?; push(fields, "code", &value.code, c)?; push(fields, "severity", &val
162:     let message = text(required(message, "missing Fault.message")?, "expected a string for Fault.message", c)?.guard_decoded();

🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs
21:         write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
32:         let mut entries = vec![("kind".to_string(), DslValue::String(self.kind.as_str().into())), ("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
51:             other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
76:     pub fn from_value_error(error: ValueError, span: TextSpan) -> Self { Self::new(error.kind, error.message, span) }
287:             Some(expected) => TextError::expected(kind, self.message, self.span, expected),
288:             None => TextError::new(kind, self.message, self.span),
301:             ("message".to_string(), DslValue::String(self.message.clone())),
324:             other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
501:         let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
519:             other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
537:             ("message".to_string(), DslValue::String(self.message.clone())),
590:             other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
625:         Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
631:         Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
674:         format!("{}: {}", self.code.0, self.message)
723:         self.message.clone()
733:     serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
757:     truncate_fault_text(&mut bounded.message, 192);
769:         if !bounded.message.is_empty() {
770:             bounded.message.pop();

🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs
52:     let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::record::schema_hash_controlled(&spec,&mut zero).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summa
56:     let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::record::schema_hash_controlled(&spec,&mut short).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summ
63:     let((first,kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let first=crate::record::schema_hash_controlled(&spec,&mut pair).unwrap();let error=crate::record::schema_hash_controlled(&spec,&mut pair).unwrap_err().into_v
68:      let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::record::schema_hash_controlled(&spec,&mut canceled).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);

🧰️framework/🔨️modules/⚠️diagnostic/🧪️tests/🎛️controlled/🦀️.rs
136:     let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };
139:     let mut cancel = |p: semio_framework_value::native_encoding::NativeEncodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };

🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🚦️refusals/🦀️.rs
15:         let kind=refusal.kind;let message=refusal.message.clone();let wrapped=PackRefusal::from(refusal);let PackRefusal::ValueRefusal(refusal)=&wrapped else{panic!("typed refusal lost at Pack boundary")};let source=std::error::Error::source(&wrapped

🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs
214:     assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::Canceled,..})));
242:     assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::DepthLimit,..})));assert_eq!(output.len(),0);finish(&mut output);
279:  let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let mut refusal=None;for row in rows{if let Err(error)=scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text).
283:  let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=scratch.note(&source,SourceTextLocator::new(&[0],SourceTextKind::Text).unwrap(),false,&mut canceled).expect_err("actual post-backing cancellation");let re
319:  let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=capsule.write_body(&options,&mut measured,&mut narrow).unwrap_err().into_value_error();let result=(error.kind,error.message.capacity());drop(error);result
322:  let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=capsule.write_body(&options,&mut output,&mut control).unwrap_err().into_value_error();let result=(error.kind,error.message.capacity());drop(error);result}
401:   let((result,diagnostic),requested,released)=crate::test_allocation::observe_backing(||match capsule.write_body(&Default::default(),&mut output,&mut control){Ok(length)=>(Ok(length),0),Err(error)=>{let error=error.into_value_error();let result=(Err(

🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🦀️.rs
41:  let ((actual,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=measure(source,spec,pack,maximum,control).expect_err("actual borrowed refusal");let result=(error.kind,error.message.capacity());drop(error);result});
145:  options.limits.max_depth=maximum-1;assert!(encode_document(&ordinary,&record,&options).is_err());let mut control=NativeEncodeControl::new(0,&mut allow);let((kind,diagnostic),requests,releases)=crate::test_allocation::observe_backing(||{let refusal=m

🧰️framework/🔨️modules/🎒️pack/🧪️tests/🧭️producer-authority/🦀️.rs
40:    "ValueRefusal"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackRefusal::from(cause)},
41:    "TextRefusal"=>{let cause=TextError::new(authored_kind(row["kind"].as_str().unwrap()),message,TextSpan{line:2,column:3,length:1});owned_pointer=Some(cause.message.as_ptr());PackRefusal::from(cause)},
42:    "Io"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackRefusal::Io{error:cause,retry:match row["retry"].as_str().unwrap(){"never"=>PackRetryDisposition::Never,"transien
55:     PackRefusal::ValueRefusal(cause)|PackRefusal::Io{error:cause,..}=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<ValueError>().unwrap(),cause));},
56:     PackRefusal::TextRefusal(cause)=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<TextError>().unwrap(),cause));},

🧰️framework/🔨️modules/🎒️pack/🧪️tests/📡️codec-authority/🦀️.rs
71:         let oracle=serde_json::json!({"kind":row["convertedKind"],"message":row["display"]});assert_eq!(serde_json::json!({"kind":converted.kind.as_str(),"message":converted.message}),oracle);
85:         let expected_kind=kind(row["kind"].as_str().unwrap());let mut cause=ValueError::new(expected_kind,row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}let allocation=cause.me
87:         let CanonicalRefusal::ValueRefusal(refusal)=&wrapper else{panic!("typed cause")};assert_eq!(refusal.kind,expected_kind);assert_eq!(refusal.message.as_ptr(),allocation);assert_eq!(wrapper.to_string(),row["display"].as_str().unwrap());
89:         let converted=wrapper.into_value_error();assert_eq!(converted.kind,expected_kind);assert_eq!(converted.message.as_ptr(),allocation);assert_eq!(converted.message,row["display"].as_str().unwrap().strip_prefix("schema error: ").unwrap());

🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs
77:     pub fn into_value_error(self) -> ValueError { match self { Self::Native(error)=>error, error=>ValueError::new(error.kind(),error.to_string()) } }

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs
359:             semio_framework_dsl_record::FieldValue::Record(record) => media_contract_from_record(record).map_err(|e| e.message),
593:             semio_framework_dsl_record::FieldValue::Record(record) => workflow_media_port_from_record(record).map_err(|e| e.message),

✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs
23:         let wire = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError::from_value_error(match error.into_value_error() { Ok(cause) => semio_framework_value::ValueError::new(cause.kind, format!("PdfIntoPresentation: {}", 
25:         let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| { let mut cause=error;cause.message=format!("PdfIntoPresentation: {}",cause.message);IoE

✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs
25:         let value = parse_json_text(text).map_err(|mut error| { error.message=format!("JsonIntoPresentation: {}",error.message);IoError::from_text_error_controlled(error,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true)).u
28:         let mut out: PresentationSnapshot = semio_framework_value::FromValue::from_value(dsl_value).map_err(|error| { let mut cause=error;cause.message=format!("JsonIntoPresentation: {}",cause.message);IoError::from_value_error(cause) })?;

🧰️framework/🔨️modules/🎒️pack/⚠️error/🔌️capture/👥️context/🧪️tests/🦀️.rs
192:   let original=semio_framework_value::ValueError::new(kind,row["message"].as_str().unwrap());let pointer=original.message.as_ptr();
195:    Err(fault)=>{let TransportContextRefusalCause::Value(error)=fault.cause else{panic!("owned creation refusal was erased")};assert_eq!(error.kind,kind);assert_eq!(error.message.as_ptr(),pointer);assert_eq!(fault.admission.committed_bytes(),16);},
199:     assert_eq!(cause.kind,kind);assert_eq!(cause.message.as_ptr(),pointer);assert_eq!(context.committed_bytes(),before);

🧰️framework/🔨️modules/🎒️pack/⚠️error/🪶️refusal/🦀️.rs
46:  pub fn into_value_error(self)->ValueError{let kind=self.kind();match self{Self::ValueRefusal(error)|Self::Io{error,..}=>error,Self::TextRefusal(error)=>ValueError::new(error.kind,error.message),error=>ValueError::new(kind,error.to_string())}}

🧰️framework/🔨️modules/🎒️pack/⚠️error/🧪️tests/⚠️refusal/🦀️.rs
22:   let allocation=error.message.as_ptr();
26:   assert_eq!(refusal.message.as_ptr(),allocation);
31:   assert_eq!(serde_json::json!({"kind":format!("{:?}",refusal.kind),"message":refusal.message}),reference);

🧰️framework/🔨️modules/🎒️pack/⚠️error/🧪️tests/🔁️value-projection/🦀️.rs
14:             let pointer=match &error{PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>error.message.as_ptr(),PackRefusal::TextRefusal(error)=>error.message.as_ptr(),_=>panic!("declared owned cause")};
15:             let wrapped=PackError::from(error.clone());let PackError::Refusal(ref inner)=wrapped else{panic!("statically semantic source")};let outer_pointer=match inner{PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>error.message.as_ptr
17:             assert_eq!(bytes,usize::try_from(fixture["contract"]["nativeOwnedProjectionBytes"].as_u64().unwrap()).unwrap());assert_eq!(value.message.as_ptr(),pointer);
18:             let output=serde_json::json!({"variant":variant,"kind":value.kind.as_str(),"message":value.message});let oracle=serde_json::json!({"variant":variant,"kind":token,"message":fixture["ownedMessage"]});assert_eq!(output,oracle);
19:             let(result,bytes)=crate::test_allocation::observe(||wrapped.into_value_error());let value=result.expect("declared wrapped semantic projection");assert_eq!(value.message.as_ptr(),outer_pointer);assert_eq!(bytes,0);assert_eq!(serde_json::js
21:         let value=PackRefusal::LimitExceeded{kind,limit:"same misleading canceled prose"}.into_value_error();assert_eq!(serde_json::json!({"kind":value.kind.as_str(),"message":value.message}),serde_json::json!({"kind":token,"message":fixture["structu
22:         let refusal=TransportCaptureRefusal{kind,reason:"same misleading canceled prose",allocated_bytes:usize::try_from(fixture["admission"]["retainedBytes"].as_u64().unwrap()).unwrap(),physical_bytes:usize::try_from(fixture["admission"]["physicalBy

🧰️framework/🔨️modules/🗜️deflate/🧪️tests/🔬️unit/🛫️controlled/🦀️.rs
25:  let f=fixture();let raw=lcg_bytes(f["random"]["seed"].as_u64().unwrap(),f["random"]["length"].as_u64().unwrap() as usize);for phase in[DeflateEncodePhase::Initialize,DeflateEncodePhase::ScanInput,DeflateEncodePhase::MatchSearch,DeflateEncodePhase::W
26:  let mut denied=Control::new(f["deniedBytes"].as_u64().unwrap() as usize);assert!(deflate_controlled(&raw,&mut denied).unwrap_err().message.contains("admission"));assert_eq!(denied.owned,0);
59:  assert!(result.unwrap_err().message.contains("canceled"));assert_eq!(WATCHED_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed),0);assert_eq!(control.events.last().unwrap().phase,DeflateEncodePhase::WriteOutput);assert_eq!(control.events.last().

🧰️framework/🔨️modules/🗜️deflate/🧪️tests/🔬️unit/🛫️controlled/⚠️refusal/🦀️.rs
21:   let error=ValueError::new(refusal_kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap().to_owned());let pointer=error.message.as_ptr();
24:   assert_eq!(actual.kind,refusal_kind(row["kind"].as_str().unwrap()));assert_eq!(actual.message,row["message"].as_str().unwrap());assert_eq!(actual.message.as_ptr(),pointer);
26:   assert_eq!(serde_json::json!({"kind":actual.kind.as_str(),"message":actual.message}),serde_json::json!({"kind":row["kind"],"message":row["message"]}));
31:   assert_eq!(actual.kind,refusal_kind(row["kind"].as_str().unwrap()));assert_eq!(actual.message,row["message"].as_str().unwrap());

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🧽️retract-run-log/🦀️.rs
29:         Ok(base.logs[kept..].iter().rev().map(|line| RunMutation::AppendRunLog(AppendRunLog { node_id: line.node_id.clone(), level: line.level.clone(), message: line.message.clone(), at: line.at.clone() })).collect())

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🪵️append-run-log/🦀️.rs
21:         protocol::MutationOutcome::new(RunDiff::step(RunStep::LogAppend(RunLogLine { node_id: self.node_id.clone(), level: self.level.clone(), message: self.message.clone(), at: self.at.clone() })))

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🦀️.rs
227:             semio_framework_dsl_record::FieldValue::Record(record) => run_trigger_from_record(record).map_err(|e| e.message),

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/📏️value/🦀️.rs
24:  for value in&value.logs{c.bytes(24)?;for text in[&value.node_id,&value.level,&value.message,&value.at]{c.text(text)?;}c.step()?;}if c.completed!=rows{return Err(invalid("Run complete SQL cell traversal differs from admitted rows"))}c.control.check_v

🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs
45:   for(index,value)in self.logs.iter().enumerate(){out.insert("run_log",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(&value.node_id),Cell::Text(&value.level),Cell::Text(&value.message),Cell::Text(&value.at)])?;if(index+1)%256==0{out.ch

🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs
147:             let message = control.copy_text(&error.message)?;
163:                 message: error.message,

🧰️framework/🔨️modules/🚪️io/🧬️schema/⚠️refusal/🧪️tests/🦀️.rs
21:         let pointer=original.message.as_ptr();
23:         if operation=="value" {assert_eq!(error.cause.message.as_ptr(),pointer);}
25:         assert_eq!(error.cause.message,message);
29:             assert_eq!(error.diagnostics[0].message,message);
41:         if row["expectedKind"]=="success" {let output=result.unwrap();assert_eq!(output.cause.kind,ValueRefusalKind::InvariantViolated);assert_eq!(output.cause.message,message);assert_eq!(output.diagnostics[0].message,message);assert_eq!(output.diagn
79:         assert_eq!(decoded.cause.message,row["message"].as_str().unwrap());

🧰️framework/🔨️modules/🚪️io/🦀️.rs
1128:             None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
1589:     let entry = resolve(&key).await.map_err(|error| error.unavailable.map_or_else(|| IoWireError::Resolve(error.message), IoWireError::Registry))?;
1602:         Err(error) => Err(IoWireError::Resolve(error.message)),
2622:             let outcome = (entry.run)(&current).map_err(|error| IoError { cause: ValueError::new(error.cause.kind, format!("io_run: hop {} -> {} failed: {}", hop.from.to_coordinate(), hop.into.to_coordinate(), error.cause.message)), diagnostics: erro
2630:     /// the `IoError.cause.message` names which hop (by dialect coordinates) failed.

🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs
45:    assert_eq!(observed,admitted+error.message.capacity(),"failed scratch admission owns only admitted scratch and its explicit typed refusal message");
70:   assert_eq!(observed,error.message.capacity(),"start refusal must allocate no traversal or payload backing");

🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🪶️sqlite/🦀️.rs
6: pub(crate) fn invalid(message: impl Into<String>) -> ValueError {
173:                 assert!(decorated.message.starts_with(case["id"].as_str().unwrap()));

🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs
652:             Self::Refused(refusal) => formatter.write_str(&refusal.message),
658:     pub fn into_value_error(self) -> semio_framework_value::ValueError {
734:         let reason = messages.iter().filter(|message| message.level == semio_framework_diagnostic::Severity::Fatal).map(|message| message.message.clone()).collect::<Vec<_>>().join("; ");

🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs
70:     assert!(matches!(export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |progress| progress.completed < 2), Err(ValueError { kind: ValueRefusalKind::Canceled, .. })));
71:     assert!(matches!(import_sqlite_database(&source, SqliteDatabaseLimits::default(), &mut |progress| progress.completed < 2), Err(ValueError { kind: ValueRefusalKind::Canceled, .. })));
141:     assert!(matches!(result, Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. }))); assert!(visited <= 2, "overflow pages were read before enforcing the value budget");
144:     assert!(matches!(import_sqlite_database_controlled(&bytes,&mut control),Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. })));assert!(control.allocation_remaining_bytes()<SqliteDatabaseLimits::default().max_allocation_bytes);control.ch
164:     assert!(matches!(result, Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. }))); assert_eq!(visited, 1, "schema overflow was traversed before applying its budget");
169: fn keyed_projection_cancels_during_heap_ordering_and_rejects_duplicate_physical_identities(){use std::cell::Cell;let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏗️projection/🔣️.json")).unwrap();let count=f["keyedOrder"]
182:     let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️allocation/🔣️.json")).unwrap();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:f["maximum

🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs
209:                 let result = (error.kind, error.message.capacity());
233:                 let result = (error.kind, error.message.capacity());

🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs
100:     assert!(!error.message.is_empty());

🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧪️tests/🦀️.rs
33:         assert!(!error.message.is_empty());

🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🧮️allocation/🦀️.rs
19:  for cap in f["tinyAllocationBytes"].as_array().unwrap(){let cap=cap.as_u64().unwrap()as usize;let mut accept=|_|true;let mut control=Control::new(&mut accept,limits(&f,cap));let(result,requested)=observe(||Projection::new(sql,&mut control).map(drop)
29:  for allowance in [cost,cost-1,0]{let mut accept=|_|true;let mut control=Control::new(&mut accept,limits(&f,n(&f,"allocationBytes")));let mut projection=Projection::new(sql,&mut control).unwrap();projection.control.limits.max_allocation_bytes=project
54:  for allowance in [cost,cost-1,0]{let mut accept=|_|true;let mut control=Control::new(&mut accept,limits(&f,allowance));let(result,requested)=observe(||ordered_row_refs(&table,2,&mut control));if allowance==cost{drop(result.unwrap());assert_eq!(reque
63:  for allowance in [cost,cost-1,0]{let mut accept=|_|true;let mut control=Control::new(&mut accept,limits(&f,n(&f,"allocationBytes")));let mut projection=Projection::new(sql,&mut control).unwrap();projection.control.limits.max_allocation_bytes=project
64:  let armed=Local::new(false);let mut callback=|_|!armed.get();let mut control=Control::new(&mut callback,limits(&f,n(&f,"allocationBytes")));let mut projection=Projection::new(sql,&mut control).unwrap();let before=projection.control.allocation_remain
82:  let cancelled=Local::new(false);let mut callback=|event:SqliteSnapshotProgress|{let stop=event.phase==Phase::ProjectSnapshot&&event.total==owner.len()&&event.completed>=n(&f["large"],"cancelAfterBytes")&&event.completed<owner.len();cancelled.set(can
93:  for allowance in [cost,cost-n(&f["reconstruction"],"refusedAllocationDelta")]{let mut accept=|_|true;let mut control=Control::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:allowance,..Default::default()});let before=control.allocation_re
94:  let interior=Local::new(false);let mut callback=|event:SqliteSnapshotProgress|{let stop=event.phase==Phase::ReconstructSnapshot&&event.total==text.len()&&event.completed>=n(&f["reconstruction"],"cancelAfterBytes")&&event.completed<text.len();interio

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs
19: pub fn projection_path_error()->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained field projection source path changed")}

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🪆️refusal/🚪️public/🦀️.rs
52:             assert_eq!(error.message,row["expectedMessage"].as_str().unwrap());

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🪆️refusal/🦀️.rs
62:             assert_eq!(error.message, row["expectedMessage"].as_str().unwrap());

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/⚠️refusal/🦀️.rs
6: impl RefusingControl { fn refusal(&self)->ValueError { ValueError::new(self.kind,self.message.clone()) } }
33:         assert_eq!(error.kind,kind);assert_eq!(serde_json::json!({"kind":error.kind.as_str(),"message":error.message}),row["expected"]);

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🔬️unit/🦀️.rs
99:     assert!(error.message.contains("jack"), "{error}");
238:     assert!(err.message.contains("coordinate") || err.message.contains("expected"), "{err}");
241:     assert!(err2.message.contains("dimension"), "{err2}");
244:     assert!(err3.message.contains("count"), "{err3}");
251:     assert!(error.message.contains("not compatible"), "wrong-dimension suffix must be a parse error, got: {error}");

🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs
889:                     ::semio_framework_dsl_record::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),

✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
2797:                         Err(error) if error.kind == semio_framework_value::ValueRefusalKind::Canceled => return Some(semio_framework_job::StepOutcome::Cancelled), Err(error) => return Some(sequence_job_fault(cx, &error.message)),
2814:                     match result { Ok(Some(value)) => self.input = Some(neural_engine::retirement::RetainedDictionaryInput::new(sequence_import_parameter_value(value))), Ok(None) => {}, Err(error) if error.kind == semio_framework_value::ValueRefusalK
2865:                 Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault))|Err(fault)=>return sequence_job_fault(cx,&fault.message),
2880:                 let message = rejected.fault.message.clone();
2942:             let step = close.close_step(maximum_items, maximum_bytes).map_err(|error| sequence_fault("sequence.retained.close", error.message))?;

🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🆔️ids/🧪️tests/🔬️unit/🦀️.rs
59:             "ValueRefusal"=>{let mut cause=ValueError::new(kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}if cause.kind==ValueRefusalKind::I

🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs
52:     let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut zero).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);summ
56:     let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut short).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error);sum
63:     let((first,kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let first=crate::os_pack::schema_hash_controlled(&spec,&mut pair).unwrap();let error=crate::os_pack::schema_hash_controlled(&spec,&mut pair).unwrap_err().into
68:      let((kind,diagnostic),actual,released)=crate::test_allocation::observe_backing(||{let error=crate::os_pack::schema_hash_controlled(&spec,&mut canceled).unwrap_err().into_value_error();let summary=(error.kind,error.message.capacity());drop(error)

🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🚦️refusals/🦀️.rs
15:         let kind=refusal.kind;let message=refusal.message.clone();let wrapped=PackRefusal::from(refusal);let PackRefusal::ValueRefusal(refusal)=&wrapped else{panic!("typed refusal lost at Pack boundary")};let source=std::error::Error::source(&wrapped

🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️binding/🦀️.rs
62:     let mut incomplete=record.clone();incomplete.fields.remove(&StagedRecord::__dsl_spec().fields[1].id);let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let error=StagedRecord::__dsl_

🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs
12: impl From<crate::list::PagedListError> for ValueError {

🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️tests/🛬️controlled/🦀️.rs
91:         match actual{Ok(value)=>assert_eq!(value,oracle),Err(error)=>assert_eq!(error.message,row["controlledError"].as_str().unwrap())}
239:     let value=DslValue::from(&value);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut callback);let error=Recursive::from_value_controlled(&value,&mut control).err().unwrap();assert!(error.message.contains("depth limit
307:     let error=std::collections::HashMap::<UncontrolledKey,bool>::from_value_controlled(&value,&mut control).err().unwrap();assert!(error.message.contains("key owner"));
451:     assert_eq!(error.message,"duplicate object key");

🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️tests/🛫️controlled/🦀️.rs
99:  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();let mut root=DslValue::Null;for _ in 0..fixture["frontier"]["intrinsicDepth"].as_u64().unwrap(){root=DslValue::Array(vec![root]);}l
100:  let mut source=NestedOutput::End;for _ in 0..fixture["frontier"]["typedDepth"].as_u64().unwrap(){source=NestedOutput::Next(Box::new(source));}let mut callback=|_|true;let mut control=NativeEncodeControl::new(10_000_000,&mut callback);assert!(source.

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs
1337:         disposer.close_step(&mut partition.store, grant.maximum_items.min(1), grant.maximum_bytes).map_err(|fault| fault.message)

✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
156:     crate::wires_composed_from_children(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new(fault.code.0, fault.message))

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔤️utf8/🦀️.rs
4: impl From<std::str::Utf8Error> for ValueError {
11: impl From<std::string::FromUtf8Error> for ValueError {

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔤️utf8/🧪️tests/🦀️.rs
15:                 assert_eq!(projected.message, row["expected"]["display"].as_str().unwrap());
26:                 assert_eq!(projected.message, row["expected"]["display"].as_str().unwrap());

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🦀️.rs
39: impl ValueError {
47:             let message = DslValue::String(control.copy_text(&self.message)?);

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧪️tests/🦀️.rs
18:         assert_eq!(error.message, row["wire"]["message"].as_str().unwrap());
66:     assert_eq!(source.get("message").unwrap().as_str().unwrap(), original.message);

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🦀️.rs
14: pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
15: impl ValueError {
21:     pub fn under(self, segment: impl std::fmt::Display) -> Self { Self::new(self.kind, format!("{segment}.{}", self.message)) }
23:     pub fn into_message(self) -> String { self.message.into_owned() }
25: impl std::fmt::Display for ValueError { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) } }
26: impl std::error::Error for ValueError {}

🧰️framework/🔨️modules/🌱️value/⚠️refusal/🧪️tests/🦀️.rs
55:     let script = "import Ajv from 'ajv/dist/2020.js';import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const valid=new Ajv({strict:true}).compile(x.schema);const db=new Database(':memory:');db.run('CREATE TABLE path(position

✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
108:             canvas::WIRES_VIEW_BODY_CANVAS => canvas::render(&crate::wires_composed_from_children(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new(fault.code.0, fault.message))?),

🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs
545:     assert!(error.message.contains("cannot progress"));
566:     assert!(error.message.contains("cannot progress"));
623:     assert!(error.message.contains("exceeded its retained clone"));
653:                 assert!(error.message.contains("exceeded its retained clone"));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🦀️.rs
105:             cause: semio_framework_value::ValueError::new(self.kind, self.message),
134:         assert_eq!(restored.message, "invalid exact subset");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
1577:             write!(formatter, "{}: {}", self.code, self.message)
2137:         ArtifactIdentity::parse(&request.source_dialect).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.source-dialect", error.message()))?;
2163:                 ArtifactIdentity::parse(owner).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.dependencies", error.message()))?;
2188:                 .saturating_add(diagnostic.message.len())
2359:             &self.message
2365:             write!(formatter, "{}: {}", self.code, self.message)
2388:             Self::new(error.code, error.message)
2394:             write!(formatter, "{}: {}", self.code, self.message)
3984:             write!(formatter, "{}: {}", self.code, self.message)
4916:             write!(formatter, "{}: {}", self.code, self.message)
8265:                 let command = A::command_from_action(&action.id, Some(&staged)).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
8561:                 DeclaredVerbOutcome::Unreachable { code, detail: fault.message }
8563:                 DeclaredVerbOutcome::Refused { code, detail: fault.message }
8853:                 let bridge = A::command_from_action(&action.id, Some(&staged)).await.map(drop).map_err(|fault| format!("{}: {}", fault.code.0, fault.message));
9125:             assert!(outcome.rejection.is_none(), "prepare unexpectedly rejected: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
15498:                 Self::Import(error) => formatter.write_str(&error.message),
16050:             let archive = self.document_archive().await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
17104:             while let Some(chunk) = self.chunks.take_chunk().map_err(|error| MediaError::Payload(self.schema.clone(), error.message))? {
22738:                 panic!("action {index} was refused with '{}' ({}) after {highest} of {ARTIFACT_LIVE_OUTPUT_SLOTS} typed-operation slots were never released by the host continuation", fault.message, fault.code.0);
22786:             panic!("typed ingress refused '{verb}' with '{}' ({})", fault.message, fault.code.0);
30106:                 .map_err(|fault| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", fault.message))?;
30276:                                 return Err(plugin_sdk_fault(format!("child root capture failed ({}) and group compensation moved {}/{} members", fault.message, compensation.undone.len(), receipt.member_edits.len())));
30954:                             format!("{carrier}: inline {action} was not applied ({}: {})", fault.code.0, fault.message),
31196:             self.refresh_cache().await.map_err(|error| fault(error.message))?;
31197:             let proof = self.qualified_tool_proof(&tool_id).map_err(|error| fault(error.message))?;
31219:             let lease = self.tool_cancellations.begin(operation_key).map_err(|error| fault(error.message))?;
31266:             let job = A::build_media_export_job(request).map_err(|error| fault(error.message))?.ok_or_else(|| fault(format!("media output port '{port}' is registered but has no concrete resumable producer")))?;
31356:                         return self.finish_media_poll(handle.operation_id.0, active, ArtifactMediaExportPoll::Failed(error.message));
31370:                         return self.finish_media_poll(handle.operation_id.0, active, ArtifactMediaExportPoll::Failed(error.message));
31390:                         Err(error) => return self.finish_media_poll(handle.operation_id.0, active, ArtifactMediaExportPoll::Failed(error.message)),
31394:                         Err(error) => return self.finish_media_poll(handle.operation_id.0, active, ArtifactMediaExportPoll::Failed(error.message)),
31397:                         return self.finish_media_poll(handle.operation_id.0, active, ArtifactMediaExportPoll::Failed(error.message));
32041:             let whole_import = match self.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
32107:             let args = match self.import_staging.admit_args(Some(&args)).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
32708:                 crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {} publication attempt {} faulted: {}: {}", mounted.verb, mounted.publication_attempt, fault.code.0, fault.message));
32912:                 crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
37209:                 self.refresh_cache().await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
37255:             let archive = PluginApp::document_archive(self).await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
37292:                 .map_err(|fault| MediaArtifactError::Payload(fault.message))
37310:             self.refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
37354:             let output = ArtifactDownloadOutput::from_media_export(handle.clone(), result.mime_type, result.chunks).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
37366:             let chunk = output.chunks.take_chunk().map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
37400:             self.refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
39747:                 let blocking: Vec<String> = messages.iter().filter(|message| protocol::MergePolicy::Normal.rejects(message.level)).take(8).map(|message| format!("{}: {}", message.code.0, message.message)).collect();
40381:                                     return Self::fault(cx, &error.message);
40387:                                     return Self::fault(cx, &error.message);
40408:                         Err(error) => return Self::fault(cx, &error.message),
40424:                             let message = rejected.fault.message.clone();
40503:                 let step = close.close_step(maximum_items, maximum_bytes).map_err(|error| Fault::from(error.message))?;
42782:                 *runtime.plugin_assembly_error.borrow_mut() = Some(Fault::new(FaultOrigin::Plugin, FaultCode::new(error.code), error.message));
42800:                 label: fault.message,
43184:                     eprintln!("[TRACE] maintenance stage={} elapsed_us={} outcome={:?}", crate::app::LAST_MAINTENANCE_STAGE.load(Ordering::Relaxed), finished_us - started_us, maintenance.as_ref().map(|_| ()).map_err(|fault| fault.message.clone()));
43213:                 Err(error) => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, error.message.as_bytes()) }),
43472:                 Err(fault) => return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, fault.message.as_bytes()) }),
43509:                 Err(error) => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, error.message.as_bytes()) }),
44547:         let route = semio_framework::io::io_mechanism::io_route(&dialect, &ArtifactDialect::from(SQLITE_SNAPSHOT), 1).await.map_err(|error| plugin_internal_fault(&error.cause.message))?.value;
44565:         let route = semio_framework::io::io_mechanism::io_route(&ArtifactDialect::from(SQLITE_SNAPSHOT), &dialect, 1).await.map_err(|error| plugin_internal_fault(&error.cause.message))?.value;
44861:         let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(port_id.to_string(), error.message))?;
44868:         let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
44875:         let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
44881:         let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
44887:         let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
45253:         .map_err(|fault| fault.message)
45293:         let (request, view_state) = wire.into_parts().map_err(|fault| fault.message)?;
45300:         .map_err(|fault| fault.message)
46346:                     let ingress = if fault.message == "runtime instance authority is busy" { ingress } else { ingress.cancel(fault) };
47250:                                     message: output.message,
47281:                                     message: output.message,

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs
340:                 assert_eq!(fault.message, row["rejectedFault"].as_str().expect("fixture rejected fault"), "{lane:?} terminal fault text");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs
1313:     assert!(fault.message.contains("mutation.target-missing") && !fault.message.contains("mutation.clamped"), "{}", fault.message);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs
1358:         for forbidden in ["A::handle", "A::ephemeral", "bounded_command_output_bytes", "serde_json::to_vec", "fault.message.as_bytes().to_vec()", "for child in emit.child_emits", "ActiveToolCommand", "BoundedFirstStepCommandJob"] {

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs
1321:             let bytes=error.message.capacity();

🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs
359:             write_optional_str(&mut out, &checkpoint.message);

🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs
114:     fn error_demand(error:&crate::ProtocolError)->usize{match error{crate::ProtocolError::Malformed{detail,..}|crate::ProtocolError::Io(detail)=>detail.capacity(),crate::ProtocolError::Pack(semio_framework_pack_error::PackError::Refusal(error))=>matc
383:                 fold.checkpoints.push(FoldCheckpoint { id: std::mem::take(&mut checkpoint.checkpoint_id), change_ids: change_ids.take(), parent_id: checkpoint.parent_id.take(), authors: std::mem::take(&mut checkpoint.authors), message: checkpoint.mes

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs
34:     assert_eq!(rejection.message, expected.cause.message);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs
482:             let step = retirement.close_step(maximum_items, maximum_bytes).map_err(|error| Fault::from(error.message))?;
1049:     assert!(error.message.contains(SURFACE_TOOL_ID), "the refusal names the editor's own verb, not the generic placeholder: {}", error.message);
1074:     assert!(!error.message.contains("does not match"), "the real canonical app id must satisfy the ownership check, got: {}", error.message);
1075:     assert!(error.message.contains("unknown action mode owner"), "expected ownership to pass and the mode lookup to fail instead, got: {}", error.message);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs
178:             ChildEmitPreparationStep::Refused(fault) => panic!("applying typed admission must never request unused wire codec: {}", fault.message),
390:     assert_eq!(error.message,demand["retirementRefusal"]["message"].as_str().unwrap());

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs
1816:     assert_eq!(fault.message, COMPOSED_PARENT_PROJECTION_REFUSAL_MESSAGE, "the app's own diagnostic survives the pump, not a flat framework string");
2317:     assert_eq!(actual.message, refusal["message"].as_str().expect("original message"));

🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs
837:         closes.begin_close(u64::from(instance), abandoned).map_err(|fault| Fault { code: "channel.not-wired".to_string(), message: format!("abandoned command owner: {}: {}", fault.code.0, fault.message) })?;
1072:         return GatewayError::new(GatewayErrorCode::Cancelled, fault.message);
1074:     GatewayError::new(GatewayErrorCode::Internal, fault.message)
1852:             semio_framework_plugin_host::PluginHostError::Refused(fault) => Fault { code: fault.code.0.clone(), message: fault.message.clone() },
2010:             let command = ::semio_framework_async::poll::resolve_ready(store::encode_app_command(real_command)).map_err(|fault| Self::not_wired("encoding AppCommand", format!("{}: {}", fault.code.0, fault.message)))?;
2012:             let mut owners = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| Self::not_wired("reserving command batch", format!("{}: {}", fault.code.0, fault.message)))?;
2015:                 return Err(Self::not_wired("admitting command owner", format!("{}: {}", fault.code.0, fault.message)));
2021:                     return Err(Self::not_wired("admitting command batch", format!("{}: {}", fault.code.0, fault.message)));
2031:             .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
2032:             .map_err(|fault| Self::not_wired("command page", format!("{}: {}", fault.code.0, fault.message)))?
2037:         self.pending_command_closes.prepare_suspend(u64::from(instance), seq).map_err(|fault| Self::not_wired("suspending command owner", format!("{}: {}", fault.code.0, fault.message)))?;
2041:                 self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
2045:                 self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
2058:             .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
2059:             .map_err(|fault| Self::not_wired("command acknowledgement", format!("{}: {} — the guest answered {:?} while this gateway drives instance {instance} seq {seq}", fault.code.0, fault.message, turn.command_ingress)))?;
2072:                 self.pending_command_closes.remove_terminal(u64::from(instance), seq).map_err(|fault| Self::not_wired("terminal command owner", format!("{}: {}", fault.code.0, fault.message)))?;
2074:                 self.await_response(instance, seq, &mut pending.response).map_err(|fault| Fault { code: fault.code, message: format!("{}; the acknowledging turn published {}", fault.message, named_shapes(&published)) })?;
2083:                 self.pending_command_closes.begin_close(u64::from(instance), seq).map_err(|fault| Self::not_wired("faulted command owner", format!("{}: {}", fault.code.0, fault.message)))?;
2369:     format!("{}: {}", decoded.code, decoded.message)
2922:     Fault { code: code.to_string(), message: error.message }
3472:         let role = hub.ready_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX)).map(|snapshot| snapshot.space.role).map_err(|error| error.message);
3526:                 return Err(Fault { code: fault.code, message: format!("the guest refused a batch its hub document delivered: {}", fault.message) });
4777:             .map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` refused ReadArtifact ({}): {}", fault.code, fault.message)))?;
4780:             Some(AppFrame::Error(fault)) => Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{plugin_id}` rejected ReadArtifact ({}): {}", fault.code, fault.message))),
5004:                     return (None, Some(error.message));
5016:             Err(error) => return (None, Some(error.message)),
5064:                             store::sync::ArtifactEvent::Conflict(message) if document_link_terminal_code(&message.code.0) => relay.record_terminal(&message.code.0, &message.message),
5065:                             store::sync::ArtifactEvent::Conflict(message) => relay.record(None, Some(format!("{}: {}", message.code.0, message.message))),
5094:         let frames = channel.exchange(0, vec![AppCommand::ReadArtifact]).map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{}` refused ReadArtifact ({}): {}", kind.plugin_id, fault.code, fault.message)))?;
5097:             Some(AppFrame::Error(fault)) => return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{}` rejected ReadArtifact ({}): {}", kind.plugin_id, fault.code, fault.message))),
5121:             .map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` refused ExportMedia on `{port}` ({}): {}", fault.code, fault.message)))?;
5124:             Some(AppFrame::Error(fault)) => Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{plugin_id}` rejected ExportMedia on `{port}` ({}): {}", fault.code, fault.message))),

🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs
524:         Self { content: vec![ContentBlock::Text { text: error.message.clone() }], structured_content: Some(error.to_tool_error_payload()), is_error: true }
1188:                 Err(error) => (false, error.message.clone()),

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/📦️preparation/🦀️.rs
169:             let bytes=error.message.capacity();
319:         if let Some(error)=self.close_refusal.as_ref(){return error.message.capacity().max(1);}
349:                 store::PackRefusal::ValueRefusal(error)|store::PackRefusal::Io{error,..}=>Ok((error.message.capacity()!=0).then_some(&mut error.message)),
351:                     if error.message.capacity()!=0{return Ok(Some(&mut error.message));}
375:     Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,"interactive-job.child-emission-retirement-refused",&error.message).with_param("refusalKind",error.kind.as_str())

🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/⚠️errors/🦀️.rs
99:         write!(formatter, "{:?}: {}", self.code, self.message)
125:         serde_json::json!({ "code": self.code, "message": self.message, "details": self.details, "retryable": self.retryable })
132:         (self.code.json_rpc_code(), self.message.clone(), serde_json::json!({ "gatewayCode": self.code, "details": self.details, "retryable": self.retryable }))

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs
432:     assert!(error.message.contains("no exact manifest declaration"), "the registry-less wrapper fails on the missing declaration, not on a missing factory: {}", error.message);

🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs
416:         VIEWER_READ_ONLY_FAULT_CODE => GatewayError::new(GatewayErrorCode::PermissionDenied, fault.message.clone())
418:         "capability-denied" => GatewayError::new(GatewayErrorCode::PermissionDenied, fault.message.clone()),
419:         HUB_EDIT_UNBOUND_FAULT_CODE => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone()),
420:         HUB_RELAY_UNACKNOWLEDGED_FAULT_CODE => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone()).retryable(),
421:         "app.command.rejected" => GatewayError::new(GatewayErrorCode::SideEffectRejected, fault.message.clone()),
422:         "transaction.member-rejected" => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone()),
423:         "interactive-job.not-ui-safe" => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone()),
424:         "interactive-job.preview-output" => GatewayError::new(GatewayErrorCode::InputInvalid, fault.message.clone()),
425:         COMMAND_TARGETS_REQUIRED_FAULT_CODE => GatewayError::new(GatewayErrorCode::InputInvalid, fault.message.clone()),
426:         COMMAND_NO_EFFECT_FAULT_CODE => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone())
428:         AGENT_LANE_UNCARRIED_FAULT_CODE | AGENT_LANE_PREVIEW_BUDGET_FAULT_CODE => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone())
430:         "transaction.generation-mismatch" => GatewayError::new(GatewayErrorCode::RevisionConflict, fault.message.clone()),
431:         "transaction.instance-busy" => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone()).retryable(),
432:         "budget.exceeded" => GatewayError::new(GatewayErrorCode::BudgetExceeded, fault.message.clone()).retryable(),
433:         "capability.not-found" => GatewayError::new(GatewayErrorCode::NotFound, fault.message.clone()),
434:         "plugin.unavailable" | "workspace.unbound" => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone()).retryable(),
435:         ACTIVATION_CANCELLED_FAULT_CODE => GatewayError::new(GatewayErrorCode::Cancelled, fault.message.clone()),
436:         code if code == store::sync::DocumentLinkStatus::AccessRevoked.code() => GatewayError::new(GatewayErrorCode::PermissionDenied, fault.message.clone()),
437:         code if code == store::sync::DocumentLinkStatus::LinkExpired.code() => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone()),
438:         _ => GatewayError::new(GatewayErrorCode::Internal, fault.message.clone()),
1142:                             warnings: vec![error.message.clone()],
1398:                 Err(error) => SettledApproval::Unreachable { details: serde_json::json!({ "approvalHandle": approval_handle, "channel": channel.as_str(), "reason": error.message }) },
1568:                 warnings.push(format!("member {label} failed: {}", error.message));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs
174:         assert!(zero_sources_err.message.contains("needs exactly 1 source"), "{}", zero_sources_err.message);
180:         assert!(two_sources_err.message.contains("needs exactly 1 source"), "{}", two_sources_err.message);
2067:             let skipped: Vec<String> = undone.diagnostics.iter().map(|diagnostic| diagnostic.message.clone()).collect();
3799:         assert_eq!(wire.message, fault.message);
3939:                 panic!("turn {turn}: the guest answered {:?} for the command this host owns as seq {seq}: {}: {}", result.command_ingress, fault.code.0, fault.message)
4838:         assert!(outcome.rejection.is_none(), "prepare admits a held child: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
5395:         assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");
7178:         assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);
7228:         assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);
7248:         assert!(error.message.contains("version"), "unexpected error: {}", error.message);
7328:         assert!(error.message.contains("not owned by app"), "unexpected error: {}", error.message);
7371:         assert!(error.message.contains("not owned by active mode edit"), "unexpected error: {}", error.message);
7495:         assert!(error.message.contains("bogus"), "unexpected error: {}", error.message);
7942:         assert!(result.diagnostics[0].message.contains("ghosts"), "the diagnostic names the refused domain: {}", result.diagnostics[0].message);
8486:         assert_eq!(fault.message, completion["checkpoint"]["completionFaultMessage"].as_str().unwrap());

🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs
31:         let mut entries = vec![("code".to_string(), crate::value::ToValue::to_value(&self.code)), ("message".to_string(), crate::value::ToValue::to_value(&self.message))];
64:         write!(formatter, "{}: {}", self.code, self.message)
1157:             ("message".to_string(), crate::value::ToValue::to_value(&self.message)),

🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs
116:         FixedCommandPage::try_copy_from(&bytes).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))
132:         FixedCommandPage::try_copy_from(&bytes).map_err(|fault| serde::de::Error::custom(fault.message))

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs
175:     assert!(cursor.close_step(1, 1).unwrap_err().message.contains("exceeded its grant"));
297:     let refusal=refusal.expect("a retirement that never progresses must be refused");assert_eq!(refusal.kind,semio_framework_value::ValueRefusalKind::InvariantViolated);assert!(refusal.message.contains("made no progress at its published close demand"

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🪆️refusal/🦀️.rs
25:         let error=ControlledRefusalRecord::__dsl_from_record_controlled(&record,&mut NativeDecodeControl::new(4096,&mut |_|true)).unwrap_err();assert_eq!(error.kind,kind);assert_eq!(error.message,corpus["derivedPaths"][1]["expectedMessage"].as_str().
28:         let error=value.__dsl_to_record_controlled(&mut NativeEncodeControl::new(4096,&mut |_|true)).unwrap_err();assert_eq!(error.kind,kind);assert_eq!(error.message,corpus["derivedPaths"][1]["expectedMessage"].as_str().unwrap());

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/📝️text/📸️snapshot/🦀️.rs
285:             semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => <WidgetDsl as semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
409:             semio_framework_dsl_record::FieldValue::Record(record) => flow_host_snapshot_dsl_to_host_snapshot(FlowHostSnapshotDsl::__dsl_from_record(record).map_err(|error| error.message)?),

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs
94:     let refusal=result.unwrap_err();assert_eq!(refusal.kind,semio_framework_value::ValueRefusalKind::InvariantViolated);assert_eq!(refusal.message,"flow mutation retirement frontier reported Complete before terminal-empty");

🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🪆️binding/🦀️.rs
11:    fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=><Self as semio_framework_dsl_record::DslVariants>::from_named_record(&value

🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs
34:     pub fn into_value_error(self) -> semio_framework_value::ValueError {

🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/⚠️emission/🦀️.rs
23:             assert_eq!(error.message,row["expected"]["message"].as_str().unwrap(),"{}",row["name"]);

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🧬️schema/🧬️mutations/🦀️.rs
115:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs
365:         (slot.generation == self.generation).then_some(slot.message)

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs
855:     assert!(refusal.message.contains("tail arrived before artifact bootstrap completion"), "the refusal names the order it enforced: {}", refusal.message);

✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs
630:         diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "adjacency.conflict".into(), message: conflict.message, entity_id: Some(conflict.adjacency_a_id), register: Some("adjacencies".into()) });
1021:         sections: vec![ReportSection { heading: "Diagnostics".into(), body: format!("{} diagnostic(s)", diagnostics.len()), bullets: diagnostics.iter().map(|d| format!("[{:?}] {}: {}", d.severity, d.code, d.message)).collect() }],
1623:     let mut findings: Vec<String> = adjacency_conflicts.iter().map(|c| format!("{}: {}", c.adjacency_a_id, c.message)).collect();
1635:             .map(|c| ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "analysis.conflict".into(), message: c.message, entity_id: Some(c.adjacency_a_id), register: Some("adjacencies".into()) })

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📨️messages/✂️clamp/🦀️.rs
66:                     self.measuring = Some(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()));
93:                         self.measuring = Some(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()));
122:                     let mut cut = self.budget.saturating_sub(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).saturating_sub(message.code.0.len()).min(message.message.len());
123:                     while !message.message.is_char_boundary(cut) { cut -= 1; }
124:                     let replacement = message.message[..cut].to_string();
125:                     let retired = MutationMessage { level: message.level, code: String::new().into(), message: std::mem::replace(&mut message.message, replacement), target: std::mem::take(&mut message.target), op_index: message.op_index };

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
1419:             state.strings[1] = Some(message.message);
1478:         if let Some(message) = self.message.as_mut() {
1483:             if let Some(bytes) = Self::take_string(&mut message.message).or_else(|| Self::take_string(&mut message.code.0)) {
1487:             drop(self.message.take());
1517:             *self.message = Some(message);
1553:         self.conflict.is_none() && self.message.is_none() && self.envelope.is_none() && self.bytes.is_none()
13478:             message: checkpoint.message,
13521:         checkpoints.push(Checkpoint { id: checkpoint.id, change_ids: checkpoint.change_ids, parent_id: checkpoint.parent_id, authors, message: checkpoint.message, timestamp: checkpoint.timestamp, composition_pins });
13718:     crate::os_spr::history::HistoryMessage { level: message.level.as_u8(), code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }
13724:     Ok(crate::os_spr::MutationMessage { level, code: semio_framework_diagnostic::FaultCode(message.code), message: message.message, target: message.target, op_index: message.op_index })
13782:     if message.level != expected_level || message.message.trim().is_empty() || message.target.iter().any(|target| target.trim().is_empty()) {
14638:         vcs: ArtifactVcs { genesis: ArtifactGenesis::born(initial_snapshot).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))?, edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alte
15881:             description: checkpoint.message.clone(),
16736:     let size = |message: &crate::os_spr::MutationMessage| message.target.iter().fold(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()), |bytes, segment| bytes.saturating_add(ARTIFACT_EDIT_MES
16761:         let mut cut = budget.saturating_sub(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).saturating_sub(message.code.0.len()).min(message.message.len());
16762:         while !message.message.is_char_boundary(cut) {
16765:         message.message.truncate(cut);
16903:         entry.messages.iter().try_fold(entry.edit_id.len(), |bytes, message| message.target.iter().try_fold(bytes.checked_add(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES)?.checked_add(message.code.0.len())?.checked_add(message.message.len())?, |bytes, segment|
18592:     content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
19017:         let next_id = content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, &self.envelope.vcs.changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &pins).await;
19097:                 messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
22269:                     message: checkpoint.message.clone(),
25241:             messages.push(crate::os_spr::MutationMessage::fatal(error.code, error.message).at(error.target));
25259:             let mut fatal = crate::os_spr::MutationMessage::fatal(error.code, error.message).at(error.target);
26769:         if let Some(message) = self.message.as_mut() {
26808:             drop(self.message.take());
26813:                 *self.message = Some(message);
26881:         self.backbone.is_none() && self.queue.is_none() && self.message.is_none() && self.bytes.is_none()
27505:                     all_messages.push(crate::os_spr::MutationMessage::fatal("mutation.invariant", error.message).at(error.target).at_op(index as u32));
28431:         semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))
28447:         semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs
42:                     let body = semio_framework_dsl_record::print_controlled(record.as_record(), &spec, semio_framework_dsl_record::JoinMode::Document, body_limit, &mut native).map_err(|error| ValueError::new(error.kind, error.message))?;

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧬️schema/🧬️mutations/🦀️.rs
118:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs
43:                         .map_err(|error| ValueError::new(error.kind, error.message))?

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs
689:         assert_eq!(format!("schema error: {}", terminal.cause.message), message);
705:     let message = text.message.clone();
712:     assert_eq!(terminal.cause.message, message);
715:     assert_eq!(terminal.diagnostics[0].message, message);

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🧪️tests/💰️allocation/🦀️.rs
63:     let observed=Cell::new(false);let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==source.literal.len()&&event.completed>threshold&&event.completed<event.total{assert_eq!(event.owned_bytes,neutral[
99:             let refusal=result.unwrap_err();assert_eq!(refusal.kind,kind);assert_eq!(refusal.message,message);let terminal=store::io_schema::IoError::from_value_error(refusal);assert_eq!(terminal.cause.kind,kind);assert_eq!(terminal.cause.message,mes
102:             let refusal=result.unwrap_err();assert_eq!(refusal.kind,kind);assert_eq!(refusal.message,message);let terminal=store::io_schema::IoError::from_value_error(refusal);assert_eq!(terminal.cause.kind,kind);assert_eq!(terminal.cause.message,mes

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/📨️messages/🦀️.rs
80:             2 => (source.message.as_str(), &mut output.message),
126:         output.message.capacity()
137:             else if output.message.capacity() != 0 { drop(std::mem::take(&mut output.message)); }

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🧬️schema/🧬️mutations/🦀️.rs
114:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs
386:             prepared.messages.push(MutationMessage::fatal(cause.code, cause.message).at(cause.target));

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs
15:   let allocation=cause.message.as_ptr();let refusal=pack::PackRefusal::from(cause);
19:   assert_eq!(value.kind,kind);assert_eq!(value.message.as_ptr(),allocation);
25:   assert_eq!(serde_json::json!({"kind":format!("{:?}",value.kind),"message":value.message}),reference);
45:   let allocation=cause.message.as_ptr();let expected_allocation=cause.expected.as_ref().map(|text|text.as_ptr());
48:   assert_eq!(value.kind,kind);assert_eq!(value.span,span);assert_eq!(value.expected.as_deref(),row["expected"].as_str());assert_eq!(value.message.as_ptr(),allocation);assert_eq!(value.expected.as_ref().map(|text|text.as_ptr()),expected_allocation);as
634:     assert_eq!(ledger.get_by_id(&first).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("first"));
635:     assert_eq!(ledger.get_by_id(&second).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("second"));
6503:     assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
7959:     assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &missing_metadata).await, Err(error) if error.message.contains("no metadata records")));
7969:     assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &dangling_text).await, Err(error) if error.message.contains("unknown operation unknown-edit")));
7988:     assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &invalid).await, Err(error) if error.message.contains("invalid edit sequence -1")));
8266:     assert!(error.message.contains("authoritative operation metadata"));
8325:     assert!(error.message.contains("unknown operation line"), "got {error:?}");
8332:     assert!(error.message.contains("expected Text"), "got {error:?}");
8340:     assert!(error.message.contains("unknown operation line"), "got {error:?}");
8430:     assert!(matches!(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &malformed_text).await, Err(error) if error.message.contains("repeats transition")));
8436:     assert!(matches!(parse_document_spr::<DemoSnapshot, DemoMutation>(&pack.pack, &malformed_spr).await, Err(error) if error.message.contains("repeats transition")));
8599:     assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
9247:     assert!(matches!(SpaceMember::close_owned_step(&mut missing, 1, 4_096), Err(reason) if reason.kind == semio_framework_value::ValueRefusalKind::InvariantViolated && reason.message.contains("no owner-supplied bounded disposer")));

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs
157:             + source.messages.iter().map(|message| message.code.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>()).sum::<usize>()

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs
754:     let bytes = |entry: &[crate::os_spr::MutationMessage], edit_id: &str| entry.iter().fold(edit_id.len(), |total, message| total + message.code.0.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>());
758:         summary.message.strip_suffix(" more messages").expect("the summary counts what it drops").parse().expect("a count")

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs
65:                     p.insert("space_history_checkpoint", &[Cell::Integer(document), Cell::Integer(fields::ordinal(index)?), Cell::Text(&row.id), row.parent_id.as_deref().map(Cell::Text).unwrap_or(Cell::Null), Cell::Text(&row.message)])?;

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs
634:                         let checkpoint = crate::os_store::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.change_ids), parent_id: source.parent_id.take(), authors: Vec::with_capacity(source.authors.len()), mess

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/📏️preflight/🫳️borrowed/🦀️.rs
27:       1=>Self::Text(&value.message),2=>Self::Authors(&value.authors),3=>Self::Clock(&value.timestamp),4=>Self::Members(&value.members),_=>return Err(absent()),

🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs
54:         fields.push((c.copy_text("message")?, text(&row.message, c)?));

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🧬️schema/🧬️mutations/🦀️.rs
113:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🧬️schema/🧬️mutations/🦀️.rs
114:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🧬️schema/🧬️mutations/🦀️.rs
136:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
4644:                         let message = rejected.fault.message.clone();

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-fmt/🦀️.rs
20:             return protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target).absorb_messages(outcome.messages().to_vec());

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️set-other-chunks/🦀️.rs
20:             return protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target).absorb_messages(outcome.messages().to_vec());

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔊️set-data/🦀️.rs
20:             return protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target).absorb_messages(outcome.messages().to_vec());

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
387:             issue.message = format!("otherChunks[{index}].padByte is nonzero but its RIFF chunk payload has even length and carries no pad byte");

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
54:     fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVa

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs
56: impl From<OpcError> for semio_framework_value::ValueError {
64:     pub fn into_value_error(self) -> semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/🦀️.rs
164:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🧬️schema/🧬️mutations/🦀️.rs
117:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/🦀️.rs
47:                 <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record).map_err(|error| error.message)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs
163: impl From<ZipError> for semio_framework_value::ValueError {
171:     pub fn into_value_error(self) -> semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🚪️io/🦀️.rs
120:             message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🧬️schema/🧬️mutations/🦀️.rs
92:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs
341: impl From<Part21Error> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs
58:         write!(formatter, "[{}] {}", self.code, self.message)
73:         return Err(MutationRefusal { code: message.code.0.to_string(), message: message.message.to_string() });
75:     *snapshot = kernel::apply_diff(outcome.diff(), snapshot).map_err(|error| MutationRefusal { code: error.code.to_string(), message: error.message.to_string() })?;
621:                 native_codec_hash(&binding.pack_schema_hash).map_err(|error| failure(format!("codec {}: {}", item.id, error.message)))?;

✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs
712:         validator.validate_dsl_fragment_with_context(&path, operation, candidate, |context| project_context(&next, context), |context| project_shape(&next, context)).map_err(|error| SnapshotEditError::new(error.code, error.path, error.message))?;

✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs
236:             write!(formatter, "{}", self.message)
238:             write!(formatter, "{} at {}", self.message, self.path)
656:     let mut fault = Fault::new(FaultOrigin::App, FaultCode::new(error.code), error.message);
1393:             return Err(Fault::new(FaultOrigin::App, FaultCode::new("snapshot-edit.leaf-refused"), format!("{}: {}", message.code.0, message.message)));
1444:                     ::core::result::Result::Err(error) => $crate::kernel::MutationOutcome::refuse(error.outcome_code(), ::std::format!("{}: {}", error.code, error.message), [error.path]),
1451:                     .map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, ::std::format!("{}: {}", error.code, error.message)))
1549:         let next = kernel::apply_diff(mutation.diff(&base).diff(), &base).map_err(|error| edit_fault("snapshot-edit.publication-invalid", error.message))?;
1552:             restored = kernel::apply_diff(inverse.diff(&restored).diff(), &restored).map_err(|error| edit_fault("snapshot-edit.inverse-invalid", error.message))?;

✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs
320:     assert!(error.message.contains("maximum"));

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs
16:             return MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]);

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs
23:     fn diff(&self,base:&PdfSnapshot)->MutationOutcome<PdfDiff>{match self.next(base){Ok(next)=>MutationOutcome::new(PdfDiff::between(base,&next)),Err(error)=>MutationOutcome::refuse(error.outcome_code(),format!("{}: {}",error.code,error.message),[err
25:         self.next(base).and_then(|_|editing::inverse_snapshot_patches(base,&self.patch)).map(|parts|parts.into_iter().map(|patch|PdfMutation::PatchSnapshot(Self{patch})).collect()).map_err(|error|semio_framework_value::ValueError::new(semio_framework

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🧬️schema/🧬️mutations/🦀️.rs
153:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🚪️io/🦀️.rs
115:             message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },

✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
608:             messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🧬️schema/🧬️mutations/🦀️.rs
148:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🚪️io/🦀️.rs
115:             message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs
185:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/⚠️refusal/🦀️.rs
19:             Self::Ownership(detail) => write!(f, "docx: retained ownership: {}",detail.message),
30: impl From<DocxError> for semio_framework_value::ValueError {
50:     pub fn into_value_error(self) -> semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
8:  for measured in [false,true]{let mut interior=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measure
85:  std::thread::Builder::new().stack_size(fixture["retirementStackBytes"].as_u64().unwrap() as usize).spawn(move||{let mut reached=false;let error=SemioValueSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |progress|{let ca

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
5172: impl From<DwgExportError> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/🦀️.rs
114:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
118:   let((kind,diagnostic),observed)=measure(||{match SemioModelSnapshot::from_sqlite_database(&database,&mut control){Err(error)=>{let result=(error.kind,error.message.capacity());drop(error);result},Ok(value)=>{value.retire_sqlite_snapshot();panic!("a

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🧬️mutations/🦀️.rs
95:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🦀️.rs
186:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🦀️.rs
104:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs
452:         let detail = context.payload_from_bytes(JobPayloadStream::Fault, fault.message.as_bytes()).unwrap_or_else(|rejected| {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs
1009: impl PngReadError {fn into_value(self)->ValueError {match self {Self::Format(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message),Self::Refusal(error)=>error}}}

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs
115:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🛡️refusal/🦀️.rs
30: impl From<XlsxError> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
2132:                         let message = rejected.fault.message.clone();
2224:                         let message = rejected.fault.message.clone();
2506:                         let message = rejected.fault.message.clone();
3092:         if let Some(step) = puzzle5d_retire_string_step(&mut cause.message, maximum_bytes)? {
3119:     if let Some(step) = puzzle5d_retire_string_step(&mut owner.message, maximum_bytes)? {
3757:                         let message = rejected.fault.message.clone();

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🦀️.rs
151:     fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVa

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
48: fn sqlite_snapshot_semio_audio_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.tags[0].value="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as usize)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs
23:  let result=(||->Result<String,ValueError>{let mut span=SpanContext{bytes:[0;96],length:0};std::fmt::write(&mut span,format_args!(" at {}:{} (length {})",error.span.line,error.span.column,error.span.length)).map_err(|_|ValueError::new(ValueRefusalKin

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🦀️.rs
181:     outcome.messages().iter().filter(|message| message.level >= semio_framework_diagnostic::Severity::Error).map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message)).collect()

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🦀️.rs
16: pub fn apply_tiff_baseline_mutation(snapshot:&mut TiffSnapshot,mutation:&TiffBaselineMutation)->protocol::MutationOutcome<TiffDiff>{let outcome=mutation.diff(snapshot);match outcome.diff().apply(snapshot){Ok(next)=>{*snapshot=next;outcome},Err(error)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
137:  assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.message,"authored refusal after field allocation");

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
47: fn sqlite_snapshot_semio_video_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.streams[0].codec="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as usi

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛬️decoding/🦀️.rs
8: impl Refusal{fn into_value_error(self)->ValueError{match self{Self::Native(error)=>error,Self::Wire(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message)}}}

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🦀️.rs
597:  controlled_decoding::decode_tiff_controlled(data,&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true),usize::MAX).map_err(|error|error.message)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
38:  let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let admission=&f["controlledAdmission"];let mut source=fixture();source.metadata=vec![source.metadata[0].clone();admission["collectionItems"].as_u64().unw
43: fn sqlite_snapshot_semio_image_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.metadata[0].value="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as us

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🦀️.rs
150:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
144:     protocol::MutationApplyError::new(error.code, error.message).at(error.target)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
8:  for measured in [false,true]{let mut interior=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measure

✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs
26:         Ok(IoOutcome::clean(store::ArtifactDsl::parse_dsl(text).map_err(|e| IoError::from_value_error(semio_framework_value::ValueError::new(e.kind,e.message)))?))

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/💰️backing/🔍️rows/🦀️.rs
5: pub(super) fn invalid(message: &str) -> ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
53:         assert!(canceled&&error.message.contains("cancel"),"{error}");
66: fn sqlite_snapshot_semio_object_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.mesh.as_mut().unwrap().child_id="x".repeat(f["nativeOutputBytes"].as_u64()

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs
84: impl From<JpgError> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
50:  for measure in[true,false]{let mut reached=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measure{ev

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🧬️mutations/🦀️.rs
98:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🦀️.rs
121:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
72: fn sqlite_snapshot_semio_text_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.runs[0].content="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as usize

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs
112:         Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/⚠️refusal/🦀️.rs
19:             Self::Ownership(e) => write!(f, "pptx: {}",e.message),
30: impl From<PptxError> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
48: fn sqlite_snapshot_semio_kit_controlled_native_collection_materialization(){let mut s=fixture();s.types=(0..1024).map(|index|SemioKitType{id:if index==0{"chair".into()}else{format!("type-{index}")},name:format!("Type {index}"),category:"furniture".in
54: fn sqlite_snapshot_semio_kit_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.objects[0].child_id="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as us

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs
115:             let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
121:                 let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
128:                 let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
7:  for measured in [false,true]{let mut interior=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measure

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
55: fn sqlite_snapshot_semio_flow_controlled_native_output(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut s=fixture();s.nodes[0].label="x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as usize)

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/🦀️.rs
30: impl From<PdfEngineError> for semio_framework_value::ValueError {

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs
59:         control.check_value_bytes(diagnostics.iter().try_fold(0usize,|sum,value|sum.checked_add(value.message.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PDF diagnostic byte count overflow")))?)?;

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs
230:             let Err(error)=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut observe,tight))else{panic!("all authored COS entities and relationships require row admission for {encoding:?}")};assert!(error.message.contains("row"),

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🦀️.rs
280: pub(crate) fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
8:  for measured in [false,true]{let mut interior=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measure

✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
735:         return Err(format!("Sourcing Curation mutation was refused by its own vocabulary: {}", message.message));
738:     let post = protocol::MutationDiff::apply(outcome.diff(), base).map_err(|error| format!("Sourcing Curation mutation could not apply onto its exact base: {}", error.message))?;

✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
1132:         return Err(format!("Process3d document mutation was refused by its own vocabulary: {}", message.message));
1135:     let post = protocol::MutationDiff::apply(outcome.diff(), base).map_err(|error| format!("Process3d document mutation could not apply onto its exact base: {}", error.message))?;

✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🪆️native-fields/🦀️.rs
11:    fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(items)if items.len()==1=><Self as semio_framework_dsl_record::DslVariants>::from_named_record(&items[0

✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
141:             main::BODY_KEY => main::render(&crate::flow_composed_snapshot(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("flow.content-unavailable", fault.message))?).map(semio_framework_plugin::built_to

✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
2392:         let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("flow.content-unavailable", fault.message))?;
2427:         let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("flow.content-unavailable", fault.message))?;
2447:             .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("flow.eval-session-owner", error.message))?

✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🛬️native/🦀️.rs
32:  let rules=c.scoped_stage(|c|->Result<Vec<WfcAdjacencyRule2d>,ValueError>{c.begin_stage(parsed.rules.len())?;let mut values=c.allocate_vec::<WfcAdjacencyRule2d>(parsed.rules.len())?;for rule in parsed.rules{let direction=direction_from_token(&rule.di

✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🦀️.rs
1498:             return match disposer.close_step(candidate, 1, RASTER_OWNED_FIELD_BYTES).map_err(|fault| ValueError::new(ValueRefusalKind::InvariantViolated, format!("{}: {}", fault.code.0, fault.message)))? {

✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
976:                 let message = rejected.fault.message.clone();

✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
79:             semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => <Grid3dTileMedia as semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|error| error.message),
```

## Missing Fixture Schemas

At inspection intrinsic and frontier retirement fixture directories each contain only their 🔣️.json payload. Targeted current Value JSON inventory finds neither prior intrinsic/🧬️schema.json nor frontier/🧬️schema.json and no canonical relocated retirement schema counterpart. Other intrinsic/refusal schemas in different owner trees are distinct and must not be substituted. Current absence proves neither who removed them nor a canonical rename/move. Reestablish explicit held schema authority before scope10 native admission; do not reconstruct historical execution or infer schema equivalence from matching fixture data.

No runtime success or historical-current equality is claimed.

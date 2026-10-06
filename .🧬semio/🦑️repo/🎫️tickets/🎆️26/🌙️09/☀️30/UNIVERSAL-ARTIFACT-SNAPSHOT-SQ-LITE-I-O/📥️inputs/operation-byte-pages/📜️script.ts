import { mkdir } from "node:fs/promises";
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const input = `${ticket}/📥️inputs/operation-byte-pages`;
const root = "🧰️framework/🔨️modules/📡️replication/🎮️mutation";
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests";
const changes: { path: string; before: string; after: string }[] = [];
async function replace(path: string, from: string, to: string) {
  const before = await Bun.file(path).text();
  if (before.split(from).length !== 2) throw Error(`exact one-span guard failed: ${path}`);
  changes.push({ path, before, after: before.replace(from, to) });
}
if(process.argv[2]==="dsl-borrowed-variant-held"){
  const binding="🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs",derive="🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs",owner="🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs",dsl="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs";
  await replace(binding,"pub trait DslVariants: Sized {","pub trait DslVariants: Sized {\n"+await Bun.file(`${input}/dsl-variant-source-trait.rs`).text());
  const owned=await Bun.file(owner).text();if(owned.includes("pub struct VariantProjection"))throw Error("borrowed variant source owner already mounted");changes.push({path:owner,before:owned,after:owned+await Bun.file(`${input}/dsl-variant-source-owner.rs`).text()});
  const before=await Bun.file(derive).text();let after=before;
  const edit=(from:string,to:string)=>{if(after.split(from).length!==2)throw Error("exact authored variant derive span");after=after.replace(from,to);};
  const region=(start:string,end:string,operation:(text:string)=>string)=>{const left=after.indexOf(start),right=after.indexOf(end,left);if(left<0||right<left)throw Error("declared derive method region");after=after.slice(0,left)+operation(after.slice(left,right))+after.slice(right);};
  region("fn retained_record_codegen(","fn retained_record_key_codegen(",text=>text.replace("fields:&Fields)","fields:&Fields,from_bindings:bool)").replace("let value=quote!{&self.#ident};","let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};"));
  region("fn retained_record_key_codegen(","fn emit_record(",text=>text.replace("fields:&Fields)","fields:&Fields,from_bindings:bool)").replace("let projection=match kind{","let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};\n        let projection=match kind{").replace("let value=&self.#ident;","let value=#value;"));
  edit("retained_record_codegen(&data.fields);","retained_record_codegen(&data.fields,false);");
  edit("retained_record_key_codegen(&data.fields);","retained_record_key_codegen(&data.fields,false);");
  edit("    let mut retirement_arms=Vec::new();","    let mut retirement_arms=Vec::new();\n    let mut variant_identity_arms=Vec::new();let mut variant_projection_arms=Vec::new();let mut variant_key_arms=Vec::new();");
  edit("                retirement_arms.push(quote!{#name::#variant_ident(inner)=><#inner_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(inner)});","                retirement_arms.push(quote!{#name::#variant_ident(inner)=><#inner_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(inner)});\n"+await Bun.file(`${input}/dsl-variant-source-newtype-derive.rs`).text());
  edit("        retirement_arms.push(quote!{#match_pattern=>{#(#retirements)*}});","        retirement_arms.push(quote!{#match_pattern=>{#(#retirements)*}});\n"+await Bun.file(`${input}/dsl-variant-source-record-derive.rs`).text());
  edit("            fn retire_decoded_variant(self){#retirement}","            fn retire_decoded_variant(self){#retirement}\n            fn projected_variant_identity(&self)->(&'static str,usize,::semio_framework_dsl_record::RecordSpecProducer){match self{#(#variant_identity_arms),*}}\n            fn projected_variant_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{match self{#(#variant_projection_arms),*}}\n            fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{match self{#(#variant_key_arms),*}}");
  changes.push({path:derive,before,after});
  const initial=await Bun.file(dsl).text();const first=initial.indexOf("    fn encode_with_into<T:DslVariants>"),last=initial.indexOf("\n    /// 🏷️ Appends a declared tagged operation",first);if(first<0||last<first)throw Error("actual original shared variant producer");
  let method=initial.slice(first,last);
  const change=(from:string,to:string)=>{if(method.split(from).length!==2)throw Error("exact original variant encoding span");method=method.replace(from,to);};
  change("        let(keyword,record)=op.to_named_record_controlled(control).map_err(crate::os_pack::PackRefusal::from)?;\n        let variants=T::variants_controlled(control).map_err(crate::os_pack::PackRefusal::from)?;\n        let ordinal=variants.iter().position(|(key,_)|key==&keyword).ok_or_else(||ProtocolError::Malformed{what:\"op variant\",offset:0,detail:format!(\"keyword '{keyword}' missing from variants()\")})?;","        control.checkpoint().map_err(crate::os_pack::PackRefusal::from)?;\n        let(keyword,ordinal,producer)=op.projected_variant_identity();\n        let source=semio_framework_dsl_record::native_encoding::VariantProjection::new(op);");
  change("let spec=variants[ordinal].1.encode(control)","let spec=producer.encode(control)");
  change("crate::os_pack::record::encode_record_body_into(&spec,&record,options,output,control)?;","crate::os_pack::record::encode_projected_record_body_into(&spec,&source,options,output,control)?;");
  changes.push({path:dsl,before:initial,after:initial.slice(0,first)+method+initial.slice(last)});
  await Bun.write(`${input}/dsl-borrowed-variant-held-pairs.json`,JSON.stringify({state:"Held coherent required DslVariants source and derive plus actual shared producer",productionMutations:0,pairs:changes},null,2)+"\n");console.log(JSON.stringify({held:changes.length,productionMutations:0}));process.exit(0);
}else if(process.argv[2]==="dsl-borrowed-variant-law-joins"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs",before=await Bun.file(path).text();let after=before;
  for(const[from,to]of[
    ["encoding.scoped_maximum(512,|encoding|variants_binary::encode_op_into(&operation,&options,&mut measurement,encoding)).unwrap();","encoding.scoped_maximum(512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&operation,&options,&mut measurement,encoding))).unwrap().unwrap();"],
    ["assert_eq!(measurement.exact_length(),Some(expected.len()));","assert_eq!(measurement.exact_length().unwrap(),expected.len());"],
    ["encoding.scoped_maximum(paid+512,|encoding|variants_binary::encode_op_into(&operation,&options,&mut prepared,encoding)).unwrap();","encoding.scoped_maximum(paid+512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&operation,&options,&mut prepared,encoding))).unwrap().unwrap();"]
  ]){if(after.split(from).length!==2)throw Error("exact scoped fixture producer error join");after=after.replace(from,to);}
  changes.push({path,before,after});
}else if(process.argv[2]==="dsl-borrowed-variant-provider"){
  const held=await Bun.file(`${input}/dsl-borrowed-variant-held-pairs.json`).json();changes.push(...held.pairs);
}else if(process.argv[2]==="dsl-borrowed-variant-demand"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";const before=await Bun.file(path).text();
  if(before.includes("original_variant_borrows8194"))throw Error("borrowed variant law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/dsl-borrowed-variant-law.rs`).text()});
}else if(process.argv[2]==="core-borrowed-depth-provider"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🦀️.rs";const before=await Bun.file(path).text();
  if(before.includes("fn level_limit("))throw Error("canonical borrowed depth already mounted");
  changes.push({path,before,after:await Bun.file(`${input}/core-borrowed-source-encoder.rs`).text()});
}else if(process.argv[2]==="core-borrowed-paid-demand"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";
  await replace(path,"assert_eq!(encode_projected_record_body_into(&spec,&source,&options,&mut prepared,&mut control).unwrap(),expected.len());","assert_eq!(control.scoped_maximum(before+case[\"maximumMetadataBytes\"].as_u64().unwrap()as usize,|control|encode_projected_record_body_into(&spec,&source,&options,&mut prepared,control)).unwrap(),expected.len());");
}else if(process.argv[2]==="core-borrowed-paid-provider"){
  await replace("🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🦀️.rs","        if measured.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,\"borrowed Record exceeds caller allocation allowance\").into())}\n","");
}else if(process.argv[2]==="core-borrowed-nested-demand"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";const before=await Bun.file(path).text();
  if(before.includes("borrowed_source_joins_actual_nested"))throw Error("nested source demand already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/core-borrowed-nested-law.rs`).text()});
}else if(process.argv[2]==="core-borrowed-source-demand"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";const before=await Bun.file(path).text();
  if(before.includes("borrowed_source_excludes_large_payload_mirrors"))throw Error("borrowed Core source demand already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/core-borrowed-source-law.rs`).text()});
  const fixture="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";const initial=await Bun.file(fixture).text(),data=JSON.parse(initial);
  if(data.borrowedSource)throw Error("borrowed source neutral demand already mounted");
  data.borrowedSource={prefix:[0,2,1,8,130,64],textHeader:[2,7,130,64],textUnit:"x",maximumMetadataBytes:512};
  changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="core-borrowed-source-provider"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🦀️.rs";if(await Bun.file(path).exists())throw Error("borrowed source emitter already mounted");
  changes.push({path,before:"",after:await Bun.file(`${input}/core-borrowed-source-encoder.rs`).text()});
  await replace("🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs","use std::{cmp::Ordering,mem::size_of};","use std::{cmp::Ordering,mem::size_of};\n#[path = \"🫳️borrowed/🦀️.rs\"]\nmod borrowed;\npub(super) use borrowed::projected_record_body_into;");
  await replace("🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs","/// 🎞️ Encodes a borrowed complete intrinsic value without a synthetic owned record.","/// 🫳️ Emits exact canonical Record bytes directly from the immutable original ordinal source.\npub fn encode_projected_record_body_into<T:semio_framework_dsl_record::native_encoding::FieldProjectionSource>(spec:&RecordSpec,source:&T,options:&EncodeOptions,output:&mut dyn protocol::mutation::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<usize,PackRefusal>{controlled_encoding::projected_record_body_into(spec,source,options,output,control)}\n\n/// 🎞️ Encodes a borrowed complete intrinsic value without a synthetic owned record.");
}else if(process.argv[2]==="operation-measurement-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;const before=await Bun.file(path).text();
  if(before.includes("measure_complete_operation_without_retaining"))throw Error("measurement demand already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/operation-measurement-law.rs`).text()});
}else if(process.argv[2]==="operation-measurement-provider"){
  const path=`${root}/📦️bytes/🦀️.rs`;const before=await Bun.file(path).text();
  if(before.includes("pub struct OperationByteMeasurement"))throw Error("actual measurement source already mounted");
  changes.push({path,before,after:before+"\n"+await Bun.file(`${input}/operation-measurement.rs`).text()});
}else if(process.argv[2]==="core-prepaid-sink-demand"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";const before=await Bun.file(path).text();
  if(before.includes("real_core_sink_uses_step_funded"))throw Error("actual prepaid Core sink law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/core-prepaid-sink-law.rs`).text()});
  const fixture="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";const initial=await Bun.file(fixture).text();const data=JSON.parse(initial);
  if(data.prepaidRecord)throw Error("actual prepaid Core neutral input already present");
  data.prepaidRecord={prefix:[0,1,1,8,130,64],fieldIndexAdmissionBytes:4,maximumPhysicalPageBytes:4096,terminalAllocatedBytes:0};
  changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="prepaid-empty-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;const before=await Bun.file(path).text();
  if(before.includes("empty_prepaid_handback_keeps_exact_zero"))throw Error("empty prepay law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/prepaid-empty-law.rs`).text()});
  const fixture=`${root}/📦️bytes/🧫️fixtures/🔣️.json`;const initial=await Bun.file(fixture).text();const data=JSON.parse(initial);
  if(data.emptyOperationLength!==undefined)throw Error("neutral empty operation already present");
  data.emptyOperationLength=0;changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="prepaid-empty-provider"){
  await replace(`${root}/📦️bytes/🦀️.rs`,"        if !self.is_funded()||self.completed!=self.exact_length{return None;}\n        self.source.take()","        if !self.is_funded()||self.completed!=self.exact_length{return None;}\n        self.source.as_mut()?.maximum_payload_bytes=self.exact_length;\n        self.source.take()");
}else if(process.argv[2]==="prepaid-operation-iterator-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;
  const before=await Bun.file(path).text();
  const one="prefix.iter().eq(expected[..256].iter())",two="prepared.accepted_prefix().unwrap().iter().eq(expected.iter())";
  if(before.split(one).length!==2||before.split(two).length!==2)throw Error("exact actual ByteSpan copied iterator demand");
  changes.push({path,before,after:before.replace(one,"prefix.iter().eq(expected[..256].iter().copied())").replace(two,"prepared.accepted_prefix().unwrap().iter().eq(expected.iter().copied())")});
}else if(process.argv[2]==="prepaid-operation-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;const before=await Bun.file(path).text();
  if(before.includes("fund_complete_backing_across_hops"))throw Error("actual prepaid operation law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/prepaid-operation-law.rs`).text()});
}else if(process.argv[2]==="prepaid-operation-provider"){
  const path=`${root}/📦️bytes/🦀️.rs`;const before=await Bun.file(path).text();
  if(before.includes("pub struct OperationBytePreparation"))throw Error("actual prepaid operation source already mounted");
  changes.push({path,before,after:before+"\n"+await Bun.file(`${input}/prepaid-operation.rs`).text()});
}else if(process.argv[2]==="store-intrinsic-demand"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.includes("paged_dsl_operation_store_intrinsic_bridge_"))throw Error("actual Store bridge law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/store-intrinsic-pages-law.rs`).text()});
  const fixture="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";
  const initial=await Bun.file(fixture).text();const data=JSON.parse(initial);
  if(data.storeIntrinsic)throw Error("actual Store intrinsic input already mounted");
  data.storeIntrinsic={fieldId:1,prefix:[0,1,1,17,8,130,64],foreignFieldId:2};
  changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="store-intrinsic-provider"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
  const marker="    /// 🧩️ Compose-only bridge — external technology; converts through `DslValue` without JSON on the wire.";
  await replace(path,marker,await Bun.file(`${input}/store-intrinsic-paged-methods.rs`).text()+"\n"+marker);
}else if(process.argv[2]==="store-intrinsic-held"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
  const before=await Bun.file(path).text(),marker="    /// 🧩️ Compose-only bridge — external technology; converts through `DslValue` without JSON on the wire.";
  if(before.split(marker).length!==2||before.includes("pub fn encode_wire_value_into(")||!before.includes("const VALUE_BRIDGE_FIELD_ID: u16 = 1;"))throw Error("actual declared Store intrinsic issuer guard");
  const after=before.replace(marker,await Bun.file(`${input}/store-intrinsic-paged-methods.rs`).text()+"\n"+marker);
  await Bun.write(`${input}/store-intrinsic-paged-held-pair.json`,JSON.stringify({status:"Held genuine Store field1 sink and immutable source issuer",productionMutations:0,pairs:[{path,before,after,beforeSha256:new Bun.CryptoHasher("sha256").update(before).digest("hex")}]},null,2));
  console.log(JSON.stringify({held:1,productionMutations:0}));process.exit(0);
}else if(process.argv[2]==="core-intrinsic-neutral-authority"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.split("value_record_body_into(0,").length!==3||before.split("ByteSpan::from_source(&output),0,").length!==3)throw Error("exact intrinsic four field locators");
  changes.push({path,before,after:before.replaceAll("value_record_body_into(0,","value_record_body_into(1,").replaceAll("ByteSpan::from_source(&output),0,","ByteSpan::from_source(&output),1,")});
  const fixture="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";
  const initial=await Bun.file(fixture).text();const data=JSON.parse(initial);
  if(data.intrinsicSource.fieldId!==0||data.intrinsicSource.prefix[2]!==0)throw Error("exact neutral intrinsic authority");
  data.intrinsicSource.fieldId=1;data.intrinsicSource.prefix[2]=1;
  changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="core-intrinsic-provider"){
  const encoder="🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs";
  const before=await Bun.file(encoder).text();
  const start=before.indexOf("pub(super) fn value_record_body("),end=before.indexOf("\nenum DocumentSource",start);
  if(start<0||end<0||before.includes("fn value_record_body_into("))throw Error("actual intrinsic encoder boundary");
  let method=before.slice(start,end);
  const edit=(from:string,to:string)=>{if(method.split(from).length!==2)throw Error("intrinsic direct producer exact span");method=method.replace(from,to);};
  edit("fn value_record_body(","fn value_record_body_into(");
  edit("options:&EncodeOptions,control:","options:&EncodeOptions,output:&mut dyn protocol::mutation::operation_bytes::OperationByteOutput,control:");
  edit("Result<Vec<u8>,PackRefusal>","Result<usize,PackRefusal>");
  edit("let mut output=Output::allocated(measure.length,encoder.control)?;","let mut output=Output{bytes:None,external:Some(output),length:0};");
  edit("Ok(output.bytes.unwrap())","Ok(output.length)");
  changes.push({path:encoder,before,after:before.slice(0,end)+"\n"+method+before.slice(end)});
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const initial=await Bun.file(path).text();
  const decoderStart=initial.indexOf("pub fn decode_value_record_body_exact_controlled("),decoderEnd=initial.indexOf("\nfn decode_record_body_inner(",decoderStart);
  if(decoderStart<0||decoderEnd<0||initial.includes("fn decode_value_record_body_span_exact_controlled("))throw Error("actual intrinsic borrowed decoder boundary");
  let decoder=initial.slice(decoderStart,decoderEnd);
  const change=(from:string,to:string)=>{if(decoder.split(from).length!==2)throw Error("intrinsic decoder exact span");decoder=decoder.replace(from,to);};
  change("fn decode_value_record_body_exact_controlled(bytes:&[u8]","fn decode_value_record_body_span_exact_controlled(bytes:crate::ByteSpan<'_>");
  change("ByteReader::new(bytes)","ByteReader::from_span(bytes)");
  const marker="/// 📦️ Emits symbols, literal document frames, chunks and footer under one ownership budget.";
  if(initial.split(marker).length!==2)throw Error("intrinsic public sink boundary");
  const sink="/// 🎞️ Appends one complete intrinsic body to the caller's sink under the same options and control.\npub fn encode_value_record_body_into(field_id:u16,value:&DslValue,options:&EncodeOptions,output:&mut dyn protocol::mutation::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<usize,PackRefusal>{controlled_encoding::value_record_body_into(field_id,value,options,output,control)}\n\n";
  const after=initial.slice(0,decoderEnd)+"\n/// 🎞️ Decodes one exact intrinsic body by borrowing the caller's complete immutable byte source.\n"+decoder+initial.slice(decoderEnd);
  changes.push({path,before:initial,after:after.replace(marker,sink+marker)});
}else if(process.argv[2]==="core-intrinsic-demand"){
  const path="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.includes("fn intrinsic_operation_pages_"))throw Error("intrinsic law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/core-intrinsic-pages-law.rs`).text()});
  const fixture="🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";
  const initial=await Bun.file(fixture).text();const data=JSON.parse(initial);
  if(data.intrinsicSource)throw Error("intrinsic neutral input already mounted");
  data.intrinsicSource={fieldId:0,prefix:[0,1,0,17,8,130,64],maximumPhysicalPageBytes:4096,terminalAllocatedBytes:0};
  changes.push({path:fixture,before:initial,after:JSON.stringify(data,null,2)+"\n"});
}else if(process.argv[2]==="current-policy-demand-correction"){
  await replace("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs",'    assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);\n    assert!(source.iter().eq([1,0]));','    assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::WorkLimit);\n    assert!(source.iter().eq([1,0]));');
  const law=`${root}/📦️bytes/🧪️tests/🦀️.rs`;
  await replace(law,'let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<32;','let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<256;');
  const before=changes.at(-1)!.after;
  if(before.split('    assert_eq!(source.len(),34);\n    assert!(source.iter().eq(expected[..34].iter().copied()));').length!==2)throw Error("exact canceled prefix demand guard");
  changes.at(-1)!.after=before.replace('    assert_eq!(source.len(),34);\n    assert!(source.iter().eq(expected[..34].iter().copied()));','    assert_eq!(source.len(),258);\n    assert!(source.iter().eq(expected[..258].iter().copied()));');
}else if(process.argv[2]==="dsl-whole-limit-demand"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.includes("paged_dsl_operation_whole_frame_policy_includes"))throw Error("actual whole frame law already present");
  changes.push({path,before,after:before+await Bun.file(`${input}/dsl-whole-frame-law.rs`).text()});
  const fixture="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json";
  await replace(fixture,'  "payloadBytes": 8194,','  "payloadBytes": 8194,\n  "wholeOperationBytes": 8202,');
}else if(process.argv[2]==="dsl-whole-limit"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs";
  const signature="    fn encode_with_into<T:DslVariants>(op:&T,tag_of:impl Fn(&str,usize)->Result<u64,ProtocolError>,options:&EncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{\n";
  await replace(path,signature,signature+"        let mut limited=crate::os_spr::operation_bytes::OperationByteLimitedOutput::new(output,options.limits.max_file_len);\n        let output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput=&mut limited;\n");
}else if(process.argv[2]==="whole-limit-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;
  const before=await Bun.file(path).text();
  if(before.includes("complete_operation_limit_counts_header"))throw Error("whole limit law already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/whole-operation-limit-law.rs`).text()});
}else if(process.argv[2]==="whole-limit"){
  const path=`${root}/📦️bytes/🦀️.rs`;
  const before=await Bun.file(path).text();
  if(before.includes("struct OperationByteLimitedOutput"))throw Error("whole limit producer already mounted");
  changes.push({path,before,after:before+"\n"+await Bun.file(`${input}/whole-operation-limit.rs`).text()});
}else if(process.argv[2]==="chart-current-held"){
  const path="🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/📦️codec/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.split("impl protocol::OpBinary for ChangeChartValue{").length!==2||!before.endsWith("}\n")||before.includes("fn encode_op_into"))throw Error("actual untouched Chart ordinary producer guard");
  const after=before.slice(0,-2)+await Bun.file(`${input}/chart-current-paged-methods.rs`).text()+"}\n";
  await Bun.write(`${input}/chart-paged-current-source-owner-held-pair.json`,JSON.stringify({status:"Held required producer/source pair",productionMutations:0,qualification:"Preserves actual ordinary Chart [1,1] header and Text/Binary body; explicit genuine shared options and exact captured source, no required trait activation",pairs:[{path,before,after,beforeSha256:new Bun.CryptoHasher("sha256").update(before).digest("hex")}]},null,2));
  console.log(JSON.stringify({held:1,productionMutations:0}));process.exit(0);
}else if(process.argv[2]==="dsl-decoder-demand"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";
  const before=await Bun.file(path).text();
  if(before.includes("paged_dsl_operation_decodes_same_8194_source"))throw Error("actual source decoder demand already present");
  changes.push({path,before,after:before+await Bun.file(`${input}/dsl-source-decoder-law.rs`).text()});
}else if(process.argv[2]==="dsl-decoder"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs";
  const marker="    fn decode_with<T: DslVariants>";
  const before=await Bun.file(path).text();
  if(before.split(marker).length!==2||before.includes("fn decode_with_span<"))throw Error("exact DSL borrowed decoder boundary");
  changes.push({path,before,after:before.replace(marker,await Bun.file(`${input}/dsl-source-decoder.rs`).text()+"\n"+marker)});
}else if(process.argv[2]==="immutable-gesture-rebase"){
  const {createPatch,parsePatch,applyPatch}=await import("diff");
  const previous=await Bun.file(`${ticket}/📥️inputs/global-child-current-immutable-frame-after-explicit-view-preview-held-pairs.json`).json();
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
  const pair=previous.pairs.find((row:{path:string})=>row.path===path);
  const patch=parsePatch(createPatch(path,pair.before,pair.after,"before","held",{context:3}))[0];
  const constructor=patch.hunks[23];
  const call=patch.hunks[44];
  if(constructor.oldStart!==16997||call.oldStart!==32723||constructor.lines[2]!=="                 provisional_generation: 0,"||call.lines.at(-1)!=="                 .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),")throw Error("exact two held gesture conflicts differ");
  constructor.lines.splice(3,0,"                 gesture: std::sync::Arc::new(GestureSlot::detached()),");constructor.oldLines++;constructor.newLines++;
  call.lines[call.lines.length-1]="                 .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())";
  call.lines.push("                 .with_gesture(self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta)),");call.oldLines++;call.newLines++;
  const current=await Bun.file(path).text();
  const after=applyPatch(current,patch,{fuzzFactor:0});
  if(after===false)throw Error("manually reconciled exact Plugin hunks still conflict");
  if(!after.includes("gesture: std::sync::Arc::new(GestureSlot::detached()),")||!after.includes(".with_gesture(self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta)),"))throw Error("actual gesture owner preservation");
  const target=`${ticket}/📥️inputs/global-child-current-immutable-frame-after-owned-emission-held-pairs.json`;
  const document=await Bun.file(target).json();
  const position=document.pairs.findIndex((row:{path:string})=>row.path===path);
  if(position<0)throw Error("latest global capsule Plugin absent");
  const beforeSha256=new Bun.CryptoHasher("sha256").update(current).digest("hex");
  document.pairs[position]={...pair,before:current,beforeSha256,after};
  document.rebaseMisses=[];
  document.provenance+="; actual retained gesture constructor and caller preserved by exact two-hunk manual reconciliation";
  await Bun.write(target,JSON.stringify(document,null,2));
  await Bun.write(`${input}/immutable-gesture-held-reconciliation.json`,JSON.stringify({path,beforeSha256,productionMutations:0,hunks:[constructor,call]},null,2));
  console.log(JSON.stringify({heldPaths:document.pairs.length,productionMutations:0,manuallyReconciled:2}));process.exit(0);
}else if(process.argv[2]==="immutable-plugin-hunks"){
  const {createPatch,parsePatch,applyPatch}=await import("diff");
  const prior=await Bun.file(`${ticket}/📥️inputs/global-child-current-immutable-frame-after-explicit-view-preview-held-pairs.json`).json();
  const pair=prior.pairs.find((row:{path:string})=>row.path==="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
  if(!pair)throw Error("actual Plugin held pair absent");
  const current=await Bun.file(pair.path).text();
  const patch=parsePatch(createPatch(pair.path,pair.before,pair.after,"before","held",{context:3}))[0];
  const rows=patch.hunks.map((hunk,index)=>({index,hunk,applies:applyPatch(current,{...patch,hunks:[hunk]},{fuzzFactor:0})!==false}));
  await Bun.write(`${input}/immutable-plugin-hunk-current-readback.json`,JSON.stringify({path:pair.path,currentSha256:new Bun.CryptoHasher("sha256").update(current).digest("hex"),rows},null,2));
  await Bun.write(`${ticket}/📓️global-child-immutable-plugin-current-hunk-conflict.md`,"# Immutable Plugin Current Hunk Conflict\n\nGlobal source remains held. Fresh exact rebase has one Plugin path conflict and zero production mutations. Per-hunk exact current readback is retained in `📥️inputs/operation-byte-pages/immutable-plugin-hunk-current-readback.json`; no fuzzy patch or stale whole-file overwrite was applied.\n\n"+rows.filter(row=>!row.applies).map(row=>`- Hunk ${row.index}, original line ${row.hunk.oldStart}: exact current region needs manual reconciliation.`).join("\n")+"\n");
  console.log(JSON.stringify({hunks:rows.length,conflicts:rows.filter(row=>!row.applies).map(row=>row.index),productionMutations:0}));process.exit(0);
}else if(process.argv[2]==="dsl-emission-probe"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";
  const before=await Bun.file(path).text();
  const from="    let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress| !(progress.total==payload.len()&&progress.completed>=256);\n    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut cancel);\n    let error=variants_binary::encode_op_into(&operation,&Default::default(),&mut source,&mut control).unwrap_err();";
  const to=await Bun.file(`${input}/dsl-emission-probe.rs`).text();
  if(before.split(from).length!==2||before.split("    assert!(source.len()>0&&source.len()<expected.len());").length!==2)throw Error("actual DSL cancellation demand exact guard");
  changes.push({path,before,after:before.replace(from,to.trimEnd()).replace("    assert!(source.len()>0&&source.len()<expected.len());","    assert!(source.len()>=256&&source.len()<expected.len());")});
}else if(process.argv[2]==="pack-verification-owner"){
  const path="🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs";
  const before=await Bun.file(path).text();
  const start=before.indexOf("/// 🛡️ How much a read verifies as it goes:");
  const end=before.indexOf("//#endregion 🔖️Verify",start);
  if(start<0||end<start)throw Error("actual Pack verification owner boundary");
  const definition=before.slice(start,end);
  const expected="    Trusted,\n    #[default]\n    Standard,\n    Full,";
  if(!definition.includes(expected)||definition.split("fn checks_").length!==3)throw Error("exact Pack verification variants and two predicates");
  const moved=definition.replaceAll("VerificationLevel","PackVerificationLevel").replaceAll("    fn checks_","    pub fn checks_").replace("    // 🚫️async: E1 pure enum-variant predicate — no I/O, no async call anywhere in the body.\n","");
  changes.push({path,before,after:before.slice(0,start)+"/// 🛡️ The exact shared Pack verification policy controls CRC and content hash checks.\npub use crate::codec::PackVerificationLevel as VerificationLevel;\n"+before.slice(end)});
  const codec="🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs";
  const original=await Bun.file(codec).text();
  const marker="/// ⚙️ Knobs for";
  const location=original.indexOf(marker);
  if(location<0||original.split("crate::format::VerificationLevel").length!==3)throw Error("two policy verification references exact source");
  const after=(original.slice(0,location)+moved+original.slice(location)).replaceAll("crate::format::VerificationLevel","PackVerificationLevel");
  changes.push({path:codec,before:original,after});
}else if(process.argv[2]==="producer-census"){
  const map=await Bun.file(`${ticket}/📥️inputs/current-opbinary-authored-implementation-locators.json`).json();
  const paths=[...new Set<string>(map.rows.map((row:{source:string})=>row.source))];
  const rows=[];
  for(const path of paths){
    const source=await Bun.file(path).text();
    for(const match of source.matchAll(/fn encode_op\s*\(&self\)/g)){
      const decode=source.indexOf("fn decode_op",match.index);
      if(decode<0)throw Error(`paired decoder absent ${path}`);
      const body=source.slice(match.index,decode);
      const content=body.slice(body.indexOf("{")+1);
      const classify=body.includes("variants_binary::encode_tagged_op")?"taggedRecord":body.includes("variants_binary::encode_op")?"ordinalRecord":body.includes("tagged_value_binary::encode_op")?"taggedIntrinsic":body.includes("op_rt::encode_op")?"intrinsic":body.includes("serde_json::to_vec")||body.includes("to_json_string")?"json":body.includes("encode_record_body")?"handwrittenRecord":body.includes("pack_value")||body.includes("encode_wire_value")?"handwrittenIntrinsic":body.includes("unreachable!")||body.includes("match *self {}")||body.includes("match self {}")||body.includes("Ok(Vec::new())")?"empty":content.includes("encode_op(")?"delegate":"authoredOther";
      rows.push({path,line:source.slice(0,match.index).split("\n").length,classify,body,qualification:"Lexical authored encoder/decoder neighborhood; cfg and macro membership not compiled"});
    }
  }
  const counts:Record<string,number>={};for(const row of rows)counts[row.classify]=(counts[row.classify]??0)+1;
  await Bun.write(`${input}/opbinary-current-producer-census.json`,JSON.stringify({counts,rows},null,2));
  await Bun.write(`${ticket}/📓️operation-pages-current-required-producer-census.md`,"# Current Required Paged Operation Producer Census\n\nExact source neighborhoods retained in `📥️inputs/operation-byte-pages/opbinary-current-producer-census.json`. This is lexical authored source classification; inline test and macro membership is not a compiled production census. No production edits or runtime assertions.\n\n"+Object.entries(counts).map(([key,count])=>`- ${key}: ${count} source neighborhoods`).join("\n")+"\n\nEach distinct intrinsic, handwritten Record, JSON and empty operation needs its genuine producer/source inverse; required trait activation remains held until these sources and actual consumers are coherent.\n");
  console.log(JSON.stringify({counts,paths:paths.length}));process.exit(0);
}else if(process.argv[2]==="policy-path-fix"){
  const replication="🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs";
  const before=await Bun.file(replication).text();
  if(before.split("crate::format::crate::format::VerificationLevel").length!==3)throw Error("exact two moved verification qualifiers");
  changes.push({path:replication,before,after:before.replaceAll("crate::format::crate::format::VerificationLevel","crate::format::VerificationLevel")});
  const held=await Bun.file(`${input}/policy-held-pairs.json`).json();
  const core=held.pairs[0].before;
  const start=core.indexOf("/// ⚙️ Knobs for [`encode_document`].");
  const end=core.indexOf("/// 🩺️ What [`decode_document`] observed",start);
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const os=await Bun.file(path).text();
  const osStart=os.indexOf("/// ⚙️ Knobs for [`encode_document`].");
  const osEnd=os.indexOf("/// 🩺️ What [`decode_document`] observed",osStart);
  if(osStart<0||osEnd<osStart||os.slice(osStart,osEnd).replaceAll("crate::os_pack::format::VerificationLevel","crate::format::VerificationLevel")!==core.slice(start,end))throw Error("exact actual Core/OS policy type equivalence");
  changes.push({path,before:os,after:os.slice(0,osStart)+"/// 🎛️ The OS uses the same first-party transport-owned Pack policy.\npub use protocol::codec::{PackEncodeOptions as EncodeOptions,PackDecodeOptions as DecodeOptions};\n\n"+os.slice(osEnd)});
}else if(process.argv[2]==="policy-held"){
  const core="🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const replication="🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs";
  const before=await Bun.file(core).text();
  const start=before.indexOf("/// ⚙️ Knobs for [`encode_document`].");
  const end=before.indexOf("/// 🩺️ What [`decode_document`] observed",start);
  if(start<0||end<start||before.includes("PackEncodeOptions as EncodeOptions"))throw Error("exact Core policy owner boundary");
  const definitions=before.slice(start,end);
  if(definitions.split("pub struct ").length!==3||definitions.split("impl Default for ").length!==3)throw Error("exact two complete policy definitions");
  const moved=definitions.replaceAll("EncodeOptions","PackEncodeOptions").replaceAll("DecodeOptions","PackDecodeOptions").replace("Knobs for [`encode_document`].","Knobs for Pack document encoding.").replace("Knobs for [`decode_document`].","Knobs for Pack document decoding.");
  const original=await Bun.file(replication).text();
  const marker="//#region 🔖️Varint";
  if(original.split(marker).length!==2)throw Error("Replication policy insertion guard");
  const pairs=[{path:core,before,after:before.slice(0,start)+"/// 🎛️ First-party Pack policy is owned by the shared transport contract.\npub use protocol::codec::{PackEncodeOptions as EncodeOptions,PackDecodeOptions as DecodeOptions};\n\n"+before.slice(end)},{path:replication,before:original,after:original.replace(marker,moved+marker)}];
  const os="🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const osBefore=await Bun.file(os).text();
  const osStart=osBefore.indexOf("/// ⚙️ Knobs for [`encode_document`].");
  const osEnd=osBefore.indexOf("/// 🩺️ What [`decode_document`] observed",osStart);
  if(osStart<0||osEnd<osStart||osBefore.slice(osStart,osEnd).replaceAll("crate::os_pack::format::VerificationLevel","crate::format::VerificationLevel")!==definitions)throw Error("OS duplicate policy exact field/default/type equivalence");
  pairs.push({path:os,before:osBefore,after:osBefore.slice(0,osStart)+"/// 🎛️ The OS uses the same first-party transport-owned Pack policy.\npub use protocol::codec::{PackEncodeOptions as EncodeOptions,PackDecodeOptions as DecodeOptions};\n\n"+osBefore.slice(osEnd)});
  await Bun.write(`${input}/policy-held-pairs.json`,JSON.stringify({qualification:"Held exact unchanged policy type extraction; no production write",pairs:pairs.map(pair=>({...pair,beforeSha256:new Bun.CryptoHasher("sha256").update(pair.before).digest("hex")}))},null,2));
  console.log(JSON.stringify({held:pairs.length}));process.exit(0);
}else if(process.argv[2]==="policy"){
  const held=await Bun.file(`${input}/policy-held-pairs.json`).json();
  for(const pair of held.pairs){const current=await Bun.file(pair.path).text();if(current!==pair.before)throw Error(`policy full-file guard changed ${pair.path}`);changes.push(pair);}
}else if(process.argv[2]==="comparison-borrow-fix"){
  await replace(`${root}/📦️bytes/🦀️.rs`,"self.source.get(self.position)!=Some(*byte)","self.source.get(self.position)!=Some(byte)");
}else if(process.argv[2]==="operation-byte-module"){
  const paths=[`${root}/🦀️.rs`,`${root}/📦️bytes/🧪️tests/🦀️.rs`,"🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs"];
  for(const path of paths){
    const before=await Bun.file(path).text();
    const after=before.replaceAll("mutation::bytes","mutation::operation_bytes").replaceAll("super::bytes","super::operation_bytes").replaceAll("pub mod bytes;","pub mod operation_bytes;").replaceAll("crate::os_spr::mutation::operation_bytes","crate::os_spr::operation_bytes");
    if(after===before)throw Error(`new operation bytes module guard ${path}`);
    changes.push({path,before,after});
  }
  await replace("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs","//#region 🔖️Reexports","//#region 🔖️Reexports\npub use protocol::mutation::operation_bytes;");
}else if(process.argv[2]==="comparison-demand"){
  const path=`${root}/📦️bytes/🧪️tests/🦀️.rs`;
  const before=await Bun.file(path).text();
  if(before.includes("canonical_comparison_retains_exact_source"))throw Error("comparison demand already mounted");
  changes.push({path,before,after:before+await Bun.file(`${input}/source-comparison-law.rs`).text()});
}else if(process.argv[2]==="comparison"){
  const path=`${root}/📦️bytes/🦀️.rs`;
  const before=await Bun.file(path).text();
  if(before.includes("struct OperationByteComparison"))throw Error("comparison producer already mounted");
  changes.push({path,before,after:before+"\n"+await Bun.file(`${input}/source-comparison.rs`).text()});
}else if (process.argv[2] === "dsl-demand") {
  const base="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl";
  const path=`${base}/🧪️tests/🔬️unit/🦀️.rs`;
  const before=await Bun.file(path).text();
  if(before.includes("paged_dsl_operation_keeps_exact_header"))throw Error("DSL demand already present");
  changes.push({path,before,after:before+await Bun.file(`${input}/dsl-streamed-law.rs`).text()});
  const fixture=`${base}/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json`;
  if(await Bun.file(fixture).exists())throw Error("DSL fixture already present");
  changes.push({path:fixture,before:"",after:await Bun.file(`${input}/dsl-streamed-fixture.json`).text()});
} else if(process.argv[2]==="dsl-demand-fix"){
  const base="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl";
  await replace(`${base}/🧪️tests/🔬️unit/🦀️.rs`,'    assert_eq!(error.kind().as_str(),fixture["canceledKind"].as_str().unwrap());','    let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=error else{panic!("typed cancellation cause lost")};\n    assert_eq!(refusal.kind().as_str(),fixture["canceledKind"].as_str().unwrap());');
  await replace(`${base}/🧪️tests/🧫️fixtures/📦️operation-pages/🔣️.json`, '"prefix": [1, 0, 0, 1, 1, 7, 130, 64]', '"prefix": [1, 0, 0, 1, 0, 7, 130, 64]');
} else if(process.argv[2]==="dsl-options-demand"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs";
  const before=await Bun.file(path).text();
  const from="variants_binary::encode_op_into(&operation,&mut source,&mut control)";
  if(before.split(from).length!==3)throw Error("DSL two actual demand callers guard");
  changes.push({path,before,after:before.replaceAll(from,"variants_binary::encode_op_into(&operation,&Default::default(),&mut source,&mut control)")});
} else if(process.argv[2]==="dsl"){
  const path="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs";
  const marker="    fn decode_with<T: DslVariants>";
  const before=await Bun.file(path).text();
  if(before.split(marker).length!==2)throw Error("DSL producer exact marker guard");
  changes.push({path,before,after:before.replace(marker,await Bun.file(`${input}/dsl-streamed-variants.rs`).text()+marker)});
} else if (process.argv[2] === "floor") {
  const path = `${root}/📦️bytes/🦀️.rs`;
  if (await Bun.file(path).exists()) throw Error("new source already exists");
  changes.push({ path, before: "", after: await Bun.file(`${input}/🦀️.rs`).text() });
  await replace(`${root}/🦀️.rs`, "#[cfg(test)]\n#[path = \"📦️bytes/🧪️tests/🦀️.rs\"]", "#[path = \"📦️bytes/🦀️.rs\"]\npub mod bytes;\n\n#[cfg(test)]\n#[path = \"📦️bytes/🧪️tests/🦀️.rs\"]");
} else if (process.argv[2] === "oracles") {
  const canonical = JSON.stringify([1.0, false]);
  if (canonical !== "[1,false]") throw Error("independent JSON oracle differs");
  await Bun.write(`${ticket}/🗑️generated/child-emission-ui-json-oracle.json`, JSON.stringify({ producer: "Bun JSON.stringify", input: "[1.0,false]", canonical, nativeExactVariantAssertionsChanged: false }, null, 2));
  await replace(`${plugin}/🔬️app-merge-ui-values/🦀️.rs`, "DslValue::from(&serde_json::json!([1.0, false]))", "DslValue::from(&serde_json::from_str::<serde_json::Value>(\"[1,false]\").expect(\"independent JSON.stringify fixture\"))");
  await replace(`${plugin}/🔬️app-typed-command-full-operation/🦀️.rs`, 'publisher[freshness..].find("if let Some(pending) = mounted.pending_artifact_publication.as_mut()")', 'publisher[freshness..].find("\\n            if let Some(pending) = mounted.pending_artifact_publication.as_mut()")');
} else if (process.argv[2] === "core-span-transfer") {
  const path = "🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const before = await Bun.file(path).text();
  const from = '        self.check_span_utf8(bytes,offset,what)?;\n        String::from_utf8(self.copy_span_bytes(bytes)?).map_err(|_|PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what,offset,detail:"validated source utf8 changed under immutable borrow"})';
  if (before.split(from).length !== 3 || before.split("trait MaterializationAdmission {").length !== 2) throw Error("exact UTF8 transfer guards");
  const after = before.replaceAll(from, "        copy_validated_span_utf8(self,bytes,offset,what)").replace("trait MaterializationAdmission {", await Bun.file(`${input}/core-validated-text-transfer.rs`).text() + "\ntrait MaterializationAdmission {");
  changes.push({ path, before, after });
} else if (process.argv[2] === "core-span") {
  const path = "🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs";
  const before = await Bun.file(path).text();
  let after = before;
  const edit = (from: string, to: string, count = 1) => { if (after.split(from).length !== count + 1) throw Error(`Core span guard: ${from}`); after = after.replaceAll(from, to); };
  const append = "    fn append_chunk(&self,_file:";
  edit(append, await Bun.file(`${input}/core-span-materialization.rs`).text() + append);
  const controlledEnd = "}\n\nfn copy_decoded_string";
  edit(controlledEnd, await Bun.file(`${input}/core-controlled-span-text.rs`).text() + controlledEnd);
  edit("limits: &PackLimits) -> Result<&'b [u8], PackRefusal>", "limits: &PackLimits) -> Result<crate::ByteSpan<'b>, PackRefusal>");
  edit('reader.read_bytes(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "inline blob byte length" })?)', 'reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "inline blob byte length" })?)');
  edit('ctx.materialization.copy_utf8(bytes,offset,"text")', 'ctx.materialization.copy_span_utf8(bytes,offset,"text")');
  edit("ctx.materialization.copy_bytes(bytes)", "ctx.materialization.copy_span_bytes(bytes)");
  edit("reader.read_bytes(row_count.div_ceil(8))?", "reader.read_span(row_count.div_ceil(8))?", 2);
  edit('reader.read_bytes(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol byte length" })?)', 'reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol byte length" })?)');
  edit('materialization.copy_utf8(raw,symbol_offset,"symbol")', 'materialization.copy_span_utf8(raw,symbol_offset,"symbol")');
  const decoder = "fn decode_inline_symbols(reader:";
  edit(decoder, await Bun.file(`${input}/core-span-decoder.rs`).text() + "\n" + decoder);
  changes.push({ path, before, after });
} else if (process.argv[2] === "core-span-demand") {
  const path = "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs";
  const before = await Bun.file(path).text();
  if (before.includes("fn record_operation_pages_decode_cross_page_")) throw Error("span demand already mounted");
  changes.push({ path, before, after: before + "\n" + await Bun.file(`${input}/core-span-law.rs`).text() });
} else if (process.argv[2] === "reader-demand") {
  const path = `${root}/📦️bytes/🧪️tests/🦀️.rs`;
  const before = await Bun.file(path).text();
  const law = await Bun.file(`${input}/borrowed-reader-law.rs`).text();
  if (before.includes("fn owned_operation_byte_pages_borrowed_reader_")) throw Error("reader demand already mounted");
  changes.push({ path, before, after: before + "\n" + law });
} else if (process.argv[2] === "reader-static-refusal") {
  await replace("🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs", 'span.contiguous().ok_or_else(|| PackRefusal::from(semio_framework_value::ValueError::new(ValueRefusalKind::UnsupportedOwner, "contiguous read requires a slice source; use the borrowed byte range")))?', 'span.contiguous().ok_or(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::UnsupportedOwner, what: "operation byte source", offset: self.pos as u64, detail: "contiguous read requires a slice source; use the borrowed byte range" })?');
  await replace(`${root}/📦️bytes/🧪️tests/🦀️.rs`, '        let crate::PackRefusal::ValueRefusal(error) = error else { panic!("source read refusal category lost") };\n        assert_eq!(error.kind, crate::value::ValueRefusalKind::UnsupportedOwner);', '        assert_eq!(error.kind(), crate::value::ValueRefusalKind::UnsupportedOwner);\n        assert!(matches!(error, crate::PackRefusal::RetainedMalformed { offset: 0, .. }));');
} else if (process.argv[2] === "reader") {
  const path = "🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs";
  const before = await Bun.file(path).text();
  const start = before.indexOf("/// 👓️ A bounds-checked cursor over a borrowed byte slice");
  const end = before.indexOf("/// ✍️ An append-only byte buffer", start);
  if (start < 0 || end < 0 || before.indexOf("pub struct ByteReader", start + 1) !== before.indexOf("pub struct ByteReader")) throw Error("exact reader region guard");
  changes.push({ path, before, after: before.slice(0, start) + await Bun.file(`${input}/borrowed-byte-reader.rs`).text() + "\n\n" + before.slice(end) });
  await replace(`${root}/📦️bytes/🦀️.rs`, "    pub fn byte_at(&self, offset: usize) -> Option<u8> { self.bytes.get(offset).copied() }", "    pub fn byte_at(&self, offset: usize) -> Option<u8> { self.bytes.get(offset).copied() }\n    pub fn byte_ref(&self, offset: usize) -> Option<&u8> { self.bytes.get(offset) }");
} else if (process.argv[2] === "core") {
  const path = "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs";
  const before = await Bun.file(path).text();
  let after = before;
  const edit = (from: string, to: string) => { if (after.split(from).length !== 2) throw Error(`Core one-span guard failed: ${from}`); after = after.replace(from, to); };
  edit("struct Output{bytes:Option<Vec<u8>>,length:usize}\nimpl Output{", "struct Output<'a>{bytes:Option<Vec<u8>>,external:Option<&'a mut dyn protocol::mutation::bytes::OperationByteOutput>,length:usize}\nimpl<'a> Output<'a>{");
  edit("Self{bytes:None,length:0}", "Self{bytes:None,external:None,length:0}");
  edit("Self{bytes:Some(control.allocate_vec(length)?),length:0}", "Self{bytes:Some(control.allocate_vec(length)?),external:None,length:0}");
  edit("if let Some(output)=&mut self.bytes{", "if let Some(output)=&mut self.external{output.write_bytes(bytes,control)?;}if let Some(output)=&mut self.bytes{");
  const marker = "/// 🎞️ Borrows one complete intrinsic owner through symbol admission and exact byte emission.";
  edit(marker, await Bun.file(`${input}/core-streamed-encoder.rs`).text() + "\n" + marker);
  changes.push({ path, before, after });
  await replace("🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs", "/// 🎞️ Encodes a borrowed complete intrinsic value without a synthetic owned record.", "/// 📦️ Appends one exact Record body after the caller's existing prefix and returns its body length.\n/// The caller retains every accepted byte across refusal; no operation header is reset or replaced.\npub fn encode_record_body_into(spec:&RecordSpec,record:&RecordValue,options:&EncodeOptions,output:&mut dyn protocol::mutation::bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<usize,PackRefusal>{controlled_encoding::record_body_into(spec,record,options,output,control)}\n\n/// 🎞️ Encodes a borrowed complete intrinsic value without a synthetic owned record.");
} else throw Error("floor, oracles, core, reader-demand or reader required");
await Bun.write(`${input}/${process.argv[2]}-guarded-pairs.json`, JSON.stringify({ pairs: changes.map(change => ({ ...change, beforeSha256: new Bun.CryptoHasher("sha256").update(change.before).digest("hex") })) }, null, 2));
for (const change of changes) {
  const current = await Bun.file(change.path).exists() ? await Bun.file(change.path).text() : "";
  if (current !== change.before) throw Error(`concurrent full-file guard changed: ${change.path}`);
  await mkdir(change.path.slice(0, change.path.lastIndexOf("/")), { recursive: true });
  await Bun.write(change.path, change.after);
}
console.log(JSON.stringify({ mounted: changes.length, command: process.argv[2] }));

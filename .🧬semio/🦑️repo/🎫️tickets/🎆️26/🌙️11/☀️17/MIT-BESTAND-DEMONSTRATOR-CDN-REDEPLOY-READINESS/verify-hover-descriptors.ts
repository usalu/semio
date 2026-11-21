import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {isDeepStrictEqual} from "node:util";
import {fileDigest} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import {decodeDescriptorPairBytesV1,verifySemanticDescriptorPairBytesV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import {normalizeCatalogDescriptorEnums,rejectDuplicateJsonObjectNames} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🛡️guards/🟦️.ts";
import {SHIPPED_COMPONENT_PROFILE} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import {canonicalDescriptorDiffV1,componentStageTargetV1,componentStageTargetsV1,settleComponentDescriptorV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import {decodePackValue,encodePackValue,packUInt,packValueToExactJson,type PackValue} from "../../../../../../../🧰️framework/🔨️modules/🎁️packs/🟦️.ts";

const workspace=resolve(import.meta.dirname,"../../../../../../..");
const sources:{path:string;text:string}[]=[];
const readSource=(relativePath:string):string=>{
  const path=join(workspace,relativePath),existing=sources.find(source=>source.path===path);
  if(existing)return existing.text;
  const text=readFileSync(path,"utf8");
  sources.push({path,text});
  return text;
};
const [actions,ids,argument,label]=[
  "🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/🕹️interaction-window-bodies/🦀️.rs",
  "🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🕹️interaction-verbs/🦀️.rs",
  "🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🧾️action-arg-definition/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🦀️.rs",
].map(readSource);
assert.ok(actions&&ids&&argument&&label);
const hoverId=ids.match(/pub const INTERACTION_HOVER_ACTION_ID: &str = "([^"]+)";/u)?.[1];
assert.equal(hoverId,"interactionHover");
assert.match(argument,/Self::with_schema\(id, label, Self::plain_string\(None\)\)/u);
assert.match(argument,/ArgSchema::String \{ options: Vec::new\(\), option_source: None, min_len: None, max_len: None, pattern: None, format \}/u);
assert.match(argument,/schema, presentation: None, required: false/u);
assert.match(argument,/self\.required = true/u);
assert.match(label,/pub fn native\(en: &str, de: &str\)/u);
assert.match(label,/Self::from_fn\(\|_terminology, locale\| \{\s*match locale \{\s*Locale::En => en,\s*Locale::De => de,/u);
const block=actions.match(/INTERACTION_HOVER_ACTION_ID, LocalizedLabel::native\([^\n]+\.with_args\(\[\n([\s\S]+?)\n\s*\]\),/u)?.[1];
assert.ok(block,"Current Rust hover argument declaration");
const args=block.split("\n").map(line=>{
  const definition=line.trim().match(/^ActionArgDef::text\("([^"]+)", LocalizedLabel::native\("([^"]+)", "([^"]+)"\)\)(\.required\(\))?,$/u);
  assert.ok(definition,`Current hover argument: ${line}`);
  const [,id,en,de,required]=definition;
  return {id,label:{native:{en,de},reuse:{en,de}},schema:{kind:"string"},required:required!==undefined};
});

const rows=[
  ["📐️cad","wgIlEJ7","031f1c25025828757358aeff","9f1024ed98a91df43b7c5dc41bc40b5e9c552fe33fc58b5c6e1cc042bdbd3544",4],
  ["🌀️procedural","wqks1dx","dc8f6f5f5c2ed62d70ef136e","c8239ef4a8155a66aabf79bf10012a171c2c13179e9099a44d29a97cd0485f0a",11],
] as const;
const readPair=(directory:string)=>({json:readFileSync(join(directory,"🔣️.json")),pack:readFileSync(join(directory,"🛂️.descriptor.semio"))});
const mode=process.argv[2];
assert.ok([undefined,"--write","--roster","--write-roster"].includes(mode),"Use --write or --write-roster to publish verified pairs");
const write=mode==="--write",writeRoster=mode==="--write-roster";
const previousArgs=args.slice(0,3).map(arg=>({...arg,required:true}));
type Port={id:string;label:unknown};
type Dialect={artifactKind:string;standard:string;subset:string};
type IoRow={owner:Dialect;counterpart:Dialect;direction:"import"|"export"};
type ComposerRow={writes:Dialect;reads:Dialect[]};
type Descriptor={
  executionProtocol:{appChannelVersion:PackValue};
  contributions:{topicContributions?:unknown[];ioEntries?:IoRow[];composerEntries?:ComposerRow[]};
  manifest:{
    topicContributions?:unknown[];
    apps:readonly {actions?:{id:string}[];windowKinds?:readonly {actions?:{id:string;args:unknown[]}[]}[];io?:{ports?:Port[]};mediaOutputs?:Port[]}[];
  };
};
const project=(descriptor:Descriptor):number=>{
  let count=0;
  for(const app of descriptor.manifest.apps)for(const window of app.windowKinds??[])for(const action of window.actions??[]){
    if(action.id!==hoverId)continue;
    if(!isCurrent(action.args))assert.deepStrictEqual(action.args,previousArgs,"Previous hover action signature");
    action.args=args;
    count++;
  }
  return count;
};
const isCurrent=(candidate:unknown)=>isDeepStrictEqual(candidate,args);
const spliceJsonArgs=(original:Buffer,from:typeof args,to:typeof args)=>{
  const source=original.toString("utf8"),before=JSON.stringify(from,null,2),after=JSON.stringify(to,null,2);
  const fragments:string[]=[];
  let cursor=0,count=0;
  for(const match of source.matchAll(/^([ \t]*)"args": (?=\[)/gmu)){
    const indentation=match[1]!,start=match.index+match[0].length;
    const expected=before.replaceAll("\n","\n"+indentation);
    if(!source.startsWith(expected,start))continue;
    fragments.push(source.slice(cursor,start),after.replaceAll("\n","\n"+indentation));
    cursor=start+expected.length;
    count++;
  }
  fragments.push(source.slice(cursor));
  return {json:Buffer.from(fragments.join("")),count};
};
const sha=(bytes:Buffer)=>createHash("sha256").update(bytes).digest("hex");
const unmatchedRows=(left:unknown[],right:unknown[])=>{
  const available=right.slice(),unmatched=[];
  for(const row of left){
    const index=available.findIndex(candidate=>isDeepStrictEqual(candidate,row));
    if(index===-1)unmatched.push(row);
    else available.splice(index,1);
  }
  return unmatched;
};
const recheckSources=()=>{
  for(const source of sources)assert.equal(readFileSync(source.path,"utf8"),source.text,"Current Rust hover schema changed during verification");
};
const localized=(en:string,de:string)=>({native:{en,de},reuse:{en,de}});
const protocolMirrors=new Set(["animate","architect","bim","block","forms","imperative","lowpoly","raster","sequence","shooting","space","vcs","writer"]);
const artifactSubset="🏅️standards/🔖️1/🪆️subsets/✳️any";
const blockArtifacts=["◻️2d","🧊️3d","🖐️5d"].map(kind=>`✏️s/🔌️plugins/🧱️block/🗿️artifacts/${kind}/${artifactSubset}`);
const animateArtifact=`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/${artifactSubset}`;
const sequenceArtifact=`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/${artifactSubset}`;
const vcsArtifact=`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/${artifactSubset}`;
const writerArtifact=`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/${artifactSubset}`;
const textCarrierOwners=new Map([
  ["animate",[{path:animateArtifact,kind:"s.animate.presentation"}]],
  ["block",blockArtifacts.map((path,i)=>({path,kind:`s.block.block${[2,3,5][i]}d`}))],
  ["sequence",[{path:sequenceArtifact,kind:"s.sequence.sequence"}]],
  ["vcs",[{path:vcsArtifact,kind:"s.vcs.vcs"}]],
  ["writer",[{path:writerArtifact,kind:"s.writer.writer"}]],
]);
const portSources=new Map([
  ["forms",{paths:["✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"],ports:["dictionary:out"]}],
  ["imperative",{paths:["✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"],ports:["result:out"]}],
  ["lowpoly",{paths:["✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"],ports:["mesh:in","mesh:out"]}],
  ["shooting",{paths:["✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️ui-helpers-and-io/🦀️.rs"],ports:["photos:out","photos:out"]}],
  ["animate",{paths:[`${animateArtifact}/✏️editor/🔌️presentation-io-and-frame-helpers/🦀️.rs`],ports:["frames:in"]}],
  ["block",{paths:blockArtifacts.map(path=>`${path}/✏️editor/🦀️.rs`),ports:["catalog:out","catalog:out","catalog:out"]}],
  ["sequence",{paths:[`${sequenceArtifact}/✏️editor/🧱️editor-vocabulary/🦀️.rs`],ports:["steps:in"]}],
  ["writer",{paths:[`${writerArtifact}/✏️editor/🧱️editor-vocabulary/🦀️.rs`],ports:["text:out"]}],
]);

/** 🚪️ Projects the registered text carrier and the Rust registry's (from, into) dialect order. */
function reconcileTextCarrier(plugin:string,json:Descriptor,pack:Descriptor):boolean{
  const owners=textCarrierOwners.get(plugin);
  if(!owners)return false;
  const schema=readSource("🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs");
  const mechanism=readSource("🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧭️io-mechanism/🦀️.rs");
  const dialect=readSource("🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🦀️.rs");
  const producer=readSource("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs");
  const carrier=schema.match(/pub const CARRIER_TEXT: Dialect = Dialect \{ artifact_kind: "([^"]+)", standard: StandardId\("([^"]+)"\), subset: SubsetId\("([^"]+)"\) \};/u);
  assert.ok(carrier);
  const [,artifactKind,standard,subset]=carrier;
  assert.ok(artifactKind&&standard&&subset);
  assert.match(dialect,/Ord,Hash,Serialize,Deserialize[\s\S]+?pub struct ArtifactDialect \{ pub artifact_kind:String,pub standard:String,pub subset:String \}/u);
  assert.match(mechanism,/type EntryKey = \(ArtifactDialect, ArtifactDialect\);\s*type EntryMap = BTreeMap<EntryKey/u);
  assert.match(mechanism,/let key = \(ArtifactDialect::from\(entry.from\), ArtifactDialect::from\(entry.into\)\);/u);
  assert.match(mechanism,/pub fn io_native_routes\(\)[\s\S]+?registry\s*\.values\(\)/u);
  assert.match(producer,/for \(writes, reads\) in io::list_composer_entries\(\)[\s\S]+?for route in io::io_mechanism::io_native_routes\(\)/u);
  for(const owner of owners){
    const io=`${owner.path}/🚪️io`;
    for(const [path,field] of [["📤️export/🧵️serializers","INTO"],["📥️import/🧩️deserializers","FROM"]]){
      const source=readSource(`${io}/${path}/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`);
      assert.match(source,/pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;/u);
      assert.match(source,new RegExp(`const ${field}: Dialect = TXT_DIALECT;`));
    }
  }
  const before=pack.contributions.ioEntries??[],authored=json.contributions.ioEntries??[];
  const legacy={artifactKind:"s.stdio.txt",standard,subset},current={artifactKind,standard,subset};
  const oldRows=before.filter(row=>isDeepStrictEqual(row.counterpart,legacy));
  if(!oldRows.length)return false;
  assert.deepStrictEqual(oldRows.map(row=>`${row.owner.artifactKind}:${row.direction}`).sort(),owners.flatMap(owner=>[`${owner.kind}:export`,`${owner.kind}:import`]).sort());
  const composerPrefix=(pack.contributions.composerEntries??[]).flatMap(({writes,reads})=>reads.map(counterpart=>({owner:writes,counterpart,direction:"import"})));
  assert.deepStrictEqual(before.slice(0,composerPrefix.length),composerPrefix);
  const key=(row:IoRow)=>row.direction==="export"?[row.owner,row.counterpart]:[row.counterpart,row.owner];
  const compare=(left:IoRow,right:IoRow)=>{
    const [from,into]=key(left),[otherFrom,otherInto]=key(right);
    const compareDialect=(a:Dialect,b:Dialect)=>Buffer.compare(Buffer.from(a.artifactKind),Buffer.from(b.artifactKind))||Buffer.compare(Buffer.from(a.standard),Buffer.from(b.standard))||Buffer.compare(Buffer.from(a.subset),Buffer.from(b.subset));
    return compareDialect(from!,otherFrom!)||compareDialect(into!,otherInto!);
  };
  const suffix=before.slice(composerPrefix.length).map(row=>isDeepStrictEqual(row.counterpart,legacy)?{...row,counterpart:current}:row).sort(compare);
  const actual=[...before.slice(0,composerPrefix.length),...suffix];
  assert.ok(isDeepStrictEqual(actual,authored),`${plugin} source-backed I/O registry order`);
  pack.contributions.ioEntries=actual;
  return true;
}

/** 🧬️ Reconciles independently declared Pack mirror fields for rosters with previously split forms. */
function reconcileSourceBackedMirror(plugin:string,json:Descriptor,pack:Descriptor):string[]{
  const changes:string[]=[];
  if(protocolMirrors.has(plugin)){
    const protocol=readSource("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs");
    const producer=readSource("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs");
    const validation=readSource("🧰️framework/🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts");
    const version=Number(protocol.match(/pub const CHANNEL_VERSION: u32 = (\d+);/u)?.[1]);
    assert.match(producer,/execution_protocol: ExecutionProtocol \{ app_channel_version: protocol::CHANNEL_VERSION \}/u);
    assert.equal(Number(validation.match(/export const APP_CHANNEL_VERSION = (\d+);/u)?.[1]),version);
    assert.equal(json.executionProtocol.appChannelVersion,version);
    const previous=packValueToExactJson(pack.executionProtocol.appChannelVersion);
    if(previous!==version){
      assert.equal(previous,version-1);
      pack.executionProtocol.appChannelVersion=packUInt(BigInt(version));
      changes.push("app-channel");
    }
  }
  const labels=portSources.get(plugin);
  if(labels){
    const definitions=new Map(labels.paths.flatMap(path=>[...readSource(path).matchAll(/id: "([^"]+)"\.into\(\),\s*label: (?:semio_plugin::)?LocalizedLabel::native\("([^"]+)", "([^"]+)"\),/gu)].map(([,id,en,de])=>[id,{en:en!,value:localized(en!,de!)}] as const)));
    const readPorts=(descriptor:Descriptor)=>descriptor.manifest.apps.flatMap(app=>[...app.io?.ports??[],...app.mediaOutputs??[]]).filter(port=>labels.ports.includes(port.id));
    const authored=readPorts(json),packed=readPorts(pack);
    assert.deepStrictEqual(authored.map(port=>port.id),labels.ports);
    assert.deepStrictEqual(packed.map(port=>port.id),labels.ports);
    for(const [index,port] of packed.entries()){
      const source=definitions.get(port.id);
      assert.ok(source,`${plugin} source label ${port.id}`);
      assert.deepStrictEqual(authored[index]!.label,source.value);
      if(!isDeepStrictEqual(port.label,source.value)){
        assert.equal(port.label,source.en);
        port.label=source.value;
        changes.push(port.id);
      }
    }
  }
  if(plugin==="vcs"){
    const source=readSource(`${vcsArtifact}/✏️editor/🧭️editor-and-manifest/🦀️.rs`);
    const name=source.match(/\.view_action\("(noOperation)", LocalizedLabel::native\("No-operation", "Keine Aktion"\)\)/u)?.[1];
    assert.ok(name);
    assert.match(source,/"noOperation" => Ok\(VcsCommand::NoOperation/u);
    const declared=json.manifest.apps.flatMap(app=>app.actions??[]).filter(action=>action.id===name);
    const previous=pack.manifest.apps.flatMap(app=>app.actions??[]).filter(action=>action.id==="noMutation"||action.id===name);
    assert.equal(declared.length,1);
    assert.equal(previous.length,1);
    if(previous[0]!.id!==name){
      previous[0]!.id=name;
      changes.push(name);
    }
  }
  if(reconcileTextCarrier(plugin,json,pack))changes.push("text-carrier");
  if(plugin==="space"){
    const schema=readSource("✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs");
    const registration=readSource("🌎️hub/🧩️compositions/🪐️space/🦀️.rs");
    const catalogue=readSource("🧰️framework/🔨️modules/🛂️manifest/🧩️contribution/🦀️.rs");
    const dialect=schema.match(/SPACE_INDEX_DIALECT:.+?artifact_kind: "([^"]+)", standard: semio_plugin::app::StandardId\("([^"]+)"\), subset: semio_plugin::app::SubsetId::ANY/u);
    const documentSchema=schema.match(/S_SPACE_INDEX_DOCUMENT_SCHEMA: &str = "([^"]+)";/u)?.[1];
    const topic=catalogue.match(/pub const SPACE_INDEX_TOPIC: &str = "([^"]+)";/u)?.[1];
    assert.ok(dialect&&documentSchema&&topic);
    assert.match(schema,/dialect: SPACE_INDEX_DIALECT\.into\(\), document_schema: S_SPACE_INDEX_DOCUMENT_SCHEMA\.to_string\(\) \}\.into_topic_contribution\(\)/u);
    assert.match(registration,/\.contributes_topic\(semio_s_artifact_space_space::space_index_topic_contribution\(\)\)/u);
    const contribution={topic,payload:{dialect:{artifactKind:dialect[1],standard:dialect[2],subset:"*"},documentSchema}};
    for(const [authored,mirror] of [[json.contributions,pack.contributions],[json.manifest,pack.manifest]]){
      const index=authored.topicContributions?.findIndex(row=>isDeepStrictEqual(row,contribution))??-1;
      assert.ok(index>=0);
      const entries=mirror.topicContributions??[];
      if(!entries.some(row=>isDeepStrictEqual(row,contribution))){
        entries.splice(index,0,contribution);
        mirror.topicContributions=entries;
        changes.push(topic);
      }
    }
  }
  return changes;
}
const evidence=[];
for(const [component,run,layout,sha256,count] of rows){
  const ownerRoot=join(workspace,"🌎️hub/🧩️compositions",component,"🧩️component");
  const target=componentStageTargetV1(workspace,join(ownerRoot,"📦️packages/🦀️rust/Cargo.toml"));
  assert.equal(target.ownerRoot,ownerRoot);
  const before=readPair(ownerRoot),candidate=readPair(join(import.meta.dirname,"🗑️generated",`describe-${run}-current`));
  const authored=verifySemanticDescriptorPairBytesV1(before.json,before.pack);
  const actual=verifySemanticDescriptorPairBytesV1(candidate.json,candidate.pack);
  assert.equal(authored.role,target.role);
  assert.deepEqual([actual.pluginId,actual.packageId,actual.role,actual.version],[target.pluginId,target.packageId,target.role,authored.version]);
  const expected=JSON.parse(before.json.toString("utf8"));
  assert.equal(project(expected),count);
  assert.deepStrictEqual(JSON.parse(candidate.json.toString("utf8")),expected,`${target.pluginId} independently projected descriptor`);
  const reverted=spliceJsonArgs(candidate.json,args,previousArgs);
  assert.equal(reverted.count,count);
  assert.ok(spliceJsonArgs(reverted.json,previousArgs,args).json.equals(candidate.json),`${target.pluginId} Rust JSON bytes`);
  const packed=decodeDescriptorPairBytesV1(before.json,before.pack).packed;
  assert.equal(project(packed as unknown as Descriptor),count);
  assert.deepStrictEqual(Buffer.from(encodePackValue(packed)),candidate.pack,`${target.pluginId} Rust Pack bytes`);
  const wasm=join(workspace,`.cache/b/r/m/components/runs/${run}/w/${layout}.linked/component.wasm`);
  assert.equal(await fileDigest(wasm,new AbortController().signal),sha256);
  evidence.push({target,before,run,count,actual,wasm,sha256,candidate});
}

for(const {target,before,run,count,actual,wasm,sha256,candidate} of evidence){
  const deliverable={profile:SHIPPED_COMPONENT_PROFILE,directory:dirname(wasm),wasm,receipt:join(dirname(wasm),"🧾️component.json")};
  const emission={...actual,componentSha256:sha256,jsonBytes:candidate.json,packBytes:candidate.pack};
  if(write){
    assert.deepStrictEqual(readPair(target.ownerRoot),before,"Current authored descriptor changed during verification");
    recheckSources();
    settleComponentDescriptorV1(workspace,target,deliverable,emission,"write");
    assert.deepStrictEqual(settleComponentDescriptorV1(workspace,target,deliverable,emission,"verify"),[]);
    const published=readPair(target.ownerRoot);
    assert.deepStrictEqual(published,candidate);
    assert.equal(verifySemanticDescriptorPairBytesV1(published.json,published.pack).semanticSha256,actual.semanticSha256);
  }
  console.log(JSON.stringify({plugin:target.pluginId,run,actions:count,semanticSha256:actual.semanticSha256,written:write}));
}

if(mode==="--roster"||writeRoster){
  const ids=new Set([
    "remodel","raster","flow","process","norm","norm-energy","norm-eurocode-materials","norm-eurocode-structural","norm-eurocode",
    "block","sequence","writer","animate","space","vcs","gis","imperative","bim","sourcing","forms","architect","shooting",
    "puzzle","playbook","lowpoly","energy",
  ]);
  const allTargets=componentStageTargetsV1(workspace);
  const targets=allTargets.filter(target=>ids.has(target.pluginId));
  assert.deepStrictEqual(new Set(targets.map(target=>target.pluginId)),ids);
  assert.equal(targets.length,ids.size);
  const candidates=targets.flatMap(target=>{
    let detail:readonly string[]=[];
    let ioDifference:unknown;
    try{
    const before=readPair(target.ownerRoot);
    const text=new TextDecoder("utf-8",{fatal:true}).decode(before.json);
    rejectDuplicateJsonObjectNames(text);
    const record=JSON.parse(text) as Descriptor;
    const packed=decodePackValue(before.pack);
    assert.ok(Buffer.from(encodePackValue(packed)).equals(before.pack),"Authored Pack canonicality");
    const count=project(record);
    assert.equal(project(packed as unknown as Descriptor),count);
    assert.ok(count>0,`${target.pluginId} Rust hover actions`);
    const reconciled=reconcileSourceBackedMirror(target.pluginId,record,packed as unknown as Descriptor);
    const {json:jsonBytes,count:changed}=spliceJsonArgs(before.json,previousArgs,args);
    assert.deepStrictEqual(JSON.parse(jsonBytes.toString("utf8")),record,`${target.pluginId} changed only the expected JSON values`);
    const normalizedJson=normalizeCatalogDescriptorEnums(record) as Descriptor,normalizedPack=normalizeCatalogDescriptorEnums(packValueToExactJson(packed)) as Descriptor;
    detail=canonicalDescriptorDiffV1(normalizedJson,normalizedPack,12);
    if(detail.some(line=>line.startsWith(".contributions.ioEntries"))){
      const jsonEntries=normalizedJson.contributions.ioEntries??[],packEntries=normalizedPack.contributions.ioEntries??[];
      const unrelated=canonicalDescriptorDiffV1({...normalizedJson,contributions:{...normalizedJson.contributions,ioEntries:[]}}, {...normalizedPack,contributions:{...normalizedPack.contributions,ioEntries:[]}},12);
      ioDifference={jsonRows:jsonEntries.length,packRows:packEntries.length,jsonOnly:unmatchedRows(jsonEntries,packEntries),packOnly:unmatchedRows(packEntries,jsonEntries),unrelated};
    }
    assert.equal(detail.length,0,"Projected JSON and Pack differ outside the common hover schema");
    const packBytes=Buffer.from(encodePackValue(packed));
    const actual=verifySemanticDescriptorPairBytesV1(jsonBytes,packBytes);
    assert.deepStrictEqual([actual.pluginId,actual.packageId,actual.role],[target.pluginId,target.packageId,target.role]);
    return [{target,before,count,changed,reconciled,pair:{jsonBytes,packBytes},actual}];
    }catch(error){
      console.log(JSON.stringify({plugin:target.pluginId,directory:relative(workspace,target.ownerRoot),exception:String(error).slice(0,450),detail,ioDifference}));
      return [];
    }
  });
  for(const {target,before,count,changed,reconciled,pair,actual} of candidates){
    const already=before.pack.equals(pair.packBytes)&&before.json.equals(pair.jsonBytes);
    const deliverable={profile:SHIPPED_COMPONENT_PROFILE,directory:target.ownerRoot};
    if(writeRoster){
      assert.deepStrictEqual(readPair(target.ownerRoot),before,"Current authored descriptor changed during verification");
      recheckSources();
      settleComponentDescriptorV1(workspace,target,deliverable,pair,"write");
      assert.deepStrictEqual(settleComponentDescriptorV1(workspace,target,deliverable,pair,"verify"),[]);
      const published=readPair(target.ownerRoot);
      assert.equal(verifySemanticDescriptorPairBytesV1(published.json,published.pack).semanticSha256,actual.semanticSha256);
    }
    console.log(JSON.stringify({plugin:target.pluginId,directory:relative(workspace,target.ownerRoot),actions:count,changed,reconciled,previousPackSha256:sha(before.pack),semanticSha256:actual.semanticSha256,jsonSha256:sha(pair.jsonBytes),written:writeRoster&&!already}));
  }
  const confirmed=["energy","fem","flow-extension-bim","flow-extension-dictionary"];
  for(const plugin of confirmed){
    const owner=allTargets.filter(target=>target.pluginId===plugin);
    assert.equal(owner.length,1);
    const target=owner[0]!,authored=readPair(target.ownerRoot);
    const receipt=JSON.parse(readFileSync(join(import.meta.dirname,"🗑️generated","confirmed",`${plugin}.json`),"utf8"));
    assert.deepStrictEqual([receipt.pluginId,receipt.packageId,receipt.profile],[target.pluginId,target.packageId,SHIPPED_COMPONENT_PROFILE]);
    const confirmedSha256=receipt.semanticDescriptorSha256,authoredSha256=verifySemanticDescriptorPairBytesV1(authored.json,authored.pack).semanticSha256;
    const projectedSha256=candidates.find(candidate=>candidate.target===target)?.actual.semanticSha256??authoredSha256;
    console.log(JSON.stringify({confirmation:plugin,confirmedSha256,authoredSha256,projectedSha256,requeue:confirmedSha256!==authoredSha256}));
  }
  if(candidates.length!==targets.length)process.exitCode=1;
}

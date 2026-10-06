import assert from "node:assert/strict";
import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";
import {createRequire} from "node:module";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch="1"]=process.argv.slice(2),out=join(ticket,"🗑️generated/board-json-serializer",command+"-"+epoch),input=join(import.meta.dir,"📥️inputs"),sha=(s:string)=>createHash("sha256").update(s).digest("hex"),require=createRequire(join(root,"package.json"));
assert.ok(["stage","native"].includes(command));assert.equal(existsSync(out),false);mkdirSync(out,{recursive:true});
const originPath=join(ticket,"🗑️generated/unified-board-caller-owner/source-1.json"),originRaw=readFileSync(originPath,"utf8"),origin=JSON.parse(originRaw),path="🧰️framework/🔨️modules/🖱️ui/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs",base=origin.rows.find((x:any)=>x.path===path);assert.ok(base);
const inputs=["🧬️schema.json","🧫️fixtures.json","held-source.json"].map(name=>({path:join(input,name),source:readFileSync(join(input,name),"utf8")})),schema=JSON.parse(inputs[0].source),fixture=JSON.parse(inputs[1].source),validate=new(require("ajv"))({strict:false,allErrors:true}).compile(schema);assert.equal(validate(fixture),true,JSON.stringify(validate.errors));
const declaration='    #[derive(semio_framework_value_derive::ToValue)]\n    struct BoardLodScaleJson {\n        id: &\'static str,\n        name: &\'static str,\n        description: &\'static str,\n        #[value(rename = "maxZoom")]\n        max_zoom: f64,\n    }\n\n',lod='    pub fn puzzle_2d_lod_scale_json(control: &mut semio_framework_value::NativeEncodeControl<\'_>) -> Result<String, NormalPortError> {\n        control.checkpoint()?;\n        let rows: [BoardLodScaleJson; 6] = std::array::from_fn(|index| {\n            let lod = &PUZZLE_2D_LODS[index];\n            BoardLodScaleJson { id: lod.id, name: lod.name, description: lod.description, max_zoom: lod.max_zoom }\n        });\n        Ok(semio_framework_pack_json::to_json_string_controlled(&rows, control)?)\n    }',pickHelper='    fn encode_board_pick_targets(rows: &Vec<BoardPickTargetJson>, control: &mut semio_framework_value::NativeEncodeControl<\'_>) -> Result<String, NormalPortError> {\n        Ok(semio_framework_pack_json::to_json_string_controlled(rows, control)?)\n    }\n\n',highlightHelper='    fn encode_board_highlight_ids(ids: &BTreeSet<String>, control: &mut semio_framework_value::NativeEncodeControl<\'_>) -> Result<String, NormalPortError> {\n        Ok(semio_framework_pack_json::to_json_string_controlled(ids, control)?)\n    }\n\n';
const extract=(source:string,name:string)=>{const start=source.indexOf(name);assert.ok(start>=0,name);const indent=name.match(/^ */)![0],open=source.indexOf("{",start),next=source.indexOf("\n"+indent+"}",open);assert.ok(next>open,name);return source.slice(start,next+indent.length+2);};
const oldLod=extract(base.after,"    pub fn puzzle_2d_lod_scale_json()"),oldPick=extract(base.after,"        pub fn pick_targets_at_screen_json("),oldHighlights=extract(base.after,"        pub fn highlighted_ids_json(");
assert.ok(oldLod.includes('crate::board::json_backend::to_string(&rows)'));assert.ok(oldPick.includes('crate::board::json_backend::to_string(&self.resolve_pick_targets_world(world))'));assert.ok(oldHighlights.includes('self.highlighted_ids.iter().cloned().collect::<Vec<_>>()'));
const newPick='        pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64, control: &mut semio_framework_value::NativeEncodeControl<\'_>) -> Result<String, NormalPortError> {\n            control.checkpoint()?;\n            let world = self.screen_to_world(Point::new(sx, sy));\n            encode_board_pick_targets(&self.resolve_pick_targets_world(world), control)\n        }',newHighlights='        pub fn highlighted_ids_json(&self, control: &mut semio_framework_value::NativeEncodeControl<\'_>) -> Result<String, NormalPortError> {\n            encode_board_highlight_ids(&self.highlighted_ids, control)\n        }';
const cuts=[{before:oldLod,after:declaration+pickHelper+highlightHelper+lod},{before:oldPick,after:newPick},{before:oldHighlights,after:newHighlights}];let after=base.after;for(const cut of cuts){assert.equal(after.split(cut.before).length,2);after=after.replace(cut.before,cut.after);}
const inverse=cuts.reduceRight((s,cut)=>{assert.equal(s.split(cut.after).length,2);return s.replace(cut.after,cut.before);},after);assert.equal(inverse,base.after);
const assertion=(s:string)=>[...s.matchAll(/assert(?:_eq|_ne)?!\(/gu)].length;assert.equal(assertion(after),assertion(base.after));
const lodStart=base.after.indexOf("    const PUZZLE_2D_LODS:"),lodEnd=base.after.indexOf("    const PUZZLE_2D_LOD_SCALE:"),lodSource=base.after.slice(lodStart,lodEnd),pickStart=base.after.lastIndexOf("    #[derive(",base.after.indexOf("    struct BoardPickTargetJson {")),pickEnd=base.after.indexOf("\n    }",pickStart)+6,pickSource=base.after.slice(pickStart,pickEnd);
assert.ok(lodStart>=0&&lodEnd>lodStart&&pickStart>=0&&pickEnd>pickStart);
const errorsStart=base.after.indexOf("    #[derive(Debug)]"),errorsEnd=base.after.indexOf("    //#endregion ⚠️ Errors",errorsStart),errorsSource=base.after.slice(errorsStart,errorsEnd);assert.ok(errorsSource.includes("impl From<semio_framework_value::ValueError>"));
const productionRoot=join(root,path),current=existsSync(productionRoot)?readFileSync(productionRoot,"utf8"):null,providers=["🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs","🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs"].map(path=>({path,source:readFileSync(join(root,path),"utf8")}));
const {Database}=await import("bun:sqlite"),db=new Database(":memory:");db.exec("CREATE TABLE cases(id TEXT PRIMARY KEY,kind TEXT,expected TEXT)");for(const row of fixture.cases)db.query("INSERT INTO cases VALUES(?,?,?)").run(row.id,row.kind,JSON.stringify(row.expected));assert.equal((db.query("SELECT COUNT(*) AS n FROM cases").get() as any).n,5);db.close();
const proposal={schema:"semio.board.controlled-serialization-supplement/v1",producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},origin:{path:originPath,source:originRaw,sha256:sha(originRaw)},inputs,providers,rows:[{path,before:base.after,after,inverse:base.after,current,physicalBefore:current,originBefore:base.before}],cuts,assertions:{before:assertion(base.after),after:assertion(after)},gaps:["pick target hit resolution and intermediate geometry projection are not bounded by this serializer supplement","underlying JSON writer typed tree retirement is not re-proved here","full external caller controls and actual held whole Board owning law remain required","physical Board owner path is absent; no source publication or recreation requested"],nativeReady:false,publicationReady:false,sourceWrites:0};
writeFileSync(join(out,"proposal.json"),JSON.stringify(proposal));
if(command==="stage"){console.log(JSON.stringify({out,rows:1,cuts:3,assertions:proposal.assertions,physicalBoardPresent:current!==null}));process.exit(0);}
const native=join(out,"native");mkdirSync(native,{recursive:true});writeFileSync(join(native,"🧫️fixtures.json"),inputs[1].source);
const tests=`
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixtures.json")).unwrap() }
    fn control<'a>(callback: &'a mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool) -> semio_framework_value::NativeEncodeControl<'a> { semio_framework_value::NativeEncodeControl::new(4 * 1024 * 1024, callback) }
    #[test]
    fn typed_serializers_equal_independent_serde_json_corpus() {
        let fixture = fixture(); let mut checkpoints = 0;
        let mut observer = |_| { checkpoints += 1; true };
        for case in fixture["cases"].as_array().unwrap() {
            let mut admission = control(&mut observer);
            let output = match case["kind"].as_str().unwrap() {
                "lod" => puzzle_2d_lod_scale_json(&mut admission).unwrap(),
                "picks" => {
                    let rows = case["input"].as_array().unwrap().iter().map(|row| BoardPickTargetJson {domain:row["domain"].as_str().unwrap().into(),id:row["id"].as_str().unwrap().into(),generality:row["generality"].as_u64().unwrap() as u32,label:row["label"].as_str().map(str::to_owned)}).collect::<Vec<_>>();
                    let own = encode_board_pick_targets(&rows, &mut admission).unwrap();
                    assert_eq!(serde_json::from_str::<serde_json::Value>(&own).unwrap(), serde_json::to_value(&rows).unwrap());
                    own
                },
                "highlights" => {
                    let ids = case["input"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<BTreeSet<_>>();
                    let own = encode_board_highlight_ids(&ids, &mut admission).unwrap();
                    assert_eq!(serde_json::from_str::<serde_json::Value>(&own).unwrap(), serde_json::to_value(&ids).unwrap());
                    own
                },
                _ => panic!("Closed corpus kind"),
            };
            assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(), case["expected"], "{}", case["id"]);
            assert!(admission.owned_bytes() > 0);
        }
        assert!(checkpoints > 0);
    }
    #[test]
    fn lod_final_infinite_zoom_keeps_the_authored_null_wire_value() {
        let mut observer = |_| true; let mut admission=control(&mut observer);
        let output = puzzle_2d_lod_scale_json(&mut admission).unwrap();
        let value:serde_json::Value=serde_json::from_str(&output).unwrap();
        assert_eq!(value.as_array().unwrap().len(),6);
        assert!(value[5]["maxZoom"].is_null());
        assert_eq!(value[0]["maxZoom"],serde_json::json!(0.15));
    }
    #[test]
    fn each_serializer_checks_actual_caller_cancellation_and_ownership() {
        let rows=vec![BoardPickTargetJson{domain:"node".into(),id:"node-a".into(),generality:0,label:Some("label".into())}];
        let ids=BTreeSet::from(["node-a".to_owned()]);
        for index in 0..3 {
            let mut callback=|_|false;let mut admission=control(&mut callback);
            let error=match index {0=>puzzle_2d_lod_scale_json(&mut admission),1=>encode_board_pick_targets(&rows,&mut admission),_=>encode_board_highlight_ids(&ids,&mut admission)}.unwrap_err();
            assert!(matches!(error,NormalPortError::Json(ref error) if error.kind==semio_framework_value::ValueRefusalKind::Canceled));
            assert_eq!(admission.owned_bytes(),0);
            let mut callback=|_|true;let mut admission=semio_framework_value::NativeEncodeControl::new(1,&mut callback);
            let error=match index {0=>puzzle_2d_lod_scale_json(&mut admission),1=>encode_board_pick_targets(&rows,&mut admission),_=>encode_board_highlight_ids(&ids,&mut admission)}.unwrap_err();
            assert!(matches!(error,NormalPortError::Json(ref error) if error.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit));
            assert!(admission.owned_bytes()<=1);
        }
    }
}`;
const source='use std::collections::BTreeSet;\nstruct Lod {id:&\'static str,name:&\'static str,description:&\'static str,max_zoom:f64}\n'+errorsSource+lodSource+declaration+pickSource+"\n"+pickHelper+highlightHelper+lod+"\n"+tests;
writeFileSync(join(native,"🦀️.rs"),source);
const manifest='[workspace]\n[package]\nname = "semio-board-json-serializer-probe"\nversion = "0.1.0"\nedition = "2021"\n[lib]\npath = "🦀️.rs"\n[dependencies]\nsemio-framework-pack-json = { path = '+JSON.stringify(join(root,"🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust"))+' }\nsemio-framework-value = { path = '+JSON.stringify(join(root,"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"))+' }\nsemio-framework-value-derive = { path = '+JSON.stringify(join(root,"🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"))+' }\n[dev-dependencies]\nserde = { version = "1.0.219", features = ["derive"] }\nserde_json = "1.0.140"\n';writeFileSync(join(native,"Cargo.toml"),manifest);assert.deepEqual(Bun.TOML.parse(manifest),require("@iarna/toml").parse(manifest));
const {runMutationInventoryProcess,MutationInventoryProcessWorkspace}=await import(join(root,"🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts")),abort=new AbortController(),cancel=()=>abort.abort();process.once("SIGINT",cancel);process.once("SIGTERM",cancel);const runs:any[]=[];
try {
for(const argv of [["generate-lockfile","--offline","--manifest-path",join(native,"Cargo.toml")],["test","--offline","--locked","--manifest-path",join(native,"Cargo.toml"),"--lib"]]){
const workspace=new MutationInventoryProcessWorkspace(),operation={signal:abort.signal,maximumUnits:100000000,maximumOwnedBytes:16777216,workspace,onProgress:(p:any)=>{if(p.stage==="waiting")console.log("[DEBUG] Board serializer units="+p.completed+" captured="+p.capturedBytes);},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))},result=await runMutationInventoryProcess({command:"cargo",argv,cwd:root,environment:{...process.env,CARGO_TARGET_DIR:join(out,"target"),CARGO_BUILD_BUILD_DIR:join(out,"build"),CARGO_BUILD_JOBS:"2"},captureDirectory:join(out,"captures"),budgetMs:300000,maximumCaptureBytes:16777216,compiler:{buildDirectory:join(out,"build"),leaseDirectory:join(out,"leases")}},operation);runs.push(result);writeFileSync(join(out,"runs.json"),JSON.stringify(runs));if(result.code!==0||result.reason!=="exit"){process.exitCode=result.code||1;break;}
}
}finally{process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
const read=(path:string)=>existsSync(path)?readFileSync(path,"utf8"):null;
const posts={inputs:inputs.map(row=>({...row,current:read(row.path),exact:read(row.path)===row.source})),providers:providers.map(row=>({...row,current:read(join(root,row.path)),exact:read(join(root,row.path))===row.source})),origin:{path:originPath,source:originRaw,current:read(originPath),exact:read(originPath)===originRaw},physical:{path,current:read(productionRoot),unchanged:read(productionRoot)===current}};
writeFileSync(join(out,"terminal.json"),JSON.stringify({proposal,runs,posts,nativeSource:{path:join(native,"🦀️.rs"),source},manifest,code:process.exitCode||0,standaloneSerializerProjection:true,wholeBoardLawExecuted:false,sourceWrites:0}));console.log(JSON.stringify({out,code:process.exitCode||0,sourceWrites:0}));


import fs from "node:fs";import path from "node:path";
const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END";
const base="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fixtureDir=base+"/🖥️host/🧪️tests/🔬️artifact-inference-router";
const schema={type:"object",required:["cases"],additionalProperties:false,properties:{cases:{type:"array",minItems:4,items:{type:"object",required:["name","complete","canonicalPayload","accepted"],additionalProperties:false,properties:{name:{type:"string"},complete:{type:"boolean"},canonicalPayload:{anyOf:[{type:"null"},{type:"array",items:{type:"integer",minimum:0,maximum:255}}]},accepted:{type:"boolean"}}}}}};
const fixture={cases:[{name:"pending without payload",complete:false,canonicalPayload:null,accepted:true},{name:"pending with candidate",complete:false,canonicalPayload:[1],accepted:true},{name:"complete empty payload",complete:true,canonicalPayload:[],accepted:true},{name:"complete without payload",complete:true,canonicalPayload:null,accepted:false}]};
const oracle='import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import fixture from"./🔣️.json";import schema from"./🧬️.json";test("inference candidates require a payload only when complete",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,fixture)).toBe(true);const validate=ajv.compile({type:"object",required:["complete","canonicalPayload"],properties:{complete:{type:"boolean"},canonicalPayload:{type:["array","null"],items:{type:"integer",minimum:0,maximum:255}}},if:{properties:{complete:{const:true}}},then:{properties:{canonicalPayload:{type:"array"}}}});const db=new Database(":memory:");for(const row of fixture.cases){const input={complete:row.complete,canonicalPayload:row.canonicalPayload};expect(validate(input)).toBe(row.accepted);const sql=db.query("select (json_extract(?1, \'$.complete\')=0 or json_type(?1, \'$.canonicalPayload\')=\'array\') as accepted").get(JSON.stringify(input))as{accepted:number};expect(Boolean(sql.accepted)).toBe(row.accepted);}db.close();});';
function create(p:string,s:string){if(fs.existsSync(p))throw Error("already exists: "+p);if(path.resolve(p).length>256)throw Error("path limit");fs.writeFileSync(p,s);}
for(const [name,value]of [["🧬️.json",JSON.stringify(schema)],["🔣️.json",JSON.stringify(fixture)],["🟦️.ts",oracle]])create(fixtureDir+"/"+name,value);
const testFile=fixtureDir+"/🦀️.rs";const original=fs.readFileSync(testFile,"utf8");
let test=original.replace("canonical_payload: request.canonical_payload.clone(),","canonical_payload: Some(request.canonical_payload.clone()),");
const marker='    assert!(validate_inference_echo(&request, &valid).await.is_ok());';
const law=`
    let corpus: serde_json::Value = serde_json::from_str(include_str!("./🔣️.json")).expect("inference corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let mut wire = serde_json::to_value(&request).expect("third-party request oracle");
        wire["complete"] = case["complete"].clone();
        wire["actualCacheMode"] = serde_json::json!("cold");
        wire["canonicalPayload"] = case["canonicalPayload"].clone();
        let text = serde_json::to_string(&wire).expect("third-party result wire");
        let echoed: InferenceRouteResult = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native optional payload");
        let foreign: InferenceRouteResult = serde_json::from_str(&text).expect("third-party decoder");
        assert_eq!(echoed, foreign);
        assert_eq!(validate_inference_echo(&request, &echoed).await.is_ok(), case["accepted"].as_bool().expect("accepted"), "{}", case["name"]);
    }`;
if(!test.includes(marker)||test===original)throw Error("native test seam");test=test.replace(marker,marker+law);
if(fs.readFileSync(testFile,"utf8")!==original)throw Error("native test concurrent change");
fs.writeFileSync(ticket+"/🗑️generated/inference-native-test.rs",test);fs.renameSync(ticket+"/🗑️generated/inference-native-test.rs",testFile);
fs.appendFileSync(ticket+"/inference-runtime-shapes-2026-11-05.md","\n### Pending Payload Contract\nAuthored language-neutral cases before the runtime fix: pending null, pending candidate, complete empty payload, and rejected complete null. AJV and SQLite provide independent acceptance oracles; the native router law compares the first-party and serde decoders. Native validation remains pending.\n");
console.log("Authored shared pending-payload contract and native law.");

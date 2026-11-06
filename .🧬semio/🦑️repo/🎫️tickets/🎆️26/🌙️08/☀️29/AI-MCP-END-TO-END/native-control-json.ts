import{readFileSync,writeFileSync,renameSync}from"node:fs";
const root="🧰️framework/🔨️modules/🎒️pack/🔤️json/👁️borrow",ticket=import.meta.dir;
function edit(path:string,change:(s:string)=>string){const old=readFileSync(path,"utf8"),next=change(old);if(next===old)throw Error("unchanged "+path);if(readFileSync(path,"utf8")!==old)throw Error("concurrent "+path);const temp=ticket+"/🗑️generated/control-json-"+Date.now()+".tmp";writeFileSync(temp,next);renameSync(temp,path)}
edit(root+"/🧪️tests/🔣️.json",s=>{const f=JSON.parse(s);for(const[kind,input]of [["grant",{maximumItems:1,maximumCopyBytes:2,maximumCapacityBytes:3,maximumReleaseBytes:4,maximumDepth:5}],["progress",{copiedItems:1,copiedBytes:2,retainedCapacityBytes:3,releasedBytes:4}]])f.cases.push({kind,input,expected:JSON.stringify(input)});return JSON.stringify(f)});
edit(root+"/🧪️tests/🧬️.json",s=>{const f=JSON.parse(s);f.properties.cases.items.properties.kind.enum.push("grant","progress");return JSON.stringify(f)});
edit(root+"/🧪️tests/🦀️.rs",s=>s+`
#[test]
fn neutral_grant_and_progress_borrow_the_exact_canonical_fields(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let expected=row["expected"].as_str().unwrap();match row["kind"].as_str().unwrap(){"grant"=>case(&serde_json::from_value::<RetainedCloneGrant>(row["input"].clone()).unwrap(),expected),"progress"=>case(&serde_json::from_value::<semio_framework_value::RetainedCloneProgress>(row["input"].clone()).unwrap(),expected),_=>{}}}
}
`);
edit(root+"/🦀️.rs",s=>s+'\n#[path="🎟️control/🦀️.rs"]mod control;\n');console.log("Mounted neutral grant/progress native JSON ports and shared laws");

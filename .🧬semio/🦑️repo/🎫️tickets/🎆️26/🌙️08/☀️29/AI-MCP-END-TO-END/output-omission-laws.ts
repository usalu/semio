import{readFileSync,writeFileSync,renameSync}from"node:fs";
const dir="🧰️framework/🔨️modules/🎠️kernel/📤️return/🔤️json",ticket=import.meta.dir;
function edit(path:string,change:(s:string)=>string){const old=readFileSync(path,"utf8"),next=change(old);if(next===old)throw Error("unchanged "+path);if(readFileSync(path,"utf8")!==old)throw Error("concurrent "+path);const temp=ticket+"/🗑️generated/omission-"+Date.now()+".tmp";writeFileSync(temp,next);renameSync(temp,path)}
edit(dir+"/🧪️tests/🔣️.json",s=>{const f=JSON.parse(s);for(const flag of[false,true]){f.cases.push({kind:"numberBound",input:{value:1.5,exclusive:flag},expected:JSON.stringify(flag?{value:1.5,exclusive:true}:{value:1.5})},{kind:"rowAction",input:{icon:"menu",verb:"open",disabled:flag},expected:JSON.stringify(flag?{icon:"menu",verb:"open",disabled:true}:{icon:"menu",verb:"open"})})}return JSON.stringify(f)});
edit(dir+"/🧪️tests/🧬️.json",s=>{const f=JSON.parse(s);f.properties.cases.items.properties.kind.enum.push("numberBound","rowAction");return JSON.stringify(f)});
edit(dir+"/🧪️tests/🟦️.ts",s=>s.replace('JSON.stringify(row.kind==="turn"?{Ok:row.input}:row.input)','JSON.stringify(wire(row))')+`
function wire(row:{kind:string,input:unknown}){if(row.kind==="turn")return{Ok:row.input};const input=row.input;if((row.kind==="numberBound"||row.kind==="rowAction")&&input&&typeof input==="object"){const value={...input}as Record<string,unknown>;const key=row.kind==="numberBound"?"exclusive":"disabled";if(value[key]===false)delete value[key];return value}return input}
test("default boolean omission agrees with independent SQLite object projection",()=>{const db=new Database(":memory:");for(const row of fixture.cases.filter(r=>r.kind==="numberBound"||r.kind==="rowAction")){const path=row.kind==="numberBound"?"$.exclusive":"$.disabled";const found=db.query("select case when json_extract(?1,?2)=0 then json_remove(?1,?2) else json(?1) end as value").get(JSON.stringify(row.input),path)as{value:string};expect(found.value).toBe(row.expected);}db.close();});
`);
edit(dir+"/🧪️tests/🦀️.rs",s=>s.replace('  "decimal"=>{',`  "numberBound"=>{let value:semio_ui_contract::UiNumberBound=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "rowAction"=>{let value:semio_ui_contract::RowAction=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "decimal"=>{`));
edit(dir+"/🖼️ui/🦀️.rs",s=>s.replaceAll('skip_serializing_if = "is_true"','skip_serializing_if = "is_false"'));
edit(dir+"/🧬️projection/🦀️.rs",s=>s.replace('pub(crate) fn is_true(value:&bool)->bool{*value}\n',''));
edit(dir+"/🦀️.rs",s=>s.replace('is_default,is_false,is_true,','is_default,is_false,'));
const p=ticket+"/kernel-output-codecs.ts";edit(p,s=>s.replace('"is_true"','"is_false"'));
console.log("Corrected exact inclusive/enabled omission semantics and authored independent boolean laws");

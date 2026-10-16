/** 🧩️ Demonstrator engine producer laws: the prepare and build targets depend on every wasm producer the framework engine catalog declares. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve,join}from"node:path";
import catalog from"../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json"with{type:"json"};
const workspace=resolve(import.meta.dir,"../../../..");
test("neutral demonstrator prepare and build targets depend on every engine producer the framework catalog declares",()=>{
 const demo=JSON.parse(readFileSync(resolve(import.meta.dir,"../../📋️project.json"),"utf8")),db=new Database(":memory:");
 try{
  db.exec("create table expected(root text primary key,producer text)");
  for(const row of catalog.packages){const project=JSON.parse(readFileSync(join(workspace,row.sourceOwner,"📋️project.json"),"utf8"));db.query("insert into expected values(?,?)").run(row.sourceOwner,project.name+":wasm");}
  const required=(db.query("select producer from expected order by producer").all()as{producer:string}[]).map(row=>row.producer);
  expect(required).toHaveLength(catalog.packages.length);
  for(const target of["prepare-prototype","prepare-release","build"])for(const producer of required)expect(demo.targets[target].dependsOn,target).toContain(producer);
 }finally{db.close();}
});

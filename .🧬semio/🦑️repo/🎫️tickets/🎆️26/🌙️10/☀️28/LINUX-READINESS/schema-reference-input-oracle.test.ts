import{test,expect}from"bun:test";import{Database}from"bun:sqlite";import{readFileSync}from"node:fs";import Ajv from"ajv";
const corpus=JSON.parse(readFileSync("🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧩️module-compile/📥️inputs.json","utf8"));
for(const row of corpus.referenceCases)test("independent native schema input "+row.id,()=>{
 const db=new Database(":memory:"),root=JSON.parse(row.root),docs=row.documents.map((body:string)=>JSON.parse(body)),ajv=new Ajv({strict:false});let compiles=true;
 try{for(const doc of docs)ajv.addSchema(doc);ajv.compile(root)}catch{compiles=false}
 expect(compiles).toBe(row.compiles);
 try{
  db.exec("CREATE TABLE documents(id TEXT NOT NULL, body TEXT NOT NULL)");
  for(let i=0;i<docs.length;i++)db.query("INSERT INTO documents VALUES(?,?)").run(docs[i].$id,row.documents[i]);
  const target="CASE WHEN instr(t.value,'#')>0 THEN substr(t.value,1,instr(t.value,'#')-1) ELSE t.value END";
  const query="WITH RECURSIVE unique_documents AS (SELECT id,body FROM documents GROUP BY id HAVING count(*)=1), reachable(id) AS (SELECT "+target+" FROM json_tree(?) t WHERE t.key='$ref' AND t.type='text' AND length("+target+")>0 UNION SELECT "+target+" FROM reachable r JOIN unique_documents d ON d.id=r.id JOIN json_tree(d.body) t WHERE t.key='$ref' AND t.type='text' AND length("+target+")>0) SELECT DISTINCT d.id FROM unique_documents d JOIN reachable r ON r.id=d.id WHERE d.id<>? ORDER BY d.id";
  expect(db.query(query).all(row.root,root.$id).map((x:any)=>x.id)).toEqual(row.expected);
 }finally{db.close()}
});

test("independent repository area roster",()=>{const taxonomy=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json","utf8"));expect(Object.keys(taxonomy.areaLayers).sort()).toEqual([...corpus.repositoryRoots].sort())});

test("independent Draft07 keyword provenance",()=>{const meta=require("ajv/dist/refs/json-schema-draft-07.json");expect(Object.keys(meta.properties).sort()).toEqual([...corpus.draft07Keywords].sort())});
for(const row of corpus.patternCases)test("independent pattern classification "+row.id,()=>{const ajv=new Ajv({strict:false});let validator:any,compiled=true;try{validator=ajv.compile({pattern:row.pattern})}catch{compiled=false}expect(compiled).toBe(row.compiles);if(validator)for(const entry of row.matches??[])expect(validator(String.fromCharCode(...entry.units))).toBe(entry.expected)});

for(const row of corpus.opacityCases)test("independent source opacity "+row.id,()=>{const taxonomy=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json","utf8")),schema=JSON.parse(readFileSync("🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json","utf8")),protocol=schema.definitions.RelativePreservationProtocol.const,retirement=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json","utf8")).definitions.Policy.const,db=new Database(":memory:");try{db.exec("CREATE TABLE opaque(path TEXT NOT NULL);CREATE TABLE prefixes(prefix TEXT NOT NULL)");for(const prefix of [protocol.capsulePrefix,retirement.markerPrefix])db.query("INSERT INTO prefixes VALUES(?)").run(prefix);for(const value of Object.values(taxonomy.pathExclusions)as any[])if(value.mode==="opaque")db.query("INSERT INTO opaque VALUES(?)").run(value.path.replace(/\/$/,""));const result=db.query("SELECT EXISTS(SELECT 1 FROM opaque WHERE ?=path OR substr(?,1,length(path)+1)=path||'/') OR EXISTS(SELECT 1 FROM json_each(?) CROSS JOIN prefixes WHERE length(value)=length(prefix)+? AND substr(value,1,length(prefix))=prefix AND substr(value,length(prefix)+1) NOT GLOB '*[^0-9a-f]*') AS opaque").get(row.path,row.path,JSON.stringify(row.path.split("/")),protocol.capsuleDigestHex)as any;expect(Boolean(result.opaque)).toBe(row.opaque)}finally{db.close()}});

for(const row of corpus.authoring.cases)test("independent authored BIM structure "+row.id,()=>{const facet=JSON.parse(readFileSync(corpus.authoring.schema,"utf8"));expect(facet.$defs?.[row.definition]).toBeDefined();const ajv=new Ajv({strict:false});for(const path of corpus.authoring.documents)ajv.addSchema(JSON.parse(readFileSync(path,"utf8")));const validate=ajv.compile({$defs:facet.$defs,$ref:"#/$defs/"+row.definition});expect(validate(row.value)).toBe(row.valid)});

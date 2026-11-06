import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = "🧰️framework/🔨️modules/🌱️value/📋️list/🟰️equal/🧪️tests/🧫️fixtures/";
const read = name => JSON.parse(readFileSync(root + name, "utf8"));
const rows = read("🟰️float.json");
const { RetainedSliceEquality } = await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🌱️value/📋️list/🟰️equal/🟦️.ts")).href);
const validate = new Ajv({ strict: true }).compile(read("🟰️float-schema.json"));
if (!validate(rows)) throw Error(JSON.stringify(validate.errors));
const database = new Database(":memory:");
try {
  for (const row of rows) {
    const parse = word => {
      const buffer = new ArrayBuffer(row.width / 8), view = new DataView(buffer);
      if (row.width === 32) { view.setUint32(0, Number.parseInt(word, 16)); return view.getFloat32(0); }
      view.setBigUint64(0, BigInt("0x" + word)); return view.getFloat64(0);
    };
    const actual = database.query("SELECT coalesce(?1 = ?2, 0) AS equal").get(parse(row.left), parse(row.right)).equal;
    if (actual !== Number(row.expected)) throw Error(`SQLite disagrees with ${JSON.stringify(row)}: ${actual}`);
    const element = { byteWidth: row.width / 8, work: 1, inlineEqual: (left, right) => left === right }, cursor = new RetainedSliceEquality(element), body = cursor.fixedCopyBytes + row.width / 4;
    const demand = cursor.demand(1, 1, body, 2);
    cursor.advance([parse(row.left)], [parse(row.right)], body, 2, { maximumItems: 1, maximumCopyBytes: demand, maximumCapacityBytes: 0, maximumReleaseBytes: 0, maximumDepth: 1 });
    if (cursor.equal !== row.expected) throw Error(`typed floating cursor disagrees with ${JSON.stringify(row)}`);
  }
  console.log(`${rows.length} floating equality vectors agree with SQLite`);
  const workRows = read("⛽️work.json"), workValidate = new Ajv({ strict: true }).compile(read("⛽️work-schema.json"));
  if (!workValidate(workRows)) throw Error(JSON.stringify(workValidate.errors));
  for (const row of workRows) {
    const left = Uint8Array.from(row.left), right = Uint8Array.from(row.right);
    const equal = database.query("SELECT ?1 = ?2 AS equal").get(left, right).equal !== 0;
    const first = database.query("WITH RECURSIVE i(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM i WHERE n < length(?1)) SELECT min(n) AS first FROM i WHERE substr(?1,n,1) <> substr(?2,n,1)").get(left, right).first;
    const immediate = row.maximumWork > 0 && (left.length !== right.length || left.length === 0);
    const compared = left.length === right.length && left.length > 0 && row.maximumWork > 1 ? Math.min(row.maximumWork - 1, Math.floor(row.payloadBodyBytes / 2), first ?? left.length) : 0;
    const actual = { work: immediate ? 1 : compared > 0 ? 1 + compared : 0, ready: immediate || (compared > 0 && (first !== null && first <= compared || compared === left.length)) };
    if (actual.work !== row.work || actual.ready !== row.ready || (actual.ready ? equal : null) !== row.equal) throw Error(`SQLite disagrees with ${JSON.stringify(row)}`);
    const cursor = new RetainedSliceEquality({ byteWidth: 1, work: 1, inlineEqual: (a, b) => a === b }), body = cursor.fixedCopyBytes + row.payloadBodyBytes, demand = cursor.demand(left.length, right.length, body, row.maximumWork);
    const receipt = cursor.advance(left, right, body, row.maximumWork, { maximumItems: 1, maximumCopyBytes: demand, maximumCapacityBytes: 0, maximumReleaseBytes: 0, maximumDepth: 1 });
    if (receipt.work !== row.work || cursor.ready !== row.ready || (cursor.equal ?? null) !== row.equal) throw Error(`typed work cursor disagrees with ${JSON.stringify(row)}`);
  }
  console.log(`${workRows.length} bounded-work vectors agree with SQLite`);
  const inspected = read("🔎️inspection.json"), inspectionValidate = new Ajv({strict:true}).compile(read("🔎️inspection-schema.json"));
  if (!inspectionValidate(inspected)) throw Error(JSON.stringify(inspectionValidate.errors));
  for (const row of inspected) { const actual=database.query("SELECT (?1 >= 3 AND ?2 = 1) AS admitted").get(row.work,Number(row.physical)).admitted; if(actual!==Number(row.inspected)) throw Error("inspection admission disagrees: "+row.id); }
  console.log(inspected.length+" staged inspection vectors agree with SQLite");
  const projectionRoot="🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🧪️tests/",projection=JSON.parse(readFileSync(projectionRoot+"projection.json","utf8")),projectionSchema=JSON.parse(readFileSync(projectionRoot+"projection.schema.json","utf8"));
  const projectionValidate=new Ajv({strict:true}).compile(projectionSchema);if(!projectionValidate(projection))throw Error(JSON.stringify(projectionValidate.errors));
  for(const row of projection){const actual=database.query("SELECT CASE WHEN ?1 < 2 THEN 0 WHEN ?2 = 0 THEN 1 WHEN ?3 = 0 THEN 2 END AS work, CASE WHEN ?1 >= 2 AND (?2 = 0 OR ?3 = 0) THEN 1 ELSE 0 END AS failed").get(row.work,Number(row.leftMatches),Number(row.rightMatches));if(actual.work!==row.reported||actual.failed!==Number(row.failed))throw Error("projection vector mismatch");}
  console.log(projection.length+" projection failure vectors agree with SQLite");
  const birth=JSON.parse(readFileSync(projectionRoot+"birth.json","utf8")),birthSchema=JSON.parse(readFileSync(projectionRoot+"birth.schema.json","utf8"));const birthValidate=new Ajv({strict:true}).compile(birthSchema);if(!birthValidate(birth))throw Error(JSON.stringify(birthValidate.errors));
  for(const row of birth){const x=database.query("SELECT (?1 = 1 AND ?2 = 0) AS constructed, CASE WHEN ?1 = 0 THEN \"none\" WHEN ?2 = 1 THEN \"failed\" ELSE \"bytes\" END AS marker").get(Number(row.admitted),Number(row.allocationFails));if(x.constructed!==Number(row.constructed)||x.marker!==row.marker)throw Error("birth vector mismatch");}
  console.log(birth.length+" birth admission vectors agree with SQLite");
  const cloneRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🧪️tests/🧫️fixtures/",cloneRows=JSON.parse(readFileSync(cloneRoot+"staged.json","utf8")),cloneSchema=JSON.parse(readFileSync(cloneRoot+"staged.schema.json","utf8"));const cloneValidate=new Ajv({strict:true}).compile(cloneSchema);if(!cloneValidate(cloneRows))throw Error(JSON.stringify(cloneValidate.errors));for(const row of cloneRows){const actual=database.query("SELECT length(CAST(? AS BLOB)) AS n").get(row.source).n;if(actual!==row.bytes)throw Error("clone UTF-8 vector mismatch");}console.log(cloneRows.length+" staged clone UTF-8 vectors agree with SQLite");
  const bindingRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️work/🧪️tests/🧫️fixtures/",bindings=JSON.parse(readFileSync(bindingRoot+"binding.json","utf8")),bindingSchema=JSON.parse(readFileSync(bindingRoot+"binding.schema.json","utf8"));const bindingValidate=new Ajv({strict:true}).compile(bindingSchema);if(!bindingValidate(bindings))throw Error(JSON.stringify(bindingValidate.errors));for(const row of bindings){const x=database.query("SELECT CASE WHEN ?3 = 0 OR ?2 < CASE WHEN ?1 = 1 THEN 5 ELSE 1 END THEN 0 WHEN ?1 = 1 AND ?4 = 0 THEN 2 WHEN ?1 = 1 THEN 5 ELSE 1 END AS reported, CASE WHEN ?3 = 0 OR ?2 < CASE WHEN ?1 = 1 THEN 5 ELSE 1 END THEN \"denied\" WHEN ?1 = 0 THEN \"unbound\" WHEN ?4 = 1 THEN \"matched\" ELSE \"mismatch\" END AS outcome").get(Number(row.bound),row.work,Number(row.physical),Number(row.matching));if(x.reported!==row.reported||x.outcome!==row.outcome)throw Error("binding vector mismatch");}console.log(bindings.length+" binding-check vectors agree with SQLite");
  const optionRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🧪️tests/🧫️fixtures/",optionRows=JSON.parse(readFileSync(optionRoot+"staged.json","utf8")),optionSchema=JSON.parse(readFileSync(optionRoot+"staged.schema.json","utf8"));const optionValidate=new Ajv({strict:true}).compile(optionSchema);if(!optionValidate(optionRows))throw Error(JSON.stringify(optionValidate.errors));for(const row of optionRows){const actual=database.query("SELECT ? IS NOT NULL AS present").get(row.value).present;if(actual!==Number(row.present))throw Error("option shape mismatch");}console.log(optionRows.length+" optional-shape vectors agree with SQLite");
  const vecRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🧪️tests/🧫️fixtures/",vecRows=JSON.parse(readFileSync(vecRoot+"staged.json","utf8")),vecSchema=JSON.parse(readFileSync(vecRoot+"staged.schema.json","utf8"));const vecValidate=new Ajv({strict:true}).compile(vecSchema);if(!vecValidate(vecRows))throw Error(JSON.stringify(vecValidate.errors));for(const row of vecRows){const actual=database.query("SELECT min(json_array_length(?1), ?2) AS n").get(JSON.stringify(row.values),row.body).n;if(actual!==row.firstBytes)throw Error("vector first-page mismatch");}console.log(vecRows.length+" vector page vectors agree with SQLite");
  const boxRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🧪️tests/🧫️fixtures/",boxRows=JSON.parse(readFileSync(boxRoot+"staged.json","utf8")),boxSchema=JSON.parse(readFileSync(boxRoot+"staged.schema.json","utf8"));const boxValidate=new Ajv({strict:true}).compile(boxSchema);if(!boxValidate(boxRows))throw Error(JSON.stringify(boxValidate.errors));for(const row of boxRows){const actual=database.query("SELECT json_extract(?1, \"$.value\") AS value").get(JSON.stringify(row)).value;if(actual!==row.value)throw Error("boxed source mismatch");}console.log(boxRows.length+" boxed source vectors agree with SQLite");
  const aliasRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔗️alias/🧪️tests/",alias=JSON.parse(readFileSync(aliasRoot+"work.json","utf8")),aliasSchema=JSON.parse(readFileSync(aliasRoot+"work.schema.json","utf8"));const aliasValidate=new Ajv({strict:true}).compile(aliasSchema);if(!aliasValidate(alias))throw Error(JSON.stringify(aliasValidate.errors));const phases=database.query("SELECT count(*) AS n FROM (SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3)").get().n;if(phases!==alias.expectedStages)throw Error("alias stage mismatch");console.log("alias stage vector agrees with SQLite");
  { const projectionRoot="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️work/🧪️tests/🧫️fixtures/",projectionRows=JSON.parse(readFileSync(projectionRoot+"projection.json","utf8")),projectionSchema=JSON.parse(readFileSync(projectionRoot+"projection.schema.json","utf8"));const projectionValidate=new Ajv({strict:true}).compile(projectionSchema);if(!projectionValidate(projectionRows))throw Error(JSON.stringify(projectionValidate.errors));for(const row of projectionRows){const result=database.query("SELECT CASE WHEN ?1<6 OR ?2=0 THEN 0 WHEN ?3=1 THEN 1 ELSE 6 END AS reported, CASE WHEN ?1<6 OR ?2=0 THEN 'denied' WHEN ?3=1 THEN 'failure' ELSE 'projected' END AS outcome, 0 AS copiedItems").get(row.work,Number(row.physical),Number(row.fails));if(result.reported!==row.reported||result.outcome!==row.outcome||result.copiedItems!==row.copiedItems)throw Error("projection admission mismatch")}console.log(projectionRows.length+" paid projection vectors agree with SQLite"); }
  {const root="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🧪️tests/🧫️fixtures/",rows=JSON.parse(readFileSync(root+"close-start.json","utf8")),schema=JSON.parse(readFileSync(root+"close-start.schema.json","utf8")),validate=new Ajv({strict:true}).compile(schema);if(!validate(rows))throw Error(JSON.stringify(validate.errors));for(const row of rows){const actual=database.query("SELECT (?1=0 AND ?2=1) AS changed").get(Number(row.started),Number(row.physical)).changed;if(actual!==Number(row.changed))throw Error("close-start admission mismatch")}console.log(rows.length+" close-start vectors agree with SQLite");}
} finally { database.close(); }

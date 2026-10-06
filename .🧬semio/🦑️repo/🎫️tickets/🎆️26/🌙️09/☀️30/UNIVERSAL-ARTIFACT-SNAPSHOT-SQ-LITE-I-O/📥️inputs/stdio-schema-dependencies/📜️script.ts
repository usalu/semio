/** 🔗️ Stages exact declaration-owned XML and Brep schema leaves and their closed consumer demand. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
const repo = resolve(import.meta.dir,"../../../../../../../../..");
const ticket = join(repo, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O");
const artifacts = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/";
const xmlSchema = artifacts+"📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema";
const brepSchema = artifacts+"🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema";
const fixtureDir = "🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧩️dependencies";
const changes: {path:string,before:string|null,after:string}[] = [];
const edit = (relative:string, transform:(text:string)=>string) => {const path=join(repo,relative);const before=readFileSync(path,"utf8");changes.push({path,before,after:transform(before)});};
const replace = (text:string,before:string,after:string) => {if(text.split(before).length!==2)throw Error("exact guarded region absent: "+before);return text.replace(before,after);};
const add = (relative:string,after:string) => {const path=join(repo,relative);let before:string|null=null;try{before=readFileSync(path,"utf8");}catch{}if(before!==null)throw Error("new demand path exists: "+path);changes.push({path,before,after});};
const phase = process.argv[2];
if(phase==="demand") {
  const dependencies = {
    schemaVersion:1,
    documents:[
      {scope:"s.stdio.xml",export:"document",id:"https://json.schemas.assets.semio-tech.com/s/stdio/xml/1.0/base/snapshot.json",schemaPath:xmlSchema+"/📸️snapshot/🔣️.json",definitions:["📰️xml","🎨️svg","📜️docx","📽️pptx","📕️xlsx"]},
      {scope:"s.stdio.semio",export:"brep-inference",id:"https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/brep/inference.json",schemaPath:brepSchema+"/💡️inferences/🔣️.json",definitions:["🧿️semio"]}
    ],
    contracts:[
      {scope:"s.stdio.svg",schemaPath:artifacts+"🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"},
      {scope:"s.stdio.docx",schemaPath:artifacts+"📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"},
      {scope:"s.stdio.pptx",schemaPath:artifacts+"📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"},
      {scope:"s.stdio.xlsx",schemaPath:artifacts+"📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"},
      {scope:"s.stdio.semio.brep",schemaPath:brepSchema+"/📸️snapshot/🔣️.json"}
    ]
  };
  add(fixtureDir+"/🔣️.json",JSON.stringify(dependencies,null,2)+"\n");
  add(fixtureDir+"/🧬️schema/🔣️.json",JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/hub/stdio/shipped-fleet/sqlite/dependencies.json",title:"Declared Structural Schema Dependencies",const:dependencies},null,2)+"\n");
  edit("🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts",text => text+`
test("primary dependent schema contracts compile independently with exact original leaves", async () => {
  const { default: Ajv } = await import("ajv");
  const dependencies = (await import("../../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧩️dependencies/🔣️.json")).default;
  const ajv = new Ajv({ strict: false, validateFormats: false });
  const read = async (path: string) => JSON.parse(await Bun.file(new URL("../../../../../../" + path, import.meta.url)).text());
  for (const document of dependencies.documents) {
    const schema = await read(document.schemaPath);
    expect(schema.$id).toBe(document.id);
    ajv.addSchema(schema);
  }
  for (const contract of dependencies.contracts) {
    expect(typeof ajv.compile(await read(contract.schemaPath))).toBe("function");
    console.log("[DEBUG] independent structural contract " + contract.scope + ": exact original dependencies resolved");
  }
});

test("primary dependent declarations explicitly own every published structural schema export", async () => {
  const dependencies = (await import("../../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧩️dependencies/🔣️.json")).default;
  for (const document of dependencies.documents) for (const artifact of document.definitions) {
    const definition = JSON.parse(await Bun.file(new URL("../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/" + artifact + "/📜️artifact-definition.json", import.meta.url)).text());
    const claim = document.scope + "#" + document.export;
    expect(definition.runtime_capabilities.filter((row: {category:string,claims:{namespace:string,value:string}[]}) => row.category === "schema" && row.claims.some(value => value.namespace === "schema-export" && value.value === claim))).toHaveLength(1);
    console.log("[DEBUG] declared structural dependency " + definition.id + ": " + claim);
  }
});
`);
} else if(phase==="provider") {
  edit(xmlSchema+"/🦀️.rs",text=>replace(text,"//#region 🔖️Descriptor",`/// 🔗️ Exact XML document leaves retained by every artifact whose structural contract references them.
pub const XML_SHARED_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "s.stdio.xml",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "document", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("📸️snapshot/🟦️.ts"), graphql: include_str!("📸️snapshot/🔗️.graphql"), json_schema: include_str!("📸️snapshot/🔣️.json"), proto: include_str!("📸️snapshot/🛰️.proto") } }],
};

//#region 🔖️Descriptor`));
  for(const artifact of ["📰️xml","🎨️svg","📜️docx","📽️pptx","📕️xlsx"]) {
    const leaf = artifact==="📰️xml" ? "schema::XML_SHARED_SCHEMA_DOCUMENTS" : "semio_s_artifact_stdio_xml::schema::XML_SHARED_SCHEMA_DOCUMENTS";
    edit(artifacts+artifact+"/🦀️.rs",text=>replace(text,"        .formats(formats)","        .formats(formats)\n        .schema_documents("+leaf+")"));
    const owner={"📰️xml":"xml","🎨️svg":"svg","📜️docx":"docx","📽️pptx":"pptx","📕️xlsx":"xlsx"}[artifact]!;
    edit(artifacts+artifact+"/📜️artifact-definition.json",text=>{
      const marker='    {\n      "id": "s.stdio.'+owner+'.schema.';
      const at=text.indexOf(marker);if(at<0)throw Error("schema capability insertion missing");
      return text.slice(0,at)+`    {
      "id": "s.stdio.${owner}.schema.schema-export-s-stdio-xml-document.v1",
      "category": "schema",
      "descriptor": "runtime-capability:schema:schema-export:s.stdio.xml#document",
      "claims": [{ "namespace": "schema-export", "value": "s.stdio.xml#document" }]
    },
`+text.slice(at);
    });
  }
  edit(artifacts+"🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs",text=>replace(text,"exports: &[semio_framework_schema_registry::SchemaExport",`exports: &[semio_framework_schema_registry::SchemaExport { id: "brep-inference", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("../../../🧊️brep/🧬️schema/💡️inferences/🦀️.rs"), typescript: include_str!("../../../🧊️brep/🧬️schema/💡️inferences/🟦️.ts"), graphql: include_str!("../../../🧊️brep/🧬️schema/💡️inferences/🔗️.graphql"), json_schema: include_str!("../../../🧊️brep/🧬️schema/💡️inferences/🔣️.json"), proto: include_str!("../../../🧊️brep/🧬️schema/💡️inferences/🛰️.proto") } }, semio_framework_schema_registry::SchemaExport`));
  edit(artifacts+"🧿️semio/📜️artifact-definition.json",text=>replace(text,'    {\n      "id": "s.stdio.semio.schema.schema-export-s-stdio-semio-child.v1",',`    {
      "id": "s.stdio.semio.schema.schema-export-s-stdio-semio-brep-inference.v1",
      "category": "schema",
      "descriptor": "runtime-capability:schema:schema-export:s.stdio.semio#brep-inference",
      "claims": [{ "namespace": "schema-export", "value": "s.stdio.semio#brep-inference" }]
    },
    {
      "id": "s.stdio.semio.schema.schema-export-s-stdio-semio-child.v1",`));
} else if(phase==="brep-includes") {
  edit(artifacts+"🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs",text=>{
    if(text.split("../../../🧊️brep/🧬️schema/💡️inferences/").length!==6)throw Error("exact five Brep include guards absent");
    return text.replaceAll("../../../🧊️brep/🧬️schema/💡️inferences/","../../🧊️brep/🧬️schema/💡️inferences/");
  });
} else if(phase==="owned-demand") {
  const fixturePath=fixtureDir+"/🔣️.json";
  const before=readFileSync(join(repo,fixturePath),"utf8");
  const fixture=JSON.parse(before);
  fixture.documents[0].definitions=[{artifact:"📰️xml",scope:"s.stdio.xml",export:"document"},{artifact:"🎨️svg",scope:"s.stdio.svg",export:"xml-document"},{artifact:"📜️docx",scope:"s.stdio.docx",export:"xml-document"},{artifact:"📽️pptx",scope:"s.stdio.pptx",export:"xml-document"},{artifact:"📕️xlsx",scope:"s.stdio.xlsx",export:"xml-document"}];
  fixture.documents[1].definitions=[{artifact:"🧿️semio",scope:"s.stdio.semio",export:"brep-inference"}];
  edit(fixturePath,()=>JSON.stringify(fixture,null,2)+"\n");
  edit(fixtureDir+"/🧬️schema/🔣️.json",text=>{const schema=JSON.parse(text);schema.const=fixture;return JSON.stringify(schema,null,2)+"\n";});
  edit("🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts",text=>{
    text=replace(text,"for (const document of dependencies.documents) for (const artifact of document.definitions)","for (const document of dependencies.documents) for (const owner of document.definitions)");
    text=replace(text,' + artifact + "/📜️artifact-definition.json"',' + owner.artifact + "/📜️artifact-definition.json"');
    return replace(text,'const claim = document.scope + "#" + document.export;','const claim = owner.scope + "#" + owner.export;');
  });
} else if(phase==="owned-provider") {
  edit(xmlSchema+"/🦀️.rs",text=>replace(text,`pub const XML_SHARED_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "s.stdio.xml",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "document", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("📸️snapshot/🟦️.ts"), graphql: include_str!("📸️snapshot/🔗️.graphql"), json_schema: include_str!("📸️snapshot/🔣️.json"), proto: include_str!("📸️snapshot/🛰️.proto") } }],
};`,`pub const XML_DOCUMENT_SCHEMA_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("📸️snapshot/🟦️.ts"), graphql: include_str!("📸️snapshot/🔗️.graphql"), json_schema: include_str!("📸️snapshot/🔣️.json"), proto: include_str!("📸️snapshot/🛰️.proto") };

/// 📚️ Publishes XML's own exact document schema export.
pub const XML_SHARED_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "s.stdio.xml",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "document", leaves: XML_DOCUMENT_SCHEMA_LEAVES }],
};`));
  for(const[artifact,owner]of[["🎨️svg","svg"],["📜️docx","docx"],["📽️pptx","pptx"],["📕️xlsx","xlsx"]]){
    edit(artifacts+artifact+"/🦀️.rs",text=>replace(text,".schema_documents(semio_s_artifact_stdio_xml::schema::XML_SHARED_SCHEMA_DOCUMENTS)",'.schema_documents(semio_framework_schema_registry::ScopeSchemaExports { scope: "s.stdio.'+owner+'", exports: &[semio_framework_schema_registry::SchemaExport { id: "xml-document", leaves: semio_s_artifact_stdio_xml::schema::XML_DOCUMENT_SCHEMA_LEAVES }] })'));
    edit(artifacts+artifact+"/📜️artifact-definition.json",text=>{
      text=replace(text,'s.stdio.'+owner+'.schema.schema-export-s-stdio-xml-document.v1','s.stdio.'+owner+'.schema.schema-export-s-stdio-'+owner+'-xml-document.v1');
      if(text.split("s.stdio.xml#document").length!==3)throw Error("exact descriptor/claim XML import guards absent");
      return text.replaceAll("s.stdio.xml#document","s.stdio."+owner+"#xml-document");
    });
  }
} else throw Error("unknown schema dependency phase");
mkdirSync(import.meta.dir,{recursive:true});
writeFileSync(join(import.meta.dir,phase+"-pairs.json"),JSON.stringify(changes,null,2)+"\n");
for(const change of changes)if(change.before!==null && readFileSync(change.path,"utf8")!==change.before)throw Error("concurrent change before guarded publication");
for(const change of changes){mkdirSync(join(change.path,".."),{recursive:true});writeFileSync(change.path,change.after);}
console.log("[DEBUG] exact schema dependency "+phase+" published paths="+changes.length);

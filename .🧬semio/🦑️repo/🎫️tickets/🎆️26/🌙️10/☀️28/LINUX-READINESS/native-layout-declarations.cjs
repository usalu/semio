
const fs=require("node:fs"),assert=require("node:assert/strict"),ts=require("typescript"),Ajv=require("ajv");
const root="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",schema=JSON.parse(fs.readFileSync(root+"🧬️schema/📋️project/🔣️.json","utf8")),project=JSON.parse(fs.readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🐹️go/📋️project.json","utf8"));
const source=fs.readFileSync(root+"🕸️graph/🚪️admission/🟦️.ts","utf8"),tree=ts.createSourceFile("admission.ts",source,ts.ScriptTarget.Latest,true),names=new Set(["readBuildRuntimeSourceEntriesV1","readBuildSourceResourceEntriesV1"]),functions=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&node.name&&names.has(node.name.text));assert.equal(functions.length,names.size);
const code=ts.transpileModule(functions.map(node=>node.getText(tree).replace(/^export /u,"")).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText;
const own=require(process.cwd()+"/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"),readers=new Function("validateJsonSchemaSubset","projectSchema",code+";return {readBuildRuntimeSourceEntriesV1,readBuildSourceResourceEntriesV1}")(own.validateJsonSchemaSubset,schema);
const ajv=new Ajv({strict:false});assert(ajv.validate(schema,project),JSON.stringify(ajv.errors));const target=project.targets["test-layout-oracle"],execution=readers.readBuildRuntimeSourceEntriesV1(target.metadata.semio.runtimeSourceEntries),resources=readers.readBuildSourceResourceEntriesV1(target.metadata.semio.closure.resources);
assert.equal(execution.length,1);assert.equal(resources.length,2);for(const entry of [...execution,...resources])assert(target.inputs.includes(entry),entry);
assert.throws(()=>readers.readBuildRuntimeSourceEntriesV1(resources));
for(const entry of resources){const file=entry.replace("{workspaceRoot}/","");assert(fs.statSync(file).size<=65536,file);}
console.log("[DEBUG] model layout actual project schema and source readers passed: 1 executable, 2 bounded DATA resources");

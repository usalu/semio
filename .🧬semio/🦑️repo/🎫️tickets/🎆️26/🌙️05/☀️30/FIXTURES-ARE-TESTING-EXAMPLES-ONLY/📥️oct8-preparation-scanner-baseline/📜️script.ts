import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
import {cargoPreparationProgramSourcesV1} from "./owner/🟦️.ts";
const workspace=process.env.NX_WORKSPACE_ROOT!,require=createRequire(join(workspace,"package.json")),ts=require("typescript");
const fixture=JSON.parse(readFileSync(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/📦️native-dependencies/🔣️.json"),"utf8"));
const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(output,{recursive:true});
for(const row of fixture.programImports)test(`retained scanner resolves original ${row.literal}`,()=>{
 const root=mkdtempSync(join(output,"scanner-baseline-")),entry=join(root,"entry.ts"),source=`import value from ${row.literal};export default value;\n`;
 writeFileSync(entry,source);writeFileSync(join(root,row.path),"export default 7;\n");
 const parsed=ts.createSourceFile(entry,source,ts.ScriptTarget.Latest,true);expect(parsed.statements[0].moduleSpecifier.text).toEqual(row.path);
 expect(cargoPreparationProgramSourcesV1(root,entry).sources.map(row=>row.path).sort()).toEqual([entry,join(root,row.path)].sort());
});

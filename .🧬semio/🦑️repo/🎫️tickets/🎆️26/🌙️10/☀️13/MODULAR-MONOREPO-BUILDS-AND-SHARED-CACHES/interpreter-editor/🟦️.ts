import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve,posix,win32}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import{Command}from"commander";
import fixture from"./fixture.json"with{type:"json"};
import schema from"./schema.json"with{type:"json"};
const root=resolve(import.meta.dir,"../../../../../../../..");
test("actual interpreter editor routes defer cache/profile/capacity to the central owned driver",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const {readBuildAuthoringCapabilitiesV1}=await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts");
 for(const file of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  const parsed=ts.parseConfigFileTextToJson(file,readFileSync(resolve(root,file),"utf8"));expect(parsed.error).toBeUndefined();const configurations=parsed.config.configurations as any[];
  for(const target of fixture.targets){const rows=configurations.filter(row=>typeof row==="object"&&typeof row.command==="string"&&row.command.includes(fixture.project+":"+target));expect(rows.length).toBeGreaterThan(0);
   for(const row of rows){const command=new Command().exitOverride().allowUnknownOption().argument("runner").argument("repo").argument("verb").argument("task").option("--configuration <mode>").option("--skip-cache").option("--exclude-dependencies").option("--parallel <count>");command.parse(row.command.split(/\s+/u),{from:"user"});expect(command.args.slice(0,4)).toEqual(["bun","repo","run",fixture.project+":"+target]);expect(["prototype","release"]).toContain(command.opts().configuration);
    for(const key of fixture.forbiddenEnvironment)expect(row.env?.[key],row.name+" "+key).toBeUndefined();const capabilities=JSON.parse(row.env.SEMIO_SCRIPT_CAPABILITIES);expect(capabilities.workspaceRoot).toBe(fixture.workspaceRoot);expect(capabilities.native.artifactDirectory).toBe(fixture.artifactDirectory);expect(capabilities.buildMode).toBe(command.opts().configuration);expect(capabilities.native.network).toEqual({offline:true});const admitted=readBuildAuthoringCapabilitiesV1(root,row.command.split(/\s+/u).slice(2),"nx",capabilities);expect(admitted.workspaceRoot).toBe(root);expect(admitted.native.artifactDirectory).toBe(resolve(root,fixture.artifactDirectory));
   }
  }
 }
 const path=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust/📜️script.ts"),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);const native=["TestCompletedNativeScript","TestPollInputNativeScript"];
 for(const name of native){const node=source.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text===name);expect(node).toBeDefined();expect(node!.getText(source)).toContain("runRepositoryExactCargoLaws");}
 for(const [paths,workspace]of[[posix,"/work/semio"],[win32,"C:\\work\\semio"]]as const){expect(paths.resolve(workspace,fixture.workspaceRoot)).toBe(workspace);expect(paths.resolve(workspace,fixture.artifactDirectory)).toBe(paths.join(workspace,...fixture.artifactDirectory.split("/")));}
 console.log("[DEBUG] Real interpreter routes keep finite Original declarations and defer cache, profile and jobs to owned central Cargo admission");
});

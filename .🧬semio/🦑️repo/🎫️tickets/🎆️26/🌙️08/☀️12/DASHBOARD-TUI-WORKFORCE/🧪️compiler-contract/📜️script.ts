import {readFileSync,writeFileSync,renameSync,mkdirSync} from "node:fs";
import {join,resolve} from "node:path";
const ticket=resolve(import.meta.dir,".."), output=join(ticket,"🗑️generated");mkdirSync(output,{recursive:true});
function edit(path:string,transform:(source:string)=>string):void {
  const before=readFileSync(path,"utf8"),after=transform(before);
  if(after===before)throw Error("No edit applied: "+path);
  const staging=join(output,"compiler-contract-stage");writeFileSync(staging,after);
  if(readFileSync(path,"utf8")!==before)throw Error("Concurrent edit: "+path);
  renameSync(staging,path);
}
const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", owner=library+"/📦️packages/🟦️typescript";
if(process.argv[2]==="register") {
  edit(owner+"/📜️script.ts",source=>source.replace('    if (segments[0] === "nx-project-inference") {',`    if (segments[0] === "cargo-library-search-path") {
      if(segments.length!==1)throw Error("Expected test cargo-library-search-path");
      const source=join(this.repoRoot,"${library}/⚡️caching/🧪️tests/📚️library-search-path/🟦️.ts");
      await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"cargo-library-search-path"),budgetMs:150000});
      return;
    }
    if (segments[0] === "nx-project-inference") {`));
  edit(owner+"/📋️project.json",source=>{const document=JSON.parse(source);document.targets["test-cargo-library-search-path"]={executor:"nx:run-commands",cache:false,dependsOn:[],outputs:[],options:{cwd:owner,command:"bun ./📜️script.ts test cargo-library-search-path"}};return JSON.stringify(document,null,2)+"\n";});
  edit(".vscode/🧩️launch.seed.jsonc",source=>source.replace('      "name": "📦️test🦀️cargo🧾️provenance",',`      "name": "📦️test🦀️cargo📚️library-search-path",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-library-search-path",
      "cwd": "\${workspaceFolder}",
      "presentation": {"group":"4_build","order":206.1779}
    },
    {
      "name": "📦️test🦀️cargo🧾️provenance",`));
}
if(process.argv[2]==="pin")edit("rust-toolchain.toml",source=>source.replace('channel = "nightly-2026-07-07"','channel = "nightly-2026-07-20"'));
if(process.argv[2]==="metadata-test") {
  edit(library+"/⚡️caching/🧫️fixtures/📚️library-search-path/🔣️.json",source=>{const document=JSON.parse(source);document.metadataSetting="DoNotEmbed";return JSON.stringify(document,null,2)+"\n";});
  edit(library+"/⚡️caching/🧬️schema/📚️library-search-path/🔣️.json",source=>{const document=JSON.parse(source);document.required.push("metadataSetting");document.properties.metadataSetting={const:"DoNotEmbed"};return JSON.stringify(document,null,2)+"\n";});
  edit(library+"/⚡️caching/🧪️tests/📚️library-search-path/🟦️.ts",source=>source+String.raw`

test("the pinned Cargo consumes the current metadata configuration without obsolete keys", async () => {
  const workspace=getWorkspaceRoot(),text=readFileSync(join(workspace,".cargo/config.toml"),"utf8"),config=Bun.TOML.parse(text) as {unstable:Record<string,unknown>};
  expect(config).toEqual(parse(text));expect(config.unstable["no-embed-metadata"]).toBeUndefined();expect(config.unstable["embed-metadata"]).toBe("DoNotEmbed");
  const child=Bun.spawn(["cargo","-Z","unstable-options","config","get","unstable.embed-metadata","--format","json"],{cwd:workspace,stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
  expect(code,errors).toBe(0);expect(errors).not.toContain("unused config key");expect(JSON.parse(output)).toEqual({unstable:{"embed-metadata":"DoNotEmbed"}});
});
`);
  edit(library+"/⚡️caching/🧫️fixtures/📚️library-search-path/🥒️.feature",source=>source+"\n  Scenario: The current Cargo consumes the metadata policy\n    Given the repository disables embedded crate metadata\n    When the pinned Cargo reads its configuration\n    Then the current embed-metadata setting is DoNotEmbed\n    And no obsolete metadata key remains\n");
}
if(process.argv[2]==="metadata-fix")edit(".cargo/config.toml",source=>source.replace('no-embed-metadata = true','embed-metadata = "DoNotEmbed"'));
if(process.argv[2]==="schema")edit(library+"/⚡️caching/🧪️tests/📚️library-search-path/🟦️.ts",source=>source.replace('import { expect, test } from "bun:test";','import { expect, test } from "bun:test";\nimport Ajv from "ajv";').replace('  const root=mkdtempSync',`  const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧬️schema/📚️library-search-path/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.libraries*fixture.libraryValue+fixture.macroValue).toBe(fixture.expected);
  const root=mkdtempSync`));
if(process.argv[2]==="session-vector") {
  const corpus=library+"/⚡️caching/🧫️fixtures/nx-contract";
  edit(corpus+"/🛂️schema/🔣️.json",source=>{const document=JSON.parse(source),session=document.properties.playgroundSessions;session.required.push("descriptorPrerequisiteSuffix");session.properties.descriptorPrerequisiteSuffix={const:":describe"};return JSON.stringify(document,null,2)+"\n";});
  edit(corpus+"/🔣️.json",source=>{const document=JSON.parse(source);document.playgroundSessions.descriptorPrerequisiteSuffix=":describe";return JSON.stringify(document,null,2)+"\n";});
  edit(library+"/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",source=>source.replace('      assert.deepEqual(session.dependsOn, [vectors.playgroundSessions.prerequisite]);',`      assert.ok(session.dependsOn.includes(vectors.playgroundSessions.prerequisite));
      const descriptors=session.dependsOn.filter((dependency:string)=>dependency!==vectors.playgroundSessions.prerequisite);
      assert.ok(descriptors.length>0);
      assert.ok(descriptors.every((dependency:string)=>dependency.endsWith(vectors.playgroundSessions.descriptorPrerequisiteSuffix)));`));
}
if(process.argv[2]==="manifest") {
  const response=await fetch("https://static.rust-lang.org/dist/2026-07-20/channel-rust-nightly.toml");if(!response.ok)throw Error("Pinned manifest download: "+response.status);
  const source=await response.text();writeFileSync(join(output,"rust-july20-manifest.toml"),source);
  const document=Bun.TOML.parse(source) as any;console.log(JSON.stringify({date:document.date,cargo:document.pkg.cargo.version,rust:document.pkg.rust.version,windows:document.pkg.rust.target["x86_64-pc-windows-msvc"].available,macos:document.pkg.rust.target["aarch64-apple-darwin"].available,linux:document.pkg.rust.target["x86_64-unknown-linux-gnu"].available}));
}

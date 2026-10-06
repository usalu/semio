
test("Nx producers and watchers inherit the pinned Bun runtime on every host", async () => {
  const bootstrap=await import("../../📜️script.ts"),{win32,posix}=await import("node:path");
  expect(bootstrap.pinnedNxRuntimeEnvironment).toBeFunction();
  const rows=JSON.parse(readFileSync(resolve(owner,"../🛠️tools/🟦️bun/🧫️fixtures/🌿️environment/🔣️.json"),"utf8")).cases;
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="pinnedNxRuntimeEnvironment")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pinned-bun-env-")),module=join(directory,"environment.mjs");
  await build({stdin:{contents:'import {win32,posix} from "node:path";\n'+operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const oracle=Bun.spawn(["node","--input-type=module","-e",'import {pinnedNxRuntimeEnvironment} from '+JSON.stringify(pathToFileURL(module).href)+';console.log(JSON.stringify('+JSON.stringify(rows)+'.map(row=>pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform))));'],{stdout:"pipe",stderr:"pipe"});
  const [stdout,stderr,status]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);
  expect(status,stderr).toBe(0);
  const observed=rows.map((row:any)=>bootstrap.pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform));expect(observed).toEqual(rows.map((row:any)=>row.expected));expect(JSON.parse(stdout)).toEqual(observed);
  for(const platform of ["win32","linux","darwin"])expect(()=>bootstrap.pinnedNxRuntimeEnvironment({},"relative/bun",platform)).toThrow("absolute");
  console.log("[DEBUG] pinned Nx Bun environment checked against esbuild/Node on three hosts");
});


test("admits the worker continuation through its schema, Nx cache and independent bundler", async () => {
  const continuation="🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts",wgpu="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
  const taxonomy=JSON.parse(readFileSync(resolve(libraryRoot,"🔣️taxonomy.json"),"utf8")),profile=taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
  const project=JSON.parse(readFileSync(resolve(repoRoot,wgpu,"📦️packages/🟦️typescript/📋️project.json"),"utf8"));
  expect(profile.sourceModulePaths).toContain(continuation);
  expect(project.namedInputs.frameWorkerSources).toContain(`{workspaceRoot}/${continuation}`);
  const {renderWgpuBrowserBundles}=await import(resolve(repoRoot,wgpu,"📽️projection/🟦️.ts"));
  const actual=await renderWgpuBrowserBundles(repoRoot,profile,{taxonomy,entryIds:["frame-worker"]});
  expect(actual.inputs).toContain(continuation);
  const esbuild=await import("esbuild"),oracle=await esbuild.build({absWorkingDir:repoRoot,entryPoints:[`${wgpu}/🎞️frame-worker/🟦️.ts`],bundle:true,platform:"browser",format:"esm",write:false,metafile:true,logLevel:"silent",define:{"import.meta.vitest":"undefined"}});
  expect(Object.keys(oracle.metafile!.inputs)).toContain(continuation);
  expect(actual.nodes).toHaveLength(1);
  expect(oracle.outputFiles).toHaveLength(1);
  for(const code of [actual.nodes[0]!.content,oracle.outputFiles![0]!.text])expect(code).toContain("yieldContinuation");
},30_000);

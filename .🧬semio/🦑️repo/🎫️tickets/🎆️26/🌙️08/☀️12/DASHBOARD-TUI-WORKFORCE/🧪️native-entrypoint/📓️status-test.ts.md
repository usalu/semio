
test("native foreground tasks retain their child failure outcome",async()=>{
  const workspace=process.env.NX_WORKSPACE_ROOT??process.cwd(),corpus=JSON.parse(readFileSync(join(import.meta.dir,"../../🧫️fixtures/🎮️renderer-inputs/🔣️.json"),"utf8"));
  const {cacheInternals}=await import(pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const targets=cacheInternals.playgroundPreparationTargets([corpus.crate+"/Cargo.toml"],workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
  for(const profile of corpus.profiles)for(const operation of ["run","smoke"])expect(targets[`${operation}-${corpus.variant}-native-${profile}`].continuous).toBe(corpus.nativeProcessStatus.continuous);
  const execute=createRequire(import.meta.url)("nx/src/executors/run-commands/run-commands.impl").default;
  const outcome=await execute({command:`bun -e "process.exit(${corpus.nativeProcessStatus.childFailureExit})"`,cwd:workspace,parallel:false,color:false,__unparsed__:[]},{root:workspace,isVerbose:false});
  expect(outcome.success).toBe(false);
});

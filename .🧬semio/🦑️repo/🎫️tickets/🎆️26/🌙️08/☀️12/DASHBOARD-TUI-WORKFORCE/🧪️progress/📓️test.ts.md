
test("interactive dashboard delegates its native owner progress", () => {
  const project=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📋️project.json"),"utf8"));
  expect(project.targets.run.options.env?.SEMIO_NATIVE_OWNER_PROGRESS).toBe("delegated");
  expect(project.targets.build.options.env?.SEMIO_NATIVE_OWNER_PROGRESS).toBeUndefined();
});

test("native progress follows the declared owner and preserves independent child output", async () => {
  const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📣️progress/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/📣️progress/🔣️.json"),"utf8")));
  expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
  const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifacts)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const output=mkdtempSync(join(artifacts,"native-progress-"));
  const workspace=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️owner-command-policy/🔣️.json"),"utf8")).cases.find((row:{classification:string})=>row.classification==="workspace");
  const manifest=join(root,workspace.manifest),cwd=join(root,workspace.cwd);
  const {build}=await import("esbuild");
  const compiled=await build({stdin:{contents:`setTimeout(()=>console.log("[DEBUG] interactive-ready"),${corpus.durationMs});`,loader:"js"},write:false,platform:"node",format:"esm"});
  const code=compiled.outputFiles[0]!.text;
  const collect=async(args:string[],environment:NodeJS.ProcessEnv)=>{
    const process=Bun.spawn(args,{cwd:root,env:environment,stdout:"pipe",stderr:"pipe"});
    const [stdout,stderr,status]=await Promise.all([new Response(process.stdout).text(),new Response(process.stderr).text(),process.exited]);
    return {stdout,stderr,status};
  };
  const wrapper=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts");
  const [oracle,...observed]=await Promise.all([collect(["node","--eval",code],process.env),...corpus.cases.map((row:{mode:string})=>collect([process.execPath,wrapper,"native","owner-command","--manifest",relative(root,manifest),"--cwd",relative(root,cwd)||".","--","node","--eval",code],{...process.env,SEMIO_NATIVE_OWNER_PROGRESS:row.mode,SEMIO_TEST_ARTIFACT_DIR:output}))]);
  expect(oracle.status,oracle.stderr).toBe(0);
  for(let index=0;index<corpus.cases.length;index++){
    const row=corpus.cases[index],result=observed[index]!;
    writeFileSync(join(output,`${row.id}.stdout.log`),result.stdout);writeFileSync(join(output,`${row.id}.stderr.log`),result.stderr);
    expect(result.status,result.stderr).toBe(0);
    expect(result.stdout.split(/\r?\n/).filter(line=>line.startsWith("[DEBUG] interactive-ready")).join("\n")+"\n").toBe(oracle.stdout);
    expect(result.stderr.includes("[native:owner-command] running elapsedMs="),row.id).toBe(row.wrapperProgress);
  }
},40000);

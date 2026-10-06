
test("dashboard-selected owner tests allocate their output without manual environment setup", async () => {
  const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📣️progress/🔣️.json"),"utf8")),ts=(await import("typescript")).default;
  const script=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts"),source=ts.createSourceFile(script,readFileSync(script,"utf8"),ts.ScriptTarget.Latest,true);
  const selected=source.statements.find((node:any)=>node.name?.text==="TestScript").members.find((node:any)=>node.name?.text==="run").body.statements.find((node:any)=>ts.isIfStatement(node)&&node.expression.getText(source)==='segments[0] === "native-owner-command-policy"');
  const environmentPath=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts"),environmentSyntax=ts.createSourceFile(environmentPath,readFileSync(environmentPath,"utf8"),ts.ScriptTarget.Latest,true),environmentSource=environmentSyntax.statements.find((node:any)=>node.name?.text==="repoTestArtifactEnvironment").getText(environmentSyntax).replace(/^export /,"");
  const compilers=[(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText];
  for(const compile of compilers){
    const allocate=new Function("resolve","mkdirSync",compile(environmentSource)+";return repoTestArtifactEnvironment;")(resolve,()=>{});
    let observed:NodeJS.ProcessEnv|undefined;
    const run=new Function("process","resolveTestLevel","join","repoTestArtifactEnvironment","runRepositoryTestCommand",compile(`async function execute(segments:string[]){${selected.getText(source)}}`)+";return execute;")({env:{},execPath:process.execPath},()=>{},join,(repo:string,route:string)=>allocate(repo,route,{}),async(_command:string,_args:string[],options:{env:NodeJS.ProcessEnv})=>{observed=options.env;});
    await run.call({repoRoot:root},[corpus.artifactEnvironment.route]);
    expect(observed?.SEMIO_TEST_ARTIFACT_DIR).toBe(resolve(root,corpus.artifactEnvironment.defaultRelativeRoot,corpus.artifactEnvironment.route));
  }
  console.log("[DEBUG] dashboard-selected owner test output checked through Bun and TypeScript compilers");
});

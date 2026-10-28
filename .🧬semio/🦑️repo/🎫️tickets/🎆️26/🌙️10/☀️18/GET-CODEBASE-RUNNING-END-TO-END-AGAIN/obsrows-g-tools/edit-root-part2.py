
# --- same-stamp analysis law ---------------------------------------------------------------------------------------
a=text.index('test("semantic source same-stamp byte changes cannot bypass current parser admission"')
b=text.index('},20000);',a)+len('},20000);')
text=text[:a]+r"""test("semantic source analysis trusts an identical stamp without reading and judges a moved stamp by the current parser; a source inside the racy window is never persisted",async()=>{
 const root=workspace(),path=join(root,"source.ts"),provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id,revision:fixture.semantic.provider.revision,analyze:semanticParser});writeFileSync(path,fixture.semantic.before);
 try{const captured=await captureObservedSourceInputsV1(()=>observedSourceAnalysisV1(provider,path)),stamp=rawFs.statSync(path),original=rawFs.statSync,readOriginal=rawFs.readFileSync;let reads=0;writeFileSync(path,fixture.semantic.importsChanged);
  const stat=spyOn(rawFs,"statSync").mockImplementation(((file:any,...args:any[])=>String(file)===path?stamp:(original as any)(file,...args)) as typeof rawFs.statSync),read=spyOn(rawFs,"readFileSync").mockImplementation(((file:any,...args:any[])=>{if(String(file)===path)reads++;return (readOriginal as any)(file,...args);}) as typeof rawFs.readFileSync);
  try{expect(await observationsHold(captured.observations)).toBe(true);expect(reads).toBe(0);}finally{stat.mockRestore();read.mockRestore();}
  expect(await observationsHold(captured.observations)).toBe(false);
  const cachePath=join(root,"racy.json");let calls=0;writeFileSync(path,fixture.semantic.before);
  const run=()=>observeInference({cachePath,key:"racy"},()=>{calls++;return observedSourceAnalysisV1(provider,path);});
  await run();await run();expect(calls).toBe(2);expect(rawFs.existsSync(cachePath)).toBe(false);
  Bun.sleepSync(2200);await run();await run();expect(calls).toBe(3);expect(rawFs.existsSync(cachePath)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);"""+text[b:]

# --- parent validation ---------------------------------------------------------------------------------------------
sub(r"""  try{const result=await observeInference({cachePath:join(root,"parent.json"),key:"provider-sharing"},async()=>{const values=[];for(const project of projects)values.push(await project());return {values,validationReads:reads};}),oracle=ts.preProcessFile(fixture.semantic.before,true,true).importedFiles.map(item=>item.fileName);expect(result.values).toEqual(Array.from({length:fixture.semantic.parentValidation.projects},()=>({command:oracle,module:{imports:oracle}})));expect(result.validationReads).toBe(fixture.semantic.parentValidation.expectedReads);}finally{mocked.mockRestore();}""",
r"""  try{const parent=async(key:string)=>{reads=0;return observeInference({cachePath:join(root,key+".json"),key},async()=>{const values=[];for(const project of projects)values.push(await project());return {values,validationReads:reads};});},oracle=ts.preProcessFile(fixture.semantic.before,true,true).importedFiles.map(item=>item.fileName),expected=Array.from({length:fixture.semantic.parentValidation.projects},()=>({command:oracle,module:{imports:oracle}}));
   const unchanged=await parent("provider-sharing-unchanged");expect(unchanged.values).toEqual(expected);expect(unchanged.validationReads).toBe(0);
   writeFileSync(path,fixture.semantic.bodyChanged);
   const result=await parent("provider-sharing");expect(result.values).toEqual(expected);expect(result.validationReads).toBe(fixture.semantic.parentValidation.expectedReads);}finally{mocked.mockRestore();}""")

# --- TOML ----------------------------------------------------------------------------------------------------------
sub('expect(error.tomlObservation.sourceLine).toBe(row.source.split("\\n")[error.tomlObservation.line]);expect(error.cause.message).toContain("0xD800 - 0xDFFF");',
    'expect(error.tomlObservation.sourceLine).toBe(row.source.split("\\n")[error.tomlObservation.line??0]);expect(error.cause.message).toMatch(/0xD800 - 0xDFFF|Unicode scalar value/u);')

  const heldProfile=fixture.profiles.find((row:{profile:string})=>row.profile===fixture.heldRun.profile);
  const ready=join(root,"held-ready"),release=join(root,"held-release"),controller=new AbortController();
  const {runNativeBinary}=await import(join(workspace,fixture.owner,"⌨️native-entrypoint/📜️script.ts"));
  const held=runNativeBinary(join(root,rust,heldProfile.output,binary),["--hold",ready,release],env,root,controller.signal);
  const settled=held.then(()=>({error:undefined}),error=>({error}));
  try {
    const started=Date.now();
    while(!existsSync(ready)){assert.ok(Date.now()-started<fixture.heldRun.readyTimeoutMs,"actual native child did not report readiness");await new Promise(accept=>setTimeout(accept,10));}
    assert.equal(readFileSync(ready,"utf8"),fixture.stdout);
    put(`${rust}/🦀️.rs`, source(fixture.changedStdout));
    await run(); assert.deepEqual(runs(), ["dev", "dev", "release", "release"]); await check(fixture.changedStdout);
    assert.equal(readFileSync(ready,"utf8"),fixture.stdout,"the already running child retains its original image");
  } finally {
    writeFileSync(release,"release\n");
    const timeout=setTimeout(()=>controller.abort(),5000);
    const result=await settled;clearTimeout(timeout);
    assert.equal(result.error,undefined,String(result.error));
  }

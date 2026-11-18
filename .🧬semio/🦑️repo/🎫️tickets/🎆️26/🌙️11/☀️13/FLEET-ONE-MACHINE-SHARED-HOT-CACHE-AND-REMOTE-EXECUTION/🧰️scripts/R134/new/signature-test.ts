 test("a task with more outputs than the old ceiling of 2048 is signed, equal to an independent hash, and a change of its last file changes the signature",async()=>{
  const root=tempRoot("state-sig-wide-");roots.push(root);const files:Record<string,string>={};for(let index=0;index<3000;index++)files["out/d"+(index%30)+"/f"+String(index).padStart(5,"0")]=String(index);writeTree(root,files);
  const retired=2048,sign=outputSignatureV1 as (...arguments_:unknown[])=>Promise<string|undefined>,signature=await sign(root,["out"],retired);
  expect(signature).toMatch(/^[0-9a-f]{64}$/u);
  const rows:string[]=[],walk=(rel:string):void=>{const status=lstatSync(root+"/"+rel);rows.push(rel+"\0"+status.mtimeMs+"\0"+status.size+"\0"+status.ino);if(status.isDirectory())for(const name of readdirSync(root+"/"+rel).sort())walk(rel+"/"+name);};walk("out");
  expect(rows.length).toBeGreaterThan(retired);
  const python=spawnSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())"],{input:rows.join("\n"),encoding:"utf8"});
  expect(python.stdout.trim()).toBe(signature);
  await Bun.sleep(5);writeFileSync(root+"/out/d29/f02999","changed longer");expect(await outputSignatureV1(root,["out"])).not.toBe(signature);
 });

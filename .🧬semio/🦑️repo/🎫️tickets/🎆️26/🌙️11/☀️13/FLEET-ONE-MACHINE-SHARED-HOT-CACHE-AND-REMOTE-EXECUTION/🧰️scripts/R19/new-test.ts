posixTest("an executable file rewritten on a POSIX machine by an editor that creates it anew without the bit keeps the recorded bit everywhere, a bit-only change replicates both ways, and a rewrite that sets the bit wins",async()=>{
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store);
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");write(server,"run.sh","#!/bin/sh\necho one\n");chmodSync(at(server,"run.sh"),0o755);cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 await seedV1(server.options);await joinV1(worker.options);
 const machines=[server,worker];
 for(const who of machines)await open(who);
 const recreate=(who:Machine,path:string,text:string,mode:number):void=>{const temporary=at(who,path)+".new";writeFileSync(temporary,text);chmodSync(temporary,mode);renameSync(temporary,at(who,path));};
 try{
  await converge(machines,()=>existsSync(at(worker,"run.sh")));
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh"),manifestExecutable(server,"run.sh"),manifestExecutable(worker,"run.sh")]).toEqual([true,true,true,true]);
  recreate(worker,"run.sh","#!/bin/sh\necho two\n",0o644);
  await converge(machines,()=>readFileSync(at(server,"run.sh"),"utf8").includes("two")&&manifestExecutable(server,"run.sh")===true&&manifestExecutable(worker,"run.sh")===true);
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh")]).toEqual([true,true]);
  chmodSync(at(server,"run.sh"),0o644);
  await converge(machines,()=>!executableOn(worker,"run.sh")&&manifestExecutable(worker,"run.sh")===false);
  recreate(server,"run.sh","#!/bin/sh\necho three\n",0o644);
  await converge(machines,()=>readFileSync(at(worker,"run.sh"),"utf8").includes("three"));
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh"),manifestExecutable(worker,"run.sh")]).toEqual([false,false,false]);
  recreate(worker,"run.sh","#!/bin/sh\necho four\n",0o755);
  await converge(machines,()=>readFileSync(at(server,"run.sh"),"utf8").includes("four")&&executableOn(server,"run.sh"));
  expect(manifestExecutable(server,"run.sh")).toBe(true);
 }finally{for(const who of machines)await shut(who);}
},120_000);

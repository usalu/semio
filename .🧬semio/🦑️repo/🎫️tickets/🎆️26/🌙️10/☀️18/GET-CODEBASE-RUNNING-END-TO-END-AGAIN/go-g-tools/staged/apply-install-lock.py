import sys
root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/"
ps=root+"📥️install/🔵️.ps1"
s=open(ps).read()
old='        try { $held = $mutex.WaitOne([TimeSpan]::FromMinutes(15)) } catch [Threading.AbandonedMutexException] { $held = $true }\n'
new='''        try { $held = $mutex.WaitOne(0) } catch [Threading.AbandonedMutexException] { $held = $true }
        if (-not $held) {
            Write-Say "Waiting for another process that installs the Go toolchain…" "Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"
            try { $held = $mutex.WaitOne([TimeSpan]::FromMinutes(15)) } catch [Threading.AbandonedMutexException] { $held = $true }
        }
'''
assert old in s
open(ps,'w').write(s.replace(old,new))
t=root+"🧪️tests/📥️install-contract/🟦️.ts"
s=open(t).read()
add='''
test.skipIf(!supported)("two first installs at once download the pinned Go once and both end with the launcher; the second says it waits, in both languages",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{bytes,sha256}=synthetic(root),script=scaffold(root,sha256);let served=0;
    const server=Bun.serve({port:0,hostname:"127.0.0.1",idleTimeout:0,fetch:async request=>{if(new URL(request.url).pathname!==`/${archive}`)return new Response("missing",{status:404});served++;await Bun.sleep(1500);return new Response(bytes);}});
    try{
      const mirror=`http://127.0.0.1:${server.port}`,[first,second]=await Promise.all([install(script,root,mirror),install(script,root,mirror)]);
      expect([first.status,second.status]).toEqual([0,0]);expect(served).toBe(1);
      expect(first.stderr+second.stderr).toContain("Waiting for another process that installs the Go toolchain");expect(first.stderr+second.stderr).toContain("Warte auf einen anderen Prozess, der die Go-Toolchain installiert");
      expect(leftovers(root)).toEqual([]);expect(existsSync(join(root,"bin/repo"))).toBe(true);
    }finally{server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("both installers hold the same lock around the toolchain install: wait at most fifteen minutes, say they wait and why they stop in both languages, and look again once the lock is theirs",()=>{
  const shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  const waiting=["Waiting for another process that installs the Go toolchain…","Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"],stopped=["installing the Go toolchain for 15 minutes","installiert seit 15 Minuten die Go-Toolchain"];
  for(const text of[shell,windows]){for(const line of[...waiting,...stopped])expect(text).toContain(line);}
  expect(shell).toContain("attempts\\" -le 900");expect(shell).toContain("mkdir \\"$1\\"");expect(windows).toContain("FromMinutes(15)");expect(windows).toContain("Global\\\\semio-go-$version");
  expect(windows).toContain("AbandonedMutexException");expect(shell).toContain("kill -0 \\"$owner\\"");
  expect(shell.indexOf("wait_for_lock \\"$target.lock\\"")).toBeLessThan(shell.indexOf("is_pinned_go \\"$target/bin/go\\"",shell.indexOf("wait_for_lock \\"$target.lock\\"")));
  expect(windows.indexOf("$held = $mutex.WaitOne")).toBeLessThan(windows.indexOf("Test-PinnedGo -Path (Join-Path $Target"));
});

test.skipIf(!supported)("the installer names every other repo on the PATH that is not the installed launcher, in both languages, and stays silent for links to it",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{bytes,sha256}=synthetic(root),script=scaffold(root,sha256);
    const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response(bytes)});
    try{
      const stale=join(root,"stale"),linked=join(root,"linked");mkdirSync(stale,{recursive:true});mkdirSync(linked,{recursive:true});
      writeFileSync(join(stale,"repo"),"#!/bin/sh\\n");chmodSync(join(stale,"repo"),0o755);
      const mirror=`http://127.0.0.1:${server.port}`;
      await install(script,root,mirror);
      symlinkSync(join(root,"bin/repo"),join(linked,"repo"));
      const run=await install(script,root,mirror,{PATH:`${linked}:${stale}:/usr/bin:/bin`});
      expect(run.status).toBe(0);
      expect(run.stderr).toContain(`Another repo on PATH shadows or duplicates the launcher: ${stale}/repo`);expect(run.stderr).toContain(`Ein anderes repo im PATH verdeckt oder dupliziert den Launcher: ${stale}/repo`);
      expect(run.stderr).not.toContain(`launcher: ${linked}/repo`);
    }finally{server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);
'''
s=s.rstrip("\n")+"\n"+add
s=s.replace("rmSync,writeFileSync}from\"node:fs\"","rmSync,symlinkSync,writeFileSync}from\"node:fs\"",1)
open(t,'w').write(s)
print("applied")

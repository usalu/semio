import{expect,test}from"bun:test";
import{chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";

const installerSource=fileURLToPath(new URL("../../",import.meta.url)),repositoryRoot=fileURLToPath(new URL("../../../../../../../",import.meta.url));
const pinsRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json",launcherRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher";
const platform=process.platform==="darwin"?"darwin":"linux",arch=process.arch==="arm64"?"arm64":"x64",goArch=arch==="arm64"?"arm64":"amd64",version="1.26.9",archive=`go${version}.${platform}-${goArch}.tar.gz`;
const supported=process.platform==="darwin"||process.platform==="linux";
const standIn=String.raw`#!/bin/sh
here=$(dirname "$0")
printf '%s|%s|%s\n' "$PWD" "$SEMIO_WORKSPACE_ROOT" "$*" >>"$here/chain.log"
if [ -n "$STAND_IN_HOLD" ]; then
  trap 'kill "$held" 2>/dev/null; exit 143' TERM
  echo $$ >"$here/chain.pid"
  sleep "$STAND_IN_HOLD" &
  held=$!
  wait "$held"
fi
[ -z "$STAND_IN_EXIT" ] || exit "$STAND_IN_EXIT"
exit 0
`;
const fakeGo=(root:string)=>`#!/bin/sh
case "$1" in
version) echo "go version go${version} ${platform}/${goArch}" ;;
build) while [ "$#" -gt 0 ]; do [ "$1" = "-o" ] && out="$2"; shift; done; cp "${join(root,"stand-in.sh")}" "$out"; chmod 755 "$out" ;;
*) exit 2 ;;
esac
`;

function synthetic(root:string):{bytes:Uint8Array<ArrayBuffer>;sha256:string}{
  const stage=join(root,"stage");mkdirSync(join(stage,"go/bin"),{recursive:true});writeFileSync(join(root,"stand-in.sh"),standIn);writeFileSync(join(stage,"go/bin/go"),fakeGo(root));chmodSync(join(stage,"go/bin/go"),0o755);
  const made=spawnSync("tar",["-czf",join(root,archive),"-C",stage,"go"]);expect(made.status).toBe(0);
  const bytes=new Uint8Array(readFileSync(join(root,archive)));return{bytes,sha256:createHash("sha256").update(bytes).digest("hex")};
}

function scaffold(root:string,sha256:string):string{
  const checkout=join(root,"checkout"),pins=join(checkout,pinsRelative);mkdirSync(join(pins,".."),{recursive:true});
  writeFileSync(pins,`{"definitions":{"Pins":{"const":{"assets":[\n{"tool":"go","version":"${version}","platform":"${platform}","arch":"${arch}","url":"https://dl.google.com/go/${archive}","sha256":"${sha256}"}\n]}}}}\n`);
  cpSync(join(installerSource),join(checkout,launcherRelative),{recursive:true,filter:path=>!path.includes("/🧪️tests")});
  return join(checkout,launcherRelative,"📥️install/🐚️.sh");
}

async function install(script:string,root:string,mirror:string,extra:Record<string,string>={},flags:string[]=[]){
  const child=Bun.spawn(["bash",script,...flags,join(root,"bin")],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:mirror,...extra},stderr:"pipe",stdout:"ignore"});
  const stderr=await new Response(child.stderr).text();
  return{status:await child.exited,stderr};
}

const leftovers=(root:string)=>existsSync(join(root,"go"))?readdirSync(join(root,"go")).filter(name=>name.startsWith(".partial")||name.endsWith(".lock")):[];

test.skipIf(!supported)("the installer fetches the pinned Go once, checks it before unpacking and reuses it, in both languages",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{bytes,sha256}=synthetic(root),script=scaffold(root,sha256);let served=0;
    const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{if(new URL(request.url).pathname!==`/${archive}`)return new Response("missing",{status:404});served++;return new Response(bytes);}});
    try{
      const mirror=`http://127.0.0.1:${server.port}`;
      const first=await install(script,root,mirror);
      expect(first.status).toBe(0);
      expect(first.stderr).toContain(`Downloading Go ${version}`);expect(first.stderr).toContain(`Lade Go ${version} herunter`);
      expect(first.stderr).toContain("Installed the repo launcher");expect(first.stderr).toContain("Repo-Launcher installiert");
      expect(existsSync(join(root,"go",`v${version}`,"bin/go"))).toBe(true);expect(existsSync(join(root,"bin/repo"))).toBe(true);expect(leftovers(root)).toEqual([]);
      const independent=spawnSync("shasum",["-a","256",join(root,archive)],{encoding:"utf8"});if(independent.status===0)expect(independent.stdout.split(" ")[0]).toBe(sha256);
      expect(served).toBe(1);
      const second=await install(script,root,mirror);
      expect(second.status).toBe(0);expect(second.stderr).not.toContain("Downloading");expect(served).toBe(1);
    }finally{server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(!supported)("a wrong archive and a foreign mirror are refused before anything is unpacked or installed",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{sha256}=synthetic(root),script=scaffold(root,sha256);
    const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("not the pinned archive")});
    try{
      const wrong=await install(script,root,`http://127.0.0.1:${server.port}`);
      expect(wrong.status).not.toBe(0);expect(wrong.stderr).toContain("Checksum mismatch");expect(wrong.stderr).toContain("Prüfsumme");
      expect(existsSync(join(root,"go",`v${version}`))).toBe(false);expect(existsSync(join(root,"bin/repo"))).toBe(false);expect(leftovers(root)).toEqual([]);
    }finally{server.stop(true);}
    const foreign=await install(script,root,"http://example.com/go");
    expect(foreign.status).not.toBe(0);expect(foreign.stderr).toContain("SEMIO_GO_MIRROR must be");expect(foreign.stderr).toContain("SEMIO_GO_MIRROR muss");expect(leftovers(root)).toEqual([]);
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(!supported)("a cancelled download exits with 130 and leaves no partial directory, lock or launcher",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{sha256}=synthetic(root),script=scaffold(root,sha256);
    let release:()=>void=()=>{};const stalled=new Promise<void>(resolve=>{release=resolve;});let started:()=>void=()=>{};const began=new Promise<void>(resolve=>{started=resolve;});
    const server=Bun.serve({port:0,hostname:"127.0.0.1",idleTimeout:0,fetch:()=>new Response(new ReadableStream({async start(controller){controller.enqueue(new Uint8Array(1024));started();await stalled;controller.close();}}))});
    try{
      const child=Bun.spawn(["bash",script,join(root,"bin")],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:`http://127.0.0.1:${server.port}`},stderr:"pipe",stdout:"ignore"});
      await began;
      child.kill("SIGINT");
      await Bun.sleep(300);
      server.stop(true);release();
      const code=await child.exited,text=await new Response(child.stderr).text();
      expect(code).toBe(130);expect(text).toContain("Cancelled");expect(text).toContain("Abgebrochen");
      expect(leftovers(root)).toEqual([]);expect(existsSync(join(root,"bin/repo"))).toBe(false);
    }finally{release();server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("both installers read the Go pin from the shared table, hold no version literal and say every message in English and German",()=>{
  const shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  for(const text of[shell,windows]){expect(text).not.toContain(version);expect(text).toContain("SEMIO_GO_MIRROR");expect(text).toContain("SEMIO_GO_ROOT");expect(text).toContain("GOTOOLCHAIN");}
  expect(shell).toContain("sha256_of");expect(windows).toContain("Get-FileHash");
  const shellSays=[...shell.matchAll(/(?:say|fail) "((?:[^"\\]|\\.)*)" "((?:[^"\\]|\\.)*)"/gu)],windowsSays=[...windows.matchAll(/(?:Write-Say|Stop-Say) "((?:[^"\\]|\\.)*)" "((?:[^"\\]|\\.)*)"/gu)];
  expect(shellSays.length).toBeGreaterThan(8);expect(windowsSays.length).toBeGreaterThan(8);
  for(const[,english,german]of[...shellSays,...windowsSays])expect(english).not.toBe(german);
  const pins=JSON.parse(readFileSync(join(repositoryRoot,pinsRelative),"utf8")).definitions.Pins.const.assets.filter((row:{tool:string})=>row.tool==="go");
  expect(pins.map((row:{platform:string;arch:string})=>`${row.platform}-${row.arch}`).sort()).toEqual(["darwin-arm64","darwin-x64","linux-arm64","linux-x64","win32-arm64","win32-x64"]);
  for(const row of pins)expect(row.url).toStartWith("https://dl.google.com/go/");
});

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

test("the Windows installer replaces a running launcher: earlier moved-aside copies go first, the new file is staged, the old one is renamed aside (Windows renames a running image but never overwrites or deletes it), then the new one takes its name, retried 10…1280 ms, and a final failure moves the old one back",()=>{
  const windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  const steps=['-Filter "repo.exe.old-*" -File','-Destination "$target.partial" -Force','$aside = "$target.old-" +','Move-Item -LiteralPath $target -Destination $aside','try { Move-Item -LiteralPath "$target.partial" -Destination $target; break }','Start-Sleep -Milliseconds ([int](10 * [Math]::Pow(2, $attempt)))','if ($aside) { Move-Item -LiteralPath $aside -Destination $target }','throw\n'].map(step=>windows.indexOf(step));
  expect(steps.every(index=>index>0)).toBe(true);expect([...steps].sort((left,right)=>left-right)).toEqual(steps);
  expect(windows).not.toContain('-Destination $target -Force');expect(windows).toContain('if ($attempt -lt 8)');expect(windows.charCodeAt(0)).toBe(0xfeff);
});

test("both installers hold the same lock around the toolchain install: wait at most fifteen minutes, say they wait and why they stop in both languages, and look again once the lock is theirs",()=>{
  const shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  const waiting=["Waiting for another process that installs the Go toolchain…","Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"],stopped=["installing the Go toolchain for 15 minutes","installiert seit 15 Minuten die Go-Toolchain"];
  for(const text of[shell,windows]){for(const line of[...waiting,...stopped])expect(text).toContain(line);}
  expect(shell).toContain("attempts\" -le 900");expect(shell).toContain("mkdir \"$1\"");expect(windows).toContain("FromMinutes(15)");expect(windows).toContain("New-InstallMutex \"semio-go-$version\"");expect(windows).toContain("Global\\$Name");expect(windows).toContain("Local\\$Name");expect(windows.indexOf("Global\\$Name")).toBeLessThan(windows.indexOf("Local\\$Name"));expect(windows).toContain("UnauthorizedAccessException");expect(windows).not.toMatch(/Mutex\]::new\(\$false, "Global\\semio/u);
  expect(windows).toContain("AbandonedMutexException");expect(shell).toContain("kill -0 \"$owner\"");
  expect(shell.indexOf("wait_for_lock \"$target.lock\"")).toBeLessThan(shell.indexOf("is_pinned_go \"$target/bin/go\"",shell.indexOf("wait_for_lock \"$target.lock\"")));
  expect(windows.indexOf("$held = $mutex.WaitOne")).toBeLessThan(windows.indexOf("Test-PinnedGo -Path (Join-Path $Target"));
});

const chainLog=(root:string)=>existsSync(join(root,"bin/chain.log"))?readFileSync(join(root,"bin/chain.log"),"utf8").split("\n").filter(Boolean):[];
const shadowed=(stderr:string)=>[...stderr.matchAll(/^\[semio\] Another repo on PATH is not the native repo command: (.+)\/repo\. /gmu)].map(match=>match[1]!);
const withInstaller=async(body:(context:{root:string;script:string;mirror:string})=>Promise<void>)=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{bytes,sha256}=synthetic(root),script=scaffold(root,sha256),server=Bun.serve({port:0,hostname:"127.0.0.1",idleTimeout:0,fetch:()=>new Response(bytes)});
    try{await body({root,script,mirror:`http://127.0.0.1:${server.port}`});}finally{server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
};

test.skipIf(!supported)("the installer ends with the native installation through the new launcher from the checkout root, says so in both languages, and neither puts the launcher on PATH nor advises a link",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const run=await install(script,root,mirror);
    expect(run.status).toBe(0);
    const lines=chainLog(root);expect(lines).toHaveLength(1);
    const[cwd,workspace,args]=lines[0]!.split("|");expect(realpathSync(cwd!)).toBe(realpathSync(join(root,"checkout")));expect(realpathSync(workspace!)).toBe(realpathSync(join(root,"checkout")));expect(args).toBe("run repo-mcp:install");
    expect(run.stderr).toContain("Installing the native repo command: repo run repo-mcp:install");expect(run.stderr).toContain("Installiere den nativen repo-Befehl: repo run repo-mcp:install");
    expect(run.stderr).toContain("The native repo command is installed");expect(run.stderr).toContain("Der native repo-Befehl ist installiert");
    for(const forbidden of["ln -sf","Add to PATH","Zum PATH hinzufügen"])expect(run.stderr).not.toContain(forbidden);
  });
},120000);

test.skipIf(!supported)("--launcher-only, before or after the destination, builds only the launcher and names the install command in both languages",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const destination=join(root,"bin");
    for(const order of[["--launcher-only",destination],[destination,"--launcher-only"]]){
      const child=Bun.spawn(["bash",script,...order],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:mirror},stderr:"pipe",stdout:"ignore"});
      const stderr=await new Response(child.stderr).text();expect(await child.exited).toBe(0);
      expect(chainLog(root)).toEqual([]);expect(stderr).toContain(`Launcher only; no repo command was put on PATH. Install the native one: ${join(root,"bin")}/repo run repo-mcp:install`);expect(stderr).toContain("Nur der Launcher; kein repo-Befehl wurde in den PATH gelegt.");
    }
    const unknown=await install(script,root,mirror,{},["--elsewhere"]);expect(unknown.status).not.toBe(0);expect(unknown.stderr).toContain("Unknown option --elsewhere");expect(unknown.stderr).toContain("Unbekannte Option --elsewhere");
  });
},120000);

test.skipIf(!supported)("a failed native installation fails the installer with its exit code in both languages and leaves the launcher installed",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const run=await install(script,root,mirror,{STAND_IN_EXIT:"7"});
    expect(run.status).toBe(1);expect(run.stderr).toContain("The native installation failed with exit code 7");expect(run.stderr).toContain("Die native Installation ist mit Exit-Code 7 fehlgeschlagen");expect(run.stderr).toContain(`retry: ${join(root,"bin")}/repo run repo-mcp:install`);
    expect(existsSync(join(root,"bin/repo"))).toBe(true);
  });
},120000);

test.skipIf(!supported)("cancelling during the native installation stops it, exits with 130, and says the launcher stays installed, in both languages",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const child=Bun.spawn(["bash",script,join(root,"bin")],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:mirror,STAND_IN_HOLD:"60"},stderr:"pipe",stdout:"ignore"});
    const pidFile=join(root,"bin/chain.pid");for(let wait=0;wait<200&&!existsSync(pidFile);wait++)await Bun.sleep(50);
    expect(existsSync(pidFile)).toBe(true);const held=Number(readFileSync(pidFile,"utf8").trim());
    child.kill("SIGINT");
    const code=await child.exited,text=await new Response(child.stderr).text();
    expect(code).toBe(130);expect(text).toContain("Cancelled; the launcher is installed, the native repo command is not.");expect(text).toContain("Abgebrochen; der Launcher ist installiert, der native repo-Befehl nicht.");
    for(let wait=0;wait<100;wait++){try{process.kill(held,0);}catch{break;}await Bun.sleep(50);}
    expect(()=>process.kill(held,0)).toThrow();expect(existsSync(join(root,"bin/repo"))).toBe(true);
  });
},120000);

test.skipIf(!supported)("the installer names every PATH repo that is not the executable of the native receipt, in both languages, never advises a link, and stays silent for the receipt's executable and links to it; the set equals an independent model",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const home=join(root,"home"),receiptDirectory=join(home,".local/share/semio/repo"),nativeDirectory=join(root,"native"),linked=join(root,"linked"),stale=join(root,"stale"),launcherDirectory=join(root,"bin");
    for(const directory of[receiptDirectory,nativeDirectory,linked,stale])mkdirSync(directory,{recursive:true});
    for(const file of[join(nativeDirectory,"repo"),join(stale,"repo")]){writeFileSync(file,"#!/bin/sh\n");chmodSync(file,0o755);}
    symlinkSync(join(nativeDirectory,"repo"),join(linked,"repo"));
    writeFileSync(join(receiptDirectory,"installed.json"),JSON.stringify({version:1,platform,digest:"a".repeat(64),executable:join(nativeDirectory,"repo"),directory:nativeDirectory,source:"/s",mode:"prototype"})+"\n");
    const search=[linked,nativeDirectory,stale,launcherDirectory,"/usr/bin","/bin"],run=await install(script,root,mirror,{PATH:search.join(":")},["--launcher-only"]);
    expect(run.status).toBe(0);
    const native=realpathSync(join(nativeDirectory,"repo")),model=search.filter(entry=>existsSync(join(entry,"repo"))&&realpathSync(join(entry,"repo"))!==native);
    expect(shadowed(run.stderr)).toEqual(model);expect(model).toEqual([stale,launcherDirectory]);
    expect(run.stderr).toContain(`Another repo on PATH is not the native repo command: ${stale}/repo. The native command owns the PATH repo, install it: repo run repo-mcp:install`);expect(run.stderr).toContain(`Ein anderes repo im PATH ist nicht der native repo-Befehl: ${stale}/repo. Der native Befehl besitzt das repo im PATH, installieren: repo run repo-mcp:install`);
    expect(run.stderr).not.toContain("ln -sf");
  });
},120000);

test.skipIf(!supported)("without a native receipt every PATH repo is named, a receipt that names a missing executable counts as none, and a PATH without repo is silent",async()=>{
  await withInstaller(async({root,script,mirror})=>{
    const stale=join(root,"stale"),receiptDirectory=join(root,"home/.local/share/semio/repo");mkdirSync(stale,{recursive:true});mkdirSync(receiptDirectory,{recursive:true});
    writeFileSync(join(stale,"repo"),"#!/bin/sh\n");chmodSync(join(stale,"repo"),0o755);
    const without=await install(script,root,mirror,{PATH:`${stale}:/usr/bin:/bin`},["--launcher-only"]);expect(shadowed(without.stderr)).toEqual([stale]);
    writeFileSync(join(receiptDirectory,"installed.json"),JSON.stringify({version:1,executable:join(root,"missing/repo")})+"\n");
    const missing=await install(script,root,mirror,{PATH:`${stale}:/usr/bin:/bin`},["--launcher-only"]);expect(shadowed(missing.stderr)).toEqual([stale]);
    const silent=await install(script,root,mirror,{PATH:"/usr/bin:/bin"},["--launcher-only"]);expect(silent.status).toBe(0);expect(shadowed(silent.stderr)).toEqual([]);
  });
},120000);

test("the launcher-only refusal of repo mcp and both installers name the one command that owns the PATH repo, in both languages, and the bootstrap callers skip the chain",()=>{
  const registry=JSON.parse(readFileSync(join(installerSource,"🗂️registry/🔣️.json"),"utf8")).messages.domainNeedsInstall,shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  const command="repo run repo-mcp:install";for(const text of[registry.en,registry.de,shell,windows])expect(text).toContain(command);
  expect(registry.en).toContain("launcher only");expect(registry.de).toContain("nur der Launcher");expect(registry.en).toContain("{verb}");expect(registry.de).toContain("{verb}");
  for(const text of[shell,windows]){expect(text).not.toContain("ln -sf");expect(text).not.toContain("Add to PATH");expect(text).not.toContain("Zum PATH hinzufügen");}
  const bootstrap=join(repositoryRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap"),action=readFileSync(join(repositoryRoot,".github/actions/repo-launcher/action.yml"),"utf8");
  expect(readFileSync(join(bootstrap,"🐚️.sh"),"utf8")).toContain('bash "$LAUNCHER_INSTALL" --launcher-only "$LAUNCHER_BIN"');expect(readFileSync(join(bootstrap,"🔵️.ps1"),"utf8")).toContain('"-Destination", $script:LauncherBin, "-LauncherOnly"');
  expect(action).toContain('🐚️.sh" --launcher-only "$dir"');expect(action).toContain("-Destination $dir -LauncherOnly");
});

test("the shell and the Windows installer read the receipt and the launcher directory the native installation policy declares",()=>{
  const policy=JSON.parse(readFileSync(join(repositoryRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json"),"utf8")).$defs.Protocol.const,shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  expect(shell).toContain(`"$HOME/${policy.unixReceiptDirectory.join("/")}/installed.json"`);expect(shell).toContain(`/${policy.launcherDirectory.join("/")}`);
  expect(windows).toContain(`"${policy.windowsDirectory.join("\\")}\\installed.json"`);expect(windows).toContain(`"${policy.launcherDirectory.join("\\")}"`);
});

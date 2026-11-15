#!/usr/bin/env python3
"""q7: bootstrap archive: the manifest names the archive by content, the manifest rename is the only commit. usage: q7-bootstrap.py <module-root>"""
import sys,re
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
def edit(path,fn):
    p=root+"/"+path
    s=open(p,encoding="utf8").read(); s=fn(s); open(p,"w",encoding="utf8").write(s)

def ts(s):
    s=sub(s,'export type BootstrapManifestV1=Readonly<{version:1;hlc:string;files:number;bytes:number;sha256:string}>;','export type BootstrapManifestV1=Readonly<{version:1;hlc:string;files:number;bytes:number;sha256:string;archive:string}>;')
    s=sub(s,'staleMilliseconds=6*3_600_000,retryMinutes=2;','staleMilliseconds=6*3_600_000,retryMinutes=2,keepArchiveMilliseconds=3_600_000,archivePattern=/^worktree(-[0-9a-f]{16})?\\.tar$/u;')
    s=sub(s,'''export function bootstrapPathsV1(storeRoot:string):Readonly<{directory:string;archive:string;manifest:string}>{''','''export function bootstrapPathsV1(storeRoot:string):Readonly<{directory:string;manifest:string}>{''')
    s=sub(s,'''return {directory,archive:join(directory,"worktree.tar"),manifest:join(directory,"manifest.json")};
}''','''return {directory,manifest:join(directory,"manifest.json")};
}

/** 🏷️ The file name of an archive: `worktree-` and the first 16 hex digits of its sha256, so that the name tells the content and a rebuilt archive never overwrites the one a reader is copying. */
export const bootstrapArchiveNameV1=(sha256:string):string=>`worktree-${sha256.slice(0,16)}.tar`;

/** 🔖️ Reads the manifest of the bootstrap archive and the path of the archive it names; `undefined` when there is none. */
export async function readBootstrapV1(storeRoot:string):Promise<Readonly<{manifest:BootstrapManifestV1;archive:string}>|undefined>{
 const paths=bootstrapPathsV1(storeRoot);
 try{
  const manifest=JSON.parse(await readFile(paths.manifest,"utf8")) as BootstrapManifestV1;
  return typeof manifest.archive==="string"&&archivePattern.test(manifest.archive)?{manifest,archive:join(paths.directory,manifest.archive)}:undefined;
 }catch{return undefined;}
}

async function pruneArchivesV1(directory:string,keep:ReadonlySet<string>):Promise<void>{
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  if(!archivePattern.test(name)||keep.has(name))continue;
  const file=join(directory,name),info=await stat(file).catch(()=>undefined);
  if(info!==undefined&&Date.now()-info.mtimeMs>keepArchiveMilliseconds)await rm(file,{force:true});
 }
}''')
    s=sub(s,'''/** 🥾️ Writes `<storeRoot>/v1/bootstrap/worktree.tar` and `manifest.json` from the folded store manifest and the local worktree: one pax tar entry per shared file (links stay to `join`), streamed in 1 MiB chunks, every entry verified against its manifest hash while it is written (a file that is missing, busy, changed or different is skipped and the entry is rewound), staged in `<storeRoot>/v1/tmp` and renamed, tar first and manifest second.''','''/** 🥾️ Writes `<storeRoot>/v1/bootstrap/worktree-<sha256 prefix>.tar` and `manifest.json` from the folded store manifest and the local worktree: one pax tar entry per shared file (links stay to `join`), streamed in 1 MiB chunks, every entry verified against its manifest hash while it is written (a file that is missing, busy, changed or different is skipped and the entry is rewound), staged in `<storeRoot>/v1/tmp`. The tar is renamed to its content name first, where no reader looks for it, and the manifest, which names it, is renamed second: that one rename is the only step a reader can observe, so a reader never sees a manifest and an archive that disagree. The archive of the previous manifest stays for an hour for readers that are copying it.''')
    s=sub(s,'''  const document:BootstrapManifestV1={version:1,hlc:folded.hlc,files,bytes,sha256};''','''  const previous=await readBootstrapV1(options.storeRoot),name=bootstrapArchiveNameV1(sha256),document:BootstrapManifestV1={version:1,hlc:folded.hlc,files,bytes,sha256,archive:name};''')
    s=sub(s,'''  await rename(archive,paths.archive);
  await rename(manifest,paths.manifest);
 }catch(error){await rm(archive,{force:true});await rm(manifest,{force:true});throw error;}''','''  await rename(archive,join(paths.directory,name));
  await rename(manifest,paths.manifest);
  await pruneArchivesV1(paths.directory,new Set([name,...previous===undefined?[]:[previous.manifest.archive]]));
 }catch(error){await rm(archive,{force:true});await rm(manifest,{force:true});throw error;}''')
    s=sub(s,'''return {hlc:folded.hlc,files,bytes,skipped,sha256,archive:paths.archive};''','''return {hlc:folded.hlc,files,bytes,skipped,sha256,archive:join(paths.directory,bootstrapArchiveNameV1(sha256))};''')
    return s
edit("🔁️replica/🥾️bootstrap/🟦️.ts",ts)

def tests(s):
    s=sub(s,'import {BootstrapRefusedV1,bootstrapPathsV1,settleBootstrapV1,','import {BootstrapRefusedV1,bootstrapArchiveNameV1,bootstrapPathsV1,readBootstrapV1,settleBootstrapV1,')
    s=sub(s,'const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");','const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");\nconst archiveOf=(store:string):string=>join(bootstrapPathsV1(store).directory,JSON.parse(readFileSync(bootstrapPathsV1(store).manifest,"utf8")).archive);')
    s=sub(s,'''const paths=bootstrapPathsV1(store),tar=readFileSync(paths.archive),found=entries(tar);
 expect(paths.archive).toBe(join(store,"v1","bootstrap","worktree.tar"));''','''const paths=bootstrapPathsV1(store),tar=readFileSync(archiveOf(store)),found=entries(tar);
 expect(archiveOf(store)).toBe(join(store,"v1","bootstrap",bootstrapArchiveNameV1(sha(tar))));
 expect(result.archive).toBe(archiveOf(store));''')
    s=sub(s,'const paths=bootstrapPathsV1(store),found=entries(readFileSync(paths.archive));','const paths=bootstrapPathsV1(store),found=entries(readFileSync(archiveOf(store)));')
    s=sub(s,'toBe(sha(readFileSync(paths.archive)));','toBe(sha(readFileSync(archiveOf(store))));')
    s=sub(s,'expect(existsSync(bootstrapPathsV1(store).archive)).toBe(false);','expect(existsSync(bootstrapPathsV1(store).manifest)).toBe(false);\n expect(existsSync(bootstrapPathsV1(store).directory)?readdirSync(bootstrapPathsV1(store).directory):[]).toEqual([]);')
    s=s.replace('bootstrapPathsV1(store).archive,"-C"','archiveOf(store),"-C"')
    s=s.rstrip("\n")+'''

test("a rebuild replaces the archive and the manifest in one visible step: the manifest names an archive that hashes to it at every moment, the previous archive stays for readers that are copying it, and the old fixed name is gone",async()=>{
 const {store,worktree,options}=await server("pair");
 await writeBootstrapArchiveV1(options);
 const first=await readBootstrapV1(store);
 expect(first?.manifest.archive).toBe(bootstrapArchiveNameV1(first!.manifest.sha256));
 expect(sha(readFileSync(first!.archive))).toBe(first!.manifest.sha256);
 writeFileSync(join(worktree,"a.txt"),"hello\\n");
 const watched:string[]=[];
 const watch=setInterval(()=>{try{const now=JSON.parse(readFileSync(bootstrapPathsV1(store).manifest,"utf8"));watched.push(sha(readFileSync(join(bootstrapPathsV1(store).directory,now.archive)))===now.sha256?"ok":"torn");}catch{watched.push("missing");}},1);
 await writeBootstrapArchiveV1(options);
 clearInterval(watch);
 const second=await readBootstrapV1(store);
 expect(watched.filter(item=>item!=="ok")).toEqual([]);
 expect(second!.manifest.archive).not.toBe(first!.manifest.archive);
 expect(existsSync(first!.archive)).toBe(true);
 expect(existsSync(join(bootstrapPathsV1(store).directory,"worktree.tar"))).toBe(false);
 expect(readdirSync(bootstrapPathsV1(store).directory).sort()).toEqual([first!.manifest.archive,"manifest.json",second!.manifest.archive].sort());
});
'''
    return s
edit("🔁️replica/🥾️bootstrap/🧪️tests/🟦️.ts",tests)

def py(s):
    s=sub(s,'if sorted(manifest) != ["bytes", "files", "hlc", "sha256", "version"]:','if sorted(manifest) != ["archive", "bytes", "files", "hlc", "sha256", "version"]:')
    s=sub(s,'    tar_path = os.path.join(base, "worktree.tar")\n','    if manifest["archive"] != "worktree-" + manifest["sha256"][:16] + ".tar":\n        problems.append("the archive name does not tell the content")\n    tar_path = os.path.join(base, manifest["archive"])\n')
    s=s.replace("reads `<store>/v1/bootstrap/worktree.tar`","reads the tar that `<store>/v1/bootstrap/manifest.json` names")
    return s
edit("🔁️replica/🥾️bootstrap/🧪️tests/🐍️.py",py)

def sh(s):
    s=sub(s,'''    { [ -f "$source/manifest.json" ] && [ -f "$source/worktree.tar" ]; } || fail "no bootstrap archive in $source: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass --archive <directory>"
    expected=$(sed -n 's/.*"sha256"[[:space:]]*:[[:space:]]*"\\([0-9a-f]\\{64\\}\\)".*/\\1/p' "$source/manifest.json")''','''    [ -f "$source/manifest.json" ] || fail "no bootstrap archive in $source: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass --archive <directory>"
    name=$(sed -n 's/.*"archive"[[:space:]]*:[[:space:]]*"\\(worktree-[0-9a-f]\\{16\\}\\.tar\\)".*/\\1/p' "$source/manifest.json")
    { [ -n "$name" ] && [ -f "$source/$name" ]; } || fail "the bootstrap manifest in $source names no archive that exists: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass --archive <directory>"
    expected=$(sed -n 's/.*"sha256"[[:space:]]*:[[:space:]]*"\\([0-9a-f]\\{64\\}\\)".*/\\1/p' "$source/manifest.json")''')
    s=sub(s,'  local source=$1 target=$2 attempt expected actual\n','  local source=$1 target=$2 attempt expected actual name\n')
    s=sub(s,'    cp "$source/worktree.tar" "$target"','    cp "$source/$name" "$target"')
    s=s.replace("#   --archive  a directory with worktree.tar and manifest.json","#   --archive  a directory with manifest.json and the archive it names (worktree-<16 hex>.tar)")
    return s
edit("🗺️topology/🥾️provision/🐚️.sh",sh)

def ps(s):
    s=sub(s,'''            if (-not ((Test-Path -LiteralPath "$source\\manifest.json") -and (Test-Path -LiteralPath "$source\\worktree.tar"))) { throw "No bootstrap archive in ${source}: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            $manifest = Get-Content -Raw -LiteralPath "$source\\manifest.json" | ConvertFrom-Json
            Copy-Item -LiteralPath "$source\\worktree.tar" -Destination $local -Force''','''            if (-not (Test-Path -LiteralPath "$source\\manifest.json")) { throw "No bootstrap archive in ${source}: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            $manifest = Get-Content -Raw -LiteralPath "$source\\manifest.json" | ConvertFrom-Json
            if (($manifest.archive -notmatch '^worktree-[0-9a-f]{16}\\.tar$') -or -not (Test-Path -LiteralPath "$source\\$($manifest.archive)")) { throw "The bootstrap manifest in ${source} names no archive that exists: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            Copy-Item -LiteralPath "$source\\$($manifest.archive)" -Destination $local -Force''')
    s=s.replace("# -ArchivePath is a directory with worktree.tar and manifest.json","# -ArchivePath is a directory with manifest.json and the archive it names (worktree-<16 hex>.tar)")
    return s
edit("🗺️topology/🥾️provision/🔵️.ps1",ps)

def fixture(s):
    s=sub(s,'''  const tar=join(root,directory,"v1/bootstrap/worktree.tar"),made=Bun.spawnSync(["tar","-cf",tar,"-C",repository,...checkoutFiles]),listing=Bun.spawnSync(["tar","-tf",tar]).stdout.toString().trim().split("\\n").length;
  expect(made.exitCode).toBe(0);
  writeFileSync(join(root,directory,"v1/bootstrap/manifest.json"),JSON.stringify({files:listing+(setup.corrupt==="count"?1:0),sha256:setup.corrupt==="sha"?"0".repeat(64):hex(new Bun.CryptoHasher("sha256").update(readFileSync(tar)).digest().buffer as ArrayBuffer)}));''','''  const staged=join(root,directory,"v1/bootstrap/staged.tar"),made=Bun.spawnSync(["tar","-cf",staged,"-C",repository,...checkoutFiles]),listing=Bun.spawnSync(["tar","-tf",staged]).stdout.toString().trim().split("\\n").length;
  expect(made.exitCode).toBe(0);
  const digest=hex(new Bun.CryptoHasher("sha256").update(readFileSync(staged)).digest().buffer as ArrayBuffer),archive=`worktree-${digest.slice(0,16)}.tar`;
  renameSync(staged,join(root,directory,"v1/bootstrap",archive));
  writeFileSync(join(root,directory,"v1/bootstrap/manifest.json"),JSON.stringify({files:listing+(setup.corrupt==="count"?1:0),sha256:setup.corrupt==="sha"?"0".repeat(64):digest,archive}));''')
    return s
edit("🗺️topology/🧪️tests/🔵️.ts",fixture)

def readme(s):
    s=sub(s,'**The archive.** `<storeRoot>/v1/bootstrap/worktree.tar` is a POSIX pax tar','**The archive.** `<storeRoot>/v1/bootstrap/worktree-<16 hex>.tar` (the first 16 hex digits of its own sha256) is a POSIX pax tar')
    s=sub(s,'written, vanished) is skipped. `manifest.json` next to it is `{"version":1,"hlc":…,"files":n,"bytes":n,"sha256":…}`:','written, vanished) is skipped. `manifest.json` next to it is `{"version":1,"hlc":…,"files":n,"bytes":n,"sha256":…,"archive":"worktree-<16 hex>.tar"}`:')
    s=sub(s,'`<storeRoot>/v1/tmp` and renamed, then the manifest the same way, so a reader that sees a mismatch retries.','`<storeRoot>/v1/tmp` and renamed to its content name, where no reader looks for it, and then the manifest that names it is renamed: that one rename is the only step a reader can observe, so a manifest and the archive it names never disagree (the archive of the previous manifest stays for an hour for readers that are copying it, then the writer removes it). A reader still verifies the sha256 and retries, which only matters for a damaged copy.')
    s=sub(s,'directory. Copy `worktree.tar` and `manifest.json` to local disk','directory. Copy `manifest.json` and the archive it names to local disk')
    return s
edit("README.md",readme)
print("ok")

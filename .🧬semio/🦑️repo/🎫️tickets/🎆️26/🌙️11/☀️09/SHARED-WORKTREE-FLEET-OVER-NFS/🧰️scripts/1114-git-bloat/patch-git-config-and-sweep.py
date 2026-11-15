import sys
root=sys.argv[1]
def edit(path,pairs,append=""):
    s=open(root+"/"+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:50])
        s=s.replace(a,b)
    open(root+"/"+path,"w").write(s+append)
edit("🌿️git/🟦️.ts",[
 ('["gc.auto","0"],["maintenance.auto","false"],["core.logallrefupdates","true"]];','["gc.auto","0"],["gc.autoPackLimit","0"],["gc.autoDetach","false"],["gc.writeCommitGraph","false"],["fetch.writeCommitGraph","false"],["receive.autogc","false"],["maintenance.auto","false"],["core.logallrefupdates","true"]];'),
 ('`gc.auto=0`, `maintenance.auto=false`, `core.logallrefupdates=true`, and removes a `commit.template`','`gc.auto=0`, `gc.autoPackLimit=0`, `gc.autoDetach=false`, `gc.writeCommitGraph=false`, `fetch.writeCommitGraph=false`, `receive.autogc=false`, `maintenance.auto=false` (no git client repacks or rewrites commit graphs on its own under the replica), `core.logallrefupdates=true`, and removes a `commit.template`'),
 ('import {copyFile,lstat,mkdir,open,readdir,readFile,rename,rm,stat,symlink,writeFile} from "node:fs/promises";','import {copyFile,lstat,mkdir,open,readdir,readFile,rename,rm,stat,symlink,unlink,writeFile} from "node:fs/promises";'),
 ('/** 🧭 Tells whether a replica path is a branch ref','''/** 🧹 Removes the dead temporary files of git writers from `.git/objects/pack` (`tmp_*` and `.tmp-*`) whose modification and change time is older than `olderThanMilliseconds` (default one hour: a live `pack-objects` or `index-pack` writes continuously, so a file that was silent for an hour belongs to a process that died, like the 3.2 GB `tmp_pack_*` of the non-atomic repack of 2026-11-14); returns the files and bytes removed. The names are local only (`gitSharedV1`), nothing is published for them. Never throws. */
export async function sweepGitGarbageV1(worktree:string,options:Readonly<{olderThanMilliseconds?:number;now?:number}>={}):Promise<Readonly<{files:number;bytes:number}>>{
 const directory=join(worktree,".git","objects","pack"),limit=options.olderThanMilliseconds??3_600_000,now=options.now??Date.now();
 let files=0,bytes=0;
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  if(!name.startsWith("tmp_")&&!name.startsWith(".tmp-"))continue;
  const file=join(directory,name),info=await lstat(file).catch(()=>undefined);
  if(info===undefined||!info.isFile()||now-Math.max(info.mtimeMs,info.ctimeMs)<=limit)continue;
  await unlink(file).then(()=>{files++;bytes+=info.size;},()=>{});
 }
 return {files,bytes};
}

/** 🧭 Tells whether a replica path is a branch ref'''),
])
edit("🌿️git/🧪️tests/🟦️.ts",[
 ('expect(changed).toEqual(expect.arrayContaining(["core.autocrlf","core.filemode","core.fsmonitor","core.untrackedCache","index.version","gc.auto","maintenance.auto","commit.template"]));','expect(changed).toEqual(expect.arrayContaining(["core.autocrlf","core.filemode","core.fsmonitor","core.untrackedCache","index.version","gc.auto","gc.autoPackLimit","gc.autoDetach","gc.writeCommitGraph","fetch.writeCommitGraph","receive.autogc","maintenance.auto","commit.template"]));'),
 (' expect(value("maintenance.auto")).toBe("false");',' expect(value("gc.autopacklimit")).toBe("0");expect(value("gc.autodetach")).toBe("false");expect(value("gc.writecommitgraph")).toBe("false");expect(value("fetch.writecommitgraph")).toBe("false");expect(value("receive.autogc")).toBe("false");\n expect(value("maintenance.auto")).toBe("false");'),
])

root=sys.argv[1]
t=root+"/🌿️git/🧪️tests/🟦️.ts"
s=open(t).read()
assert s.count("normaliseGitConfigV1,")==1
s=s.replace("normaliseGitConfigV1,","normaliseGitConfigV1,sweepGitGarbageV1,")
s+=open(__file__.rsplit("/",1)[0]+"/sweep-test.txt").read()
open(t,"w").write(s)

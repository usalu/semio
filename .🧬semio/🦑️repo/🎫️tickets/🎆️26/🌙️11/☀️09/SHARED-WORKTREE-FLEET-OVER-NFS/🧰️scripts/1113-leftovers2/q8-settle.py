#!/usr/bin/env python3
"""q8: bootstrap settle never stages wrong bytes. usage: q8-settle.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
def edit(path,fn):
    p=root+"/"+path
    s=open(p,encoding="utf8").read(); s=fn(s); open(p,"w",encoding="utf8").write(s)
def tests(s):
    anchor='test.skipIf(!system||process.platform==="win32")("settle also moves aside a file whose executable bit'
    new='''test("settle refuses a blob that does not hash to its name instead of staging wrong bytes, and leaves the file where it is",async()=>{
 const {store,worktree,options}=await server("settle-corrupt");
 await writeBootstrapArchiveV1(options);
 const target=temp("settle-corrupt-target");
 writeFileSync(join(target,"a.txt"),"hello\\n");
 writeFileSync(join(worktree,"a.txt"),"newer\\n");
 await seedV1(options);
 writeFileSync(blobPathV1(store,sha("newer\\n")),"rotten");
 await expect(settleBootstrapV1({worktree:target,storeRoot:store,now:()=>1_700_000_000_003})).rejects.toMatchObject({key:"blob",variables:{path:"a.txt"}});
 expect(readFileSync(join(target,"a.txt"),"utf8")).toBe("hello\\n");
 expect(existsSync(join(target,".cache","f","lost","bootstrap-1700000000003","a.txt"))).toBe(false);
 expect(readdirSync(join(target,".cache","f","tmp")).filter(name=>name.startsWith("settle-"))).toEqual([]);
});

'''
    return sub(s,anchor,new+anchor)
edit("🔁️replica/🥾️bootstrap/🧪️tests/🟦️.ts",tests)
def ts(s):
    s=sub(s,'readonly key:"identity"|"empty"|"joined"|"store";','readonly key:"identity"|"empty"|"joined"|"store"|"blob";')
    s=sub(s,'constructor(key:"identity"|"empty"|"joined"|"store",','constructor(key:"identity"|"empty"|"joined"|"store"|"blob",')
    s=sub(s,'''     await copyFile(blobPathV1(options.storeRoot,wanted!.sha256!),staged);''','''     if(await copyVerifiedV1(blobPathV1(options.storeRoot,wanted!.sha256!),staged,wanted!.sha256!)==="corrupt")throw new BootstrapRefusedV1("blob",{path:entry.path});''')
    s=sub(s,'import {chmod,copyFile,lstat,','import {chmod,lstat,')
    s=sub(s,'import {blobPathV1,requireOriginIdV1,','import {blobPathV1,copyVerifiedV1,requireOriginIdV1,')
    return s
edit("🔁️replica/🥾️bootstrap/🟦️.ts",ts)
def texts(s):
    return sub(s,'''    "error.bootstrap.store": {''','''    "error.bootstrap.blob": {
      "en": "The store blob of {path} does not hash to its name; settle stopped and left the file where it is. Another machine puts the blob back (see the wants of the fleet); run bootstrap-settle again later.",
      "de": "Der Store-Blob von {path} stimmt nicht mit seinem Namen überein; settle hat angehalten und die Datei an ihrem Platz gelassen. Eine andere Maschine stellt den Blob wieder her (siehe die Wünsche des Fleets); führe bootstrap-settle später erneut aus."
    },
    "error.bootstrap.store": {''')
edit("🔁️replica/📖️texts/🔣️.json",texts)
print("ok")

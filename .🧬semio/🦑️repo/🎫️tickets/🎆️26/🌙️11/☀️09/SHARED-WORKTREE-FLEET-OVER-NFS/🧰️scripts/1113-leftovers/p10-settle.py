import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🔁️replica/🥾️bootstrap/🟦️.ts",[
 ('''   const target=absolute(folder,entry.path);
   await mkdir(dirname(target),{recursive:true});
   await rename(absolute(options.worktree,entry.path),target);
   const wanted=folded.manifest.get(entry.path);
   if(wanted?.operation==="put"){
    const restored=absolute(options.worktree,entry.path);
    await copyFile(blobPathV1(options.storeRoot,wanted.sha256!),restored);
    if(!windows)await chmod(restored,wanted.executable?0o755:0o644);
   }''','''   const target=absolute(folder,entry.path),wanted=folded.manifest.get(entry.path),restored=absolute(options.worktree,entry.path),staged=wanted?.operation==="put"?`${restored}.${token()}.settle`:undefined;
   if(staged!==undefined){
    try{
     await copyFile(blobPathV1(options.storeRoot,wanted!.sha256!),staged);
     if(!windows)await chmod(staged,wanted!.executable?0o755:0o644);
    }catch(error){await rm(staged,{force:true});throw error;}
   }
   await mkdir(dirname(target),{recursive:true});
   await rename(restored,target);
   if(staged!==undefined)await rename(staged,restored);'''),
])

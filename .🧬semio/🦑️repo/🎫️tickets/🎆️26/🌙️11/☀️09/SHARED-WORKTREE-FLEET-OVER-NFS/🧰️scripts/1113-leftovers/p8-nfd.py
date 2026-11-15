import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🔁️replica/🟦️.ts",[
 ('''const listed=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).some(name=>name.normalize("NFC")===basename(path));''','''const listed=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).some(name=>name.normalize("NFC")===basename(path));
const spelled=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).includes(basename(path));
const asciiOnly=/^[\\u0000-\\u007f]*$/u;'''),
 ('''  const other=replica.folds.find(path);
  if(other!==undefined&&other!==path){
   if(await listed(root,other))continue;
   missing.push(other);
  }''','''  if(!asciiOnly.test(path)&&!await spelled(root,path)){noteOnce(replica,`refused:${path}:not-nfc`,{event:"refused",path,reason:"not-nfc"});continue;}
  const other=replica.folds.find(path);
  if(other!==undefined&&other!==path){
   if(await listed(root,other))continue;
   missing.push(other);
  }'''),
])
edit(w+"/🔁️replica/🧪️tests/🟦️.ts",[
 ('test("a path longer than the limit is refused with a visible event when scanned and when touched,','''test("a name that is stored decomposed on disk is refused by the event path like the scan refuses it, instead of being published under its composed spelling",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 const composed="caf\\u00e9.txt",decomposed="cafe\\u0301.txt";
 write(a,"plain.txt","plain");
 writeFileSync(join(a.options.worktree,decomposed),"decomposed");
 await publishOnceV1(a.replica!,undefined,[composed]);
 await publish(a);await follow(b);
 expect(read(b,"plain.txt")).toBe("plain");
 expect(a.replica!.manifest.get(composed)).toBeUndefined();
 expect(readdirSync(b.options.worktree).filter(name=>name.normalize("NFC")===composed)).toEqual([]);
 if(readdirSync(a.options.worktree).includes(decomposed))expect(a.events.filter(event=>event.event==="refused"&&event.reason==="not-nfc").length).toBeGreaterThan(0);
 await shut(a);await shut(b);
});

test("a path longer than the limit is refused with a visible event when scanned and when touched,'''),
])

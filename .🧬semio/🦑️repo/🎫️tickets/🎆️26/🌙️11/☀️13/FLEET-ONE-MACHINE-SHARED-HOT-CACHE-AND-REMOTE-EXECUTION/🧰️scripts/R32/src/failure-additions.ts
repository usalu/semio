
test("an EPERM that two reads of the relative call agree on, for a place that reading by path reaches, is no missing macOS consent: the service keeps serving and says what happened",()=>{
  const reported=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio/.cache/b`,directory:`${home}/Documents/semio/.cache/b/g`,entry:".reset.json",reads:2}),lines:string[]=[];
  const run=(error:unknown,inspect:(path:string)=>unknown)=>failureRefusalV1(error,{log:(event,detail)=>{lines.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",root:`${home}/Documents/semio`,environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:4242,ppid:77,inspect});
  const absent=run(reported,()=>{throw Object.assign(new Error("gone"),{code:"ENOENT"});});
  expect(absent).toMatchObject({code:"unserved",key:"host-relative-call-refused",retry:"later",fields:{place:`${home}/Documents/semio/.cache/b/g/.reset.json`,reads:"2",found:"ENOENT",log:"/log"}});
  expect(lines.at(-1)).toContain(`where: root=${home}/Documents/semio/.cache/b directory=${home}/Documents/semio/.cache/b/g entry=.reset.json code=EPERM reads=2 route=direct pid=4242 ppid=77 pathcheck=ENOENT`);
  expect(lines.at(-1)).toContain(absent.fields.reference);
  expect(run(reported,()=>({})).fields).toMatchObject({found:"present"});
  expect(run(reported,()=>{throw Object.assign(new Error("no"),{code:"EACCES"});}).fields).toMatchObject({found:"EACCES"});
});

test("an EPERM that reading by path confirms stays the missing consent and names the directory of the refused entry; an error without the reads of the relative call is judged as before",()=>{
  const reported=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio/.cache/b`,directory:`${home}/Documents/semio/.cache/b/g`,entry:".reset.json",reads:2}),checked:string[]=[];
  const options={log:()=>{},logPath:"/log",home,platform:"darwin",root:`${home}/Documents/semio`};
  const denied=failureRefusalV1(reported,{...options,inspect:path=>{checked.push(path);throw Object.assign(new Error("no"),{code:"EPERM"});}});
  expect(denied).toMatchObject({code:"consent",key:"access-denied",fields:{place:`${home}/Documents/semio/.cache/b/g/.reset.json`}});
  expect(checked).toEqual([`${home}/Documents/semio/.cache/b/g/.reset.json`]);
  checked.length=0;
  const plain=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio`,entry:".cache"});
  expect(failureRefusalV1(plain,{...options,inspect:path=>{checked.push(path);return{};}})).toMatchObject({code:"consent",key:"access-denied"});
  expect(checked).toEqual([]);
  expect(deniedPlaceOfV1(reported)).toBe(`root=${home}/Documents/semio/.cache/b directory=${home}/Documents/semio/.cache/b/g entry=.reset.json code=EPERM reads=2`);
});

test("the default check reads the place by path with the access of the service: absent and present entries of a real scratch directory",()=>{
  const fakeHome=mkdtempSync(join(tmpdir(),"relative-refused-")),root=join(fakeHome,"Documents","semio"),directory=join(root,".cache","b","g");
  try{
    mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"image.json"),"{}");
    const run=(entry:string)=>failureRefusalV1(Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:join(root,".cache","b"),directory,entry,reads:2}),{log:()=>{},logPath:"/log",home:fakeHome,platform:"darwin",root});
    expect(run(".reset.json")).toMatchObject({code:"unserved",key:"host-relative-call-refused",fields:{found:"ENOENT",place:join(directory,".reset.json")}});
    expect(run("image.json").fields.found).toBe("present");
  }finally{rmSync(fakeHome,{recursive:true,force:true});}
});

test("the refusal text exists in English and German with the same placeholders and says it is not the macOS consent",()=>{
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["host-relative-call-refused"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all),holes=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(table.de["host-relative-call-refused"])).toEqual(holes(table.en["host-relative-call-refused"]));
  for(const name of ["place","reads","found","reference","log"])expect(table.en["host-relative-call-refused"]).toContain("{"+name+"}");
  expect(table.en["host-relative-call-refused"]).toContain("not the proven missing macOS file access consent");
  expect(table.de["host-relative-call-refused"]).toContain("nicht die belegte fehlende macOS-Dateizugriffserlaubnis");
});

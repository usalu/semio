import sys
root=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path[-30:],a[:60],s.count(a))
        s=s.replace(a,b)
    open(path,"w",encoding="utf8").write(s)
edit(root+"/🗺️paths/🟦️.ts",[
('''  if(argument===pass.source){rewritten.push(inside?''','''  if(argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")){
   const joinedRemap=argument.includes("="),spec=joinedRemap?argument.slice(argument.indexOf("=")+1):args[++index]??"",from=spec.slice(0,Math.max(0,spec.lastIndexOf("=")));
   if(isAbsolute(from)&&!SYSTEM.some(prefix=>from.startsWith(prefix)))continue;
   rewritten.push(...joinedRemap?[argument]:[argument,spec]);continue;
  }
  if(argument===pass.source){rewritten.push(inside?'''),
])
edit(root+"/🧬️schema/🔣️.json",[('''        "TMPDIR",
''','''        "TMPDIR",
        "TMP",
        "TEMP",
''')])
edit(root+"/🧪️tests/🟦️.ts",[('''test("an unreachable cache endpoint is a quick, silent decline''','''test("the requester's own temporary directories and path remaps are left behind, not carried to the other machine",()=>{
 const checkout=chain("remap"),remap=`--remap-path-prefix=${checkout.root}/.cache/cargo=/cargo`,foreign=`--remap-path-prefix=/usr/lib=/lib`;
 const result=planOffloadV1({args:[...checkout.arguments("c",["b"]),remap,"--remap-path-prefix",`${checkout.root}/t=/t`,foreign],environment:{...checkout.environment("c"),TEMP:`${checkout.root}/.tmp`,TMP:`${checkout.root}/.tmp`},cwd:checkout.root});
 if("declined" in result)throw Error(JSON.stringify(result));
 expect(result.arguments.some(argument=>argument.includes(checkout.root))).toBe(false);
 expect(result.arguments).toContain(foreign);expect(result.environment.TEMP).toBeUndefined();expect(result.environment.TMP).toBeUndefined();
});

test("an unreachable cache endpoint is a quick, silent decline''')])

import sys
P=sys.argv[1]
s=open(P,encoding="utf8").read()
def r(a,b):
    global s
    assert s.count(a)==1,(a[:80],s.count(a))
    s=s.replace(a,b)
r('''const within=(parent:string,path:string):boolean=>path===parent||path.startsWith(parent.endsWith(sep)?parent:parent+sep);''','''const within=(parent:string,path:string):boolean=>path===parent||path.startsWith(parent.endsWith(sep)?parent:parent+sep);
const commonAncestor=(left:string,right:string):string=>{let at=left;while(!within(at,right)){const up=dirname(at);if(up===at)return at;at=up;}return at;};''')
r(''' if(!within(manifest,sourceReal))return decline("unmapped-path","source outside CARGO_MANIFEST_DIR");
 const inside=within(cwd,manifest),packageVirtual=inside?relative(cwd,manifest).split(sep).join("/"):virtual.package,files=new Map<string,string>(),mappings:OffloadMappingV1[]=[];
 const ignored=["target",".git","node_modules",".cache",virtual.base];
 const walked=walkFilesV1(manifest,ignored);
 if(walked===undefined)return decline("inputs-too-large","package "+manifest);
 const place=(path:string,real:string):void=>{files.set(path,real);};
 for(const [path,real] of walked)place(packageVirtual===""?path:packageVirtual+"/"+path,real);''',''' const inside=within(cwd,manifest),shipped=within(manifest,sourceReal)?manifest:commonAncestor(manifest,sourceReal);
 if(shipped!==manifest&&!(inside&&within(cwd,shipped)))return decline("unmapped-path","source outside CARGO_MANIFEST_DIR");
 const packageVirtual=inside?relative(cwd,manifest).split(sep).join("/"):virtual.package,shippedVirtual=inside?relative(cwd,shipped).split(sep).join("/"):virtual.package,files=new Map<string,string>(),mappings:OffloadMappingV1[]=[];
 const ignored=["target",".git","node_modules",".cache",virtual.base];
 const walked=walkFilesV1(shipped,ignored);
 if(walked===undefined)return decline("inputs-too-large","package "+shipped);
 const place=(path:string,real:string):void=>{files.set(path,real);};
 for(const [path,real] of walked)place(shippedVirtual===""?path:shippedVirtual+"/"+path,real);''')
r('''if(argument===pass.source){rewritten.push(inside?argument:packageVirtual+"/"+relative(manifest,sourceReal).split(sep).join("/"));continue;}''','''if(argument===pass.source){rewritten.push(inside?relative(cwd,sourceReal).split(sep).join("/"):packageVirtual+"/"+relative(manifest,sourceReal).split(sep).join("/"));continue;}''')
r('''    if(within(manifest,real))continue;
    if(!within(cwd,real))''','''    if(within(shipped,real))continue;
    if(!within(cwd,real))''')
open(P,"w",encoding="utf8").write(s)

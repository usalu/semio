import os
P="🧰️framework/🔨️modules/🏃️process/"
B=P+"🏗️build/"
H=B+"🌡️host-capacity/"
N=P+"📦️artifacts/🏗️native-build/💾️storage/"
A=P+"🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/"
def edit(path,old,new,count=1):
    s=open(path,encoding="utf8").read()
    if s.count(new)>0 and s.count(old)==0: print("already",path[-40:]);return
    assert s.count(old)==count,(path,old,s.count(old))
    t=path+".h54tmp"
    open(t,"w",encoding="utf8").write(s.replace(old,new));os.replace(t,path)
f=P+"🎛️owned-execution/📥️capture/🚪️entry/🧪️tests/🟦️.ts"
for o,n in [('if("entryUnits"in row){','if(row.entryUnits!==undefined){'),('if("specifierUnits"in row)','if(row.specifierUnits!==undefined)'),('"physicalSourcePath"in row?','row.physicalSourcePath!==undefined?'),('if("originUnits"in row)','if(row.originUnits!==undefined)')]: edit(f,o,n)
edit(H+"🛡️warden/🧪️tests/🟦️.ts",'expect(requireHostWardenV1("State",state).subjects.byKind["local-unit"]','expect(requireHostWardenV1<{subjects:{byKind:Record<string,{count:number}>}}>("State",state).subjects.byKind["local-unit"]')
edit(H+"🧪️tests/🎟️retained.test.ts",'expect({unrealized:result.unrealizedMegabytes','expect<unknown>({unrealized:result.unrealizedMegabytes')
edit(H+"🧪️tests/📡️sampling/🟦️.ts",'reason:"unsupported"','reason:"platform"',4)
edit(H+"🧪️tests/📣️need/🟦️.ts",'{peakMegabytes:400,status:"failed"}','{peakMegabytes:400,status:"failed" as const}')
f=H+"🧪️tests/🦀️cargo-grant/🟦️.ts"
edit(f,'for(const environment of[{},{CARGO_BUILD_JOBS:"5"},{CARGO_BUILD_JOBS:"zero"}])','for(const environment of environments)')
edit(f,'const grid:{arguments:string[]','const environments:Record<string,string>[]=[{},{CARGO_BUILD_JOBS:"5"},{CARGO_BUILD_JOBS:"zero"}];\n const grid:{arguments:string[]')
f=B+"🗣️language/📣️progress/🧪️tests/🏷️typed-refusals/🟦️.ts"
edit(f,'import source from "../../🟦️.ts" with {type:"text"};','import {readFileSync} from "node:fs";')
edit(f,'import schema from','const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");\nimport schema from')
f=B+"🧰️toolchains/🧪️tests/📦️cargo-tools/🟦️.ts"
edit(f,'expect(entry.setup).toEqual','expect<unknown>(entry.setup).toEqual');edit(f,'expect(entry.locate).toEqual([','expect<unknown>(entry.locate).toEqual([')
for d in ["🏚️stranded","🛟️reserve"]: edit(N+"🍎️image/"+d+"/🧪️tests/🟦️.ts",'python===undefined','python===null')
f=N+"🍎️image/👟️kick/🧪️tests/🟦️.ts"
edit(f,'python===undefined','python===null');edit(f,'(script.outputs as Record<string,string[]>)[key]??[""]','Object.entries(script.outputs).find(([name])=>name===key)?.[1]??[""]')
f=N+"📜️journal/🧷️append/🧪️tests/🟦️.ts"
edit(f,'.catch((value:unknown)=>value as Error&{kind:string;texts:{en:string;de:string}});','.catch((value:unknown)=>value);')
edit(f,'expect(error.kind).toBe("changed");expect(error.texts.de).toContain("Versuche");','expect(error).toBeInstanceOf(Error);expect(Reflect.get(error as object,"kind")).toBe("changed");expect(Reflect.get(Reflect.get(error as object,"texts"),"de")).toContain("Versuche");')
edit(N+"📤️publication/🌐️platform/🧮️read-plan/🧪️tests/🔒️journal/🟦️.ts",'{...options,owner:"fixture-journal-writer",timeoutMs:0}','{...options,timeoutMs:0}')
edit(N+"📤️publication/📸️observation/🧪️tests/🟦️.ts",'"reconcile"in row?row.reconcile:"settles"','row.reconcile??"settles"')
edit(N+"🧹️whole/🧪️tests/🟦️.ts",'row.binding?.toPrefix??null','row.binding!==undefined&&"toPrefix"in row.binding?row.binding.toPrefix:null')
edit(P+"📦️artifacts/🏗️native-build/📥️source/📎️retained/🧪️tests/🟦️.ts",'environment:{},budgetMs:60000,label','environment:{},label')
f=P+"🔒️leases/🧪️tests/🔁️replaced-file/🟦️.ts"
edit(f,'import {Database} from "bun:sqlite";','import {SqliteDatabaseV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🗃️sqlite/🟦️.ts";')
edit(f,'new Database(','new SqliteDatabaseV1(',3)
edit(P+"🧪️testing/🧰️toolchain/🧪️tests/🟦️.ts",'if(row.accepted)expect(api.requirePinnedTestToolV1("python",selected)).toBe(','if(row.accepted)expect<unknown>(api.requirePinnedTestToolV1("python",selected)).toBe(')
edit(P+"🧰️tools/🧪️tests/🏃️run/🟦️.ts",'expect(ask.kind).toBe(row.kind)','expect<unknown>(ask.kind).toBe(row.kind)')
f=A+"dag.ts"
edit(f,'const measured=[];','const measured:{depth:number;uniqueNodes:number;clockReads:number;maximum:number}[]=[];')
edit(f,'graphlib.alg.preorder(graph,String(depth))','graphlib.alg.preorder(graph,[String(depth)])')
edit(A+"extra.ts",'graphlib.alg.preorder(graph,"child")','graphlib.alg.preorder(graph,["child"])')
f=A+"wait-model.ts"
edit(f,'if("parent"in row)graph.setEdge','if(row.parent!==undefined)graph.setEdge');edit(f,'if("follows"in row)graph.setEdge','if(row.follows!==undefined)graph.setEdge');edit(f,'graphlib.alg.preorder(graph,fixture.target)','graphlib.alg.preorder(graph,[fixture.target])')
edit(A+"freshness.ts",'expect(result).toBeDefined();measured.push(result);','if(result===undefined)throw new Error("no measurement");measured.push(result);')

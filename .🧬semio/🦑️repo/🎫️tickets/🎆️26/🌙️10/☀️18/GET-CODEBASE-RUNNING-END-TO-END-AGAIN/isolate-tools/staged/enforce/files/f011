import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve,join}from"node:path";
import corpus from"@semio-tech/process/state/testing.json";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();

test("neutral persistent development state is excluded from recomputable destinations and source capture",async()=>{const{classifyNxOutputV1}=await import("../../🟦️.ts"),{requireRecomputableStatePathV1}=await import("@semio-tech/process/state/layout"),root=resolve(repository.root,".cache/b/p/m"),db=new Database(":memory:");try{db.exec("create table paths(path text primary key,allowed integer)");for(const row of corpus.paths){db.query("insert into paths values(?,?)").run(row.path,Number(row.allowed));const path=join(root,row.path),allowed=db.query<{allowed:number},[string]>("select allowed from paths where path=?").get(row.path)?.allowed;if(allowed){expect(requireRecomputableStatePathV1(root,path)).toBe(path);expect(classifyNxOutputV1({rust:resolve(repository.root,".cache/b/r/m"),products:root},path).partition).toBe("products");}else{expect(()=>requireRecomputableStatePathV1(root,path)).toThrow("Persistent state");expect(()=>classifyNxOutputV1({rust:resolve(repository.root,".cache/b/r/m"),products:root},path)).toThrow("Persistent state");}}const source=readFileSync(resolve(import.meta.dir,"../../../../../../📦️artifacts/🟦️.ts"),"utf8");expect(source.includes("requireRecomputableStatePathV1(repositoryCachePartitionRootDataV1(scope.options.workspace,\"products\"),source)")).toBe(true);}finally{db.close();}});

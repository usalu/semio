import os
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🧲️closure-leaves/🟦️.ts"
s=open(P,encoding="utf8").read()
anchor='test("an upload sends each named leaf once and a restore fetches'
assert s.count(anchor)==1
new='''test("a closure port with a shared reader stats every leaf under the lease and answers the same size as the plain stat",async()=>{
 const workspace=scratch(),plain=workspaceClosurePortV1(workspace)!,runs:string[]=[],reader={run:async<T>(body:()=>T|Promise<T>):Promise<T>=>{runs.push("run");return body();},close:async()=>undefined},leased=workspaceClosurePortV1(workspace,reader)!;
 const bytes=Buffer.alloc(33,7),key=sha(bytes),directory=plain.directory(NAMESPACE);
 expect(plain.sizeLeased).toBeUndefined();
 await expect(leased.sizeLeased!(NAMESPACE,key)).resolves.toBeUndefined();
 mkdirSync(directory,{recursive:true});writeFileSync(join(directory,key+".json"),bytes);
 await expect(leased.sizeLeased!(NAMESPACE,key)).resolves.toBe(33);
 expect(leased.size(NAMESPACE,key)).toBe(33);expect(plain.size(NAMESPACE,key)).toBe(33);expect(runs).toEqual(["run","run"]);
});

'''
s=s.replace(anchor,new+anchor)
open(P+".f121tmp","w",encoding="utf8").write(s);os.replace(P+".f121tmp",P)

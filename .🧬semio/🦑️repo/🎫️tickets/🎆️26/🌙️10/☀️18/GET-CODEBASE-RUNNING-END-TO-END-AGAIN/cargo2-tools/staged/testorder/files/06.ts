import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import ts from"typescript";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const root=fileURLToPath(new URL("../../../../../../../../../../../../",import.meta.url));
const scopes=["withCompilerCapacityConsumptionV1","withCompilerCapacityScopeV1"],acquire="acquireCompilerCapacityLeaseV1",writers=["withCompilerWorkspaceSharedWriteV1","withCompilerWorkspaceWriteV1","withCompilerWriterCustodyV1","withCompilerSharedHeadroomV1","withCompilerInitialHeadroomV1","withCompilerWorkspaceBorrowV1","acquireCargoBuildLeaseV1","acquireCargoTargetRecipeV1"];
const definitions=new Set(["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🟦️.ts"]);
const files=():string[]=>spawnSync("git",["ls-files","-z","--cached","--others","--exclude-standard","--","🧰️framework/*.ts"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(path=>path&&existsSync(join(root,path))&&!/\/🧪️tests\//u.test(path)&&!definitions.has(path)&&/withCompiler(Capacity|Workspace|Writer|SharedHeadroom|InitialHeadroom)|acquireCompilerCapacityLeaseV1|acquireCargoTargetRecipeV1/u.test(readFileSync(join(root,path),"utf8")));

function violations(path:string):string[]{
 const text=readFileSync(join(root,path),"utf8"),source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true),out:string[]=[],calls:{name:string;node:ts.CallExpression}[]=[];
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))calls.push({name:node.expression.text,node});ts.forEachChild(node,visit);};visit(source);
 for(const scope of calls.filter(row=>scopes.includes(row.name)))for(const inner of calls)if(writers.includes(inner.name)&&inner.node.getStart()>scope.node.getStart()&&inner.node.getEnd()<=scope.node.getEnd())out.push(`${path}: ${inner.name} inside ${scope.name}`);
 for(const lease of calls.filter(row=>row.name===acquire))for(const inner of calls)if(writers.includes(inner.name)&&inner.node.getStart()>lease.node.getEnd())out.push(`${path}: ${inner.name} after ${acquire}`);
 return out;
}

test("no compile path waits for the compiler writer, the build-directory custody (or any other lease) while it holds cpu units: the writer custody is always the outer scope and the build lease the first lease",()=>{
 const paths=files(),actual=paths.flatMap(violations).sort();
 const program=`import json,re,subprocess,sys
root=sys.argv[1]
scopes=["withCompilerCapacityConsumptionV1","withCompilerCapacityScopeV1"];acquire="acquireCompilerCapacityLeaseV1"
writers=["withCompilerWorkspaceSharedWriteV1","withCompilerWorkspaceWriteV1","withCompilerWriterCustodyV1","withCompilerSharedHeadroomV1","withCompilerInitialHeadroomV1","withCompilerWorkspaceBorrowV1","acquireCargoBuildLeaseV1","acquireCargoTargetRecipeV1"]
paths=json.loads(sys.argv[2])
def span(text,start):
    depth=0;quote=None;i=start
    while i<len(text):
        c=text[i]
        if quote:
            if c=="\\\\":i+=1
            elif c==quote:quote=None
        elif c in "\\"'\`":quote=c
        elif c=="(":depth+=1
        elif c==")":
            depth-=1
            if depth==0:return i
        i+=1
    return len(text)
out=[]
for path in paths:
    text=open(root+"/"+path,encoding="utf8").read()
    for name in scopes:
        for m in re.finditer(r"\\b"+name+r"\\s*(?:<[^>(]*>)?\\(",text):
            end=span(text,m.end()-1)
            for w in writers:
                for n in re.finditer(r"\\b"+w+r"\\s*(?:<[^>(]*>)?\\(",text):
                    if m.end()<n.start()<end:out.append(path+": "+w+" inside "+name)
    for m in re.finditer(r"\\b"+acquire+r"\\s*(?:<[^>(]*>)?\\(",text):
        end=span(text,m.end()-1)
        for w in writers:
            for n in re.finditer(r"\\b"+w+r"\\s*(?:<[^>(]*>)?\\(",text):
                if n.start()>end:out.append(path+": "+w+" after "+acquire)
print(json.dumps(sorted(out)))`;
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",program,root,JSON.stringify(paths)],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 expect(paths.length).toBeGreaterThan(10);
 expect(JSON.parse(run.stdout)).toEqual(actual);
 expect(actual).toEqual([]);
});

const leases=["acquireCargoBuildLeaseV1","acquireCargoTargetRecipeV1"],writerScopes=writers.filter(name=>name.startsWith("with"));
function leasesInsideWriters(path:string):string[]{
 const text=readFileSync(join(root,path),"utf8"),source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true),out:string[]=[],calls:{name:string;node:ts.CallExpression}[]=[];
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))calls.push({name:node.expression.text,node});ts.forEachChild(node,visit);};visit(source);
 for(const scope of calls.filter(row=>writerScopes.includes(row.name)))for(const inner of calls)if(leases.includes(inner.name)&&inner.node.getStart()>scope.node.getStart()&&inner.node.getEnd()<=scope.node.getEnd())out.push(`${path}: ${inner.name} inside ${scope.name}`);
 return out;
}

test("the build lease (and with it the build-directory custody) is taken before the shared writer custody, never inside it: one global order custody, writer, cpu units",()=>{
 const paths=files(),actual=paths.flatMap(leasesInsideWriters).sort();
 const program=`import json,re
root=__import__("sys").argv[1]
leases=${JSON.stringify(leases)};scopes=${JSON.stringify(writerScopes)}
paths=json.loads(__import__("sys").argv[2])
def span(text,start):
    depth=0;quote=None;i=start
    while i<len(text):
        c=text[i]
        if quote:
            if c=="\\\\":i+=1
            elif c==quote:quote=None
        elif c in "\\"'\`":quote=c
        elif c=="(":depth+=1
        elif c==")":
            depth-=1
            if depth==0:return i
        i+=1
    return len(text)
out=[]
for path in paths:
    text=open(root+"/"+path,encoding="utf8").read()
    for name in scopes:
        for m in re.finditer(r"\\b"+name+r"\\s*(?:<[^>(]*>)?\\(",text):
            end=span(text,m.end()-1)
            for lease in leases:
                for n in re.finditer(r"\\b"+lease+r"\\s*(?:<[^>(]*>)?\\(",text):
                    if m.end()<n.start()<end:out.append(path+": "+lease+" inside "+name)
print(json.dumps(sorted(out)))`;
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",program,root,JSON.stringify(paths)],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(actual);
 expect(actual).toEqual([]);
});

const inputScopes=["withWriter","withBuildInputs","withCargoInputs"],inputFiles=():string[]=>spawnSync("git",["ls-files","-z","--cached","--others","--exclude-standard","--","🧰️framework/*.ts"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(path=>path&&existsSync(join(root,path))&&!/\/🧪️tests\//u.test(path)&&/withWriter|withBuildInputs|withCargoInputs/u.test(readFileSync(join(root,path),"utf8"))&&/acquireCargoBuildLeaseV1/u.test(readFileSync(join(root,path),"utf8")));
function leasesInsideInputScopes(path:string,text:string=readFileSync(join(root,path),"utf8")):string[]{
 const source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true),out:string[]=[],calls:{name:string;node:ts.CallExpression}[]=[],declarations=new Map<string,ts.Node[]>();
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)){const callee=node.expression;if(ts.isIdentifier(callee))calls.push({name:callee.text,node});else if(ts.isPropertyAccessExpression(callee))calls.push({name:callee.name.text,node});}
  if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.initializer&&(ts.isArrowFunction(node.initializer)||ts.isFunctionExpression(node.initializer)))declarations.set(node.name.text,[...(declarations.get(node.name.text)??[]),node.initializer]);
  ts.forEachChild(node,visit);
 };visit(source);
 for(const scope of calls.filter(row=>inputScopes.includes(row.name))){
  const ranges:ts.Node[]=[scope.node,...scope.node.arguments.filter(ts.isIdentifier).flatMap(argument=>declarations.get(argument.text)??[])];
  for(const range of ranges)for(const inner of calls)if(inner.name==="acquireCargoBuildLeaseV1"&&inner.node.getStart()>=range.getStart()&&inner.node.getEnd()<=range.getEnd())out.push(`${path}: acquireCargoBuildLeaseV1 inside ${scope.name}`);
 }
 return [...new Set(out)];
}

const inputScopeOracle=`import json,re,sys
root=sys.argv[1];paths=json.loads(sys.argv[2]);scopes=${JSON.stringify(inputScopes)};lease="acquireCargoBuildLeaseV1"
def span(text,start,open_,close):
    depth=0;quote=None;i=start
    while i<len(text):
        c=text[i]
        if quote:
            if c=="\\\\":i+=1
            elif c==quote:quote=None
        elif c in "\\"'\`":quote=c
        elif c==open_:depth+=1
        elif c==close:
            depth-=1
            if depth==0:return i
        i+=1
    return len(text)
def split(text):
    parts=[];depth=0;quote=None;last=0;i=0
    while i<len(text):
        c=text[i]
        if quote:
            if c=="\\\\":i+=1
            elif c==quote:quote=None
        elif c in "\\"'\`":quote=c
        elif c in "([{":depth+=1
        elif c in ")]}":depth-=1
        elif c=="," and depth==0:parts.append(text[last:i]);last=i+1
        i+=1
    parts.append(text[last:]);return parts
out=set()
for path in paths:
    text=open(root+"/"+path,encoding="utf8").read()
    leases=[m.start() for m in re.finditer(r"\\b"+lease+r"\\s*(?:<[^>(]*>)?\\(",text)]
    for name in scopes:
        for m in re.finditer(r"(?<!function )\\b"+name+r"\\s*(?:<[^>(]*>)?\\(",text):
            end=span(text,m.end()-1,"(",")");ranges=[(m.end(),end)]
            for argument in split(text[m.end():end]):
                argument=argument.strip()
                if re.fullmatch(r"[A-Za-z_]\\w*",argument):
                    for d in re.finditer(r"\\b(?:const|let|var)\\s+"+argument+r"\\s*=",text):
                        head=re.match(r"\\s*(?:async\\s*)?(?:\\([^)]*\\)|[A-Za-z_]\\w*)\\s*(?::[^=]*?)?=>\\s*\\{",text[d.end():])
                        if head:ranges.append((d.end(),span(text,d.end()+head.end()-1,"{","}")))
            for low,high in ranges:
                if any(low<=position<high for position in leases):out.add(path+": "+lease+" inside "+name)
print(json.dumps(sorted(out)))`;

test("the build lease is never taken inside an entered compiler writer (the writer an input scope offers through withWriter, or a port that holds it around its continuation: withBuildInputs, withCargoInputs; including the callbacks they receive by name): the build-directory custody follows the configuration and registry leases and precedes the writer on every test, lint and exact-law path",()=>{
 const paths=inputFiles(),actual=paths.flatMap(path=>leasesInsideInputScopes(path)).sort();
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",inputScopeOracle,root,JSON.stringify(paths)],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 expect(paths.length).toBeGreaterThanOrEqual(2);
 expect(JSON.parse(run.stdout)).toEqual(actual);
 expect(actual).toEqual([]);
});

test("the writer detector flags a lease taken inside an entered writer, in the callback of a writer-holding port or in a callback passed by name, and accepts the lease taken inside the input scope before its writer, identically in the oracle",()=>{
 const retired="async function a(){await withCargoCompilerInputsV1(x,port,async actual=>{await actual.withWriter(async()=>{const lease=await acquireCargoBuildLeaseV1(o);});});}",named="async function b(){const execute=async(c)=>{const lease=await acquireCargoBuildLeaseV1(o);};await port.withBuildInputs(a,b,execute);}",hoisted="async function c(){await withCargoCompilerInputsV1(x,port,async actual=>{const lease=await acquireCargoBuildLeaseV1(o);try{await actual.withWriter(async()=>1);}finally{lease.release();}});}";
 expect(leasesInsideInputScopes("retired.ts",retired)).toEqual(["retired.ts: acquireCargoBuildLeaseV1 inside withWriter"]);
 expect(leasesInsideInputScopes("named.ts",named)).toEqual(["named.ts: acquireCargoBuildLeaseV1 inside withBuildInputs"]);
 expect(leasesInsideInputScopes("hoisted.ts",hoisted)).toEqual([]);
 const dir=mkdtempSync(join(tmpdir(),"input-scope-"));try{const rows={"retired.ts":retired,"named.ts":named,"hoisted.ts":hoisted};for(const[name,text]of Object.entries(rows))writeFileSync(join(dir,name),text);const run=spawnSync(testPythonV1()??"python3",["-I","-c",inputScopeOracle,dir,JSON.stringify(Object.keys(rows))],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);expect(JSON.parse(run.stdout)).toEqual(Object.keys(rows).flatMap(name=>leasesInsideInputScopes(name,rows[name as keyof typeof rows])).sort());}finally{rmSync(dir,{recursive:true,force:true});}
});

test("the detector flags the retired order: a writer custody taken inside the cpu scope",()=>{
 const source=ts.createSourceFile("model.ts","withCompilerCapacityConsumptionV1(a,async()=>{await withCompilerWorkspaceSharedWriteV1(s,d,c,()=>1);});",ts.ScriptTarget.Latest,true),names:string[]=[];
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))names.push(node.expression.text);ts.forEachChild(node,visit);};visit(source);
 expect(names).toEqual(["withCompilerCapacityConsumptionV1","withCompilerWorkspaceSharedWriteV1"]);
});

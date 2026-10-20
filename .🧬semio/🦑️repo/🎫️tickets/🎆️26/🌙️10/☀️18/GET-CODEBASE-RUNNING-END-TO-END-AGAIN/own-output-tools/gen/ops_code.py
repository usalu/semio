import os

E = lambda code: chr(code) + "️"
LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
OWNER = os.path.join(LIB, E(0x1F3D7) + "build", E(0x1F578) + "graph", E(0x1F4E5) + "inference", E(0x1F50E) + "owner-discovery", E(0x1F7E8) + ".mjs")


def sub(text, old, new, label):
    count = text.count(old)
    if count != 1:
        raise SystemExit("anchor %s found %d times" % (label, count))
    return text.replace(old, new)


def subs(text, old, new, label, expected):
    count = text.count(old)
    if count != expected:
        raise SystemExit("anchor %s found %d times, expected %d" % (label, count, expected))
    return text.replace(old, new)


COMMAND_OLD = ''' for(const target of Object.values(selected)){
  const sources=await targetScriptSources(target,workspaceRoot,scripts,owner.root,owner.manifest.endsWith("Cargo.toml")?owner.manifest:json.metadata?.nativeRoot?buildPath(join(json.metadata.nativeRoot,"Cargo.toml")):undefined);
  for(const input of sources.closure??[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));

 }
 const commandModules=new Set(scripts.modules);
'''

COMMAND_NEW = ''' const roots={targets:new Map(targets.map(name=>[name,{compile:new Set(),declared:new Set(),command:new Set()}])),shared:{compile:new Set(),declared:new Set(),command:new Set()}},attributed=new Set();
 for(const [name,target] of Object.entries(selected)){
  const sources=await targetScriptSources(target,workspaceRoot,scripts,owner.root,owner.manifest.endsWith("Cargo.toml")?owner.manifest:json.metadata?.nativeRoot?buildPath(join(json.metadata.nativeRoot,"Cargo.toml")):undefined);
  for(const input of sources.closure??[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/")){const local=input.slice("{workspaceRoot}/".length);addFile(local);roots.targets.get(name).command.add(join(workspaceRoot,local));}
 }
 for(const row of roots.targets.values())for(const file of row.command){if(scripts.modules.has(file))attributed.add(file);else row.command.delete(file);}
 for(const file of scripts.modules)if(!attributed.has(file))roots.shared.command.add(file);
'''

HEAD = ''' const generatedDependencies={},roles=ownerAdmissionSchema.$defs.sourceDemand.const,admitProducer=producer=>{addFile(producer.manifest);if(producer.project!==json.name)dependencies.add(producer.project);},attach=(name,producer)=>{addInput(producer.project,"default");const rows=generatedDependencies[name]??=[];if(!rows.includes(producer.target))rows.push(producer.target);},consume=(name,producer,path,phase)=>{if(producer.target===json.name+":"+name)throw Error("Generated task cannot consume its own output: "+JSON.stringify({target:producer.target.slice(0,256),importer:buildPath(relative(workspaceRoot,path)).slice(0,256),output:producer.output.slice(0,256),phase}));attach(name,producer);};
 const compiler=createRequire(import.meta.url)("typescript"),host=observedTypeScriptHost(compiler.sys),expansions=new Map();let index=0;
 const expandImports=async(absolute,phase)=>{
  const key=phase+"\\0"+absolute,known=expansions.get(key);if(known)return known;
  await yieldInferenceV1(context,"build:selected-compile-imports",++index);
  const path=buildPath(relative(workspaceRoot,absolute)),children=[],hits=[];readPhysicalSource(absolute);const facts=observedSourceAnalysisV1(prepareSourceAnalysesV1().module,absolute),imports=phase===roles.compileInput?facts.imports:facts.runtimeImports;
'''

TAIL = ''' const expansion={children,hits};expansions.set(key,expansion);return expansion;
 };
 const closeImports=async(root,receive)=>{
  const pending=[],demands=new Map(),visited=new Map(),enqueue=(file,role)=>{const next=buildSourceDemandV1(demands.get(file),role);if(next!==demands.get(file)){demands.set(file,next);pending.push(file);}};
  for(const file of root.compile)enqueue(file,roles.compileInput);for(const file of root.declared)enqueue(file,roles.declaredEntry);for(const file of root.command)enqueue(file,roles.command);
  while(pending.length){const absolute=pending.pop(),phase=demands.get(absolute);if(visited.get(absolute)===phase)continue;visited.set(absolute,phase);const {children,hits}=await expandImports(absolute,phase);for(const file of children)enqueue(file,phase);for(const producer of hits)receive(producer,absolute,phase);}
 };
 await stage("compile-closure");
 for(const name of targets)await closeImports(roots.targets.get(name),(producer,path,phase)=>consume(name,producer,path,phase));
 await closeImports(roots.shared,producer=>{for(const name of targets)if(producer.target!==json.name+":"+name)attach(name,producer);});
'''


def owner_discovery(text):
    text = sub(text, "const matched=new Set(),compileMatched=new Set(),walked=new Map(),", "const matched=new Set(),compileMatched=new Set(),compiled=scans.map(()=>new Set()),walked=new Map(),", "scan-sets")
    text = sub(text, "for(const scan of scans){const patterns=scan.patterns,", "for(const [scanIndex,scan] of scans.entries()){const patterns=scan.patterns,", "scan-loop")
    text = sub(text, "if(scan.compile)compileMatched.add(path);", "if(scan.compile){compileMatched.add(path);compiled[scanIndex].add(path);}", "scan-compile")
    text = sub(text, " return {matched,compileMatched};", " return {matched,compileMatched,compiled};", "scan-return")
    text = sub(text, COMMAND_OLD, COMMAND_NEW, "command-loop")
    text = sub(text, "const expand=(values,patterns)=>{", "const expand=(values,patterns,sink)=>{", "expand-head")
    text = subs(text, "expand([value.input],patterns)", "expand([value.input],patterns,sink)", "expand-input", 1)
    text = sub(text, "expand([value.fileset],patterns)", "expand([value.fileset],patterns,sink)", "expand-fileset")
    text = sub(text, "for(const entry of loaded.readSourceInputContract(join(workspaceRoot,contract)).entries)declaredCompileEntries.add(join(workspaceRoot,entry));",
              "for(const entry of loaded.readSourceInputContract(join(workspaceRoot,contract)).entries){const file=join(workspaceRoot,entry);declaredCompileEntries.add(file);sink.add(file);}", "declared-add")
    text = sub(text, "expand(named[value],patterns);active.delete(value)", "expand(named[value],patterns,sink);active.delete(value)", "expand-named")
    text = sub(text, 'expand(normalized.targets[name].inputs??["default"],patterns);scans.push({patterns,compile:',
              'expand(normalized.targets[name].inputs??["default"],patterns,roots.targets.get(name).declared);scans.push({patterns,target:name,compile:', "scan-push")
    text = sub(text, "const requested=[];expand(inputNames,requested);", "const requested=[];expand(inputNames,requested,roots.shared.declared);", "requested")
    text = sub(text, "const {matched,compileMatched}=await readBuildSourceInputMatchesV1(scans,workspaceRoot,context);", "const {matched,compileMatched,compiled}=await readBuildSourceInputMatchesV1(scans,workspaceRoot,context);", "matches")
    text = sub(text, "  for(const path of modules)compileModules.add(path);\n",
              "  for(const path of modules)compileModules.add(path);\n  scans.forEach((scan,index)=>{if(scan.target)for(const path of compiled[index])if(/\\.[cm]?[jt]sx?$/u.test(path)){const file=join(workspaceRoot,path);if(declaredCompileEntries.has(file)||!scripts.resources.has(file)||scripts.modules.has(file))roots.targets.get(scan.target).compile.add(file);}});\n", "target-compile")
    start = text.index(" const generatedDependencies={},roles=")
    end = text.index(" for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);")
    lines = text[start:end].split("\n")
    assert lines[5].startswith("  for(const specifier of imports){") and lines[12] == "  }" and lines[13] == " }", (lines[5][:40], lines[12], lines[13])
    body = lines[5:12]
    body[3] = body[3].replace("if(producer){generated(producer,absolute,phase);continue;}", "if(producer){admitProducer(producer);hits.push(producer);continue;}")
    body[6] = body[6].replace("const produced=scripts.generatedOutput(file);if(produced){generated(produced,absolute,phase);continue;}", "const output=scripts.generatedOutput(file);if(output){admitProducer(output);hits.push(output);continue;}").replace("enqueue(file,phase);", "children.push(file);")
    assert "generated(" not in "\n".join(body) and "enqueue(" not in "\n".join(body) and body[1].lstrip().startswith("await yieldInferenceV1")
    body.append("  }")
    new_region = HEAD + "\n".join(body) + "\n" + TAIL
    return text[:start] + new_region + text[end:]

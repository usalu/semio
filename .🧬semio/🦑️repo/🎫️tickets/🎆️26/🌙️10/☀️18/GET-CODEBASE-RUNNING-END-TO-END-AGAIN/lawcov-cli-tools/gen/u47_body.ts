const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const witness=(row:{defect:string})=>row.defect?test.failing:test;
const find=(root:Node,type:string):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(node.type===type)found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
for(const row of corpus.keys)test("dependency row "+JSON.stringify(row.row),()=>{expect(dependencyRowKey(row.row)).toBe(row.key);expect(dependencyIdent(row.key)).toBe(row.ident);});
for(const row of corpus.cfg)test("cfg attribute "+JSON.stringify(row.attribute),()=>{
 expect(testGateFeatures(row.attribute)).toEqual(row.gate);expect(cfgFeatures(row.attribute)).toEqual(row.cfg);expect(testOnlyGate([row.attribute])!==null).toBe(row.testOnly);
});
for(const row of corpus.only)test("test-only gate of "+JSON.stringify(row.attributes),()=>{expect(testOnlyGate(row.attributes)).toBe(row.expected);});
const shown=(text:string)=>scanTestGates(text).map(gate=>({features:[...gate.features],text:text.slice(gate.start,gate.end),names:[...gate.names],method:gate.method,owner:gate.owner,use:gate.use}));
for(const row of corpus.gates)witness(row)("gates: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{expect(shown(row.text)).toEqual(row.gates);});
test("a custom feature reader decides which attributes are gates",()=>{
 const text='#[cfg(feature = "a")]\npub fn one() {}\n#[cfg(feature = "b")]\npub fn two() {}\n';
 expect(scanTestGates(text).length).toBe(0);expect(scanTestGates(text,attribute=>cfgFeatures(attribute).filter(feature=>feature==="b")).map(gate=>[gate.features,gate.names])).toEqual([[["b"],["two"]]]);
});
test("tree-sitter reads every gated span as the item the scanner names (methods inside their impl)",async()=>{
 const parser=await parsed();
 for(const row of corpus.gates)for(const gate of row.gates){
  const wrapped=gate.method?"impl "+(gate.owner??"X")+" {\n"+gate.text+(gate.text.endsWith(";")?"":"")+"\n}":gate.text.includes("let x")?"fn w() {\n"+gate.text+"\n}":gate.text,root=parser.parse(wrapped).rootNode;
  if(gate.method&&gate.names.length===0)continue;
  expect(root.hasError(),row.name+": "+gate.text).toBe(false);
  if(gate.use){const declaration=find(root,"use_declaration")[0]!;expect(declaration.text.includes("use"),row.name).toBe(true);for(const name of gate.names)expect(declaration.text,row.name).toContain(name);continue;}
  if(gate.names.length===0)continue;
  const named=[...find(root,"function_item"),...find(root,"struct_item"),...find(root,"enum_item"),...find(root,"type_item"),...find(root,"const_item"),...find(root,"static_item"),...find(root,"mod_item"),...find(root,"macro_definition")].map(node=>node.childForFieldName("name")?.text??node.namedChildren.find(child=>child.type==="identifier")?.text);
  expect(named,row.name).toContain(gate.names[0]);
 }
});
for(const row of corpus.declared)witness(row)("declared names of "+JSON.stringify(row.text.slice(0,40))+(row.defect?" ["+row.defect+"]":""),()=>{expect([...declaredNames(row.text)].sort()).toEqual(row.expected);});
const declaredByTreeSitter=(root:Node):string[]=>{
 const out:string[]=[];
 const visit=(node:Node):void=>{
  if(["impl_item","trait_item"].includes(node.type)){if(node.type==="trait_item")out.push(node.childForFieldName("name")!.text);return;}
  if(["function_item","struct_item","enum_item","union_item","type_item","const_item","static_item","mod_item"].includes(node.type)){const name=node.childForFieldName("name");if(name)out.push(name.text);}
  if(node.type==="macro_definition")out.push(node.childForFieldName("name")!.text);
  for(const child of node.children)visit(child);
 };
 visit(root);return out.sort();
};
test("tree-sitter lists the same declared names (outside impl and trait bodies)",async()=>{
 const parser=await parsed();
 for(const row of corpus.declared.filter(item=>!item.defect))expect(declaredByTreeSitter(parser.parse(row.text).rootNode),row.text).toEqual(row.expected);
 for(const row of corpus.declared.filter(item=>item.defect))expect(declaredByTreeSitter(parser.parse(row.text).rootNode),row.text).toEqual(row.expected);
});
for(const row of corpus.externals)test("external dependencies: "+row.name,()=>{
 const found=externalDependencies(row.manifest,{dependencies:row.dependencies,devDependencies:row.devDependencies});
 const document=parse(row.manifest) as any,normal=document.dependencies??{},dev=document["dev-dependencies"]??{},features=(spec:any):string[]=>typeof spec==="object"&&spec!==null&&Array.isArray(spec.features)?spec.features.map(String):[];
 const expected=[...new Set([...Object.keys(normal),...Object.keys(dev)])].sort().map(key=>({key,ident:key.replaceAll("-","_"),dependency:key in normal,devOnly:!(key in normal)&&key in dev,devFeatures:features(dev[key]).filter(feature=>!features(normal[key]).includes(feature))}));
 expect(found.map(entry=>({key:entry.key,ident:entry.ident,dependency:entry.dependency,devOnly:entry.devOnly,devFeatures:[...entry.devFeatures]}))).toEqual(expected);
 for(const entry of found){
  const devRow=row.devDependencies.find(line=>dependencyRowKey(line)===entry.key)??null,anyRow=[...row.dependencies,...row.devDependencies].reverse().find(line=>dependencyRowKey(line)===entry.key)??null;
  expect(entry.devRow).toBe(devRow);expect(entry.row).toBe([...row.dependencies,...row.devDependencies].filter(line=>dependencyRowKey(line)===entry.key).at(-1)??null);void anyRow;
 }
});
for(const row of corpus.optional)witness(row)("optional row "+JSON.stringify(row.row)+(row.defect?" ["+row.defect+"]":""),()=>{
 const result=optionalDependencyRow(row.row);expect(result).toBe(row.expected);
 const before=parse("[dependencies]\n"+row.row.replace(/\.workspace\s*=\s*true/u," = { workspace = true }")) as any,after=parse("[dependencies]\n"+result) as any,key=dependencyRowKey(row.row),old=before.dependencies[key],now=after.dependencies[key];
 expect(typeof now==="object"&&now.optional===true).toBe(true);
 if(typeof old==="object")for(const field of Object.keys(old))expect(now[field]).toEqual(old[field]);else expect(now.version).toBe(old);
});
test("optional rows are idempotent and parse as TOML with optional = true",()=>{
 for(const row of corpus.optional.filter(item=>!item.defect)){const once=optionalDependencyRow(row.row);expect(optionalDependencyRow(once)).toBe(once);}
});
for(const row of corpus.required)test("required test features: "+row.name,()=>{expect(requiredTestFeatures(new Set(row.targetTokens),new Map(Object.entries(row.gated).map(([feature,names])=>[feature,new Set(names)])))).toEqual(row.expected);});
for(const row of corpus.withRequired)witness(row)("required-features: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{
 const result=withRequiredFeatures(row.manifest,new Map(Object.entries(row.byTarget)));expect(result).toBe(row.expected);
 const document=parse(result) as any,original=parse(row.manifest) as any;
 for(const target of document.test??[]){const wanted=row.byTarget[target.name]??[],before=(original.test??[]).find((entry:any)=>entry.name===target.name)?.["required-features"]??[];if(wanted.length)expect(target["required-features"]).toEqual([...new Set([...before,...wanted])]);else expect(target["required-features"]).toEqual(before.length?before:undefined);}
});
for(const row of corpus.featureRows)witness(row)("feature row of "+row.feature.name+(row.defect?" ["+row.defect+"]":""),()=>{expect(testkitFeatureRow(row.feature)).toBe(row.expected);expect((parse(row.expected) as any)[row.feature.name]).toEqual(row.feature.entries);});
for(const row of corpus.withFeatures)witness(row)("feature rows: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{
 const result=withFeatureRows(row.manifest,row.features);expect(result).toBe(row.expected);
 const document=parse(result) as any,original=parse(row.manifest) as any;
 for(const feature of row.features)expect(document.features[feature.name]).toEqual(feature.entries);
 for(const[name,value]of Object.entries<any>(original.features??{}))if(!row.features.some(feature=>feature.name===name))expect(document.features[name]).toEqual(value);
 expect(document.dependencies).toEqual(original.dependencies);expect(withFeatureRows(result,row.features)).toBe(result);
});
const crateOf=async(row:any):Promise<TestkitCrate>=>buildTestkitCrate({name:row.name,ident:row.ident,dependsOn:row.dependsOn,devDependsOn:row.devDependsOn,ownedFiles:row.owned===null?undefined:new Set<string>(row.owned),files:new Map(Object.entries<string>(row.files))},async feature=>new Set<string>(row.featureFiles[feature]??[]));
test("a crate is scanned for gates and asks the closure for every gate feature once, sorted",async()=>{
 const asked:string[]=[];
 const crate=await buildTestkitCrate({name:"c",ident:"c",dependsOn:[],files:new Map(Object.entries<string>(corpus.build.files))},async feature=>{asked.push(feature);return new Set<string>((corpus.build.closures as Record<string,string[]>)[feature]??[]);});
 expect(asked).toEqual(["testkit"]);expect([...crate.gates.keys()].sort()).toEqual(["src/a.rs","src/c.rs"]);
 expect([...crate.featureFiles].map(([feature,files])=>[feature,[...files].sort()])).toEqual([["testkit",["src/a.rs","src/extra.rs"]]]);
 expect(crate.gates.get("src/a.rs")!.map(gate=>[[...gate.features],[...gate.names]])).toEqual([[["testkit"],["make"]]]);
});
for(const row of corpus.wiring)test("wiring: "+row.name,async()=>{
 const crates=await Promise.all(row.crates.map(crateOf)),externals:ExternalDependency[]=row.externals.map(entry=>({...entry,devFeatures:[...entry.devFeatures]}));
 const wiring=planTestkitWiring(crates,externals,new Set<string>(row.declared)),shownWiring:Record<string,unknown>={};
 for(const[name,entry]of[...wiring].sort())shownWiring[name]={features:entry.features.map(feature=>({name:feature.name,entries:[...feature.entries]})),optionalDependencies:[...entry.optionalDependencies],consumed:Object.fromEntries([...entry.consumed].sort().map(([dependency,features])=>[dependency,[...features]])),gatedNames:Object.fromEntries([...entry.gatedNames].sort().map(([feature,names])=>[feature,[...names].sort()]))};
 expect(shownWiring).toEqual(row.expect);
 const again=planTestkitWiring(crates,externals,new Set<string>(row.declared));expect([...again.keys()]).toEqual([...wiring.keys()]);
});
test("every wired feature list is valid TOML once written as manifest rows",async()=>{
 for(const row of corpus.wiring){
  const crates=await Promise.all(row.crates.map(crateOf)),wiring=planTestkitWiring(crates,row.externals.map(entry=>({...entry,devFeatures:[...entry.devFeatures]})),new Set<string>(row.declared));
  for(const[,entry]of wiring){const text=withFeatureRows('[package]\nname = "x"\n\n[dependencies]\n',entry.features),document=parse(text) as any;for(const feature of entry.features)expect(document.features[feature.name],row.name).toEqual(feature.entries);}
 }
});

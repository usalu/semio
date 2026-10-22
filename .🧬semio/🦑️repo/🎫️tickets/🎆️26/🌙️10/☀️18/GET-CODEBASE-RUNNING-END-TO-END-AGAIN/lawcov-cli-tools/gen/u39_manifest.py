import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror/📝manifest"
base = '[package]\nname = "x"\nversion = "0.1.0"\n\n[dependencies]\nalpha = "1"\nzeta = { version = "2" }\n\n[dev-dependencies]\nmock = "3"\n'
decl = [
 {"name": "dependency sections including target and build tables", "manifest": base + "\n[build-dependencies]\ncc = \"1\"\n\n[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n\n[features]\nalpha = []\n", "expected": {"alpha": ["dependencies"], "zeta": ["dependencies"], "mock": ["dev-dependencies"], "cc": ["build-dependencies"], "libc": ["target.'cfg(unix)'.dependencies"]}},
 {"name": "a dependency in several sections", "manifest": '[dependencies]\nserde = "1"\n[dev-dependencies]\nserde = "1"\n', "expected": {"serde": ["dependencies", "dev-dependencies"]}},
 {"name": "keys outside dependency tables are ignored", "manifest": '[package]\nname = "p"\n[features]\nserde = []\n[workspace.dependencies]\nws = "1"\n', "expected": {"ws": ["workspace.dependencies"]}},
 {"name": "an empty manifest", "manifest": "", "expected": {}},
]
decl_defects = [{"name": "dotted-key and table-form dependencies are declared", "manifest": '[dependencies]\nserde.workspace = true\n[dependencies.tabled]\nversion = "3"\n', "expected": {"serde": ["dependencies"], "tabled": ["dependencies"]}, "defect": "📝️manifest/🟦️.ts:declaredDependencies matches only `name =` at the start of a line inside a `…dependencies` section: `serde.workspace = true` and `[dependencies.tabled]` are not declared dependencies (same blind spot as the dependency inventory)"}]
add = [
 {"name": "a new key sorts between its neighbours", "manifest": base, "additions": [{"name": "mid", "path": "../mid", "section": "dependencies"}], "expected": base.replace('zeta = { version = "2" }', 'mid = { path = "../mid" }\nzeta = { version = "2" }')},
 {"name": "a key that sorts last goes after the last entry", "manifest": base, "additions": [{"name": "zzz", "path": "../z", "section": "dependencies"}], "expected": base.replace('zeta = { version = "2" }\n', 'zeta = { version = "2" }\nzzz = { path = "../z" }\n')},
 {"name": "a missing section is created at the end", "manifest": '[package]\nname = "x"\n\n[dependencies]\nalpha = "1"\n', "additions": [{"name": "t", "path": "../t", "section": "dev-dependencies"}], "expected": '[package]\nname = "x"\n\n[dependencies]\nalpha = "1"\n\n[dev-dependencies]\nt = { path = "../t" }\n'},
 {"name": "an empty section takes the line right after its header", "manifest": '[dependencies]\n\n[features]\nf = []\n', "additions": [{"name": "a", "path": "../a", "section": "dependencies"}], "expected": '[dependencies]\na = { path = "../a" }\n\n[features]\nf = []\n'},
 {"name": "two additions keep byte order", "manifest": '[dependencies]\nm = "1"\n', "additions": [{"name": "b", "path": "../b", "section": "dependencies"}, {"name": "a", "path": "../a", "section": "dependencies"}], "expected": '[dependencies]\na = { path = "../a" }\nb = { path = "../b" }\nm = "1"\n'},
 {"name": "a backslash path is escaped", "manifest": '[dependencies]\nm = "1"\n', "additions": [{"name": "w", "path": "..\\win", "section": "dependencies"}], "expected": '[dependencies]\nm = "1"\nw = { path = "..\\\\win" }\n'},
 {"name": "trailing blank lines before a new section are collapsed", "manifest": '[package]\nname = "x"\n\n\n\n', "additions": [{"name": "d", "path": "../d", "section": "dependencies"}], "expected": '[package]\nname = "x"\n\n[dependencies]\nd = { path = "../d" }\n'},
 {"name": "no additions change nothing", "manifest": base, "additions": [], "expected": base},
 {"name": "an uppercase name sorts before lowercase in byte order", "manifest": '[dependencies]\nalpha = "1"\n', "additions": [{"name": "Zed", "path": "../Zed", "section": "dependencies"}], "expected": '[dependencies]\nZed = { path = "../Zed" }\nalpha = "1"\n'},
]
rem = [
 {"name": "plain and inline-table entries are removed from both sections", "manifest": '[dependencies]\na = "1"\nb = { version = "2" }\nc = "3"\n[dev-dependencies]\na = "1"\nd = "4"\n', "names": ["a", "b"], "expected": '[dependencies]\nc = "3"\n[dev-dependencies]\nd = "4"\n'},
 {"name": "other sections keep their entries", "manifest": '[package]\nname = "a"\n[features]\na = []\n[dependencies]\na = "1"\n', "names": ["a"], "expected": '[package]\nname = "a"\n[features]\na = []\n[dependencies]\n'},
 {"name": "a multi-line inline table is kept whole", "manifest": '[dependencies]\na = {\n  version = "1"\n}\nb = "2"\n', "names": ["a"], "expected": '[dependencies]\na = {\n  version = "1"\n}\nb = "2"\n'},
 {"name": "names that are not listed stay", "manifest": '[dependencies]\nx = "1"\n', "names": ["y"], "expected": '[dependencies]\nx = "1"\n'},
 {"name": "target tables count as dependency sections", "manifest": "[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\nother = \"1\"\n", "names": ["libc"], "expected": "[target.'cfg(unix)'.dependencies]\nother = \"1\"\n"},
 {"name": "build dependencies are never removed", "manifest": '[build-dependencies]\ncc = "1"\n', "names": ["cc"], "expected": '[build-dependencies]\ncc = "1"\n'},
]
rem_defects = [{"name": "a single-line entry with a features array is removed", "manifest": '[dependencies]\na = { version = "1", features = ["x"] }\nb = "2"\n', "names": ["a"], "expected": '[dependencies]\nb = "2"\n', "defect": "📝️manifest/🟦️.ts:removeDependencyLines refuses any line containing `[`, so an unreferenced dependency with a `features = [...]` array stays in the manifest"}, {"name": "a dotted-key workspace entry is removed", "manifest": '[dependencies]\nserde.workspace = true\nb = "2"\n', "names": ["serde"], "expected": '[dependencies]\nb = "2"\n', "defect": "📝️manifest/🟦️.ts:removeDependencyLines' key pattern does not match `serde.workspace = true`"}]
corpus = {"version": 1, "declared": decl, "declaredDefects": decl_defects, "add": add, "remove": rem, "removeDefects": rem_defects}
sets = S.map(S.strs())
adds = S.arr(S.obj({"name": S.str, "path": S.str, "section": S.enum("dependencies", "dev-dependencies")}))
schema = S.obj({"version": S.const(1), "declared": S.arr(S.obj({"name": S.str, "manifest": S.str, "expected": sets}), 1), "declaredDefects": S.arr(S.obj({"name": S.str, "manifest": S.str, "expected": sets, "defect": S.str}), 1),
 "add": S.arr(S.obj({"name": S.str, "manifest": S.str, "additions": adds, "expected": S.str}), 1), "remove": S.arr(S.obj({"name": S.str, "manifest": S.str, "names": S.strs(), "expected": S.str}), 1), "removeDefects": S.arr(S.obj({"name": S.str, "manifest": S.str, "names": S.strs(), "expected": S.str, "defect": S.str}), 1)})
body = '''const plain=(map:Map<string,Set<string>>):Record<string,string[]>=>Object.fromEntries([...map.entries()].map(([name,sections])=>[name,[...sections].sort()]));
const dependencyTables=(document:any):Record<string,string[]>=>{
 const found:Record<string,Set<string>>={};
 const visit=(table:any,path:string):void=>{
  if(table===null||typeof table!=="object")return;
  for(const[key,value]of Object.entries<any>(table)){
   if(/^(?:dev-|build-)?dependencies$/u.test(key)&&value&&typeof value==="object"){for(const name of Object.keys(value))(found[name]??=new Set()).add(path?path+"."+key:key);}
   else if(key==="target"&&value&&typeof value==="object"){for(const[cfg,inner]of Object.entries<any>(value))for(const section of Object.keys(inner)){if(/^(?:dev-|build-)?dependencies$/u.test(section))for(const name of Object.keys(inner[section]))(found[name]??=new Set()).add("target.'"+cfg+"'."+section);}}
   else if(key==="workspace"&&value&&typeof value==="object")visit(value,"workspace");
  }
 };
 visit(document,"");return Object.fromEntries(Object.entries(found).map(([name,sections])=>[name,[...sections].sort()]));
};
for(const row of corpus.declared)test("declared dependencies: "+row.name,()=>{expect(plain(declaredDependencies(row.manifest))).toEqual(row.expected);});
for(const row of corpus.declaredDefects)test.failing("declared dependencies defect: "+row.name,()=>{expect(plain(declaredDependencies(row.manifest))).toEqual(row.expected);});
test("@iarna/toml reads the same declared dependencies for every well-formed manifest",()=>{for(const row of corpus.declared.filter(item=>item.manifest!==""))expect(dependencyTables(parse(row.manifest)),row.name).toEqual(row.expected);for(const row of corpus.declaredDefects)expect(dependencyTables(parse(row.manifest)),row.name).toEqual(row.expected);});
for(const row of corpus.add)test("add dependency lines: "+row.name,()=>{expect(addDependencyLines(row.manifest,row.additions as DependencyAddition[])).toBe(row.expected);});
test("added dependencies are valid TOML path dependencies and every earlier dependency survives (@iarna/toml)",()=>{
 for(const row of corpus.add){
  const before=parse(row.manifest) as any,after=parse(addDependencyLines(row.manifest,row.additions as DependencyAddition[])) as any;
  for(const addition of row.additions)expect(after[addition.section][addition.name],row.name+" "+addition.name).toEqual({path:addition.path});
  for(const section of["dependencies","dev-dependencies"])for(const[name,value]of Object.entries<any>(before[section]??{}))expect(after[section][name],row.name+" "+name).toEqual(value);
  expect(after.package).toEqual(before.package);
 }
});
for(const row of corpus.remove)test("remove dependency lines: "+row.name,()=>{expect(removeDependencyLines(row.manifest,new Set(row.names))).toBe(row.expected);});
for(const row of corpus.removeDefects)test.failing("remove dependency lines defect: "+row.name,()=>{expect(removeDependencyLines(row.manifest,new Set(row.names))).toBe(row.expected);});
test("removal leaves valid TOML without the removed dependencies (@iarna/toml)",()=>{
 for(const row of corpus.remove.filter(item=>!item.manifest.includes("{\\n"))){const after=parse(removeDependencyLines(row.manifest,new Set(row.names))) as any;for(const section of["dependencies","dev-dependencies"])for(const name of row.names)if(!row.name.includes("build"))expect(Object.keys(after[section]??{}),row.name).not.toContain(name);}
});'''
ts = lib.ts_wrap(UNIT, "📝️ Contract: unmirror manifest edits list the declared dependency tables, insert path dependencies in byte order (creating the section when missing), and remove single-line dependency entries by plain line edits that keep hand-formatted manifests intact; the corpus is language-agnostic and `@iarna/toml` is the independent third-party oracle for the declared tables, the validity of every edited manifest and the survival of everything else.",
 'import {parse} from "@iarna/toml";\nimport {addDependencyLines,declaredDependencies,removeDependencyLines,type DependencyAddition} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)

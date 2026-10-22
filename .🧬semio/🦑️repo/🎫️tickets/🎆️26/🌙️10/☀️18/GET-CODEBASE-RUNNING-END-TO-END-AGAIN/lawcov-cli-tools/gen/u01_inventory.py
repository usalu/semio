import re, lib
from lib import S, resolve, LIBN
UNIT = "🕸dependencies/📇inventory"
src = open(resolve(LIBN + "/" + UNIT + "/🟦.ts"), encoding="utf8").read()
owner_manifest = re.search(r'three: Object\.freeze\(\{[^}]*manifest: "([^"]+)"', src).group(1)

def entry(eco, name, version, kinds, users, decls, reach):
    return {"ecosystem": eco, "name": name, "version": version, "kinds": kinds, "users": users, "productionReachable": reach,
            "declarations": [{"user": u, "version": v, "kind": k} for (u, v, k) in decls]}

RT, BU, TR, TO = "production-runtime", "production-build", "test-runner", "repository-tooling"
CA, CX, CM, CT = "crates/a/Cargo.toml", "crates/fixtures/x/Cargo.toml", "crates/m/Cargo.toml", None
cargo_files = {
 "Cargo.toml": '[workspace]\nmembers = ["crates/a", "crates/m", "crates/fixtures/x"]\n\n[workspace.dependencies]\nserde = "1.0.200"\ntokio = { version = "1.38", features = ["rt"] }\nsemio-core = { path = "crates/core" }\nanyhow = { version = "1.0.80" }\n',
 CA: '[package]\nname = "a"\nversion = "0.1.0"\n\n[dependencies]\nserde.workspace = true\ntokio = { workspace = true }\nsemio-core = { workspace = true }\nregex = "1.10"\nlocal = { path = "../local" }\nsemio-extra = "0.1"\ndb_core = "0.1"\nmulti = { version = "2.0", features = [\n  "a",\n] }\n\n[dev-dependencies]\nproptest = "1.4"\n\n[build-dependencies]\ncc = "1.0"\n\n[target.\'cfg(unix)\'.dependencies]\nlibc = "0.2"\n\n[[bin]]\nname = "x"\npath = "src/main.rs"\n',
 CX: '[package]\nname = "x"\nversion = "0.1.0"\n\n[dependencies]\ninsta = "1.39"\nregex = "1.9"\n',
 CM: '[package]\nname = "m"\nversion = "0.1.0"\n\n[lib]\nproc-macro = true\n\n[dependencies]\nsyn = "2"\nquote = { version = "1" }\n',
}
cargo_expected = [
 entry("rust", "cc", "1.0", [BU], [CA], [(CA, "1.0", BU)], True),
 entry("rust", "insta", "1.39", [TR], [CX], [(CX, "1.39", TR)], False),
 entry("rust", "libc", "0.2", [RT], [CA], [(CA, "0.2", RT)], True),
 entry("rust", "multi", "2.0", [RT], [CA], [(CA, "2.0", RT)], True),
 entry("rust", "proptest", "1.4", [TR], [CA], [(CA, "1.4", TR)], False),
 entry("rust", "quote", "1", [BU], [CM], [(CM, "1", BU)], True),
 entry("rust", "regex", "1.10", [RT, TR], [CA, CX], [(CA, "1.10", RT), (CX, "1.9", TR)], True),
 entry("rust", "serde", "1.0.200", [RT], [CA], [(CA, "1.0.200", RT)], True),
 entry("rust", "syn", "2", [BU], [CM], [(CM, "2", BU)], True),
 entry("rust", "tabled", "3.0", [RT], [CA], [(CA, "3.0", RT)], True),
 entry("rust", "tokio", "1.38", [RT], [CA], [(CA, "1.38", RT)], True),
]
cargo_expected_current = [e for e in cargo_expected if e["name"] != "tabled"]
cargo_files_tabled = dict(cargo_files)
cargo_files_tabled[CA] = cargo_files[CA].replace("[dev-dependencies]", '[dependencies.tabled]\nversion = "3.0"\n\n[dev-dependencies]')

import json
PA, PB, PT = "package.json", "pkgs/a/package.json", "pkgs/tests/p/package.json"
js_files = {
 PA: json.dumps({"name": "root", "workspaces": ["pkgs/a"], "dependencies": {"left-pad": "^1.3.0", "@semio-tech/core": "workspace:*"}, "devDependencies": {"vitest": "^1.0.0", "internal-lib": "1.0.0"}}),
 PB: json.dumps({"name": "internal-lib", "version": "1.0.0", "dependencies": {"react": "^18.0.0", "linked": "link:../x"}, "peerDependencies": {"react-dom": "^18"}, "optionalDependencies": {"fsevents": "^2"}, "devDependencies": {"vitest": "^1.1.0"}}),
 PT: json.dumps({"dependencies": {"react": "^17"}}),
 "node_modules/x/package.json": json.dumps({"dependencies": {"hidden": "1"}}),
 "compose/p/package.json": json.dumps({"dependencies": {"composed": "1"}}),
 "pkgs/bad/package.json": "{",
}
js_expected = [
 entry("js", "fsevents", "^2", [RT], [PB], [(PB, "^2", RT)], True),
 entry("js", "left-pad", "^1.3.0", [RT], [PA], [(PA, "^1.3.0", RT)], True),
 entry("js", "react", "^18.0.0", [RT, TR], [PB, PT], [(PB, "^18.0.0", RT), (PT, "^17", TR)], True),
 entry("js", "react-dom", "^18", [RT], [PB], [(PB, "^18", RT)], True),
 entry("js", "vitest", "^1.0.0", [TO], [PA, PB], [(PA, "^1.0.0", TO), (PB, "^1.1.0", TO)], False),
]
GA, GC = "a/go.mod", "c/go.mod"
go_files = {
 "go.work": "go 1.22\n\nuse (\n\t./a\n\t./compose/b\n)\n\nuse ./c\n",
 GA: "module example.com/a\n\ngo 1.22\n\nrequire (\n\tgithub.com/x/y v1.2.3\n\tgithub.com/z/w v0.1.0 // indirect\n\texample.com/local v0.0.0\n\tsemio.tech/q v1.0.0\n\tnodots v1.0.0\n)\n\nreplace example.com/local => ../local\n",
 GC: "module example.com/c\n\ngo 1.22\n\nrequire github.com/x/y v1.3.0\nrequire example.com/a v0.0.0\n",
 "compose/b/go.mod": "module example.com/b\n\nrequire github.com/hidden/h v1.0.0\n",
}
go_expected = [
 entry("go", "github.com/x/y", "v1.2.3", [RT], [GA, GC], [(GA, "v1.2.3", RT), (GC, "v1.3.0", RT)], True),
 entry("go", "github.com/z/w", "v0.1.0", [BU], [GA], [(GA, "v0.1.0", BU)], True),
]
PP = "py/pyproject.toml"
py_files = {
 PP: '[project]\nname = "p"\ndependencies = [\n  "requests>=2.31",\n  "semio-tools",\n  "numpy",\n]\n\n[project.optional-dependencies]\ndev = [\n  "pytest>=8",\n]\n',
}
py_expected = [
 entry("python", "numpy", "*", [RT], [PP], [(PP, "*", RT)], True),
 entry("python", "pytest", ">=8", [TR], [PP], [(PP, ">=8", TR)], False),
 entry("python", "requests", ">=2.31", [RT], [PP], [(PP, ">=2.31", RT)], True),
]
PL = "py/lines/pyproject.toml"
pyline_files = {PL: '[project]\nname = "q"\ndependencies = ["flask>=3", "click"]\n'}
pyline_expected = [
 entry("python", "click", "*", [RT], [PL], [(PL, "*", RT)], True),
 entry("python", "flask", ">=3", [RT], [PL], [(PL, ">=3", RT)], True),
]
DA, DT = "dn/App/App.csproj", "dn/Tests/Tests.csproj"
dn_files = {
 DA: '<Project Sdk="Microsoft.NET.Sdk">\n  <ItemGroup>\n    <PackageReference Include="Newtonsoft.Json" Version="13.0.1" />\n    <PackageReference Include="Serilog" />\n  </ItemGroup>\n</Project>\n',
 DT: '<Project Sdk="Microsoft.NET.Sdk">\n  <ItemGroup>\n    <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />\n    <PackageReference Include="xunit" Version="2.9.0" />\n  </ItemGroup>\n</Project>\n',
}
dn_expected = [
 entry("dotnet", "Newtonsoft.Json", "13.0.1", [RT, TR], [DA, DT], [(DA, "13.0.1", RT), (DT, "13.0.3", TR)], True),
 entry("dotnet", "Serilog", "*", [RT], [DA], [(DA, "*", RT)], True),
 entry("dotnet", "xunit", "2.9.0", [TR], [DT], [(DT, "2.9.0", TR)], False),
]
DC = "dn/Child/Child.csproj"
dnchild_files = {DC: '<Project Sdk="Microsoft.NET.Sdk">\n  <ItemGroup>\n    <PackageReference Include="Polly">\n      <Version>8.4.0</Version>\n    </PackageReference>\n  </ItemGroup>\n</Project>\n'}
dnchild_expected = [entry("dotnet", "Polly", "8.4.0", [RT], [DC], [(DC, "8.4.0", RT)], True)]
mixed_files = {"compose/Cargo.toml": '[dependencies]\nhidden = "1"\n', "temp/compose/x/package.json": json.dumps({"dependencies": {"hid": "1"}}), "target/Cargo.toml": '[dependencies]\nt = "1"\n', ".git/x/package.json": json.dumps({"dependencies": {"g": "1"}})}

def case(name, eco_oracle, files, expected, defect=None, note=""):
    d = {"name": name, "oracle": eco_oracle, "files": files, "expected": expected}
    if defect: d["defect"] = defect
    return d

freeze = [
 case("cargo manifests: workspace refs, internal names, proc-macro, dev/build/target tables, merge across manifests", "ts", cargo_files, cargo_expected_current),
 case("cargo table-form dependencies are inventoried", "ts", cargo_files_tabled, cargo_expected, defect="dependencyParseCargoToml ignores `[dependencies.<name>]` table-form sections (📇️inventory/🟦️.ts:175: section regex only matches `...dependencies` suffix)"),
 case("package.json sections, internal names, skipped trees, test-owned manifests", "ts", js_files, js_expected),
 case("go.mod requirements, indirect, replaces, workspace membership", "python", go_files, go_expected),
 case("pyproject multi-line arrays", "ts", py_files, py_expected),
 case("pyproject single-line dependency arrays", "ts", pyline_files, pyline_expected, defect="dependencyCollectPython (📇️inventory/🟦️.ts:889) only reads one requirement string per line, so `dependencies = [\"a\", \"b\"]` yields nothing"),
 case("csproj attribute versions and test projects", "python", dn_files, dn_expected),
 case("csproj child-element Version", "python", dnchild_files, dnchild_expected, defect="dependencyCollectDotnet regex only reads the Version attribute (📇️inventory/🟦️.ts:937), not a `<Version>` child element"),
 case("composition, target, git and empty manifests are ignored", "ts", mixed_files, []),
]
taxonomy = {"testsDirName": "tests", "testFixturesDirName": "fixtures", "testExamplesDirName": "examples", "testOraclesDirName": "oracles", "testProbeDirName": "probes", "testGeneratorDirName": "generators", "testDomainPath": "qa"}
taxpath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

def ent(kinds, users, decls, name="pkg", eco="js"):
    return {"ecosystem": eco, "name": name, "version": "1.0.0", "kinds": kinds, "users": users, "productionReachable": False, "declarations": [{"user": u, "version": "1.0.0", "kind": k} for u, k in decls]}
classify = [
 {"name": "no registry claim leaves the entry alone", "entry": ent([RT], ["a/package.json"], [("a/package.json", RT)]), "oracleIds": None, "expected": ent([RT], ["a/package.json"], [("a/package.json", RT)])},
 {"name": "only test-owned declarations become test-oracle with sorted ids", "entry": ent([TR], ["qa/x/package.json", "pkg/oracles/package.json"], [("qa/x/package.json", RT), ("pkg/oracles/package.json", BU)]), "oracleIds": ["z-id", "a-id"], "expected": dict(ent(["test-oracle"], ["qa/x/package.json", "pkg/oracles/package.json"], [("qa/x/package.json", RT), ("pkg/oracles/package.json", BU)]), oracleIds=["a-id", "z-id"])},
 {"name": "a production declaration is an oracle conflict", "entry": ent([RT, TR], ["a/package.json", "qa/x/package.json"], [("a/package.json", RT), ("qa/x/package.json", TR), ("b/package.json", BU)]), "oracleIds": ["o1"], "expected": dict(ent([RT, TR], ["a/package.json", "qa/x/package.json"], [("a/package.json", RT), ("qa/x/package.json", TR), ("b/package.json", BU)]), oracleIds=["o1"], oracleConflictUsers=["a/package.json", "b/package.json"])},
 {"name": "a repository-tooling declaration is not production", "entry": ent([TO], ["a/package.json"], [("a/package.json", TO)]), "oracleIds": ["o1"], "expected": dict(ent(["test-oracle"], ["a/package.json"], [("a/package.json", TO)]), oracleIds=["o1"])},
 {"name": "the interface owner manifest is exempt for its own ecosystem", "entry": ent([RT], [owner_manifest], [(owner_manifest, RT)], name="three"), "oracleIds": ["three-ref"], "expected": dict(ent(["test-oracle"], [owner_manifest], [(owner_manifest, RT)], name="three"), oracleIds=["three-ref"])},
 {"name": "the interface owner exemption does not cross ecosystems", "entry": ent([RT], [owner_manifest], [(owner_manifest, RT)], name="three", eco="rust"), "oracleIds": ["three-ref"], "expected": dict(ent([RT], [owner_manifest], [(owner_manifest, RT)], name="three", eco="rust"), oracleIds=["three-ref"], oracleConflictUsers=[owner_manifest])},
]
testdomain = [
 {"name": "every directory kind and the domain path", "taxonomy": taxonomy, "expected": {"directoryNames": ["tests", "fixtures", "examples", "oracles", "probes", "generators"], "domainPath": "qa"}},
 {"name": "a missing directory name is rejected", "taxonomy": {k: v for k, v in taxonomy.items() if k != "testProbeDirName"}, "expected": None},
 {"name": "an empty directory name is rejected", "taxonomy": dict(taxonomy, testsDirName=""), "expected": None},
 {"name": "a missing domain path is rejected", "taxonomy": {k: v for k, v in taxonomy.items() if k != "testDomainPath"}, "expected": None},
]
composition = [{"path": p, "expected": e} for p, e in [("compose/a/package.json", True), ("temp/compose/a/Cargo.toml", True), ("pkgs/compose/a.json", False), ("composer/a", False), ("compose", False), ("a/temp/compose/x", False)]]
gomod = [
 {"name": "block and single-line requires, indirect, local and remote replaces", "content": "module example.com/m\n\nrequire (\n\ta.io/b v1.0.0\n\tc.io/d v2.0.0 // indirect\n)\nrequire e.io/f v3.0.0\n\nreplace (\n\ta.io/b => ../b\n\tc.io/d v2.0.0 => example.org/d v2.1.0\n\tg.io/h => C:\\\\work\\\\h\n)\nreplace i.io/j => /abs/j\nreplace k.io/l v1.0.0 => ./l\n", "expected": {"module": "example.com/m", "localReplaces": ["a.io/b", "g.io/h", "i.io/j", "k.io/l"], "requirements": [{"name": "a.io/b", "version": "v1.0.0", "kind": RT}, {"name": "c.io/d", "version": "v2.0.0", "kind": BU}, {"name": "e.io/f", "version": "v3.0.0", "kind": RT}]}},
 {"name": "an empty file has no module", "content": "", "expected": {"localReplaces": [], "requirements": []}},
 {"name": "CRLF line endings", "content": "module x.io/m\r\nrequire (\r\n\ta.io/b v1.0.0\r\n)\r\n", "expected": {"module": "x.io/m", "localReplaces": [], "requirements": [{"name": "a.io/b", "version": "v1.0.0", "kind": RT}]}},
]
locks = [
 {"name": "every mismatch kind in deterministic order", "manifests": [["package.json", {"dependencies": {"a": "1", "b": "2"}, "devDependencies": {"c": "3"}}], ["pkgs/x/package.json", {"dependencies": {"d": "1"}}], ["pkgs/y/package.json", {}]], "workspaces": {"": {"dependencies": {"a": "1", "b": "3", "e": "9"}}, "pkgs/x": {"dependencies": {"d": "1"}, "devDependencies": {"z": "1"}}},
  "expected": [
   {"dependency": "b", "kind": "version-mismatch", "lockVersion": "3", "manifest": "package.json", "manifestVersion": "2", "section": "dependencies"},
   {"dependency": "e", "kind": "stale-in-lock", "lockVersion": "9", "manifest": "package.json", "section": "dependencies"},
   {"dependency": "c", "kind": "missing-in-lock", "manifest": "package.json", "manifestVersion": "3", "section": "devDependencies"},
   {"dependency": "z", "kind": "stale-in-lock", "lockVersion": "1", "manifest": "pkgs/x/package.json", "section": "devDependencies"},
   {"kind": "workspace-missing", "manifest": "pkgs/y/package.json"}]},
 {"name": "backslash manifest paths address the posix workspace key", "manifests": [["pkgs\\x\\package.json", {"dependencies": {"d": "1"}}]], "workspaces": {"pkgs\\x": {"dependencies": {"d": "1"}}}, "expected": [{"kind": "workspace-missing", "manifest": "pkgs\\x\\package.json"}]},
 {"name": "a non-object lock snapshot makes every workspace missing", "manifests": [["package.json", {}]], "workspaces": None, "expected": [{"kind": "workspace-missing", "manifest": "package.json"}]},
 {"name": "peer and optional sections are compared too", "manifests": [["package.json", {"peerDependencies": {"p": "^1"}, "optionalDependencies": {"o": "^2"}}]], "workspaces": {"": {"peerDependencies": {"p": "^2"}, "optionalDependencies": {}}}, "expected": [{"dependency": "o", "kind": "missing-in-lock", "manifest": "package.json", "manifestVersion": "^2", "section": "optionalDependencies"}, {"dependency": "p", "kind": "version-mismatch", "lockVersion": "^2", "manifest": "package.json", "manifestVersion": "^1", "section": "peerDependencies"}]},
]
freeze_errors = [{"name": "a Cargo manifest outside every workspace fails the freeze", "files": {"orphan/Cargo.toml": '[package]\nname = "o"\n'}, "message": "Cargo package has no workspace: orphan/Cargo.toml"}]
corpus = {"version": 1, "freezeErrors": freeze_errors, "taxonomyPath": taxpath, "taxonomy": taxonomy, "freeze": freeze, "classify": classify, "testDomain": testdomain, "composition": composition, "goModule": gomod, "lockMismatches": locks}

decl = S.obj({"user": S.str, "version": S.str, "kind": S.enum(RT, BU, TR, TO, "test-oracle")})
kinds = S.arr(S.enum(RT, BU, TR, TO, "test-oracle"))
entry_s = S.obj({"ecosystem": S.enum("rust", "js", "go", "python", "dotnet"), "name": S.str, "version": S.str, "kinds": kinds, "users": S.strs(), "productionReachable": S.bool, "declarations": S.arr(decl)})
entry_c = S.obj({"ecosystem": S.enum("rust", "js", "go", "python", "dotnet"), "name": S.str, "version": S.str, "kinds": kinds, "users": S.strs(), "productionReachable": S.bool, "declarations": S.arr(decl), "oracleIds": S.strs(), "oracleConflictUsers": S.strs()}, required=["ecosystem", "name", "version", "kinds", "users", "productionReachable", "declarations"])
mm = S.obj({"dependency": S.str, "kind": S.enum("invalid-lockfile", "missing-in-lock", "stale-in-lock", "version-mismatch", "workspace-missing"), "lockVersion": S.str, "manifest": S.str, "manifestVersion": S.str, "section": S.str}, required=["kind", "manifest"])
req = S.obj({"name": S.str, "version": S.str, "kind": S.enum(RT, BU)})
schema = S.obj({
 "version": S.const(1), "taxonomyPath": S.str, "taxonomy": S.map(S.str),
 "freeze": S.arr(S.obj({"name": S.str, "oracle": S.enum("ts", "python"), "files": S.map(S.str), "expected": S.arr(entry_s), "defect": S.str}, required=["name", "oracle", "files", "expected"]), 1),
 "freezeErrors": S.arr(S.obj({"name": S.str, "files": S.map(S.str), "message": S.str}), 1),
 "classify": S.arr(S.obj({"name": S.str, "entry": entry_s, "oracleIds": S.one(S.strs(), S.null), "expected": entry_c}), 1),
 "testDomain": S.arr(S.obj({"name": S.str, "taxonomy": S.map(S.str), "expected": S.one(S.obj({"directoryNames": S.strs(), "domainPath": S.str}), S.null)}), 1),
 "composition": S.arr(S.obj({"path": S.str, "expected": S.bool}), 1),
 "goModule": S.arr(S.obj({"name": S.str, "content": S.str, "expected": S.obj({"module": S.str, "localReplaces": S.strs(), "requirements": S.arr(req)}, required=["localReplaces", "requirements"])}), 1),
 "lockMismatches": S.arr(S.obj({"name": S.str, "manifests": S.arr(S.arr(S.anyv)), "workspaces": S.anyv, "expected": S.arr(mm)}), 1),
})

TS = r'''/** 📇️ Contract: the dependency inventory reads Cargo, package.json, go.mod, pyproject and csproj manifests of a synthetic repository into one baseline, classifies oracle declarations and audits the Bun lock against manifests; the corpus is language-agnostic, `@iarna/toml` and JSON model the Cargo, pyproject and package.json inventories independently, and `@@O@@` models Go, .NET, lock audit, classification and the test domain. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "@@TOOLCHAIN@@";
import {DEPENDENCY_ECOSYSTEMS,dependencyClassifyOracleEntry,dependencyFreezeCurrentThirdParty,dependencyIsCompositionManifest,dependencyJsLockWorkspaceMismatches,dependencyParseGoModule,dependencyTestDomain} from "@@UNIT@@";
import corpus from "@@CI@@";
import schema from "@@SI@@";

type Row=Record<string,any>;
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),clean=(value:unknown):any=>JSON.parse(JSON.stringify(value));
const materialize=(files:Record<string,string>):string=>{
 const root=mkdtempSync(join(tmpdir(),"inventory-"));
 const all:Record<string,string>={...files,[corpus.taxonomyPath]:JSON.stringify(corpus.taxonomy)};
 for(const[path,text]of Object.entries(all)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};
const real=(files:Record<string,string>):Row[]=>{const root=materialize(files);try{return clean(dependencyFreezeCurrentThirdParty(root));}finally{rmSync(root,{recursive:true,force:true});}};
const testOwned=(path:string):boolean=>path.startsWith(corpus.taxonomy.testDomainPath+"/")||path.split("/").slice(0,-1).some(part=>Object.values(corpus.taxonomy).includes(part));
const kindOf=(section:string,procMacro:boolean):string=>section==="dev-dependencies"?"test-runner":section==="build-dependencies"||procMacro?"production-build":"production-runtime";
const internalRust=(name:string,table:any,workspace:Row):boolean=>name.startsWith("semio-")||name.startsWith("semio_")||name==="ports"||/^db(?:_[a-z0-9]+)*$/.test(name)||(typeof table==="object"&&table!==null&&"path" in table)||(typeof table==="object"&&table!==null&&table.workspace===true&&typeof workspace[name]==="object"&&"path" in workspace[name]);
const versionOf=(table:any,workspace:Row,name:string):string=>{
 const own=typeof table==="string"?table:table?.version;
 if(typeof table==="object"&&table?.workspace===true){const shared=workspace[name];return typeof shared==="string"?shared:shared?.version??"*";}
 return own??"*";
};
const rows=(entries:[string,string,string,string,string][]):Row[]=>{
 const byKey=new Map<string,Row>();
 for(const[eco,name,version,kind,user]of entries){
  const declared=testOwned(user)&&(kind==="production-runtime"||kind==="production-build")?"test-runner":kind,key=eco+":"+name,found=byKey.get(key);
  if(found){if(!found.kinds.includes(declared))found.kinds.push(declared);if(!found.users.includes(user))found.users.push(user);if(!found.declarations.some((row:Row)=>row.user===user&&row.version===version&&row.kind===declared))found.declarations.push({user,version,kind:declared});continue;}
  byKey.set(key,{ecosystem:eco,name,version,kinds:[declared],users:[user],productionReachable:false,declarations:[{user,version,kind:declared}]});
 }
 for(const row of byKey.values()){row.declarations.sort((a:Row,b:Row)=>a.user.localeCompare(b.user)||a.kind.localeCompare(b.kind)||a.version.localeCompare(b.version));row.kinds.sort();row.users.sort();row.productionReachable=row.kinds.some((kind:string)=>kind==="production-runtime"||kind==="production-build");}
 return[...byKey.values()].sort((a,b)=>a.ecosystem===b.ecosystem?a.name.localeCompare(b.name):a.ecosystem.localeCompare(b.ecosystem));
};
const modelCargoJsPython=(files:Record<string,string>):Row[]=>{
 const found:[string,string,string,string,string][]=[],skipped=(path:string)=>path.split("/").some(part=>["compose","node_modules",".git","target"].includes(part)),paths=Object.keys(files).filter(path=>!skipped(path)).sort();
 const workspace:Row={};
 for(const path of paths)if(path==="Cargo.toml")Object.assign(workspace,(files[path]?parse(files[path]!):{}).workspace?.dependencies??{});
 const names=new Set<string>();
 for(const path of paths)if(path.endsWith("package.json")){try{const name=JSON.parse(files[path]!).name;if(name)names.add(name);}catch{}}
 for(const path of paths){
  const text=files[path]!;
  if(path.endsWith("Cargo.toml")&&text){
   const document:Row=parse(text),procMacro=document.lib?.["proc-macro"]===true,tables:[string,Row][]=[];
   for(const section of["dependencies","dev-dependencies","build-dependencies"]){if(document[section])tables.push([section,document[section]]);for(const target of Object.values<Row>(document.target??{}))if(target[section])tables.push([section,target[section]]);}
   for(const[section,table]of tables)for(const[name,value]of Object.entries<any>(table))if(!internalRust(name,value,workspace))found.push(["rust",name,versionOf(value,workspace,name),kindOf(section,procMacro&&section==="dependencies"),path]);
  }
  if(path.endsWith("package.json")){
   let document:Row;try{document=JSON.parse(text);}catch{continue;}
   for(const[section,kind]of[["dependencies","production-runtime"],["peerDependencies","production-runtime"],["optionalDependencies","production-runtime"],["devDependencies","repository-tooling"]]as const)for(const[name,version]of Object.entries<any>(document[section]??{}))if(!(names.has(name)||name.startsWith("@semio-tech/")||/^(workspace|file|link):/.test(String(version))))found.push(["js",name,String(version),kind,path]);
  }
  if(path.endsWith("pyproject.toml")&&text){
   const document:Row=parse(text),groups:[string,string[]][]=[["production-runtime",document.project?.dependencies??[]]];
   for(const[group,list]of Object.entries<string[]>({...(document.project?.["optional-dependencies"]??{}),...(document["dependency-groups"]??{})}))if(["dev","test","lint","docs"].includes(group))groups.push(["test-runner",list]);
   for(const[kind,list]of groups)for(const requirement of list){const match=/^([A-Za-z0-9._-]+)\s*(.*)$/.exec(requirement)!;if(!match[1]!.startsWith("semio"))found.push(["python",match[1]!,match[2]!.trim()||"*",kind,path]);}
  }
 }
 return rows(found);
};

test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the ecosystems the freeze tracks are the five shipped ones",()=>{expect([...DEPENDENCY_ECOSYSTEMS]).toEqual(["rust","js","go","python","dotnet"]);});
for(const row of corpus.freeze){
 const defect=(row as Row).defect!==undefined,run=defect?test.failing:test;
 run("freeze: "+row.name,()=>{expect(real(row.files)).toEqual(row.expected);});
 if(row.oracle==="ts")test("independent TOML/JSON model: "+row.name,()=>{expect(modelCargoJsPython(row.files)).toEqual(row.expected);});
}
for(const row of corpus.freezeErrors)test("freeze fails: "+row.name,()=>{expect(()=>real(row.files)).toThrow(row.message);});
for(const row of corpus.classify)test("classify: "+row.name,()=>{
 const entry=clean(row.entry);dependencyClassifyOracleEntry(entry,row.oracleIds??undefined,dependencyTestDomain(corpus.taxonomy));expect(entry).toEqual(row.expected);
});
for(const row of corpus.testDomain)test("test domain: "+row.name,()=>{
 if(row.expected===null)expect(()=>dependencyTestDomain(row.taxonomy)).toThrow("[dependencies]");
 else expect(clean(dependencyTestDomain(row.taxonomy))).toEqual(row.expected);
});
for(const row of corpus.composition)test("composition: "+row.path,()=>{expect(dependencyIsCompositionManifest(row.path)).toBe(row.expected);});
for(const row of corpus.goModule)test("go.mod: "+row.name,()=>{expect(clean(dependencyParseGoModule(row.content))).toEqual(row.expected);});
for(const row of corpus.lockMismatches)test("lock audit: "+row.name,()=>{expect(clean(dependencyJsLockWorkspaceMismatches(new Map(row.manifests as [string,unknown][]),row.workspaces))).toEqual(row.expected);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"@@O@@"),join(here,"@@C@@")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
'''
ORACLE = r'''#!/usr/bin/env python3
"""🐍️ Independent model of the dependency inventory for Go, .NET, classification, the test domain, the composition rule and the Bun lock audit: reads the same corpus and shares no code with the Bun test (stdlib only: json, re, xml.etree).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys
import xml.etree.ElementTree as ET

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
RT, BU, TR, TO = "production-runtime", "production-build", "test-runner", "repository-tooling"
tax = corpus["taxonomy"]
dirs = [tax[k] for k in ("testsDirName", "testFixturesDirName", "testExamplesDirName", "testOraclesDirName", "testProbeDirName", "testGeneratorDirName")]


def owned(path):
    return path.startswith(tax["testDomainPath"] + "/") or any(p in dirs for p in path.split("/")[:-1])


def go_dirs(work):
    found = []
    inblock = False
    for raw in work.splitlines():
        line = raw.strip()
        if inblock:
            if line == ")":
                inblock = False
            elif line and not line.startswith("//"):
                found.append(line)
        elif re.match(r"use\s*\($", line):
            inblock = True
        else:
            m = re.match(r"use\s+(\S+)$", line)
            if m:
                found.append(m.group(1))
    stripped = {d[2:] if d.startswith("./") else d for d in found}
    return sorted(d for d in stripped if d not in ("", "compose") and not d.startswith("compose/"))


def go_module(text):
    mod, replaces, reqs, block = None, set(), [], None
    for raw in text.splitlines():
        line = raw.strip()
        m = re.match(r"module\s+(\S+)$", line)
        if m:
            mod = m.group(1)
        if re.match(r"(require|replace)\s*\($", line):
            block = line.split()[0]
            continue
        if block and line == ")":
            block = None
            continue
        rq = line if block == "require" else line[len("require "):] if line.startswith("require ") else ""
        m = re.match(r"(\S+)\s+(\S+)(\s*//\s*indirect)?", rq)
        if m:
            reqs.append({"name": m.group(1), "version": m.group(2), "kind": BU if m.group(3) else RT})
        rp = line if block == "replace" else line[len("replace "):] if line.startswith("replace ") else ""
        m = re.match(r"(\S+)(?:\s+v\S+)?\s+=>\s+(\S+)", rp)
        if m and re.match(r"(\.{1,2}[\\/]|[\\/]|[A-Za-z]:[\\/])", m.group(2)):
            replaces.add(m.group(1))
    out = {"localReplaces": sorted(replaces), "requirements": reqs}
    if mod:
        out["module"] = mod
    return out


def merge(records):
    by = {}
    for eco, name, version, kind, user in records:
        kind = TR if owned(user) and kind in (RT, BU) else kind
        e = by.setdefault((eco, name), {"ecosystem": eco, "name": name, "version": version, "kinds": [], "users": [], "productionReachable": False, "declarations": []})
        if kind not in e["kinds"]:
            e["kinds"].append(kind)
        if user not in e["users"]:
            e["users"].append(user)
        d = {"user": user, "version": version, "kind": kind}
        if d not in e["declarations"]:
            e["declarations"].append(d)
    for e in by.values():
        e["declarations"].sort(key=lambda d: (d["user"], d["kind"], d["version"]))
        e["kinds"].sort()
        e["users"].sort()
        e["productionReachable"] = any(k in (RT, BU) for k in e["kinds"])
    return sorted(by.values(), key=lambda e: (e["ecosystem"], e["name"].lower(), e["name"]))


def go_freeze(files):
    work = files.get("go.work", "")
    dirs_ = go_dirs(work)
    internal = set(go_module(work)["localReplaces"])
    for d in dirs_:
        mod = go_module(files.get(d + "/go.mod", ""))
        internal |= set(mod["localReplaces"]) | ({mod["module"]} if "module" in mod else set())
    records = []
    for d in dirs_:
        path = d + "/go.mod"
        for r in go_module(files.get(path, ""))["requirements"]:
            if "." not in r["name"] or r["name"].startswith("semio.tech/") or r["name"] in internal:
                continue
            records.append(("go", r["name"], r["version"], r["kind"], path))
    return merge(records)


def dotnet_freeze(files):
    records = []
    for path in sorted(files):
        if not path.endswith(".csproj") or "compose" in path.split("/"):
            continue
        root = ET.fromstring(files[path])
        refs = [(r.get("Include"), r.get("Version") if r.get("Version") is not None else (r.findtext("Version") or "*")) for r in root.iter("PackageReference")]
        is_test = (root.findtext(".//IsTestProject") or "").strip().lower() == "true" or any(n.startswith(("xunit", "Microsoft.NET.Test", "NUnit")) for n, _ in refs)
        for name, version in refs:
            records.append(("dotnet", name, version, TR if is_test else RT, path))
    return merge(records)


for case in corpus["freeze"]:
    if case["oracle"] != "python":
        continue
    got = go_freeze(case["files"]) if any(p.endswith("go.mod") or p == "go.work" for p in case["files"]) else dotnet_freeze(case["files"])
    if got != case["expected"]:
        failures.append("freeze: " + case["name"])

for case in corpus["goModule"]:
    if go_module(case["content"]) != case["expected"]:
        failures.append("go.mod: " + case["name"])

for case in corpus["composition"]:
    p = case["path"]
    if (p.startswith("compose/") or p.startswith("temp/compose/")) != case["expected"]:
        failures.append("composition: " + p)

for case in corpus["testDomain"]:
    t = case["taxonomy"]
    keys = ("testsDirName", "testFixturesDirName", "testExamplesDirName", "testOraclesDirName", "testProbeDirName", "testGeneratorDirName")
    valid = all(isinstance(t.get(k), str) and t.get(k) for k in keys) and isinstance(t.get("testDomainPath"), str)
    if valid != (case["expected"] is not None) or (valid and case["expected"] != {"directoryNames": [t[k] for k in keys], "domainPath": t["testDomainPath"]}):
        failures.append("testDomain: " + case["name"])

OWNER = corpus["classify"][4]["entry"]["users"][0]
for case in corpus["classify"]:
    e = json.loads(json.dumps(case["entry"]))
    ids = case["oracleIds"]
    if ids is not None:
        e["oracleIds"] = sorted(ids)
        owner = e["name"] == "three" and e["ecosystem"] == "js"
        prod = [d for d in e["declarations"] if not owned(d["user"]) and d["kind"] in (RT, BU) and not (owner and d["user"] == OWNER)]
        if prod:
            e["oracleConflictUsers"] = sorted({d["user"] for d in prod})
        else:
            e["kinds"] = ["test-oracle"]
    if e != case["expected"]:
        failures.append("classify: " + case["name"])

SECTIONS = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"]
for case in corpus["lockMismatches"]:
    ws = case["workspaces"] if isinstance(case["workspaces"], dict) else {}
    out = []
    for manifest, value in case["manifests"]:
        norm = manifest.replace("\\", "/")
        key = "" if norm == "package.json" else re.sub(r"/package\.json$", "", norm)
        lock = ws.get(key)
        if not isinstance(lock, dict):
            out.append({"kind": "workspace-missing", "manifest": manifest})
            continue
        for section in SECTIONS:
            mt, lt = value.get(section) or {}, lock.get(section) or {}
            for dep in sorted(set(mt) | set(lt)):
                row = {"dependency": dep, "manifest": manifest, "section": section}
                if dep not in mt:
                    row.update(kind="stale-in-lock", lockVersion=str(lt[dep]))
                elif dep not in lt:
                    row.update(kind="missing-in-lock", manifestVersion=str(mt[dep]))
                elif str(mt[dep]) != str(lt[dep]):
                    row.update(kind="version-mismatch", lockVersion=str(lt[dep]), manifestVersion=str(mt[dep]))
                else:
                    continue
                out.append(row)
    out.sort(key=lambda r: (r["manifest"], r.get("section", ""), r.get("dependency", ""), r["kind"]))
    if out != case["expected"]:
        failures.append("lock: " + case["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
'''
ts = TS.replace("@@TOOLCHAIN@@", "@@TCH@@").replace("@@UNIT@@", "@@" + UNIT + "/🟦.ts@@")
ts = ts.replace("@@TCH@@", lib.toolchain_import(UNIT))
tdir = lib.write_unit(UNIT, corpus, schema, ts, ORACLE)
print(tdir)

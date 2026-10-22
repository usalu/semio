import json, lib
from lib import S
V = "️"
# ---------------- allowlist
UA = "🚨policy/🔒allowlist"
FN = "🔒" + V + "policy-allowlist.json"
skip = ["node_modules", ".git", ".nx", ".venv", "target", "dist", "build", "out", "__pycache__", "obj", "bin", "storybook-static", "🎫" + V + "tickets", "⚡" + V + "cache", "🤖" + V + "generated", "🗑" + V + "generated"]
files = {FN: json.dumps({"rule": ["root-a", "shared"], "other": ["o1"]}), "area/x/" + FN: json.dumps({"rule": ["x-b", "shared"]}), "area/y/" + FN: json.dumps({"other": ["o2"]}), "area/bad/" + FN: "{not json", "area/nested/deep/" + FN: json.dumps({"rule": ["deep"]}),
         "area/typed/" + FN: json.dumps({"rule": 5}), "area/empty/" + FN: json.dumps({})}
for s in skip:
    files["area/" + s + "/" + FN] = json.dumps({"rule": ["skipped-" + s]})
files["area/other.json"] = json.dumps({"rule": ["wrong-file-name"]})
allow = {"rule": ["deep", "root-a", "shared", "x-b"], "other": ["o1", "o2"], "absent": []}
corpus_a = {"version": 1, "filename": FN, "skipped": skip, "files": files, "expected": allow, "defects": [{"name": "a string value is not a list of entries", "files": {FN: json.dumps({"rule": "abc"})}, "key": "rule", "expected": [], "defect": "🔒️allowlist/🟦️.ts:policyDiscoveredAllowlist iterates `parsed[key]` with for-of without checking for an array, so a string value contributes its characters (`abc` → a, b, c) instead of failing closed"}]}
schema_a = S.obj({"version": S.const(1), "filename": S.str, "skipped": S.strs(1), "files": S.map(S.str), "expected": S.map(S.strs()), "defects": S.arr(S.obj({"name": S.str, "files": S.map(S.str), "key": S.str, "expected": S.strs(), "defect": S.str}), 1)})
body_a = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const build=(files:Record<string,string>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"allowlist-")));roots.push(root);for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
test("the contribution filename is the documented one",()=>{expect(POLICY_ALLOWLIST_FILENAME).toBe(corpus.filename);});
test("every area contributes to a rule, skipped trees, unreadable files, wrong types and other names contribute nothing",()=>{
 const root=build(corpus.files);for(const[key,expected]of Object.entries(corpus.expected))expect([...policyDiscoveredAllowlist(root,key)].sort(),key).toEqual(expected);
});
test("a missing repository root contributes nothing",()=>{expect(policyDiscoveredAllowlist(join(tmpdir(),"definitely-missing-"+process.pid),"rule").size).toBe(0);});
for(const row of corpus.defects)test.failing("allowlist defect: "+row.name,()=>{expect([...policyDiscoveredAllowlist(build(row.files),row.key)]).toEqual(row.expected);});'''
ts_a = lib.ts_wrap(UA, "🔒️ Contract: a policy rule's allowlist is the union of the entries every area contributes through its own allowlist file, discovered by convention while build output, caches, tickets and generated trees are skipped and unreadable or mistyped files fail closed; the corpus is language-agnostic and `@@O@@` walks the same tree.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";\nimport {POLICY_ALLOWLIST_FILENAME,policyDiscoveredAllowlist} from "@@' + UA + '/🟦.ts@@";', body_a)
orc_a = lib.PY_HEAD.format(doc="🐍️ Independent model of the allowlist discovery: os.walk over the rebuilt tree with the same skip list (stdlib only).") + r'''import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="allowlist-oracle-")
try:
    for p, t in corpus["files"].items():
        f = os.path.join(root, *p.split("/"))
        os.makedirs(os.path.dirname(f), exist_ok=True)
        open(f, "w", encoding="utf8").write(t)
    skip = set(corpus["skipped"])
    found = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in skip]
        if corpus["filename"] in fn:
            try:
                doc = json.load(open(os.path.join(dp, corpus["filename"]), encoding="utf8"))
            except ValueError:
                continue
            for k, v in doc.items():
                if isinstance(v, list):
                    found.setdefault(k, set()).update(x for x in v if isinstance(x, str))
    for k, want in corpus["expected"].items():
        if sorted(found.get(k, [])) != want:
            failures.append("allowlist " + k)
finally:
    shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UA, corpus_a, schema_a, ts_a, orc_a)
# ---------------- cli
UC = "🚨policy/🖥cli"
items = [({"items": [1, 2]}, [1, 2]), ({"items": []}, []), ({"items": "x"}, []), ({}, []), (None, []), ("s", []), ([1], []), (7, [])]
fake = {"ok": {"lines": ['noise', '{"data":{"a":1}}', '{"data":{"a":2}}'], "exit": 0, "expected": {"a": 2}}, "payloadWithoutData": {"lines": ['{"x":1}'], "exit": 0, "expected": {"x": 1}}, "graphqlErrors": {"lines": ['{"errors":[{"message":"boom"},{"message":"bang"}]}'], "exit": 0, "message": "[repo/cli] graphql errors: boom; bang"},
        "noJson": {"lines": ["plain", "text"], "exit": 0, "message": "[repo/cli] no JSON lines in stdout: plain"}, "exitCode": {"lines": [], "exit": 3, "stderr": "bad things", "message": "[repo/cli] exit 3: bad things"}, "arrayLast": {"lines": ['{"data":1}', '[1,2]'], "exit": 0, "expected": [1, 2]}}
corpus_c = {"version": 1, "items": [{"node": a, "expected": b} for a, b in items], "bins": {"envCli": "REPO_CLI_BIN", "envMcp": "REPO_MCP_BIN"}, "fake": fake, "defaultSuffix": [".🧬semio", "🦑" + V + "repo", "⚡" + V + "cache", "🗃" + V + "bin"], "modes": ["prototype", "release"], "badMode": "debug"}
fk = S.obj({"lines": S.strs(), "exit": S.int, "expected": S.anyv, "message": S.str, "stderr": S.str}, required=["lines", "exit"])
schema_c = S.obj({"version": S.const(1), "items": S.arr(S.obj({"node": S.anyv, "expected": S.arr(S.anyv)}), 1), "bins": S.obj({"envCli": S.str, "envMcp": S.str}), "fake": S.map(fk), "defaultSuffix": S.strs(1), "modes": S.strs(2), "badMode": S.str})
body_c = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"cli-")));roots.push(root);return root;};
const withEnv=<T,>(values:Record<string,string|undefined>,action:()=>T):T=>{const saved:Record<string,string|undefined>={};for(const key of Object.keys(values)){saved[key]=process.env[key];if(values[key]===undefined)delete process.env[key];else process.env[key]=values[key];}try{return action();}finally{for(const[key,value]of Object.entries(saved))if(value===undefined)delete process.env[key];else process.env[key]=value;}};
const executable=(root:string,spec:{lines:string[];exit:number;stderr?:string}):string=>{const path=join(root,"fake-repo");writeFileSync(path,"#!/usr/bin/env node\\n"+"const lines="+JSON.stringify(spec.lines)+";for(const line of lines)console.log(line);if("+spec.exit+")process.stderr.write("+JSON.stringify(spec.stderr??"")+");process.exit("+spec.exit+");\\n");chmodSync(path,0o755);return path;};
for(const[index,row]of corpus.items.entries())test("fixture items of node "+index,()=>{expect(fixtureItemsOf(row.node)).toEqual(row.expected);});
test("the binaries resolve from the environment, else from the cache by build mode",()=>{
 const root=scratch(),directory=join(root,...corpus.defaultSuffix);
 withEnv({REPO_CLI_BIN:undefined,REPO_MCP_BIN:undefined,SEMIO_BUILD_MODE:undefined},()=>{
  for(const mode of corpus.modes as ("prototype"|"release")[]){const expected=join(directory,mode,"repo","repo"+(process.platform==="win32"?".exe":""));expect(resolveCliBin(root,mode)).toBe(expected);expect(resolveMcpBin(root,mode)).toBe(expected);}
  expect(resolveCliBin(root)).toBe(join(directory,"prototype","repo","repo"+(process.platform==="win32"?".exe":"")));expect(()=>resolveCliBin(root,corpus.badMode as never)).toThrow("Build mode must be prototype or release");
 });
 withEnv({REPO_CLI_BIN:"  "+join(root,"custom-cli")+"  ",REPO_MCP_BIN:"relative/mcp"},()=>{expect(resolveCliBin(root,"release")).toBe(join(root,"custom-cli"));expect(resolveMcpBin(root,"release")).toBe(resolve("relative/mcp"));});
 withEnv({REPO_CLI_BIN:"   ",REPO_MCP_BIN:""},()=>{expect(resolveCliBin(root,"release")).toBe(join(directory,"release","repo","repo"+(process.platform==="win32"?".exe":"")));});
});
test("the library root is the neighbouring packages directory",()=>{expect(getLibRoot().endsWith("\\u{1F4E6}\\ufe0fpackages")).toBe(true);});
for(const[name,spec]of Object.entries(corpus.fake))test("graphql through the CLI: "+name,()=>{
 if(process.platform==="win32")return;
 const root=scratch(),bin=executable(root,spec as never);
 withEnv({REPO_CLI_BIN:bin},()=>{const run=()=>runCliGraphql("query { x }",{v:1},{repoRoot:root,cwd:root});const row=spec as {expected?:unknown;message?:string};if(row.message!==undefined)expect(run).toThrow(row.message);else expect(run()).toEqual(row.expected);});
});
test("the query, variables and repository are passed as arguments",()=>{
 if(process.platform==="win32")return;
 const root=scratch(),path=join(root,"echo-repo");writeFileSync(path,"#!/usr/bin/env node\\nconsole.log(JSON.stringify({data:{argv:process.argv.slice(2),cwd:process.cwd()}}));\\n");chmodSync(path,0o755);
 withEnv({REPO_CLI_BIN:path},()=>{const data=runCliGraphql("query Q { y }",{a:1},{repoRoot:root,cwd:root}) as {argv:string[];cwd:string};expect(data.argv).toEqual(["--repo",root,"--json","graphql","--query","query Q { y }","-v",'{"a":1}']);expect(realpathSync(data.cwd)).toBe(root);});
});'''
ts_c = lib.ts_wrap(UC, "🖥️ Contract: the repo CLI client resolves its binaries from the environment or the cache by build mode, passes the query and variables as one closed argument vector and returns the last JSON line's `data`, failing with a fixed message on a non-zero exit, GraphQL errors or no JSON; the corpus is language-agnostic and `@@O@@` re-derives the last-line and error rules.",
 'import {afterAll} from "bun:test";\nimport {chmodSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {join,resolve} from "node:path";\nimport {fixtureItemsOf,getLibRoot,resolveCliBin,resolveMcpBin,runCliGraphql} from "@@' + UC + '/🟦.ts@@";', body_c)
orc_c = lib.PY_HEAD.format(doc="🐍️ Independent model of the CLI output contract: last JSON line wins, GraphQL errors fail, no JSON fails (stdlib only).") + r'''
def payload(lines):
    last = None
    for l in lines:
        l = l.strip()
        if not l:
            continue
        try:
            last = json.loads(l)
        except ValueError:
            continue
    if not isinstance(last, (dict, list)):
        raise ValueError("no JSON lines")
    if isinstance(last, list):
        return last
    if last.get("errors"):
        raise ValueError("graphql errors: " + "; ".join(e["message"] for e in last["errors"]))
    return last.get("data", last)


for name, spec in corpus["fake"].items():
    if spec["exit"] != 0:
        continue
    try:
        got = payload(spec["lines"])
        if "message" in spec or got != spec["expected"]:
            failures.append("fake " + name)
    except ValueError as e:
        if "message" not in spec or str(e) not in spec["message"]:
            failures.append("fake " + name)
for c in corpus["items"]:
    node = c["node"]
    got = node["items"] if isinstance(node, dict) and isinstance(node.get("items"), list) else []
    if got != c["expected"]:
        failures.append("items")
''' + lib.PY_TAIL
lib.write_unit(UC, corpus_c, schema_c, ts_c, orc_c)
# ---------------- entrypoints
UE = "🚨policy/🧭entrypoints"
corpus_e = {"version": 1, "dispatch": [{"segments": [], "key": "run", "default": "run", "calls": [], "usage": False}, {"segments": ["a", "x", "y"], "key": "a", "default": None, "calls": ["x", "y"], "usage": False}, {"segments": [], "key": "run", "default": "run", "calls": [], "usage": False}, {"segments": ["zzz"], "key": None, "default": None, "calls": None, "usage": True}],
            "usageText": "tool <a|b>", "defects": [{"segments": ["toString"], "usage": True, "defect": "🧭️entrypoints/🟦️.ts:dispatchSubcommand looks the handler up with `handlers[key]` on a plain object, so an inherited name (`toString`, `constructor`, `hasOwnProperty`) is called as a handler instead of printing the usage line"}, {"segments": ["constructor"], "usage": True, "defect": "same"}], "policy": [{"segments": [], "expected": False}, {"segments": ["test"], "expected": False}, {"segments": ["Policy"], "expected": False}]}
schema_e = S.obj({"version": S.const(1), "dispatch": S.arr(S.obj({"segments": S.strs(), "key": S.one(S.str, S.null), "default": S.one(S.str, S.null), "calls": S.one(S.strs(), S.null), "usage": S.bool}), 1), "usageText": S.str, "defects": S.arr(S.obj({"segments": S.strs(), "usage": S.bool, "defect": S.str}), 1), "policy": S.arr(S.obj({"segments": S.strs(), "expected": S.bool}), 1)})
body_e = '''class Exit extends Error{constructor(readonly code:number){super("exit "+code);}}
const guarded=<T,>(action:()=>T):{value?:T;errors:string[];exit:number|null}=>{const errors:string[]=[],error=console.error,exit=process.exit;let code:number|null=null,value:T|undefined;console.error=(...parts:unknown[])=>{errors.push(parts.join(" "));};(process as unknown as {exit:(c?:number)=>never}).exit=(c?:number)=>{throw new Exit(c??0);};try{value=action();}catch(thrown){if(thrown instanceof Exit)code=thrown.code;else throw thrown;}finally{console.error=error;process.exit=exit;}return{value,errors,exit:code};};
for(const[index,row]of corpus.dispatch.entries())test("subcommand dispatch "+index+": "+JSON.stringify(row.segments),()=>{
 const calls:Record<string,string[]|undefined>={};const handlers:Record<string,(rest:string[])=>void>={a:rest=>{calls.a=rest;},run:rest=>{calls.run=rest;}};
 const outcome=guarded(()=>dispatchSubcommand(row.segments,handlers,corpus.usageText,row.default??undefined));
 if(row.usage){expect(outcome.exit).toBe(1);expect(outcome.errors).toEqual(["usage: "+corpus.usageText]);}else{expect(outcome.exit).toBeNull();expect(calls[row.key!]).toEqual(row.calls!);}
});
test("an async handler's promise is returned to the caller",async()=>{const result=dispatchSubcommand(["a"],{a:async()=>{await Promise.resolve();}},"u");expect(result).toBeInstanceOf(Promise);await result;});
for(const row of corpus.defects)test.failing("subcommand dispatch defect "+JSON.stringify(row.segments),()=>{const outcome=guarded(()=>dispatchSubcommand(row.segments,{a:()=>{}},corpus.usageText));expect(outcome.exit).toBe(1);});
for(const row of corpus.policy)test("policy argv "+JSON.stringify(row.segments)+" is not a policy run",async()=>{expect(await dispatchPolicyArgv(row.segments,import.meta.url)).toBe(row.expected);});'''
ts_e = lib.ts_wrap(UE, "🧭️ Contract: bundle entry dispatch hands the remaining arguments to the handler named by the first argument (or the default) and prints the usage line and exits 1 for anything else, and only the exact `policy` argument starts a policy run; the corpus is language-agnostic and `@@O@@` models the handler selection.",
 'import {dispatchPolicyArgv,dispatchSubcommand} from "@@' + UE + '/🟦.ts@@";', body_e)
orc_e = lib.PY_HEAD.format(doc="🐍️ Independent model of the subcommand dispatch selection (stdlib only).") + r'''
HANDLERS = {"a", "run"}
for c in corpus["dispatch"]:
    key = c["segments"][0] if c["segments"] else c["default"]
    handler = key if key in HANDLERS else None
    if (handler is None) != c["usage"]:
        failures.append("dispatch %r" % c["segments"])
    elif handler is not None and (key != c["key"] or c["segments"][1:] != c["calls"]):
        failures.append("dispatch args %r" % c["segments"])
''' + lib.PY_TAIL
lib.write_unit(UE, corpus_e, schema_e, ts_e, orc_e)

#!/usr/bin/env python3
"""Generates staged/sb-corpus.json: language-agnostic cases (hand-stated expectations) replayed by the TypeScript scanner and by the independent Python oracle. usage: gen_corpus.py"""
import json,os,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE+"/staged")
import importlib.util
spec=importlib.util.spec_from_file_location("oracle",HERE+"/staged/sb-oracle.py"); oracle=importlib.util.module_from_spec(spec); spec.loader.exec_module(oracle)
TS="x/🧪️tests/a/🟦️.ts"; GO="x/🧪️tests/a/🐹️.go"; RS="x/🧪️tests/a/🦀️.rs"
HEAD='import {test} from "bun:test";\n'
def ts(id,body,want): return {"id":id,"path":TS,"language":"ts","text":HEAD+body,"expected":[{"rule":r,"line":l} for r,l in want]}
def go(id,body,want): return {"id":id,"path":GO,"language":"go","text":"package a\n"+body,"expected":[{"rule":r,"line":l} for r,l in want]}
def rs(id,body,want): return {"id":id,"path":RS,"language":"rs","text":"#[test]\n"+body,"expected":[{"rule":r,"line":l} for r,l in want]}
scans=[
 ts("cwd-write",'test("a",()=>{writeFileSync(join(process.cwd(),"x"),"y");});\n',[("checkout-write",2),("unsandboxed-writer",2)]),
 ts("meta-dir-write",'test("a",()=>{mkdirSync(import.meta.dir+"/out");});\n',[("checkout-write",2),("unsandboxed-writer",2)]),
 ts("copy-source-read-is-not-a-write",'test("a",()=>{cpSync(resolve(import.meta.dir,".."),owner);});\n',[("unsandboxed-writer",2)]),
 ts("copy-destination-is-a-write",'test("a",()=>{cpSync(source,join(process.cwd(),"dest"));});\n',[("checkout-write",2),("unsandboxed-writer",2)]),
 ts("mkdtemp-root-is-safe",'test("a",()=>{const root=mkdtempSync(join(tmpdir(),"p-"));writeFileSync(join(root,"x"),"y");});\n',[]),
 ts("named-root-write",'test("a",()=>{rmSync(join(repoRoot,"out"),{force:true});});\n',[("checkout-write-named",2),("unsandboxed-writer",2)]),
 ts("cache-write-relative",'test("a",()=>{rmSync(".cache/b/o/m/x",{force:true});});\n',[("cache-write",2),("real-cache-path",2),("unsandboxed-writer",2)]),
 ts("cache-path-read",'test("a",()=>{expect(join(root,".cache/b/l")).toBe(x);});\n',[("real-cache-path",2)]),
 ts("ticket-scratch",'test("a",()=>{const root=resolve(process.cwd(),".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/T/🗑️generated");});\n',[("ticket-scratch-root",2),("ticket-state-path",2)]),
 ts("persistent-call",'test("a",async()=>{await withNativePersistentStateV1(options,body);});\n',[("persistent-native-state",2)]),
 ts("persistent-name-in-string",'test("a",()=>{expect(owner).toContain("withNativePersistentStateV1(");});\n',[]),
 ts("image-without-sandbox",'test("a",async()=>{await withNativeImageStorageV1(options,body);});\n',[("native-image-unsandboxed",2)]),
 ts("image-with-scratch-root",'test("a",async()=>{const root=mkdtempSync(join(tmpdir(),"i-"));await withNativeImageStorageV1(options,body);});\n',[]),
 ts("hdiutil-spawn",'test("a",()=>{spawnSync("hdiutil",["attach",image]);});\n',[("native-image-unsandboxed",2)]),
 ts("fixed-listen",'test("a",()=>{server.listen(8080,"127.0.0.1");});\n',[("fixed-port-bind",2)]),
 ts("ephemeral-listen",'test("a",()=>{server.listen(0,"127.0.0.1");});\n',[]),
 ts("serve-port-option",'test("a",()=>{Bun.serve({hostname:"127.0.0.1",port:5000,fetch});});\n',[("fixed-port-bind",2),("fixed-port-config",2)]),
 ts("serve-port-mocked",'test("a",()=>{serveMock({port:5000});});\n',[]),
 ts("port-url",'test("a",()=>{expect(url).toBe("http://127.0.0.1:8787/x");});\n',[("fixed-port-url",2)]),
 ts("port-variable",'test("a",()=>{server.listen(port,"127.0.0.1",done);});\n',[("port-variable-bind",2)]),
 ts("tmp-constant-dir",'test("a",()=>{const d=join(tmpdir(),"fixed-name");});\n',[("fixed-tmp-dir",2)]),
 ts("tmp-mkdtemp-prefix",'test("a",()=>{const d=mkdtempSync(join(tmpdir(),"fixed-"));});\n',[]),
 ts("tmp-literal",'test("a",()=>{expect(p).toBe("/tmp/test.go");});\n',[("fixed-tmp-literal",2)]),
 ts("home-dir",'test("a",()=>{const h=homedir();});\n',[("real-home",2)]),
 ts("pkill",'test("a",()=>{spawnSync("pkill",["-f","x"]);});\n',[("global-process-kill",2)]),
 ts("pkill-asserted-absent",'test("a",()=>{expect(text).not.toContain("pkill");});\n',[]),
 ts("chdir",'test("a",()=>{process.chdir(root);});\n',[("process-global-state",2)]),
 ts("comment-is-skipped",'// writeFileSync(join(process.cwd(),"x"),"y");\ntest("a",()=>{});\n',[]),
 {"id":"helper-is-not-a-suite","path":TS,"language":"ts","text":'export function prove(){writeFileSync(join(process.cwd(),"x"),"y");}\n',"expected":[]},
 go("go-temp-dir-is-safe",'func TestA(t *testing.T){dir:=t.TempDir();os.WriteFile(filepath.Join(dir,"a"),nil,0o644)}\n',[]),
 go("go-bare-writer",'func TestA(t *testing.T){os.MkdirAll("out",0o755)}\n',[("go-unsandboxed-writer",2)]),
 go("go-fixture-helper-is-safe",'func TestA(t *testing.T){root:=withMonorepoFixture(t);os.RemoveAll(root)}\n',[]),
 go("go-relative-source-write",'func TestA(t *testing.T){os.WriteFile("../../x.txt",nil,0o644)}\n',[("go-unsandboxed-writer",2),("go-checkout-relative",2)]),
 go("go-fixed-port",'func TestA(t *testing.T){net.Listen("tcp", "127.0.0.1:8080")}\n',[("fixed-port-bind",2),("fixed-port-url",2)]),
 go("go-fixed-tmp",'func TestA(t *testing.T){d:=filepath.Join(os.TempDir(), "fixed")}\n',[("fixed-tmp-dir",2)]),
 rs("rust-fixed-tmp",'fn a(){let d=std::env::temp_dir().join("semio-x-oracle");std::fs::create_dir_all(&d).unwrap();}\n',[("fixed-tmp-dir",2)]),
 rs("rust-unique-tmp",'fn a(){let d=std::env::temp_dir().join(format!("x-{}",std::process::id()));std::fs::create_dir_all(&d).unwrap();}\n',[]),
 rs("rust-manifest-write",'fn a(){std::fs::write(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("x"),b"y").unwrap();}\n',[("rust-checkout-write",2),("rust-unsandboxed-writer",2)]),
 rs("rust-bind-fixed",'fn a(){let l=TcpListener::bind("127.0.0.1:7000");}\n',[("fixed-port-bind",2),("fixed-port-url",2)]),
 rs("rust-bind-ephemeral",'fn a(){let l=TcpListener::bind("127.0.0.1:0");}\n',[]),
]
owners=[
 {"id":"go-root-lookup-unsandboxed","sources":[{"language":"go","text":"package a\nfunc TestA(t *testing.T){wd,_:=os.Getwd();_=wd}\n"}],"expected":[{"rule":"go-owner-unsandboxed"}]},
 {"id":"go-root-lookup-sandboxed","sources":[{"language":"go","text":"package a\nfunc TestA(t *testing.T){wd,_:=os.Getwd();_=wd}\n"},{"language":"go","text":"package a\nfunc TestMain(m *testing.M){os.Exit(sandbox.Run(\"a\",nil,m.Run))}\n"}],"expected":[]},
 {"id":"go-pure-unit","sources":[{"language":"go","text":"package a\nfunc TestA(t *testing.T){}\n"}],"expected":[]},
]
paths=[
 {"path":"m/🧪️tests/a/🟦️.ts","test":True},{"path":"m/🧪️tests/🧫️fixtures/a/🟦️.ts","test":False},{"path":"m/🟦️.ts","test":False},
 {"path":"m/x.test.ts","test":True},{"path":"m/x_test.go","test":True},{"path":"m/🧪️tests/a/🔣️.json","test":False},
 {"path":"m/🧪️tests/node_modules/a.ts","test":False},{"path":"m/src/a.rs","test":False},{"path":"m/src/a.rs","declared":["m/src/a.rs"],"test":True},
 {"path":"m/🧪️tests/🗑️generated/a.ts","test":False},{"path":"m/🧪️tests/a/🐍️.py","test":True},
]
CTX={"cwd":"/work/repo/pkg","platform":"linux","checkout":"/work/repo","allowed":["/tmp/st/ab12cd34","/private/tmp","/tmp"],"sandboxRoots":["/tmp/st/ab12cd34"],"protectedPrefixes":[".cache",".🧬semio",".git","node_modules"],"allowedCheckoutPrefixes":[".cache/b/o/m/tests"]}
def w(id,fn,target,want): return {"id":id,"context":CTX,"function":fn,"target":target,"expected":want}
writes=[
 w("sandbox-write","writeFileSync","/tmp/st/ab12cd34/tmp/x",None),
 w("checkout-source-write","writeFileSync","/work/repo/pkg/x.ts","checkout-write"),
 w("relative-write-resolves-against-cwd","mkdirSync","out","checkout-write"),
 w("protected-cache-write","rmSync","/work/repo/.cache/b/o/m/reports/x.json","protected-write"),
 w("declared-diagnostics-prefix-is-allowed","writeFile","/work/repo/.cache/b/o/m/tests/t/r/x",None),
 w("ticket-state-is-protected","writeFileSync","/work/repo/.🧬semio/🦑️repo/🎫️tickets/x","protected-write"),
 w("outside-checkout-is-not-judged","writeFileSync","/home/u/x",None),
 w("parent-traversal-out-of-sandbox","writeFileSync","/tmp/st/ab12cd34/../../../work/repo/a","checkout-write"),
 w("sibling-prefix-is-not-inside","writeFileSync","/work/repo-sibling/x",None),
]
S2={**CTX,"platform":"darwin"}
INTMP={**CTX,"checkout":"/tmp/work","cwd":"/tmp/work"}
writes.append({"id":"checkout-below-a-temp-root-is-still-judged","context":INTMP,"function":"writeFileSync","target":"/tmp/work/a.ts","expected":"checkout-write"})
writes.append({"id":"sandbox-below-a-temp-checkout-is-allowed","context":INTMP,"function":"writeFileSync","target":"/tmp/st/ab12cd34/tmp/x","expected":None})
writes.append({"id":"sandbox-nested-inside-a-temp-checkout-is-allowed","context":{**INTMP,"sandboxRoots":["/tmp/work/st/ab12cd34"]},"function":"writeFileSync","target":"/tmp/work/st/ab12cd34/tmp/x","expected":None})
writes.append({"id":"darwin-case-folding","context":S2,"function":"writeFileSync","target":"/WORK/REPO/a.ts","expected":"checkout-write"})
def sp(id,cmd,args,want): return {"id":id,"context":CTX,"command":cmd,"args":args,"expected":want}
spawns=[
 sp("hdiutil-attach-sandbox-image","/usr/bin/hdiutil",["attach","/tmp/st/ab12cd34/images/a.dmg"],None),
 sp("hdiutil-attach-checkout-image","hdiutil",["attach","-nobrowse","/work/repo/.cache/b/d/image.dmg"],"kernel-image"),
 sp("hdiutil-info-has-no-path","hdiutil",["info","-plist"],None),
 sp("diskutil-eject-relative","diskutil",["eject","./vol/x"],"kernel-image"),
 sp("pkill-always","pkill",["-f","bun"],"process-kill"),
 sp("killall-exe","killall.exe",["bun"],"process-kill"),
 sp("ordinary-command","git",["status"],None),
]
def ln(id,port,leased,want): return {"id":id,"context":CTX,"port":port,"leased":leased,"expected":want}
listens=[ln("ephemeral",0,[],None),ln("privileged",80,[],None),ln("fixed",8080,[],"fixed-port"),ln("leased",51000,[51000],None),ln("other-lease",51001,[51000],"fixed-port")]
corpus={"version":1,"scans":scans,"owners":owners,"paths":paths,"writes":writes,"spawns":spawns,"listens":listens}
json.dump(corpus,open(HERE+"/staged/sb-corpus.json","w",encoding="utf8"),ensure_ascii=False,indent=1); open(HERE+"/staged/sb-corpus.json","a").write("\n")
r=subprocess.run([sys.executable,HERE+"/staged/sb-oracle.py",HERE+"/staged/sb-protocol.json",HERE+"/staged/sb-corpus.json"],capture_output=True,text=True)
print(r.stdout,r.stderr)
sys.exit(r.returncode)

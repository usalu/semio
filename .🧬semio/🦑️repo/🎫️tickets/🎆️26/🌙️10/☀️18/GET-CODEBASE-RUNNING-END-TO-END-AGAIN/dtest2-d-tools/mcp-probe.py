import json,subprocess,sys,os,tempfile
d=tempfile.mkdtemp(prefix="rp-",dir=sys.argv[1])
os.makedirs(d+"/src"); open(d+"/local.rs","w").write("pub fn local() {}\n"); open(d+"/src/child.ts","w").write("export const child = 1;\n")
argv=json.loads(sys.argv[2].replace("DIR",d))
env=dict(os.environ,SEMIO_REPO_MCP_CLIENT=sys.argv[3])
p=subprocess.Popen(["/Users/ueli/.local/bin/repo"]+argv,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,cwd=sys.argv[4] if len(sys.argv)>4 else "/Users/ueli/Documents/semio",env=env,text=True)
def send(m): p.stdin.write(json.dumps(m)+"\n"); p.stdin.flush()
send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{"roots":{"listChanged":False}},"clientInfo":{"name":"t","version":"1"}}})
def rd():
    l=p.stdout.readline(); return json.loads(l) if l else None
print(rd())
send({"jsonrpc":"2.0","method":"notifications/initialized"})
send({"jsonrpc":"2.0","id":2,"method":"resources/list"})
for _ in range(3):
    m=rd(); print(str(m)[:600])
    if m and m.get("id")==2: break
send({"jsonrpc":"2.0","id":3,"method":"resources/read","params":{"uri":"repo://files"}})
for _ in range(3):
    m=rd(); print(str(m)[:800])
    if m and m.get("id")==3: break
p.stdin.close(); p.terminate(); print("stderr:",p.stderr.read()[:800])

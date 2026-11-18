import sys,os,json,hashlib
ROOT=os.environ.get("R161_ROOT","/Users/ueli/Documents/semio")
P=ROOT+"/🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve/🟦️.ts"
FILES={"t":P}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
NEW='''  test("a machine with a declared state directory finds its provisioned tools below it, not below the checkout", () => {
    const digest = "d".repeat(64), pins = { ["/repo/" + bun.pin.digests!.file]: JSON.stringify({ definitions: { Release: { const: { version: "1.4.2", distributions: [{ platform: "darwin", arch: "arm64", musl: false, archive: "a", bytes: 1, sha256: digest }] } } } }) }, declared = process.env.SEMIO_MACHINE_STATE_DIRECTORY;
    process.env.SEMIO_MACHINE_STATE_DIRECTORY = "/lab/s";
    try {
      expect(resolveToolV1(bun, host({ pins, execPath: "/runtime/node", present: [`/lab/s/b/t/m/bun/${digest}/bun`] })).source).toBe("provisioned");
      expect(resolveToolV1(bun, host({ pins, execPath: "/runtime/node", present: [`/repo/.cache/b/t/m/bun/${digest}/bun`] })).source).toBe("missing");
    } finally { if (declared === undefined) delete process.env.SEMIO_MACHINE_STATE_DIRECTORY; else process.env.SEMIO_MACHINE_STATE_DIRECTORY = declared; }
  });
'''
def t(x): return one(x,'  test("builds the host and 16-digit placeholders of a provisioned Go", () => {',NEW+'  test("builds the host and 16-digit placeholders of a provisioned Go", () => {')
PATCH={"t":t}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(p,new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)

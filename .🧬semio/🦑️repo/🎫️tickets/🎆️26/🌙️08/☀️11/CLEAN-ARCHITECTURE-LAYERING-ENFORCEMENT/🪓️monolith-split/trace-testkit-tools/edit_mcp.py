import os,re,sys
R="/Users/ueli/Documents/semio"
M=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"
NEW=R+"/🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust"
KEY="semio-framework-trace-memory-testing"
dry="--apply" not in sys.argv
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    if not dry: open(p,"w",encoding="utf8").write(s)
def sub1(s,a,b,what):
    assert s.count(a)==1,(what,s.count(a)); return s.replace(a,b)
mount='#[cfg(any(test, feature = "testkit"))]\n#[path = "../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]\npub mod test_allocation;\n\n'
p=M+"/🧱️foundation/🦀️.rs"; wr(p,sub1(rd(p),mount,"","foundation mount")); print("foundation root: mount removed")
p=M+"/🧱️foundation/📦️packages/🦀️rust/Cargo.toml"; s=rd(p)
s=sub1(s,'[features]\ntestkit = ["dep:semio-framework-trace"]\n\n',"","foundation features")
s=sub1(s,'semio-framework-trace = { workspace = true, optional = true }\n',"","foundation optional trace")
wr(p,s); print("foundation manifest: testkit feature and optional trace removed")
OLDPATH="semio_framework_os_mcp_foundation::test_allocation::"
NEWPATH="semio_framework_trace_memory_testing::"
for rel in ["/📦️packages/🦀️rust/🦀️.rs","/⚙️plugin-runtime/🦀️.rs","/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs","/🏠️workspace/🪶️sqlite/🧪️tests/guest-input-slot.rs"]:
    p=M+rel; s=rd(p); n=s.count(OLDPATH); assert n>0,rel
    wr(p,s.replace(OLDPATH,NEWPATH)); print("rewrote",n,"paths in",rel)
for pkg,rel in [("/📦️packages/🦀️rust","../../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust"),("/⚙️plugin-runtime/📦️packages/🦀️rust",None)]:
    p=M+pkg+"/Cargo.toml"; s=rd(p)
    old=re.search(r'(?m)^semio-framework-os-mcp-foundation = \{ path = "([^"]+)", features = \["testkit"\] \}\n',s); assert old,pkg
    assert KEY not in s
    path=os.path.relpath(NEW,M+pkg)
    assert s.count("[dev-dependencies]\n")==1
    s=s.replace(old.group(0),f'{KEY} = {{ path = "{path}" }}\n')
    wr(p,s); print("manifest",pkg,"dev-dep foundation/testkit ->",KEY)

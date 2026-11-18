import subprocess,sys,os
root="/Users/ueli/Documents/semio"
here=os.path.dirname(os.path.abspath(__file__))
subprocess.check_call([sys.executable,here+"/composition-route-fix.py","apply",root+"/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs"])
p=root+"/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️identity/🦀️.rs"
s=open(p).read()
a='format!("no declared {} capability owns the runtime claims", kind.as_str())'
b='format!("no declared {} capability of {} owns the runtime claims [{}]", kind.as_str(), definition.identity, claims.iter().map(|claim| format!("{}={}", claim.namespace().as_str(), claim.value())).collect::<Vec<_>>().join(", "))'
assert s.count(a)==1
open(p,"w").write(s.replace(a,b))

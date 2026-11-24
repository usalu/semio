import json,os,shutil,sys
W="/Users/ueli/Documents/semio"
D=W+"/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"
DER=D+"/📦️packages/🦀️rust"
MUT=W+"/🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️packages/🦀️rust"
VAL=W+"/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
root=sys.argv[1]
loc="🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/"
owner="probe/🧪️tests/🧬️mutations/🎯️probe-op"
shutil.rmtree(root,ignore_errors=True)
def w(p,t):
    p=os.path.join(root,p);os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t)
w("package.json",'{"workspaces":[]}');w("📋️project.json","{}")
for n in["🔣️mutation-authority.json","🗂️schema-document-index.json"]:
    os.makedirs(os.path.join(root,loc),exist_ok=True);shutil.copy(os.path.join(D,n),os.path.join(root,loc,n))
w("Cargo.toml",'cargo-features = ["trim-paths"]\n[workspace]\nresolver = "3"\nmembers = ["probe"]\n')
w("probe/Cargo.toml",f'''[package]
name = "probe"
version = "0.1.0"
edition = "2021"

[lib]
path = "🦀️.rs"

[dependencies]
dsl_derive = {{ package = "semio-os-kernel-dsl-derive", path = "{DER}" }}
semio-framework-replication-mutation = {{ path = "{MUT}" }}
semio-framework-value = {{ path = "{VAL}" }}
''')
w("probe/🦀️.rs",'#[path = "🧪️tests/🧬️mutations/🎯️probe-op/🦀️.rs"]\npub mod op;\n')
w(owner+"/🦀️.rs",'''use dsl_derive::MutationLeaf;
#[derive(Clone, Debug, PartialEq, MutationLeaf)]
#[mutation_leaf(contract = ::semio_framework_replication_mutation)]
pub struct ProbeOp { pub value: Option<String> }
''')
d=json.load(open(D+"/🧪️tests/🧫️fixtures/🛂️mutation-source-authority/🔣️.json"))["descriptor"]
d["owner"]=owner;d["semanticKind"]="probe-op";d["displayName"]="Probe Op";d["aggregateVariant"]="ProbeOp";d["payloadSchema"]="🧬️schema/🔣️.json";d["editable"]=True
w(owner+"/🔣️.json",json.dumps(d,ensure_ascii=False,indent=2))
w(owner+"/🧬️schema/🔣️.json",json.dumps({"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://example.invalid/probe/schema.json","type":"object","additionalProperties":False,"properties":{"value":{"type":["string","null"]}}},indent=2))

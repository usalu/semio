E="✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
F=E+"/🏭️direct-store-factory-and-child-group-route/🦀️.rs"
T=E+"/📦️packages/🦀️rust/Cargo.toml"
s=open(F).read()
a="    pub fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {"
assert s.count(a)==1 and "fn detached" not in s
b='    #[cfg(feature = "testkit")]\n    pub fn detached(tool_id: &\'static str) -> Self {\n        Self { tool_id, instance_owner: None, output: None, output_retirement: None, completed: false, closing: false }\n    }\n\n'
open(F,"w").write(s.replace(a,b+a))
t=open(T).read()
assert t.count("\n[features]\n")==0 and t.count("\n[dependencies]\n")==1
open(T,"w").write(t.replace("\n[dependencies]\n",'\n[features]\ntestkit = []\n\n[dependencies]\n'))
R=E+"/🎮️commands/♻️retirement/🧪️tests/📦️packages/🦀️rust/Cargo.toml"
r=open(R).read()
a='semio-s-artifact-flow-flow-editor = { path = "../../../../../📦️packages/🦀️rust" }'
assert r.count(a)==1
open(R,"w").write(r.replace(a,'semio-s-artifact-flow-flow-editor = { path = "../../../../../📦️packages/🦀️rust", features = ["testkit"] }'))

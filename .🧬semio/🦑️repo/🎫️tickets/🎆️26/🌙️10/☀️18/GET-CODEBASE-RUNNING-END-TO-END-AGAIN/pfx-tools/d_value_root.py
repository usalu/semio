import rs,re,subprocess,unused
p=rs.SUBSETS+"🔢️value/"+rs.SNAP+"🦀️.rs"
s=open(p,encoding="utf-8").read()
for n in ("reconstruct_value_forest","value_push"):
    a,b=rs.span(s,n);s=s[:a]+s[b:]
k=s.index("pub fn project_sqlite_database(&self")
e=s.index("\n",k)
s=s[:k]+"pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(control.limits())?;projection::project(self,Self::SQLITE_SCHEMA,control)} "+s[e:]
k=s.index("pub(crate)fn admit_values(")
e=s.index("\n",k)
s=s[:k]+"pub(crate)fn admit_values(snapshot:&SemioValueSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;projection::admit(snapshot,phase,control)}"+s[e:]
s=s.replace('#[path="🧮️semantic/🦀️.rs"]\npub mod semantic;','#[path="🧮️semantic/🦀️.rs"]\npub mod semantic;\n#[path="🫙️projection/🦀️.rs"]\npub mod projection;',1)
s=s.replace('#[path = "💰️reconstruction/🦀️.rs"]\nmod reconstruction;','#[path = "💰️reconstruction/🦀️.rs"]\nmod reconstruction;\npub use reconstruction::{ValueForestStorage,reconstruct_value_forest_into};',1)
open(p,"w",encoding="utf-8").write(s)
r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",p],capture_output=True,text=True);print(r.returncode,r.stderr[:500])
print(unused.report(p))
# projection visibility
pp=rs.SUBSETS+"🔢️value/"+rs.SNAP+"🫙️projection/🦀️.rs"
t=open(pp,encoding="utf-8").read()
t=t.replace("pub(crate)use tree::TreeScratch;\npub(crate)use tree::visit as visit_tree;","pub use tree::TreeScratch;\npub use tree::visit as visit_tree;")
t=t.replace("pub(crate)trait References","pub trait References")
open(pp,"w",encoding="utf-8").write(t)
tp=rs.SUBSETS+"🔢️value/"+rs.SNAP+"🫙️projection/🌳️tree/🦀️.rs"
u=open(tp,encoding="utf-8").read()
u=u.replace("pub(crate)struct TreeScratch","pub struct TreeScratch").replace("pub(crate)fn empty()","pub fn empty()").replace("pub(crate)fn visit(","pub fn visit(")
open(tp,"w",encoding="utf-8").write(u)
for q in (pp,tp):
    r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",q],capture_output=True,text=True);print(r.returncode,r.stderr[:300])

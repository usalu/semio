import rs,re,subprocess
p=rs.SUBSETS+"🕸️graph/"+rs.SNAP+"🦀️.rs"
s=open(p,encoding="utf-8").read()
# 1 remove old fns
for n in ("invalid","node_name","optional_text","group","order_relationships","restore_properties","float_rows","ordered_float_rows"):
    a,b=rs.span(s,n);s=s[:a]+s[b:]
# 2 replace reconstruct impl block
i=s.index("impl SemioGraphSnapshot {\n    /// 🧩️ Restores every typed child field")
j=s.index("\n}\n",i)+3
s=s[:i]+'''impl SemioGraphSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}
'''+s[j:]
# 3 project + admit
s=s.replace("semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()","semantic::layout(control.limits())?;projection::project(self,Self::SQLITE_SCHEMA,control)",1)
k=s.index("pub(crate)fn admit_values(")
e=s.index("\n",k)
s=s[:k]+"pub(crate)fn admit_values(snapshot:&SemioGraphSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;projection::admit(snapshot,phase,control)}"+s[e:]
# 4 mounts
s=s.replace('#[path="🧮️semantic/🦀️.rs"]\nmod semantic;','#[path="🧮️semantic/🦀️.rs"]\nmod semantic;\n#[path="🫙️projection/🦀️.rs"]\npub(crate)mod projection;\n#[path="💰️reconstruction/🦀️.rs"]\nmod reconstruction;',1)
open(p,"w",encoding="utf-8").write(s)
r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",p],capture_output=True,text=True)
print(r.returncode,r.stderr[:800])

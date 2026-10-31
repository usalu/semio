import os,sys
ROOT=sys.argv[1]
M="semio_s_artifact_fem_2d_mutations::cold_laws::"
SNAP="semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot"
rules=[("protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(",M+"assert_cold_mutation_inverse_sum_law("),
("vcs::apply_mutation(",M+"apply_cold_mutation("),
("SemanticMutation<Fem2dSnapshotValue>","SemanticMutation<"+SNAP+">")]
n=0;files=0
for d,_,fs in os.walk(ROOT):
    if "/target" in d or "node_modules" in d: continue
    for f in fs:
        if f!="🦀️.rs":continue
        p=os.path.join(d,f)
        try:s=open(p,encoding="utf8").read()
        except Exception:continue
        t=s
        for a,b in rules:t=t.replace(a,b)
        if t!=s:
            open(p,"w",encoding="utf8").write(t);files+=1
print("files",files)

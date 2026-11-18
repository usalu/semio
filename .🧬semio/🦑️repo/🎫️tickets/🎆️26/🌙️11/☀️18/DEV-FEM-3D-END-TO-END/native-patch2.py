import sys
p=sys.argv[1];s=open(p,encoding="utf-8").read()
def sub(o,n,c=1):
    global s
    k=s.count(o)
    if k!=c: sys.exit("%d != %d: %s"%(k,c,o[:80]))
    s=s.replace(o,n)
sub("pack::snapshot::ArtifactPack::encode_pack(&s)","pack::snapshot::ArtifactPack::encode_pack(&*own(&s))",2)
sub("pack::snapshot::ArtifactDsl::print_dsl(&s)","pack::snapshot::ArtifactDsl::print_dsl(&*own(&s))",2)
sub("let result=match phase{","let failed=match phase{")
sub("o.to_sqlite_database(&mut c).map(|_|())","o.to_sqlite_database(&mut c).is_err()")
sub("Fem3dSnapshot::from_sqlite_database(&d,&mut c).map(|_|())","Fem3dSnapshot::from_sqlite_database(&d,&mut c).map(Own::new).is_err()")
sub("encode_native(&*o,SnapshotEncoding::Text,&mut c).map(|_|())};assert!(result.is_err());","encode_native(&*o,SnapshotEncoding::Text,&mut c).is_err()};assert!(failed);")
sub("let literal=Owned::new(literal);","let literal=own(&literal);")
open(p,"w",encoding="utf-8",newline="\n").write(s)

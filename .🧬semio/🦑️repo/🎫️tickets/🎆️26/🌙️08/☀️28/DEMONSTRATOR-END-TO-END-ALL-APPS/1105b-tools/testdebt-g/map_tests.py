import re
E="crate::retained_clone::effect"
def edit(path,subs):
    t=open(path).read()
    for pat,rep,n in subs:
        got=len(re.findall(pat,t))
        assert got==n,(path,pat,got,n)
        t=re.sub(pat,rep,t)
    open(path,"w").write(t)
edit("➖️remove/🧪️tests/🦀️.rs",[
 (r"cursor\.advance\((denied|g|grant\(\))\)",r"%s::run(\1.retirement,|effect|cursor.advance(\1,effect))"%E,6),
])
edit("🧪️tests/🔬️unit/🦀️.rs",[
 (r"advance_demands\(source\.borrow\(\),body\)",r"advance_demands(source.borrow(),body,%s::CPU)"%E,2),
 (r"Err\(error\)=>assert!\(matches!\(error\.kind,",r"Err(error)=>assert!(matches!(error.into_value_error().kind,",1),
 (r"cursor\.take\(take\)\.result",r"%s::take(&mut cursor,take).result"%E,2),
 (r"cursor\.advance_demands\(source\.borrow\(\),grant\.maximum_copy_bytes\)",r"cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,%s::CPU)"%E,1),
 (r"cursor\.advance\((grant|g)\)\.result",r"%s::run(\1.retirement,|effect|cursor.advance(\1,effect)).result"%E,7),
 (r"(\w+)\.compare\(([\w.()]+), ?([\w.()]+), ?grant, ?physical_grant\(7\)\)",r"%s::run(physical_grant(7),|effect|\1.compare(\2,\3,grant,effect))"%E,3),
 (r"(\w+)\.advance\(([\w.()]+), ?([\w.()]+), ?grant, ?physical_grant\(7\)\)",r"%s::run(physical_grant(7),|effect|\1.advance(\2,\3,grant,effect))"%E,4),
])

import re,sys
sys.path.insert(0,__file__.rsplit("/",1)[0])
from port_native import match_close
PAT=re.compile(r"(?<!fn )\b(?:(?:de|en)code_sqlite_snapshot_native|receive_pack|to_sqlite_database_receiving|from_sqlite_database_receiving)\s*\(")
FOLLOW=re.compile(r"\s*\.(unwrap|expect|unwrap_err|expect_err|is_err|is_ok)\b")
def run(path,write):
    s=open(path,encoding="utf8").read();out=[];i=0;n=0
    while True:
        m=PAT.search(s,i)
        if not m: out.append(s[i:]);break
        if s[max(0,m.start()-3):m.start()]=="fn ": out.append(s[i:m.end()]);i=m.end();continue
        j=match_close(s,m.end()-1,"(",")")+1
        out.append(s[i:j])
        if FOLLOW.match(s,j): out.append(".result");n+=1
        i=j
    t="".join(out)
    if n and write: open(path,"w",encoding="utf8").write(t)
    return n
if __name__=="__main__":
    w=sys.argv[1]=="write"
    for p in sys.argv[2:]:
        n=run(p,w)
        if n: print(n,p[-90:])

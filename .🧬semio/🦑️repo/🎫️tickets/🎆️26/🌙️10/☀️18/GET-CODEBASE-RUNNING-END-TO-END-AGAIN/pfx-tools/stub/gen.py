import re,os,sys,subprocess,shutil
sys.path.insert(0,"..")
import rs
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.abspath(os.path.join(HERE,"../../🗑️generated/pfx-stub"))
os.makedirs(OUT,exist_ok=True)
RUSTC=["rustc","+nightly-2026-07-20","--edition","2021","--crate-type","lib","--emit=metadata","--cap-lints","allow","-L",OUT]
def sanitize(src):
    out=[];i=0;n=len(src);derives=set();default_enum=False
    lines=src.split("\n")
    buf=[];pending=set();k=0
    # tokenise by items using brace matching
    res=[];pos=0
    text=src
    while pos<len(text):
        m=re.compile(r"(?m)^(?:[ \t]*(?:///[^\n]*|//[^\n]*|#\[[^\n]*\])\n)*[ \t]*(pub (?:struct|enum) \w+|impl Default for \w+|pub const \w+[^\n]*;|impl\b[^\n]*\{|pub fn[^\n]*\{)").search(text,pos)
        if not m:break
        header_start=m.start();kind=m.group(1)
        attrs=text[header_start:m.start(1)]
        if kind.startswith("pub const"):
            res.append(kind);pos=m.end();continue
        # find body
        j=text.index("{",m.start(1)) if kind.startswith(("pub struct","pub enum")) else text.index("{",m.start(1))
        # tuple/unit struct handling
        if kind.startswith("pub struct") and ";" in text[m.end(1):j] and text[m.end(1):j].strip().endswith(";"):
            res.append(kind+";");pos=m.end(1)+text[m.end(1):].index(";")+1;continue
        d=0;t=j
        while True:
            e=rs.skip(text,t)
            if e is not None:t=e;continue
            c=text[t]
            if c=="{":d+=1
            elif c=="}":
                d-=1
                if d==0:break
            t+=1
        body=text[j:t+1]
        if kind.startswith("pub struct") or kind.startswith("pub enum"):
            ds=set()
            for a in re.findall(r"#\[derive\(([^\]]*)\)\]",attrs):
                for x in re.split(r"[,\s]+",a):
                    if x in("Clone","Copy","Debug","PartialEq","Eq","Default"):ds.add(x)
            # strip field attrs/docs
            b=re.sub(r"(?m)^[ \t]*///[^\n]*\n","",body)
            b=re.sub(r"(?m)^[ \t]*#\[(?!default\])[^\n]*\]\n","",b)
            b=re.sub(r"(?m)^[ \t]*//[^\n]*\n","",b)
            dl="#[derive("+",".join(sorted(ds))+")]\n" if ds else ""
            res.append(dl+kind+" "+b)
        elif kind.startswith("impl Default for"):
            name=kind.split()[-1]
            res.append("impl Default for %s{fn default()->Self{unimplemented!()}}"%name)
        pos=t+1
    return "\n".join(res)+"\n"
if __name__=="__main__":
    p=rs.SUBSETS+sys.argv[1]+"/🧬️schema/📸️snapshot/🦀️.rs"
    print(sanitize(open(p,encoding="utf-8").read()))

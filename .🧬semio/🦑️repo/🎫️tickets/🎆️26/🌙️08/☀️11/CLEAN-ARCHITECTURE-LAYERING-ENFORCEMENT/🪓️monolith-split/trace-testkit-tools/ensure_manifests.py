import json,os,re,sys
R="/Users/ueli/Documents/semio"
here=os.path.dirname(os.path.abspath(__file__))
units=json.load(open(here+"/units.json"))
NEW="🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust"
KEY="semio-framework-trace-memory-testing"
apply="--apply" in sys.argv
for name,pkg in sorted({u["name"]:u["pkg"] for u in units}.items()):
    mp=os.path.join(R,pkg,"Cargo.toml"); s=open(mp,encoding="utf8").read()
    if re.search(r'(?m)^'+KEY+r'\s*=',s): continue
    rel=os.path.relpath(os.path.join(R,NEW),os.path.join(R,pkg))
    m=re.search(r'(?m)^\[dev-dependencies\]\n',s); assert m,name
    se=re.search(r'(?m)^\[',s[m.end():]); sec=s[m.end():m.end()+(se.start() if se else len(s))]
    tm=re.search(r'(?m)^semio-framework-trace(?: =|\.workspace)[^\n]*\n',sec)
    pos=m.end()+(tm.end() if tm else 0)
    print("ADD",name,rel[:60])
    if apply: open(mp,"w",encoding="utf8").write(s[:pos]+f'{KEY} = {{ path = "{rel}" }}\n'+s[pos:])

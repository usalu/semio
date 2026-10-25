import re,sys,collections
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O=C+"/🗑️generated/dbg2-d/orig"
TOK=re.compile(r'''//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|`(?:\\.|[^`\\])*`|'(?:\\.|[^'\\\n])'|'(?:\\.|[^'\\])*'(?=[^A-Za-z_])|r#*"|[A-Za-z_][A-Za-z0-9_]*|\d[\w.]*|\S''',re.S)
def toks(t):return [m for m in TOK.findall(t) if not m.startswith("//") and not m.startswith("/*")]
BR={"(":")","[":"]","{":"}"}
HEAD={"println","eprintln","print","eprint","writeln","log","error","warn","debug","info","write","stdout","stderr"}
bad=0;total=0
for l in open(O+"/map.txt",encoding="utf8"):
    n,f=l.rstrip("\n").split(" ",1)
    a=open(f"{O}/{n}.bak",encoding="utf8").read();b=open(R+"/"+f,encoding="utf8").read()
    if a==b:continue
    ta,tb=collections.Counter(toks(a)),collections.Counter(toks(b))
    rem=ta-tb;add=tb-ta
    notes=[]
    for o,c in BR.items():
        if rem[o]!=rem[c]:notes.append(f"unbalanced {o}{c} removed {rem[o]}/{rem[c]}")
    addp=[k for k in add if not (k.startswith('"') or k.startswith('`') or k.startswith("'") )]
    if addp:notes.append("added tokens "+repr(addp[:8]))
    if add and any(k.startswith(('"','`',"'")) and "[DEBUG]" in k for k in add):notes.append("added DEBUG string")
    total+=1
    if notes:
        bad+=1;print("FLAG",n,f.split("/")[-4:],notes)
print("files",total,"flagged",bad)

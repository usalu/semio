import re,sys
p=sys.argv[1]
s=open(p,encoding='utf8').read()
lines=s.split("\n")
fi=next(i for i,l in enumerate(lines) if l.startswith("pub async fn plugin_exchange<"))
body="\n".join(lines[fi:])
body=body.replace("    let mut mutated = false;\n    let mut started_typed_operation: Option<u64> = None;\n","    let mut flags = ExchangeFlags::default();\n",1)
body=re.sub(r"(?<![\w.])mutated\b(?!:)","flags.mutated",body)
body=re.sub(r"(?<![\w.])started_typed_operation\b(?!:)","flags.started_typed_operation",body)
lines=lines[:fi]+body.split("\n")
# struct definition before the first doc comment of the function
di=fi
while di>0 and lines[di-1].startswith("///"): di-=1
struct=["/// \U0001F9FE️ The two facts the command arms of one exchange record for its tail: whether the document mutated and which typed operation the command started.","#[derive(Default)]","struct ExchangeFlags {","    mutated: bool,","    started_typed_operation: Option<u64>,","}",""]
lines[di:di]=struct
open(p,"w",encoding="utf8").write("\n".join(lines))
print("prep ok")

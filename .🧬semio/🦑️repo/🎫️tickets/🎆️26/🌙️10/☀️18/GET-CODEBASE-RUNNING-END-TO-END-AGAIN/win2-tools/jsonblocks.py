import json,re
def remove_blocks(path,indent,names_regex):
    lines=open(path,encoding="utf-8").read().split("\n")
    pad=" "*indent
    out=[];i=0;removed=[]
    while i<len(lines):
        m=re.match(r'^'+pad+r'"('+names_regex+r')": ',lines[i])
        if not m: out.append(lines[i]);i+=1;continue
        depth=0;j=i
        while True:
            instr=False;esc=False
            for ch in lines[j]:
                if instr:
                    if esc: esc=False
                    elif ch=="\\": esc=True
                    elif ch=='"': instr=False
                else:
                    if ch=='"': instr=True
                    elif ch in "{[": depth+=1
                    elif ch in "}]": depth-=1
            if depth<=0: break
            j+=1
        removed.append(m.group(1));i=j+1
    # fix dangling comma before closing brace
    for k in range(len(out)-1):
        if out[k].rstrip().endswith(",") and out[k+1].strip() in ("}","},","]","],"):
            out[k]=out[k].rstrip()[:-1]
    open(path,"w",encoding="utf-8").write("\n".join(out))
    json.load(open(path,encoding="utf-8"))
    return removed

import json
def targets_span(s):
    i=s.index('"targets"'); j=s.index('{',i)
    depth=0; k=j; instr=False; esc=False
    while True:
        c=s[k]
        if instr:
            if esc: esc=False
            elif c=='\\': esc=True
            elif c=='"': instr=False
        else:
            if c=='"': instr=True
            elif c=='{': depth+=1
            elif c=='}':
                depth-=1
                if depth==0: return j,k
        k+=1
def add_targets(s,new):
    if '"targets"' not in s:
        body=json.dumps({'targets':new},indent=2,ensure_ascii=False).split('\n')[1:-1]
        k=s.rstrip().rindex('}')
        head=s[:k].rstrip()
        return head+',\n'+'\n'.join(body)+'\n'+s[k:]
    j,k=targets_span(s)
    body=s[j+1:k]
    empty=body.strip()==''
    out=[]
    for name,t in new.items():
        txt=json.dumps(t,indent=2,ensure_ascii=False)
        txt='\n'.join('    '+l for l in txt.split('\n'))
        out.append('    '+json.dumps(name,ensure_ascii=False)+': '+txt.lstrip())
    ins=',\n'.join(out)
    head=s[:k].rstrip()
    if empty: return s[:j+1]+'\n'+ins+'\n  '+s[k:]
    return head+',\n'+ins+'\n  '+s[k:]
def remove_target(s,name):
    key=json.dumps(name,ensure_ascii=False)+': {'
    i=s.index(key)
    j=i+len(key)-1
    depth=0;k=j;instr=False;esc=False
    while True:
        c=s[k]
        if instr:
            if esc: esc=False
            elif c=='\\': esc=True
            elif c=='"': instr=False
        else:
            if c=='"': instr=True
            elif c=='{': depth+=1
            elif c=='}':
                depth-=1
                if depth==0: break
        k+=1
    start=s.rfind('\n',0,i)
    before=s[:start].rstrip()
    after=s[k+1:]
    if before.endswith(','): 
        return before[:-1].rstrip()+after if after.lstrip().startswith('}') or after.lstrip().startswith('\n  }') else before+after.lstrip(',') if False else before[:-1]+(after if after.lstrip().startswith(',') is False else after)
    return before+after.lstrip(',')

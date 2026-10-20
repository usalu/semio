import re,sys,os
def closure(root):
    seen=[];stack=[root]
    pat=re.compile(r'#\[path\s*=\s*"([^"]+)"\]|include!\(\s*"([^"]+)"\s*\)')
    while stack:
        f=stack.pop()
        if f in seen or not os.path.isfile(f): continue
        seen.append(f)
        src=open(f,encoding='utf8').read()
        for m in pat.finditer(src):
            rel=m.group(1) or m.group(2)
            stack.append(os.path.normpath(os.path.join(os.path.dirname(f),rel)))
    return seen
if __name__=='__main__':
    root=sys.argv[1]
    toks=sys.argv[2:]
    for f in closure(root):
        src=open(f,encoding='utf8').read()
        hits={t:len(re.findall(t,src)) for t in toks}
        print(f.split('/Documents/semio/')[-1][-110:], hits)

import re,sys,os
pat=re.compile(r'(if|while) let .*?=\s*(?:[\w.]+\.)?(\w+)\.lock\(\)[^;{]*\{\s*$')
hits=[]
for root,dirs,files in os.walk(sys.argv[1]):
    for f in files:
        if not f.endswith('.rs'): continue
        p=os.path.join(root,f)
        lines=open(p,encoding='utf-8').read().split('\n')
        for i,l in enumerate(lines):
            m=pat.search(l)
            if not m: continue
            field=m.group(2)
            depth=l.count('{')-l.count('}')
            j=i+1
            while j<len(lines) and depth>0:
                depth+=lines[j].count('{')-lines[j].count('}')
                if re.search(r'\b'+field+r'\.lock\(\)',lines[j]) or re.search(r'\b'+field+r'\.try_lock\(\)',lines[j]):
                    hits.append((p,i+1,j+1,field,l.strip()[:120]))
                    break
                j+=1
for h in hits: print(f"{h[0]}:{h[1]} -> relock {h[3]} at {h[2]} :: {h[4]}")

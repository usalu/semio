import sys,re,json,subprocess
D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/'
files=['🎮️registry/🛂️tokens-and-validation/🦀️.rs','🎮️registry/🧶️build-and-derive/🦀️.rs','🎮️registry/🩺️check-and-json/🦀️.rs','🎮️registry/🧾️declarations/🦀️.rs','📚️inventory/🦀️.rs']
def balanced(s,i):
    d=0;j=i;instr=False;esc=False
    while j<len(s):
        c=s[j]
        if instr:
            if esc: esc=False
            elif c=='\\': esc=True
            elif c=='"': instr=False
        else:
            if c=='"': instr=True
            elif c=="'" and s[j+2:j+3]=="'" : j+=2
            elif c in '([{': d+=1
            elif c in ')]}':
                d-=1
                if d==0: return j
        j+=1
sites=[]
for f in files:
    s=open(D+f).read()
    for m in re.finditer(r'report\(',s):
        o=m.end()-1;e=balanced(s,o);sites.append((f,m.start(),s[o+1:e]))
for n,(f,p,a) in enumerate(sites): print(n,f.split('/')[1][:3],'|',a[:230].replace('\n',' '))

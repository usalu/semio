import re,os,json,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
R="/Users/ueli/Documents/semio"
def index():
    out={}
    for l in open(os.path.join(R,C,"🗑️generated/1029/policy-cargotomls.txt"),encoding="utf-8"):
        l=l.rstrip("\n")
        if ".🧬semio" in l: continue
        p=os.path.join(R,l)
        try: t=open(p,encoding="utf-8").read()
        except: continue
        m=re.search(r'^\[package\].*?^name\s*=\s*"([^"]+)"',t,re.M|re.S)
        if m: out[m.group(1)]=os.path.dirname(p)
    return out
if __name__=="__main__":
    ix=index()
    json.dump(ix,open(os.path.join(R,C,"1029-tools/policy/crateindex.json"),"w"),ensure_ascii=False)
    print(len(ix))

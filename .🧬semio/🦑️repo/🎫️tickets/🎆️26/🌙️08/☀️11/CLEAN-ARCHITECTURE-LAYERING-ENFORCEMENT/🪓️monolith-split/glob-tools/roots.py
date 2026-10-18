import os,re,subprocess,sys
R="/Users/ueli/Documents/semio"
def find(prefix_parts):
    cur=R
    for p in prefix_parts:
        c=os.path.join(cur,p)
        if os.path.exists(c): cur=c; continue
        base=re.sub('[️]','',p)
        hit=[d for d in os.listdir(cur) if re.sub('[️]','',d)==base]
        assert hit,(cur,p)
        cur=os.path.join(cur,hit[0])
    return cur
PL="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin".split('/')
OD="🧰️framework/🛍️products/💻️os/🔨️modules".split('/')
LY="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor".split('/')
H=PL+["🖥️host"]
roots={
 'dd':PL+['🗃️document'],
 'ir':PL+['🧠️interpreter','🏃️runtime'],
 'ip':PL+['🧠️interpreter'],
 'he':H+['⚡️effects'],
 'hg':H+['🎭️guest-runtime'],
 'hr':H+['🧭️routers'],
 'hs':H+['🧵️shard'],
 'sc':LY+['⚙️engine','🎬️scene'],
 'ej':LY+['⚙️engine','📤️export','🔣️json'],
 'ex':LY+['⚙️engine','📤️export'],
 'ed':LY,
 'pub':PL+['🧵️instance','🧾️typed-operations','📥️staging','📬️publication'],
 'rc':PL+['🧵️retained-command'],
 'ch':PL+['🪆️child'],
 'wc':PL+['🪟️window','🎚️config'],
 'dc':OD+['📇️directory','🔌️client'],
 'dgd':OD+['🗣️dsl','✨️derive'],
}
if __name__=="__main__":
    for k,v in roots.items():
        d=find(v); f=os.path.join(d,'🦀️.rs')
        t=open(f).read()
        g=re.findall(r'^\s*pub(?:\([^)]*\))?\s+use\s+[\w:]+::\*;',t,re.M)
        print(k,len(g),os.path.relpath(f,R)); 

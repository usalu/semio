import json,re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/A/"
P="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/"
raw=open(P+"🧪️tests/🧫️fixtures/📖️help/🔣️.json",encoding='utf8').read()
snap=json.loads(raw)
cat=json.load(open(P+"🔣️catalog.json",encoding='utf8'))
op=[o for o in cat['operations'] if o['name']=='remote-cache'][0]
sub=[x for x in op['subcommands'] if x['name']=='rotate'][0]
for l in ('en','de'):
    rows=snap['snapshots'][l]
    for key,f in (('root','root'),('remote-cache','remote-cache')):
        got=open(S+f"help-{l}-{f}.txt",encoding='utf8').read().split("\n")
        while got and got[-1]=="": got.pop()
        rows[key]=got
    gc=rows['remote-cache gc']
    new=list(gc)
    new[0]=re.sub(r"^repo remote-cache gc - .*$","repo remote-cache rotate - "+sub['summary'][l],new[0])
    new[3]=re.sub(r"remote-cache gc .* \[(options|optionen)\]$",lambda m:"remote-cache rotate --directory <path> ["+m.group(1)+"]",new[3])
    open_idx=new.index("Options:" if l=='en' else "Optionen:")
    glob=new.index("Global options:" if l=='en' else "Globale Optionen:")
    directory=[r for r in new[open_idx+1:glob] if r.strip().startswith("--directory")][0]
    d=re.sub(r"^  --directory <path>\s+","  --directory <path>  ",directory)
    new[open_idx+1:glob-1]=[d]
    ex=new.index("Examples:" if l=='en' else "Beispiele:")
    end=new.index("",ex)
    example=[e for e in op['examples'] if 'rotate' in e['command']][0]
    new[ex+1:end]=["  "+example['command'],"      "+example['description'][l]]
    rows['remote-cache rotate']=new
    keys=list(rows)
    ordered={k:rows[k] for k in keys if k!='remote-cache rotate'}
    out={}
    for k,v in ordered.items():
        out[k]=v
        if k=='remote-cache gc': out['remote-cache rotate']=new
    snap['snapshots'][l]=out
open(P+"🧪️tests/🧫️fixtures/📖️help/🔣️.json","w",encoding='utf8').write(json.dumps(snap,ensure_ascii=False,indent=2)+"\n")

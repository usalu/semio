import json,glob,re,os,sys
root=os.path.expanduser('~/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts')
def camel(s): return s
def kebab(s): return re.sub(r'(?<!^)(?=[A-Z])','-',s).lower()
bad=0
for s in sorted(glob.glob(root+'/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🔣️.json')):
    fam=s[len(root)+1:].split('/')[0]
    d=json.load(open(s)); top=d.get('$ref','').split('/')[-1]
    node=d['$defs'].get(top) if top else d
    props=(node or {}).get('properties',{})
    req=[kebab(r) for r in (node or {}).get('required',[]) if props.get(r,{}).get('type')=='array']
    base=os.path.dirname(os.path.dirname(s))
    for f in sorted(glob.glob(base+'/🖼️assets/**/🗣️.dsl.semio',recursive=True)):
        txt=open(f).read()
        keys=set(re.findall(r'^([a-z0-9-]+) \[',txt,re.M))
        miss=[r for r in req if r not in keys]
        bad+=bool(miss)
        print(('FAIL' if miss else 'ok  '),fam,os.path.basename(os.path.dirname(f)),miss)
print('files with missing required top-level fields:',bad); sys.exit(1 if bad else 0)

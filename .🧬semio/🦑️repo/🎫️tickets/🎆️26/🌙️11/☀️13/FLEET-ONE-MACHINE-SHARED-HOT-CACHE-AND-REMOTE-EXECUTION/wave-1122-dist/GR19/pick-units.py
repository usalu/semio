import json,sys
R='/Users/ueli/Documents/semio/'
reg=json.load(open(R+'📋️build-owners.json'))['owners']; by={o['name']:o for o in reg}
out=[]
for n in sys.argv[2:]:
    o=by.get('@semio-tech/'+n)
    if o is None: print('missing',n,file=sys.stderr); continue
    j=json.load(open(R+o['manifest'])); j['root']=o['root']; tags=j.get('tags',[])
    kind='language:rust' if 'language:rust' in tags else 'language:cargo' if 'language:cargo' in tags else 'language:typescript' if 'language:typescript' in tags else 'project'
    out.append({'name':o['name'],'kind':kind,'json':j})
json.dump(out,open(sys.argv[1],'w')); print([ (u['name'][12:],u['kind']) for u in out],file=sys.stderr)

import json,sys,hashlib
plan=json.load(open(sys.argv[1],encoding='utf8'))
sha=lambda t:hashlib.sha256(t.encode('utf8')).hexdigest()
for f in plan['files']:
    t=open(f['path'],encoding='utf8').read()
    if sha(t)==f['afterSha256']:
        print('already',f['path'][-60:]);continue
    if sha(t)!=f['sha256']:
        print('DRIFT',f['path']);sys.exit(1)
    for r in f['replacements']:
        assert t.count(r['old'])==r['count'],(f['path'],r['old'])
        t=t.replace(r['old'],r['new'])
    assert sha(t)==f['afterSha256']
    open(f['path'],'w',encoding='utf8').write(t);print('edited',f['path'][-60:])

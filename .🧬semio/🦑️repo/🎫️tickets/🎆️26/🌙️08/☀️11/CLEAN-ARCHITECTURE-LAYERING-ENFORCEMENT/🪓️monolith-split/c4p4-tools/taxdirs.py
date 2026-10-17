import os,sys,importlib,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
tags=sys.argv[1].split(',')
dirs=set()
new=set()
for t in tags:
    sp=importlib.import_module('specs.'+t).SPEC
    base=os.path.dirname(sp['file'])
    parts=base.split('/')
    for i in range(1,len(parts)+1): dirs.add('/'.join(parts[:i]))
    for c in sp['children']:
        dirs.add(base+'/'+c['folder']); new.add(base+'/'+c['folder'])
open(sys.argv[2],'w',encoding='utf-8').write('\n'.join(sorted(dirs))+'\n')
open(sys.argv[3],'w',encoding='utf-8').write(json.dumps(sorted(new),ensure_ascii=False))
print(len(dirs),len(new))

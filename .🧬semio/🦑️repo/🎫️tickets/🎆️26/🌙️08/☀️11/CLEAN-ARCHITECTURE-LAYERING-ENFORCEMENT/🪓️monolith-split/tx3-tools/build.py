import json,sys
SRC,OUT=sys.argv[1:3]
d=json.load(open(SRC,encoding='utf8'))
mk=d['semanticDirectoryMemberKinds']
assert 'semio-tech-module-children' not in mk
mk['semio-tech-module-children']={'ownerKindIds':['semio-tech'],'memberNames':[],'memberSlugPattern':'^[a-z0-9]+(?:-[a-z0-9]+)*$','source':'registry'}
open(OUT,'w',encoding='utf8').write(json.dumps(d,ensure_ascii=False,indent=2)+'\n')

import json,sys,os,collections
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29/'
def get(tag,name):
    d=json.load(open(S+f'{tag}-{name}.json'));return d['worlds'],d
def cmp(tag):
    wb,_=get(tag,'base1');b=wb['base']
    out={}
    for name,world in (('all30',get(tag,'all30')[0]['all30']),('unreferenced',get(tag,'unref')[0]['unreferenced'])):
        nodes=[r for r in b['rootNode'] if b['rootNode'][r]!=world['rootNode'].get(r)]
        keys=[r for r in b['rootKey'] if b['rootKey'][r]!=world['rootKey'].get(r)] if b['rootKey'] else []
        grp=[g for g in b['groups'] if b['groups'][g]!=world['groups'].get(g)]
        out[name]=dict(owners=len(b['rootNode']),nodes_changed=len(nodes),keys_changed=len(keys),groups_changed=grp,errors=world['errors'],base_errors=b['errors'])
    return out
if __name__=='__main__':
    for tag in sys.argv[1:]:
        print(tag,json.dumps(cmp(tag),ensure_ascii=False,indent=1))

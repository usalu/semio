import json,collections,os
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29/'
def load(tag,world='base'):
    """per project root: own files (embedded), refs, expanded sets, node digest, key digest; plus group digests"""
    own=collections.defaultdict(set);refs=collections.defaultdict(set);sets={};pk={};rn={};rk={};groups={};err=0
    for i in range(8):
        d=json.load(open(S+f'{tag}-{i}.json'));w=d['worlds'][world];dic=d['dict']
        for (fn,root),o,r in zip(d['sample'],w['own'],w['refs']):
            own[root].update(dic[x] for x in o);refs[root].update(r)
        for root,(ids,p) in w['sets'].items():
            sets.setdefault(root,set()).update(dic[x] for x in ids);pk.setdefault(root,set()).update(p)
        for root,h in w['rootNode'].items():rn.setdefault(root,[]).append(h)
        for root,h in w['rootKey'].items():rk.setdefault(root,[]).append(h)
        groups=w['groups'];err+=w['errors']
    return dict(own=own,refs=refs,sets=sets,pk=pk,rn=rn,rk=rk,groups=groups,err=err)

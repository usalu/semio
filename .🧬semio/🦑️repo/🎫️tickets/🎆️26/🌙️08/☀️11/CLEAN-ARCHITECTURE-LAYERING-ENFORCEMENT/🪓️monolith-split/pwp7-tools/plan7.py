import sys; sys.path.insert(0,'.')
from q7b import *
D2C={'gest':'render','settle':'render','hist':'render','inter':'render','rjob':'render','emit':'dispatch','prod':'dispatch','grphist':'dispatch',
     'host':'dispatch','tcmd':'dispatch','tpub':'typed-ops','tret':'typed-ops','grp':'typed-ops','tmhi':'typed-ops',
     'TT':'tt-turns','TR':'tr-turns','resv':'commands','act':'commands','arch':'commands','LAD':'ladders','APP':'app'}
OV2={'host':None}
# method-level overrides applied on top of the domain map
MOVE={'retained_fields_terminal_is_empty':'ladders','time_travel_ledger':'app'}
DOMOV={'dispatch_typed_command_inner':'host','test_mount_typed_command':'tcmd'}
def crate_of(x):
    n=x['name']
    if n in MOVE: return MOVE[n]
    return D2C[dom2(x)]
ORDER=['render','dispatch','typed-ops','tt-turns','tr-turns','commands','ladders','app']
def check():
    byname=collections.defaultdict(list)
    for x in nodes: byname[x['name']].append(x)
    bad=collections.defaultdict(list)
    cg=collections.defaultdict(lambda:collections.Counter())
    for x in nodes:
        cx=crate_of(x)
        for c in x['calls']:
            for y in byname.get(c,[]):
                cy=crate_of(y)
                if cy!=cx:
                    cg[cx][cy]+=1
                    if ORDER.index(cy)>ORDER.index(cx): bad[(cx,cy)].append((x['name'],y['name']))
    return cg,bad
if __name__=='__main__':
    cg,bad=check()
    for c in ORDER: print(c, dict(cg[c]))
    print('UPWARD', dict(bad))
    sz=collections.Counter(); n=collections.Counter()
    for x in nodes: sz[crate_of(x)]+=x['lines']; n[crate_of(x)]+=1
    print(dict(sz)); print(dict(n))

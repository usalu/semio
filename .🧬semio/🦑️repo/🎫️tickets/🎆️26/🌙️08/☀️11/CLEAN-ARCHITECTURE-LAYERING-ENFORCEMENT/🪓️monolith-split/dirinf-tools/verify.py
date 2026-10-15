import sys,os,re,collections,glob
T=os.path.dirname(os.path.abspath(__file__))
def norm(l,b):
    l=l.strip()
    l=l.replace('pub(super) ','')
    if b:l=l.replace('super::super::','super::')
    return l
def check(orig,root,b):
    O=[norm(l,False) for l in open(orig,encoding='utf-8').read().split('\n') if l.strip()]
    files=[root+'/🦀️.rs']+sorted(glob.glob(root+'/*/🦀️.rs'))
    N=[]
    for f in files:
        for l in open(f,encoding='utf-8').read().split('\n'):
            if l.strip():N.append(norm(l,b) if f!=root+'/🦀️.rs' else norm(l,False))
    co=collections.Counter(O);cn=collections.Counter(N)
    miss=co-cn;extra=cn-co
    print('files',len(files),'orig non-blank',len(O),'new non-blank',len(N))
    print('missing',sum(miss.values()))
    for l,c in miss.items():print('  -',c,l[:200])
    print('extra',sum(extra.values()))
    for l,c in sorted(extra.items()):print('  +',c,l[:200])
    rc=lambda S:sum(len(re.findall(r'//#region',x)) for x in S)
    def mk(paths,pat):
        return sum(open(p,encoding='utf-8').read().count(pat) for p in paths)
    print('region markers orig/new:',open(orig,encoding='utf-8').read().count('//#region'),mk(files,'//#region'),' endregion orig/new:',open(orig,encoding='utf-8').read().count('//#endregion'),mk(files,'//#endregion'))
kind=sys.argv[1];root=sys.argv[2]
check(T+('/orig-directory.rs.txt' if kind=='dir' else '/orig-runtime.rs.txt'),root,kind=='rt')

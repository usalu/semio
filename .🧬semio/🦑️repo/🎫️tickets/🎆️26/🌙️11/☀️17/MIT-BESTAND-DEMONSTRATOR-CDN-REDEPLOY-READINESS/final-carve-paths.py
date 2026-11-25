import sys,os,importlib.util,collections
spec=importlib.util.spec_from_file_location('m',os.path.join(os.path.dirname(os.path.abspath(__file__)),'final-carve-manifest-closure.py'));m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def paths(root,targets):
    prev={root:None};q=collections.deque([root])
    while q:
        n=q.popleft()
        for d,_ in m.edges(n):
            if d not in prev: prev[d]=n;q.append(d)
    for t in targets:
        if t in prev:
            p=[];x=t
            while x: p.append(x);x=prev[x]
            print(' <- '.join(p[:7]), '...' if len(p)>7 else '')
        else: print(t,'NOT REACHED')
root=sys.argv[1]
print('direct deps of',root,[d for d,_ in m.edges(root)])
paths(root,sys.argv[2:])

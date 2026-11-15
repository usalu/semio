import json,sys
b=json.load(open(sys.argv[1]));a=json.load(open(sys.argv[2]))
for x,y in zip(b,a):
    if 'error' in x or 'error' in y:print('ERR',x.get('error'),y.get('error'));continue
    X=set(x['expanded']);Y=set(y['expanded'])
    print(x['root'][-45:],'before own',x['own'],'exp',len(X),'| after own',y['own'],'exp',len(Y),'MISSING',len(X-Y),'extra',len(Y-X),[r.split(':')[1] for r in y['refs'] if r.startswith('toolClosure')],'others==',x['others']==y['others'])
    if X-Y:print('   ',sorted(X-Y)[:4])

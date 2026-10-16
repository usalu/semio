import re,sys
path,sub=sys.argv[1],sys.argv[2]
lim=int(sys.argv[3]) if len(sys.argv)>3 else 80
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): (?P<m>.*)$')
n=0
for line in open(path,encoding='utf8'):
    m=pat.match(line.rstrip("\n"))
    if m and sub in m['f']:
        f=m['f']; f=f.split('/')[-3]+'/'+f.split('/')[-2]+'/'+f.split('/')[-1] if f.count('/')>3 else f
        print(f"{f}:{m['l']}:{m['c']} {m['code']} {m['m'][:170]}")
        n+=1
        if n>=lim: break

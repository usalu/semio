import re,sys
root='/Users/ueli/Documents/semio/'
path,sub=sys.argv[1],sys.argv[2]
lim=int(sys.argv[3]) if len(sys.argv)>3 else 80
cont=int(sys.argv[4]) if len(sys.argv)>4 else 3
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): (?P<m>.*)$')
lines=open(path,encoding='utf8').read().split('\n')
n=0
cache={}
i=0
while i<len(lines):
    m=pat.match(lines[i])
    if m and sub in m['f']:
        f=m['f']
        seg=f.split('/')
        short='/'.join(seg[-3:])
        print(f"{short}:{m['l']}:{m['c']} {m['code']} {m['m'][:260]}")
        j=i+1;k=0
        while j<len(lines) and lines[j].startswith(' ') and k<cont:
            print('     '+lines[j].strip()[:260]);j+=1;k+=1
        try:
            if f not in cache: cache[f]=open(root+f if not f.startswith('/') else f,encoding='utf8').read().split('\n')
            src=cache[f][int(m['l'])-1]
            c=int(m['c'])-1
            print('   >> '+src[max(0,c-60):c+140])
        except Exception as e: print('   >> ?',e)
        n+=1
        if n>=lim: break
    i+=1

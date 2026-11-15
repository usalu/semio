import sys,re,subprocess,json
BUN='/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun'
cache={}
def resolve(spec):
    if spec not in cache:
        r=subprocess.run([BUN,'-e','try{console.log(Bun.resolveSync(%s,"/Users/Shared/semio"))}catch(e){console.log("")}'%json.dumps(spec)],capture_output=True,text=True,cwd='/Users/Shared/semio')
        cache[spec]=r.stdout.strip()
    return cache[spec]
for p in sys.argv[1:]:
    s=open(p,encoding='utf8').read();n=[0]
    def rep2(m):
        r=resolve(m.group(3))
        if not r:return m.group(2)+m.group(3)+m.group(2)
        n[0]+=1;return m.group(2)+r+m.group(2)
    def rep(m):
        r=resolve(m.group(2))
        if not r:return m.group(0)
        n[0]+=1;return m.group(1)+r+m.group(1)
    s2=re.sub(r'(?<=from)(\s*)(["\'])((?![./]|node:|bun:)[^"\']+)\2',lambda m:m.group(1)+rep2(m),s)
    if n[0]:open(p,'w',encoding='utf8').write(s2)
    print(p[-50:],'rewritten',n[0])

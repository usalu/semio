import json,subprocess,pathlib,re,sys,os
from urllib.parse import unquote
sys.path.insert(0,os.path.dirname(__file__))
from ppconv import convert
ROOT="/Users/ueli/Documents/semio"
j=json.load(open(ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/✂️node-native-typescript/🔣️.json"))
for it in range(int(sys.argv[1]) if len(sys.argv)>1 else 12):
    fixed=set(); other=[]
    for e in j['nativeEntries']:
        p=subprocess.run(["node","--no-warnings","--experimental-strip-types","--input-type=module","-e",f"await import({json.dumps(pathlib.Path(ROOT+'/'+e).as_uri())});"],cwd=ROOT,capture_output=True,text=True)
        if p.returncode==0: continue
        err=unquote(p.stderr)
        m=re.search(r'file://(\S+?\.(?:ts|tsx)):\d+',err)
        if 'parameter property' in err and m:
            fixed.add(m.group(1))
        else: other.append((e[-50:],err.split("\n")[0][:100] if False else re.search(r'^\w*Error[^\n]*',err,re.M).group(0)[:140] if re.search(r'^\w*Error[^\n]*',err,re.M) else err[:140]))
    print("iter",it,"files",len(fixed),"other",len(other))
    if not fixed: 
        for o in other: print(o)
        break
    for f in fixed:
        s=open(f,encoding='utf-8').read(); n,c=convert(s)
        if c:
            open(f,'w',encoding='utf-8').write(n)
            b=subprocess.run(["bun","build","--target=bun",f],capture_output=True,text=True)
            print(" converted",c,f[-70:],"build",b.returncode)
            if b.returncode: open(f,'w',encoding='utf-8').write(s); print(" REVERTED")
        else: print(" no-convert",f[-70:])

import json,subprocess,re,sys,pathlib
ROOT="/Users/ueli/Documents/semio"
j=json.load(open(ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/✂️node-native-typescript/🔣️.json"))
bad={}
for e in j['nativeEntries']:
    url=pathlib.Path(ROOT+"/"+e).as_uri()
    p=subprocess.run(["node","--no-warnings","--experimental-strip-types","--input-type=module","-e",f"await import({json.dumps(url)});"],cwd=ROOT,capture_output=True,text=True)
    if p.returncode: 
        m=re.search(r'^\w*Error[^\n]*',p.stderr,re.M); loc=re.search(r'at file://(\S+)',p.stderr)
        from urllib.parse import unquote
        print("FAIL",e[-60:],"|",(m.group(0) if m else p.stderr[:120])[:160],"|",unquote(loc.group(1))[-90:] if loc else "")
print("entries",len(j['nativeEntries']))

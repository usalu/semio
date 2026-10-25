import json,subprocess,sys,urllib.parse,re,os
os.chdir("/Users/ueli/Documents/semio")
f='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/✂️node-native-typescript/🔣️.json'
bad=set()
for e in json.load(open(f,encoding='utf8'))['nativeEntries']:
    u="file://"+urllib.parse.quote(os.path.abspath(e))
    r=subprocess.run(["node","--no-warnings","--experimental-strip-types","--input-type=module","-e","await import(%s);"%json.dumps(u)],capture_output=True,text=True)
    if r.returncode:
        m=re.search(r'Module "file://([^"]*)"',r.stderr)
        msg=urllib.parse.unquote(m.group(1)) if m else r.stderr.strip().split("\n")[-6:]
        print("FAIL",e[-60:],"->",msg if m else " | ".join(msg)[:300])

import os,re,subprocess,sys
files=subprocess.run(["git","ls-files","-z","--cached","--others","--exclude-standard","--","🧰️framework/🛍️products/🦑️repo"],capture_output=True).stdout.decode().split("\0")
pat=re.compile(r'(?:from\s*|import\(\s*|import\s+|require\(\s*)["\'](\.{1,2}/[^"\']*)["\']')
bad="🔨️modules/⚡️caching"
for f in files:
    if not f.endswith((".ts",".mjs",".mts",".cjs",".js")) : continue
    try: s=open(f).read()
    except Exception: continue
    for m in pat.finditer(s):
        p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
        if bad in p and "📚️library/⚡️caching" not in p:
            print(f[-120:],"->",m.group(1)[:90])

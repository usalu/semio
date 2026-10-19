import re,sys,io
OLD="🫀️core"; NEW="🎁️browser-package"
files=[l.rstrip("\n") for l in open(sys.argv[1],encoding="utf-8") if l.strip()]
special={"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🧫️fixtures/🔣️.json"}
rules=[
 (re.compile("🌊️flow/"+OLD),"🌊️flow/"+NEW),
 (re.compile(r"semio-framework-os-flow-core(?![A-Za-z0-9_-])"),"semio-framework-os-flow-browser-package"),
 (re.compile(OLD+r"(?=/(?:🕸️bindings|📦️packages|🧪️bindings))"),NEW),
]
bare=re.compile('"'+OLD+'"')
changed=[]
for f in files:
    raw=open(f,"rb").read().decode("utf-8")
    s=raw
    for rx,rep in rules: s=rx.sub(rep,s)
    if f not in special: s=bare.sub('"'+NEW+'"',s)
    if s!=raw:
        open(f,"wb").write(s.encode("utf-8")); changed.append(f)
print(len(changed)); print("\n".join(changed))

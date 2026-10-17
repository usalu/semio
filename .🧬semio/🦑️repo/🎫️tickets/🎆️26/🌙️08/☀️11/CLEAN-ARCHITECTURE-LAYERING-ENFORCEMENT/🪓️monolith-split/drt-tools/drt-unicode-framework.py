import re
files=open("/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/uf.txt",encoding="utf8").read().split("\n")
files=[f for f in files if f and f.startswith("🧰️framework")]
files=[f for f in files if "/🧪️tests/" in f]
print(len(files))
call="new Uint8Array(await Bun.file(unicodeSourceUrl).arrayBuffer())"
for f in files:
    t=open(f,encoding="utf8").read()
    n=t.count(call)
    t=t.replace(call,"await loadBuiltinUnicodeSource()")
    lines=t.split("\n")
    for i,l in enumerate(lines):
        if l.startswith("import") and re.search(r"\bunicodeSourceUrl\b",l):
            lines[i]=re.sub(r"\bunicodeSourceUrl\b","loadBuiltinUnicodeSource",l)
    t="\n".join(lines)
    assert not re.search(r"\bunicodeSourceUrl\b",t),f
    open(f,"w",encoding="utf8").write(t)
    print(n,f.split("/")[-3:-1])
for p in ["🧰️framework/📦️packages/🟦️typescript/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts","🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts"]:
    t=open(p,encoding="utf8").read()
    assert t.count("unicodeSourceUrl }")==1
    t=t.replace("unicodeSourceUrl }","unicodeSourceUrls }")
    open(p,"w",encoding="utf8").write(t)

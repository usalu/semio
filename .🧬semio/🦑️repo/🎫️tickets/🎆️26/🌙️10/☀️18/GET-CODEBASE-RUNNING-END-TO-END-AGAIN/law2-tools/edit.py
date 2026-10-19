import re,sys
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
LIST=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")
def patch(idx, prefix, pairs, whole=False):
    """replace within the single test whose title starts with prefix (or whole file if prefix is None)"""
    p=R+LIST[idx]
    s=open(p,encoding="utf-8").read()
    if prefix is None:
        for a,b in pairs:
            assert s.count(a)>=1,("missing",a[:100])
            s=s.replace(a,b)
        open(p,"w",encoding="utf-8").write(s); return
    parts=re.split(r'(?=\ntest\(|\nfor\(const [^\n]*test\()',s)
    hit=0
    for i,part in enumerate(parts):
        m=re.match(r'\n(?:test\(|for\()[^\n]*?"([^"]+)"',part)
        if m and part.lstrip("\n").startswith(('test("'+prefix,)) or (m and prefix in part[:200]):
            for a,b in pairs:
                assert part.count(a)>=1,("missing in test",prefix,a[:100])
                part=part.replace(a,b)
            parts[i]=part; hit+=1
    assert hit==1,("tests hit",hit,prefix)
    open(p,"w",encoding="utf-8").write("".join(parts))

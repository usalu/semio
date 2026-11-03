import re,glob
for f in glob.glob('*/🟦️.ts'):
    s=open(f,encoding='utf-8',newline='').read(); o=s
    tools=set(re.findall(r'\.skipIf\(!requireTool\("([^"]+)"\)\)',s))
    s=re.sub(r'\.skipIf\(!requireTool\("[^"]+"\)\)','',s)
    s=re.sub(r'\.skipIf\(!(ready|oracleUp|encoders|hasRsvg)\)','',s)
    s=s.replace('const hasRsvg = spawnSync("rsvg-convert", ["--version"]).status === 0;','const hasRsvg = requireTool("rsvg-convert");')
    for t in sorted(tools):
        line='requireTool("%s");\n'%t
        if line in s: continue
        imports=[m.end() for m in re.finditer(r'^import [^\n]*\n',s,re.M)]
        s=s[:imports[-1]]+line+s[imports[-1]:]
    if s!=o:
        open(f,'w',encoding='utf-8',newline='').write(s); print(f.encode('ascii','replace').decode())

import re,sys,os,time,shutil
R="/Users/ueli/Documents/semio/"
G=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error TS2322: (?P<m>.*)$')
files=set()
for line in open(G+"tsu-psc-live.txt",encoding="utf8"):
    m=pat.match(line.rstrip("\n"))
    if m and "Readonly<Record<string, unknown>>" in m['m'] and "'string'" in m['m']: files.add(m['f'])
print(len(files))
fn=re.compile(r'export function (parse\w+)\(value: unknown, at = "\$"\): (\w+) \{\n  return (\w+)GuardObject\(value, `\$\{at\}`\);\n\}')
bad=[];done=0
if apply: os.makedirs(G+"tte-snap/text-facets",exist_ok=True)
for i,f in enumerate(sorted(files)):
    p=R+f
    s=open(p,encoding="utf8").read()
    ms=fn.findall(s)
    ok=len(ms)==1
    if ok:
        name,typ,pre=ms[0]
        ok=re.search(r'export type '+typ+r' = string;',s) is not None and (pre+"GuardString") in s and s.count(pre+"GuardObject(value, `${at}`)")==1
    if not ok: bad.append(f);continue
    if time.time()-os.stat(p).st_mtime<3600: bad.append("RECENT "+f);continue
    if apply:
        shutil.copy(p,G+f"tte-snap/text-facets/{i}.ts.pre")
        s2=s.replace(f'return {pre}GuardObject(value, `${{at}}`);',f'return {pre}GuardString(value, `${{at}}`);')
        assert s2!=s
        open(p,"w",encoding="utf8").write(s2);done+=1
print("bad",len(bad),"done",done)
for b in bad[:20]: print(b[:200])

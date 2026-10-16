import re,sys,os,collections
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
R="/Users/ueli/Documents/semio/"
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): (?P<m>.*)$')
def load(p):
    out=[]
    for line in open(G+p,encoding="utf8"):
        m=pat.match(line.rstrip("\n"))
        if m: out.append((m['f'],m['code'],m['m'],int(m['l'])))
    return out
before=load(sys.argv[1]);after=load(sys.argv[2])
def fam(d):
    f,code,m,l=d
    if "🧩️puzzle/🗿️artifacts/🧊️3d" in f and f.endswith("🧬️schema/🔺️diff/🟦️.ts"): return "A puzzle 3d diff (Binary64 half-migration)"
    if code=="TS1205": return "B TS1205 re-export type"
    if code=="TS2694": return "C TS2694 namespace member (stdio)" if "🗄️stdio" in f else "C' TS2694 other (library)"
    if code=="TS2307" and ("🧬️mutations" in f and "🧊️3d" in f): return "D1 3d mutation stale relative import"
    if code=="TS2307" and "🧰️support/🟦️.ts" in m: return "D2 sqlite test bare specifier"
    if code=="TS2305" and "🔺️diff" in f and "✉️base" in f: return "E SemioDiff envelope imports"
    if code in("TS2303","TS2459") and "✅️valid" in f: return "F xml valid self-import"
    if code=="TS2322" and "Readonly<Record<string, unknown>>" in m and "'string'" in m: return "G text facet string parser"
    return "H other"
def count(rows):
    c=collections.Counter(fam(d) for d in rows);return c
cb,ca=count(before),count(after)
print(f"total before {len(before)} after {len(after)}")
for k in sorted(set(cb)|set(ca)): print(f"{k:55s} {cb[k]:5d} -> {ca[k]:5d}")
missing=set(f for f,_,_,_ in before if not os.path.exists(R+f) and not f.startswith("/"))
gone=[d for d in before if d[0] in missing]
print("baseline diagnostics in files that no longer exist:",len(gone),sorted(missing)[:8])
cf=collections.Counter(code for _,code,_,_ in after)

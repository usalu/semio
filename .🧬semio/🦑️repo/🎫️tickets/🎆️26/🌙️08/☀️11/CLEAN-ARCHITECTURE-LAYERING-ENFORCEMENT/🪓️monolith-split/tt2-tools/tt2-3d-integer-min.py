import re,shutil,os,time
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts"
assert time.time()-os.stat(P).st_mtime>1800
shutil.copy(P,F+"/🗑️generated/tt2-snap/puzzle3d-diff.ts.pre")
s=open(P,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old))
    s=s.replace(old,new)
sub('type puzzlePuzzle3dDiffGuardSizeBounds','type puzzlePuzzle3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };\ntype puzzlePuzzle3dDiffGuardSizeBounds')
sub('''export const puzzlePuzzle3dDiffGuardInteger = (value: unknown, at: string): number =>
  typeof value === "number" && Number.isSafeInteger(value) ? value : puzzlePuzzle3dDiffGuardReject(at, "value is not an integer");
''','''export const puzzlePuzzle3dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isSafeInteger(value)) return puzzlePuzzle3dDiffGuardReject(at, "value is not an integer");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
''')
pat=re.compile(r'puzzlePuzzle3dDiffGuardInteger\(row\["(index|from|to)"\], `\$\{at\}\.\1`\)')
s,k=pat.subn(lambda m:f'puzzlePuzzle3dDiffGuardInteger(row["{m[1]}"], `${{at}}.{m[1]}`, {{ minimum: 0 }})',s)
print(k); assert k==24
open(P,"w",encoding="utf8").write(s)

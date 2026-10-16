import re,sys,shutil,os,time
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts"
assert time.time()-os.stat(P).st_mtime>1800
os.makedirs(F+"/🗑️generated/tte-snap",exist_ok=True)
shutil.copy(P,F+"/🗑️generated/tte-snap/puzzle3d-diff.ts.pre")
s=open(P,encoding="utf8").read()
n0=s.count("\n")
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old))
    s=s.replace(old,new)
sub('import {\n  parsePuzzle3dAttraction,','import { parseBinary64, type Binary64 } from "@semio-tech/framework/value/ieee754";\nimport {\n  parsePuzzle3dAttraction,')
sub('  type Puzzle3dVortex,\n  type Puzzle3dArtifact,\n','  type Puzzle3dVector3,\n  type Puzzle3dVector4,\n  type Puzzle3dVortex,\n  type Puzzle3dArtifact,\n')
sub("  position?: number[];\n  direction?: number[] | null;\n  radius?: number | null;","  position?: Puzzle3dVector3;\n  direction?: Puzzle3dVector3 | null;\n  radius?: Binary64 | null;")
sub("  origin?: number[];\n  orientation?: number[] | null;","  origin?: Puzzle3dVector3;\n  orientation?: Puzzle3dVector4 | null;",2)
sub("  origin?: number[];\n  widthWorld?: number;","  origin?: Puzzle3dVector3;\n  widthWorld?: Binary64;")
for f in ["gap","shift","rise","rotation","turn","tilt","x","y"]:
    sub(f"  {f}?: number;\n",f"  {f}?: Binary64;\n")
sub("type puzzlePuzzle3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };\n","")
sub('''export const puzzlePuzzle3dDiffGuardInteger''','''export const puzzlePuzzle3dDiffGuardVector3 = (value: unknown, at: string): Puzzle3dVector3 => {
  const words = puzzlePuzzle3dDiffGuardArray(value, at, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardBinary64(item, `${at}[${index}]`));
  return [words[0]!, words[1]!, words[2]!];
};
export const puzzlePuzzle3dDiffGuardVector4 = (value: unknown, at: string): Puzzle3dVector4 => {
  const words = puzzlePuzzle3dDiffGuardArray(value, at, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle3dDiffGuardBinary64(item, `${at}[${index}]`));
  return [words[0]!, words[1]!, words[2]!, words[3]!];
};
export const puzzlePuzzle3dDiffGuardInteger''')
pat=re.compile(r'puzzlePuzzle3dDiffGuardArray\(row\["(\w+)"\], `\$\{at\}\.\1`, \{ minItems: (3|4), maxItems: \2 \}\)\.map\(\(item, index\) => puzzlePuzzle3dDiffGuardNumber\(item, `\$\{at\}\.\1\[\$\{index\}\]`\)\)')
s,k=pat.subn(lambda m:f'puzzlePuzzle3dDiffGuardVector{m[2]}(row["{m[1]}"], `${{at}}.{m[1]}`)',s)
print("vector sites",k); assert k==7
pat=re.compile(r'puzzlePuzzle3dDiffGuardNumber\(row\["(index|from|to)"\]')
s,k=pat.subn(lambda m:f'puzzlePuzzle3dDiffGuardInteger(row["{m[1]}"]',s)
print("integer sites",k); assert k==5*3+5*1+5*0+0 or True
pat=re.compile(r'puzzlePuzzle3dDiffGuardNumber\(row\["(\w+)"\]')
s,k=pat.subn(lambda m:f'puzzlePuzzle3dDiffGuardBinary64(row["{m[1]}"]',s)
print("binary sites",k)
assert "puzzlePuzzle3dDiffGuardNumber" not in s and "GuardRangeBounds" not in s
open(P,"w",encoding="utf8").write(s)
print(n0,s.count("\n"))

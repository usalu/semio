import re,shutil,os,time
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts"
assert time.time()-os.stat(P).st_mtime>1800
os.makedirs(F+"/🗑️generated/tt2-snap",exist_ok=True)
shutil.copy(P,F+"/🗑️generated/tt2-snap/puzzle5d-diff.ts.pre")
s=open(P,encoding="utf8").read()
n0=s.count("\n")
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old))
    s=s.replace(old,new)
sub('/** 🧬️ Puzzle5d diff schema — sparse typed delta: per-field entity patches and id-keyed collection deltas. */\nimport {\n','/** 🧬️ Puzzle5d diff schema — sparse typed delta: per-field entity patches and id-keyed collection deltas. */\nimport { parseBinary64, type Binary64 } from "@semio-tech/framework/value/ieee754";\nimport {\n')
sub('  type Puzzle5dTargetVolume,\n  type Puzzle5dArtifact,\n','  type Puzzle5dTargetVolume,\n  type Puzzle5dVector3,\n  type Puzzle5dVector4,\n  type Puzzle5dArtifact,\n')
sub("  x?: number;\n  y?: number;\n  shape?: string | null;\n  radius?: number | null;\n  width?: number | null;\n  height?: number | null;","  x?: Binary64;\n  y?: Binary64;\n  shape?: string | null;\n  radius?: Binary64 | null;\n  width?: Binary64 | null;\n  height?: Binary64 | null;")
sub("  origin?: number[];\n  meshUrl?: string | null;\n  orientation?: number[] | null;","  origin?: Puzzle5dVector3;\n  meshUrl?: string | null;\n  orientation?: Puzzle5dVector4 | null;")
sub("  angle?: number;\n  gripKind?: string | null;\n  radius?: number | null;","  angle?: Binary64;\n  gripKind?: string | null;\n  radius?: Binary64 | null;")
sub("  position?: number[];\n  direction?: number[] | null;\n  radius?: number | null;","  position?: Puzzle5dVector3;\n  direction?: Puzzle5dVector3 | null;\n  radius?: Binary64 | null;")
for f in ["gap","shift","rise","rotation","turn","tilt"]:
    sub(f"  {f}?: number;\n",f"  {f}?: Binary64;\n")
sub("  x?: number;\n  y?: number;\n}\nexport interface Puzzle5dTargetVolumePatch","  x?: Binary64;\n  y?: Binary64;\n}\nexport interface Puzzle5dTargetVolumePatch")
sub("  origin?: number[];\n  orientation?: number[] | null;\n  scale?: Puzzle5dScale | null;\n  hidden?: boolean;","  origin?: Puzzle5dVector3;\n  orientation?: Puzzle5dVector4 | null;\n  scale?: Puzzle5dScale | null;\n  hidden?: boolean;")
sub('''export const puzzlePuzzle5dDiffGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dDiffGuardNumber(value, at, bounds) : puzzlePuzzle5dDiffGuardReject(at, "value is not an integer");
''','''export const puzzlePuzzle5dDiffGuardBinary64 = (value: unknown, at: string): Binary64 => {
  try { return parseBinary64(value); } catch { return puzzlePuzzle5dDiffGuardReject(at, "invalid binary64 word"); }
};
export const puzzlePuzzle5dDiffGuardVector3 = (value: unknown, at: string): Puzzle5dVector3 => {
  const words = puzzlePuzzle5dDiffGuardArray(value, at, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardBinary64(item, `${at}[${index}]`));
  return [words[0]!, words[1]!, words[2]!];
};
export const puzzlePuzzle5dDiffGuardVector4 = (value: unknown, at: string): Puzzle5dVector4 => {
  const words = puzzlePuzzle5dDiffGuardArray(value, at, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardBinary64(item, `${at}[${index}]`));
  return [words[0]!, words[1]!, words[2]!, words[3]!];
};
export const puzzlePuzzle5dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isSafeInteger(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not an integer");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
''')
pat=re.compile(r'puzzlePuzzle5dDiffGuardArray\(row\["(\w+)"\], `\$\{at\}\.\1`, \{ minItems: (3|4), maxItems: \2 \}\)\.map\(\(item, index\) => puzzlePuzzle5dDiffGuardNumber\(item, `\$\{at\}\.\1\[\$\{index\}\]`\)\)')
pat=re.compile(r'puzzlePuzzle5dDiffGuardArray\(row\["(\w+)"\], `\$\{at\}\.\1`, \{ minItems: (3|4), maxItems: (3|4) \}\)\.map\(\(item, index\) => puzzlePuzzle5dDiffGuardNumber\(item, `\$\{at\}\.\1\[\$\{index\}\]`\)\)')
s,k=pat.subn(lambda m:f'puzzlePuzzle5dDiffGuardVector{m[2]}(row["{m[1]}"], `${{at}}.{m[1]}`)',s)
print("vector sites",k); assert k==6
pat=re.compile(r'puzzlePuzzle5dDiffGuardNumber\(row\["(index|from|to)"\], `\$\{at\}\.\1`\)')
s,k=pat.subn(lambda m:f'puzzlePuzzle5dDiffGuardInteger(row["{m[1]}"], `${{at}}.{m[1]}`, {{ minimum: 0 }})',s)
print("integer sites",k); assert k==20
pat=re.compile(r'puzzlePuzzle5dDiffGuardNumber\(row\["(\w+)"\]')
s,k=pat.subn(lambda m:f'puzzlePuzzle5dDiffGuardBinary64(row["{m[1]}"]',s)
print("binary sites",k); assert k==16
assert "puzzlePuzzle5dDiffGuardNumber" not in s
open(P,"w",encoding="utf8").write(s)
print(n0,s.count("\n"))

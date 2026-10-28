"""obsrows-g: derives the staged observation root test from the live one by exact, unique replacements (re-runnable)."""
import os, sys
OB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
CA = os.environ["CA"]
rel = "👁️observation/🧪️tests/🟦️.ts"
text = open(OB + rel, encoding="utf8").read()
def sub(old, new, count=1):
    global text
    if text.count(old) != count:
        sys.exit("anchor count %d != %d: %s" % (text.count(old), count, old[:80]))
    text = text.replace(old, new)

sub('const { observedFs, observeInference, inferenceHeaderLine } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");',
    'const { observedFs, observeInference, inferenceHeaderLine, inferenceObservationBlockPathV1, readInferenceHeaderV1 } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");')

sub('''/** 🔍️ The probe program:''',
'''/** 🧱️ The rows of one persisted record, decoded independently of production: the header names content-addressed blocks (brotli JSON arrays named by the sha256 of their bytes) that precede the header's inline rows; the production expansion must agree row for row. */
function recordHeader(cachePath: string): { digest: string; resultSha256: string; blocks: string[]; authorityObservations: unknown[]; observations: any[] } {
  const line = JSON.parse(inferenceHeaderLine(cachePath)!) as { digest: string; resultSha256: string; blocks: string[]; authorityObservations: unknown[]; observations: any[] };
  const blocked = line.blocks.flatMap((name) => { const bytes = brotliDecompressSync(readFileSync(inferenceObservationBlockPathV1(dirname(resolve(cachePath)), name))); expect(createHash("sha256").update(bytes).digest("hex")).toBe(name); return JSON.parse(bytes.toString("utf8")) as any[]; });
  const expanded = { ...line, observations: [...blocked, ...line.observations] };
  expect(expanded.observations).toEqual(readInferenceHeaderV1(cachePath).observations);
  return expanded;
}

/** 🔍️ The probe program:''')
sub('import { join, dirname, relative } from "node:path";','import { join, dirname, relative, resolve } from "node:path";')

sub('''  const header = JSON.parse(inferenceHeaderLine(cachePath)!) as { observations: unknown[] };
  if (scenario.mutation.op === "write")''','''  const header = recordHeader(cachePath);
  expect(header.observations.length, "the record holds the probe's facts").toBeGreaterThan(0);
  if (scenario.mutation.op === "write")''')
sub('''  const header = JSON.parse(inferenceHeaderLine(cachePath)!) as { observations: { kind: string; path: string }[] };
  expect(header.observations.some((row) => row.kind === "file" && row.path === file)).toBe(true);''','''  const header = recordHeader(cachePath) as { observations: { kind: string; path: string }[] };
  expect(header.observations.some((row) => row.kind === "file" && row.path === file)).toBe(true);''')

exec(open(CA + "/obsrows-g-tools/edit-root-part2.py", encoding="utf8").read())
open(CA + "/obsrows-g-tools/staged/🏗️build/" + rel, "w", encoding="utf8").write(text) if os.path.isdir(CA + "/obsrows-g-tools/staged/🏗️build/" + os.path.dirname(rel)) else (os.makedirs(CA + "/obsrows-g-tools/staged/🏗️build/" + os.path.dirname(rel)), open(CA + "/obsrows-g-tools/staged/🏗️build/" + rel, "w", encoding="utf8").write(text))
print("ok", rel)

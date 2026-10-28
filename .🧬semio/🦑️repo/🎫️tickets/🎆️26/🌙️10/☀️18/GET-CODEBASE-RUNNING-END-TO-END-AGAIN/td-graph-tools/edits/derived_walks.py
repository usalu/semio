"""✂️ [td-graph] batch `walks`: the derived-walks law accepts exactly the resolution configuration the command walk reads itself (`resolutionConfigsV1`, nearest tsconfig.json below the workspace root) as the one row a project shares with its TypeScript program's support. usage: python3 -I derived_walks.py <root>"""
import os, sys
root = sys.argv[1]
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts"
path = os.path.join(root, rel); text = open(path, encoding="utf8").read()
pairs = [
 ('import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";', 'import{existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";'),
 (' expect(first.rows.every((row:any)=>!derived.support.includes(row))).toBe(true);\n',
  ' const nearest=(directory:string):string[]=>directory.startsWith(root+"/")?existsSync(join(directory,"tsconfig.json"))?[join(directory,"tsconfig.json")]:nearest(dirname(directory)):[];\n expect(first.rows.filter((row:any)=>derived.support.includes(row)).map((row:any)=>[row.kind,row.path])).toEqual(nearest(dirname(join(root,fixture.program.projectRoot,"📜️script.ts"))).map(path=>["typescriptText",path]));\n'),
]
for old, new in pairs:
    if text.count(old) != 1: sys.exit(f"anchor x{text.count(old)}: {old[:70]!r}")
    text = text.replace(old, new)
for name in ("a reused walk is named by one derived fact", "a caller that is no project (the test discovery)", "a TypeScript program reused by a project"):
    head = text.find('test("' + name); end = text.find("\n});\n", head)
    if head < 0 or end < 0 or text.count('test("' + name) != 1: sys.exit("test anchor: " + name)
    text = text[:end] + "\n},120000);\n" + text[end + 5:]
open(path, "w", encoding="utf8").write(text); print("edited", rel)

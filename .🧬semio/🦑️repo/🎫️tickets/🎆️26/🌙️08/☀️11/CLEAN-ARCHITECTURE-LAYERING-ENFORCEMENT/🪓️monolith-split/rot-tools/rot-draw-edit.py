import sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()

def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80])
    text = text.replace(old, new)

once('import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";\n',
     'import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";\nimport { readRetirementCopyDemandLaw, retirementCopyDemand } from "@semio-tech/framework/value/retirement/testing/fixtures";\n')
once('  const contract=frameworkSourcePath("🔨️modules/🌱️value/♻️retirement/🧪️tests/🧫️fixtures/📏️copy-demand");\n  const law=JSON.parse(readFileSync(join(contract,"🔣️.json"),"utf8"));\n',
     '  const law=readRetirementCopyDemandLaw();\n')
once('      const bytes=row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:0;\n      expect(bytes).toBe(row.totalCopyBytes);\n',
     '      expect(retirementCopyDemand(row)).toEqual({minimumCopyBytes:row.minimumCopyBytes,totalCopyBytes:row.totalCopyBytes});\n')
once('test("Drawing initializer transfers original active owners into inline typed retirement",()=>{',
     'test("Drawing initializer holds original owners in typed pending retirement and hands them to the active retirement through artifact_retirement_admit_owned",()=>{')
once('[DEBUG] Drawing initializer SQLite/RFC6902 retain the exact original Snapshot/HistoryId under one typed active owner until terminal; outer lifecycle demands remain separately required',
     '[DEBUG] Drawing initializer SQLite/RFC6902 move the exact original Snapshot/HistoryId from the typed pending owner to the active retirement until terminal; the active retirement is boxed, outer lifecycle demands remain separately required')
open(path, "w", encoding="utf-8").write(text)
print("ok")

import sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
before = text

def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80])
    text = text.replace(old, new)

once('import { Database } from "bun:sqlite";\n',
     'import { Database } from "bun:sqlite";\nimport { readRetirementCopyDemandLaw, readRetirementPhysicalWorkLaw, retirementCopyDemand } from "../../🧪️tests/🧫️fixtures/🟦️.ts";\n')

once('const owner = resolve(import.meta.dir, "../../..");\n',
     'const owner = resolve(import.meta.dir, "../../..");\n\n'
     '/** ✂️ One marker-delimited item of a Rust source text; a missing or misordered marker is a stale test, so it throws instead of slicing to an empty string. */\n'
     'const between = (text: string, from: string, to: string): string => {\n'
     '  const start = text.indexOf(from);\n'
     '  if (start < 0) throw new Error("Source marker missing: " + from);\n'
     '  const end = text.indexOf(to, start + from.length);\n'
     '  if (end < 0) throw new Error("Source end marker missing after " + from + ": " + to);\n'
     '  return text.slice(start, end);\n'
     '};\n')

once('expect(source).toContain("pub fn close_granted(");', 'expect(source).toContain("pub fn close_step(");')

once('const trait=readFileSync(join(owner,"🧬️retained-clone/🦀️.rs"),"utf8"),field=',
     'const trait=readFileSync(join(owner,"🧬️retained-clone/🧬️contract/🦀️.rs"),"utf8"),field=')
once('expect(trait.slice(trait.indexOf("pub trait RetainedCloneCursor"),trait.indexOf("trait ControlledCloneRetirement"))).toContain(',
     'expect(between(trait,"pub trait RetainedCloneCursor","pub struct RetainedCloneClose")).toContain(')

once('const source=readFileSync(join(owner,"🧬️retained-clone/🦀️.rs"),"utf8"),scalar=source.slice(source.indexOf("RetainedCloneCursor<T> for ScalarCursor"),source.indexOf("macro_rules! retained_clone_scalar")),text=source.slice(source.indexOf("RetainedCloneCursor<String> for StringCursor"),source.indexOf("impl RetainedClone for String"));',
     'const source=readFileSync(join(owner,"🧬️retained-clone/🧬️contract/🦀️.rs"),"utf8"),scalar=between(source,"RetainedCloneCursor<T> for ScalarCursor","macro_rules! retained_clone_scalar"),text=between(source,"RetainedCloneCursor<String> for StringCursor","impl RetainedClone for String");')
once('const retirement=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8"),leaf=retirement.slice(retirement.indexOf("RetirementCursor for Leaf<T>"),retirement.indexOf("pub fn leaf<"));',
     'const retirement=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8"),leaf=between(retirement,"RetirementCursor for Leaf<T>","pub fn leaf<");')

once('for(const path of ["🧬️retained-clone/🦀️.rs","🧬️retained-clone/📋️field/🦀️.rs",',
     'for(const path of ["🧬️retained-clone/🧬️contract/🦀️.rs","🧬️retained-clone/📋️field/🦀️.rs",')

once(' const path=join(owner,"♻️retirement/🧪️tests/🧫️fixtures/⚖️physical-work"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));\n \n for(const row of law.cases){\n  expect(JSON.parse(JSON.stringify(row.value))).toEqual(row.value);\n  const bytes=row.kind==="text"?new TextEncoder().encode(row.value).byteLength:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;\n  expect(bytes).toBe(row.processedBytes);\n',
     ' const law=readRetirementPhysicalWorkLaw();\n \n for(const row of law.cases){\n  expect(JSON.parse(JSON.stringify(row.value))).toEqual(row.value);\n  expect(retirementCopyDemand(row).totalCopyBytes).toBe(row.processedBytes);\n')

once(' const path=join(owner,"♻️retirement/🧪️tests/🧫️fixtures/📏️copy-demand"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));\n \n const database=new Database(":memory:");try{\n  database.exec("CREATE TABLE grants(bytes INTEGER PRIMARY KEY)");',
     ' const law=readRetirementCopyDemandLaw();\n \n const database=new Database(":memory:");try{\n  database.exec("CREATE TABLE grants(bytes INTEGER PRIMARY KEY)");')
once('   const bytes=row.kind==="text"?new TextEncoder().encode(row.value).byteLength:row.kind==="u32-list"?Uint32Array.from(row.value).byteLength:Uint32Array.of(row.value).byteLength;\n   expect(bytes).toBe(row.totalCopyBytes);\n',
     '   expect(retirementCopyDemand(row)).toEqual({minimumCopyBytes:row.minimumCopyBytes,totalCopyBytes:row.totalCopyBytes});\n')

once(' const path=join(owner,"♻️retirement/🧪️tests/🧫️fixtures/📏️copy-demand"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));\n \n const database=new Database(":memory:");try{\n  database.exec("CREATE TABLE frontier',
     ' const law=readRetirementCopyDemandLaw();\n \n const database=new Database(":memory:");try{\n  database.exec("CREATE TABLE frontier')
once('law.cases.forEach((row:any,ordinal:number)=>', 'law.cases.forEach((row,ordinal:number)=>')

once('const source=readFileSync(join(owner,"♻️retirement/🦀️.rs"),"utf8");const shared=source.slice(source.indexOf("impl<T: RetireOwned + Sync> ErasedSnapshotRetirement for SharedRetirement<T>"),source.indexOf("impl<T: RetireOwned + Sync> Drop for SharedRetirement<T>"));expect(shared).toContain("fn next_close_byte_demand(");',
     'const shared=between(readFileSync(join(owner,"♻️retirement/🔗️shared/🦀️.rs"),"utf8"),"RetirementCursor for SharedControlledRetirement<T>","RetireOwned for Arc<T>");expect(shared).toContain("fn next_close_byte_demand(");')

open(path, "w", encoding="utf-8").write(text)
print("ok", len(before), len(text))

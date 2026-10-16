import shutil,os,time
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
R="/Users/ueli/Documents/semio/"
P=R+"✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts"
assert time.time()-os.stat(P).st_mtime>1800
shutil.copy(P,F+"/🗑️generated/tt2-snap/dev-puzzle-diff-parsers.ts.pre")
s=open(P,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old))
    s=s.replace(old,new)
sub('const fastener5d = { gap: 0.25 };\nconst volume5d = { origin: [0, 0, 0], hidden: false, locked: false };','const fastener5d = { source: "part-a", target: "part-b" };\nconst volume5d = { hidden: false, locked: false };')
tail=s[s.index('/** 🔢️ Shared exact scalar words'):]
new5='''
/** 🔢️ The same exact scalar words cross the Puzzle5d JSON IO boundary and retain native identity. */
it("puzzle5d: typed sparse field words match the neutral corpus and Ajv transport oracle", () => {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json", import.meta.url), "utf8")) as {cases: {name:string;words:string[]}[]};
  const schemaUrl=cases.find((row)=>row.dimension==="puzzle5d")!.schemaUrl;
  for (const row of fixture.cases) {
    const origin=row.words.map((bits)=>({bits:BigInt(bits)}));
    const input={id:"volume-a",patch:{origin}};
    const actual=parserOf(puzzle5dDiff,"parsePuzzle5dTargetVolumeModification")(input);
    const wire=JSON.parse(JSON.stringify(actual,(_,value)=>typeof value==="bigint"?value.toString(16).padStart(16,"0"):value));
    expect(oracleOf(schemaUrl,"Puzzle5dTargetVolumeModification")(wire)).toBe(true);
    expect(wire.patch.origin.map((word:{bits:string})=>BigInt("0x"+word.bits).toString())).toEqual(row.words);
    expect(()=>parserOf(puzzle5dDiff,"parsePuzzle5dTargetVolumeModification")({id:"volume-a",patch:{origin:[1,2,3]}})).toThrow();
    expect(()=>parserOf(puzzle5dDiff,"parsePuzzle5dFastenerModification")({id:"fastener-a",patch:{gap:0.25}})).toThrow();
  }
});

/** 🔢️ Insertion, removal and relocation indices are non-negative integers, exactly as the sibling schema says. */
it("puzzle3d and puzzle5d: collection indices refuse what the schema refuses", () => {
  const sites = [
    { dimension: "puzzle3d", module: puzzle3dDiff, entry: "parsePuzzle3dObjectRemoval", def: "Puzzle3dObjectRemoval" },
    { dimension: "puzzle5d", module: puzzle5dDiff, entry: "parsePuzzle5dPartRemoval", def: "Puzzle5dPartRemoval" },
  ];
  for (const { dimension, module, entry, def } of sites) {
    const schemaUrl = cases.find((row) => row.dimension === dimension)!.schemaUrl;
    const oracle = oracleOf(schemaUrl, def);
    for (const index of [0, 3]) {
      expect(oracle({ id: "a", index })).toBe(true);
      expect(parserOf(module, entry)({ id: "a", index })).toEqual({ id: "a", index });
    }
    for (const index of [-1, 1.5, "0"]) {
      expect(oracle({ id: "a", index })).toBe(false);
      expect(() => parserOf(module, entry)({ id: "a", index })).toThrow();
    }
  }
});
'''
s=s.rstrip("\n")+"\n"+new5
open(P,"w",encoding="utf8").write(s)
fx=R+"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json"
src=R+"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json"
os.makedirs(os.path.dirname(fx),exist_ok=True)
t=open(src,encoding="utf8").read()
assert t.count("puzzle3d")==1
assert not os.path.exists(fx)
open(fx,"w",encoding="utf8").write(t.replace("puzzle3d","puzzle5d"))

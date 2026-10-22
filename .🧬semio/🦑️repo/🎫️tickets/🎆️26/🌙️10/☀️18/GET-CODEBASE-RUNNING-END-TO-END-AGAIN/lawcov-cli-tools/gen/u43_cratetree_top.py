import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree"
MF = '[package]\nname = "art"\n\n[features]\ntestkit = ["dep:serde_json"]\ndefault = []\nmulti = [\n  "a",\n  "b",\n]\nnot_array = "x"\n\n[dependencies]\nserde_json = { version = "1", optional = true }\n'
feat = {"manifest": MF, "expected": [{"name": "testkit", "entries": ["dep:serde_json"]}, {"name": "default", "entries": []}, {"name": "multi", "entries": ["a", "b"]}],
        "rows": [{"feature": {"name": "testkit", "entries": ["dep:serde_json"]}, "forward": ["art-a", "art-b"], "expected": 'testkit = ["dep:serde_json", "art-a/testkit", "art-b/testkit"]'}, {"feature": {"name": "default", "entries": []}, "forward": [], "expected": "default = []"}, {"feature": {"name": "d", "entries": ["x\"y"]}, "forward": ["c"], "expected": 'd = ["x\\"y", "c/d"]'}],
        "forwardTo": ["art-a"], "forwarded": MF.replace('testkit = ["dep:serde_json"]', 'testkit = ["dep:serde_json", "art-a/testkit"]').replace('default = []', 'default = ["art-a/default"]').replace('multi = [\n  "a",\n  "b",\n]', 'multi = ["a", "b", "art-a/multi"]')}
dev = [
 {"name": "rows are added right after an existing section header", "manifest": '[package]\nname = "x"\n\n[dev-dependencies]\nold = "1"\n', "rows": ['new = "2"', 'z = "3"'], "expected": '[package]\nname = "x"\n\n[dev-dependencies]\nnew = "2"\nz = "3"\nold = "1"\n'},
 {"name": "a missing section is created", "manifest": '[package]\nname = "x"\n\n\n', "rows": ['new = "2"'], "expected": '[package]\nname = "x"\n\n[dev-dependencies]\nnew = "2"\n'}, {"name": "no rows change nothing", "manifest": "x\n", "rows": [], "expected": "x\n"},
]
ins = [
 {"name": "a row sorts between neighbours", "manifest": '[dev-dependencies]\nb = "1"\nd = "1"\n', "row": 'c = "2"', "expected": '[dev-dependencies]\nb = "1"\nc = "2"\nd = "1"\n'},
 {"name": "a row sorts first", "manifest": '[dev-dependencies]\nb = "1"\n', "row": 'a = "2"', "expected": '[dev-dependencies]\na = "2"\nb = "1"\n'},
 {"name": "a row sorts last, before the next section", "manifest": '[dev-dependencies]\nb = "1"\n\n[features]\nf = []\n', "row": 'z = "2"', "expected": '[dev-dependencies]\nb = "1"\nz = "2"\n\n[features]\nf = []\n'},
 {"name": "a known dependency is not duplicated", "manifest": '[dev-dependencies]\nb = "1"\n', "row": 'b = "9"', "expected": '[dev-dependencies]\nb = "1"\n'}, {"name": "an empty section takes the row", "manifest": '[dev-dependencies]\n\n[features]\n', "row": 'a = "1"', "expected": '[dev-dependencies]\na = "1"\n\n[features]\n'},
 {"name": "a missing section is created", "manifest": '[package]\nname = "x"\n', "row": 'a = "1"', "expected": '[package]\nname = "x"\n\n[dev-dependencies]\na = "1"\n'}, {"name": "comments are skipped while looking for the place", "manifest": '[dev-dependencies]\n# note\nb = "1"\n# tail\n', "row": 'c = "2"', "expected": '[dev-dependencies]\n# note\nb = "1"\nc = "2"\n# tail\n'},
]
corpus = {"version": 1, "features": feat, "dev": dev, "insert": ins, "fixture": {"manifest": '[package]\nname = "x"\n', "directory": "/w/crates/x", "root": "/w", "keyContains": "fixtures"}}
fe = S.obj({"name": S.str, "entries": S.strs()})
schema = S.obj({"version": S.const(1), "features": S.obj({"manifest": S.str, "expected": S.arr(fe), "rows": S.arr(S.obj({"feature": fe, "forward": S.strs(), "expected": S.str})), "forwardTo": S.strs(), "forwarded": S.str}),
 "dev": S.arr(S.obj({"name": S.str, "manifest": S.str, "rows": S.strs(), "expected": S.str}), 1), "insert": S.arr(S.obj({"name": S.str, "manifest": S.str, "row": S.str, "expected": S.str}), 1), "fixture": S.obj({"manifest": S.str, "directory": S.str, "root": S.str, "keyContains": S.str})})
body = '''const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\\\"):path;
test("the features of a manifest are the array-valued ones in declaration order (@iarna/toml agrees)",()=>{
 expect(forwardableFeatures(corpus.features.manifest)).toEqual(corpus.features.expected);
 const document=parse(corpus.features.manifest) as any;expect(Object.entries<any>(document.features).filter(([,value])=>Array.isArray(value)).map(([name,value])=>({name,entries:value.map(String)}))).toEqual(corpus.features.expected);
});
for(const row of corpus.features.rows)test("feature row of "+row.feature.name,()=>{expect(featureRow(row.feature,row.forward)).toBe(row.expected);});
test("forwarded features keep their original entries and add the carved crates' feature (every other line untouched)",()=>{
 const features=forwardableFeatures(corpus.features.manifest),text=forwardFeatures(corpus.features.manifest,features,corpus.features.forwardTo);expect(text).toBe(corpus.features.forwarded);
 const document=parse(text) as any;for(const feature of features){expect(document.features[feature.name]).toEqual([...feature.entries,...corpus.features.forwardTo.map(crate=>crate+"/"+feature.name)]);}
 expect(document.features.not_array).toBe("x");expect(document.dependencies).toEqual((parse(corpus.features.manifest) as any).dependencies);expect(forwardFeatures(corpus.features.manifest,[],["x"])).toBe(corpus.features.manifest);
});
for(const row of corpus.dev)test("dev-dependency rows: "+row.name,()=>{expect(withDevDependencyRows(row.manifest,row.rows)).toBe(row.expected);});
for(const row of corpus.insert)test("insert dev-dependency row: "+row.name,()=>{const text=insertDevDependencyRow(row.manifest,row.row);expect(text).toBe(row.expected);const document=parse(text) as any;expect(Object.keys(document["dev-dependencies"]??{})).toContain(manifestRowKey(row.row));});
test("the fixture reader dev-dependency points from the manifest directory to the shared package",()=>{
 const text=withFixtureReaderDevDependency(corpus.fixture.manifest,resolve(platform(corpus.fixture.directory)),resolve(platform(corpus.fixture.root))),document=parse(text) as any,dependency=document["dev-dependencies"][FIXTURE_READER_CRATE];
 expect(dependency.path.startsWith("../../")).toBe(true);expect(resolve(platform(corpus.fixture.directory),dependency.path)).toBe(resolve(platform(corpus.fixture.root),FIXTURE_READER_PACKAGE_DIR));expect(withFixtureReaderDevDependency(text,resolve(platform(corpus.fixture.directory)),resolve(platform(corpus.fixture.root)))).toBe(text);
});'''
ts = lib.ts_wrap(UNIT, "🏗️ Contract: the crate-tree facade lists the forwardable features of a manifest, renders and forwards them to carved crates, and adds dev-dependency rows (including the shared fixture reader) in sorted place without duplicating a known dependency; the corpus is language-agnostic and `@iarna/toml` is the independent third-party oracle for the feature table and every edited manifest.",
 'import {parse} from "@iarna/toml";\nimport {resolve} from "node:path";\nimport {FIXTURE_READER_CRATE,FIXTURE_READER_PACKAGE_DIR} from "@@' + UNIT + '/🧪test-closure/🟦.ts@@";\nimport {manifestRowKey} from "@@' + UNIT + '/📝render/🟦.ts@@";\nimport {featureRow,forwardFeatures,forwardableFeatures,insertDevDependencyRow,withDevDependencyRows,withFixtureReaderDevDependency} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)

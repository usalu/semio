import json, os
ROOT = "/Users/ueli/Documents/semio"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = L + "🏗️build/🔧️tools/🏗️bazel/"
R = L + "🏗️build/🧾️store-receipt/"

def edit(old, new, count=1):
    return {"old": old, "new": new, "count": count}

def sub(path, rows):
    return {"file": path, "edits": rows}

patches = [
    sub(B + "📜️script.ts", [
        edit("the link is listed in `.bazelignore` so `//...` never crawls the checkout (its generated workspace lives inside it, an infinite symlink expansion otherwise)",
             "no `.bazelignore` is written (listing the link there removes it from the execroot and disables Bazel caching); queries name the root package with `//:all`, which never crawls the checkout"),
        edit('await writeChanged(join(workspace,".bazelignore"),actions.sources.link+"\\n");',
             'await rm(join(workspace,".bazelignore"),{force:true});'),
    ]),
    sub(R + "🟨️.mjs", [
        edit('if(!Array.isArray(outputs)||!outputs.includes(storeReceiptOutputV1(target)))return undefined;',
             'if(!Array.isArray(outputs)||!outputs.some(output=>typeof output==="string"&&output.startsWith(policy.projectRootToken+"/"+policy.directory+"/"))||!outputs.includes(storeReceiptOutputV1(target)))return undefined;'),
    ]),
]

fixture_path = R + "🧪️tests/🧫️fixtures/🔣️.json"
workspace_test = B + "🧪️tests/📥️workspace/🟦️.ts"
workspace_fixture = B + "🧪️tests/📥️workspace/🔣️.json"
workspace_schema = B + "🧪️tests/📥️workspace/🧬️schema.json"

patches.append(sub(fixture_path, [
    edit('''    {
      "name": "a project at the workspace root",''',
         '''    {
      "name": "a target that declares no receipt output is never name-validated, whatever its characters",
      "projectRoot": "a/b",
      "target": "\U0001F3ED\ufe0fgenerator",
      "outputs": [],
      "expected": null
    },
    {
      "name": "a target with other outputs only is never name-validated",
      "projectRoot": "a/b",
      "target": "\U0001F3ED\ufe0fgenerator-manifests",
      "outputs": [
        "{projectRoot}/dist"
      ],
      "expected": null
    },
    {
      "name": "a project at the workspace root",''')
]))

patches.append(sub(workspace_fixture, [
    edit('''  "link": {
    "name": "s",
    "ignore": "s\\n"
  },''', '''  "link": {
    "name": "s"
  },'''),
]))
patches.append(sub(workspace_schema, [
    edit('''      "required": [
        "name",
        "ignore"
      ],''', '''      "required": [
        "name"
      ],'''),
    edit('''        "name": {
          "type": "string",
          "minLength": 1
        },
        "ignore": {
          "type": "string",
          "pattern": "\\\\n$"
        }
      }''', '''        "name": {
          "type": "string",
          "minLength": 1
        }
      }'''),
]))

patches.append(sub(workspace_test, [
    edit('/** 🗺️ Contract: the generated Bazel workspace links the checkout without letting recursive queries crawl it, and a task\'s own Cargo directories are task data resolved against the workspace. */',
         '/** 🗺️ Contract: the generated Bazel workspace links the checkout without an ignore file (so the link stays in the execroot and actions are cached), and a task\'s own Cargo directories are task data resolved against the workspace. */'),
    edit('import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";',
         'import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";'),
    edit('expect(corpus.link.name).toBe(component.definitions.Actions.const.sources.link);expect(corpus.link.ignore).toBe(corpus.link.name+"\\n");',
         'expect(corpus.link.name).toBe(component.definitions.Actions.const.sources.link);'),
    edit('test("Bazel links the checkout into the workspace and lists the link in .bazelignore, idempotently and replacing a stale link",async()=>{',
         'test("Bazel links the checkout into the workspace without a .bazelignore, idempotently, replacing a stale link and removing a stale ignore file",async()=>{'),
    edit('''  expect(readFileSync(join(space,".bazelignore"),"utf8")).toBe(corpus.link.ignore);expect(realpathSync(join(space,corpus.link.name))).toBe(realpathSync(checkout));expect(readFileSync(join(space,corpus.link.name,"a.txt"),"utf8")).toBe("a\\n");
  const past=new Date(Date.now()-60000);utimesSync(join(space,".bazelignore"),past,past);const before=lstatSync(join(space,".bazelignore")).mtimeMs;
  await api.linkBazelSourcesV1(checkout,space);expect(lstatSync(join(space,".bazelignore")).mtimeMs).toBe(before);
  writeFileSync(join(space,".bazelignore"),"wrong\\n");await api.linkBazelSourcesV1(checkout,space);expect(readFileSync(join(space,".bazelignore"),"utf8")).toBe(corpus.link.ignore);
''', '''  expect(existsSync(join(space,".bazelignore"))).toBe(false);expect(realpathSync(join(space,corpus.link.name))).toBe(realpathSync(checkout));expect(readFileSync(join(space,corpus.link.name,"a.txt"),"utf8")).toBe("a\\n");
  const before=lstatSync(join(space,corpus.link.name)).ino;
  await api.linkBazelSourcesV1(checkout,space);expect(lstatSync(join(space,corpus.link.name)).ino).toBe(before);
  writeFileSync(join(space,".bazelignore"),corpus.link.name+"\\n");await api.linkBazelSourcesV1(checkout,space);expect(existsSync(join(space,".bazelignore"))).toBe(false);
'''),
    edit('''test.skipIf(!binary||process.platform==="win32")("the real Bazel crawls //... only with the link listed in .bazelignore, while explicit labels through the link keep resolving",async()=>{''',
         '''test.skipIf(!binary||process.platform==="win32")("real Bazel: the linked checkout stays in the execroot so an action output reaches the disk cache, //:all never crawls the checkout and //... would",async()=>{'''),
    edit('''const directory=mkdtempSync(join(tmpdir(),"bazel-ignore-")),checkout=join(directory,"checkout"),space=join(directory,"workspace"),base=join(directory,"base"),install=join(directory,"install"),''',
         '''const directory=mkdtempSync(join(tmpdir(),"bazel-link-")),checkout=join(directory,"checkout"),space=join(directory,"workspace"),base=join(directory,"base"),install=join(directory,"install"),disk=join(directory,"disk"),'''),
    edit('''  writeFileSync(join(space,"MODULE.bazel"),'module(name="ignore")\\n');writeFileSync(join(space,"BUILD.bazel"),`filegroup(name = "x", srcs = ["${corpus.link.name}/a.txt"])\\n`);
  await api.linkBazelSourcesV1(checkout,space);
  const ignored=bazel(["query","--lockfile_mode=off","//..."]);expect(ignored.status,ignored.stderr).toBe(0);expect(ignored.stdout).toContain("//:x");
  const label=bazel(["query","--lockfile_mode=off",`//:${corpus.link.name}/a.txt`]);expect(label.status,label.stderr).toBe(0);
  rmSync(join(space,".bazelignore"));
  const crawled=bazel(["query","--lockfile_mode=off","//..."]);expect(crawled.status).not.toBe(0);expect(crawled.stderr).toMatch(/infinite symlink expansion/iu);
''', '''  writeFileSync(join(space,"MODULE.bazel"),'module(name="link")\\n');writeFileSync(join(space,"BUILD.bazel"),`filegroup(name = "x", srcs = ["${corpus.link.name}/a.txt"])\\ngenrule(name = "g", srcs = ["${corpus.link.name}/a.txt"], outs = ["g.txt"], cmd = "cat $(SRCS) > $@")\\n`);
  await api.linkBazelSourcesV1(checkout,space);
  const all=bazel(["query","--lockfile_mode=off","//:all"]);expect(all.status,all.stderr).toBe(0);expect(all.stdout).toContain("//:x");
  const label=bazel(["query","--lockfile_mode=off",`//:${corpus.link.name}/a.txt`]);expect(label.status,label.stderr).toBe(0);
  const build=()=>bazel(["build","--lockfile_mode=off","--spawn_strategy=local","--guard_against_concurrent_changes",`--disk_cache=${disk}`,"//:g"]);
  const cold=build();expect(cold.status,cold.stderr).toBe(0);expect(cold.stderr).not.toContain("Skipping uploading");
  expect(bazel(["clean"]).status).toBe(0);
  const warm=build();expect(warm.status,warm.stderr).toBe(0);expect(warm.stderr).toContain("1 disk cache hit");
  const crawled=bazel(["query","--lockfile_mode=off","//..."]);expect(crawled.status).not.toBe(0);expect(crawled.stderr).toMatch(/infinite symlink expansion/iu);
'''),
]))

for patch in patches:
    path = os.path.join(ROOT, patch["file"])
    text = open(path, encoding="utf-8").read()
    for e in patch["edits"]:
        have = text.count(e["old"])
        assert have == e["count"], (patch["file"], have, e["old"][:80])

here = os.path.dirname(os.path.abspath(__file__))
out = os.path.join(here, "staged", "patches", "bzl-real.json")
json.dump(patches, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("wrote", out, len(patches))

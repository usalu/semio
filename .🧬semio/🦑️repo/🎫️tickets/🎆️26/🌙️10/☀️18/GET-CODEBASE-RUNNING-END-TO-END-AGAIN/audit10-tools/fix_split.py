"""🌀 Builds the audit10 copy of batch `split` from the lane's staged bytes (anchored, exit 1 = anchor lost): an export whose freshness basis moved while it compiled (a digest the export cannot know, `""`) is refused before it writes anything, so the cacheable `build` action fails instead of caching an artifact that is stale at birth and that every later action-cache hit on the same input bytes would restore. usage: python3 -I fix_split.py"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
SRC = os.path.join(C, "dashhot-tools", "staged", "split"); DST = os.path.join(HERE, "staged", "split")
def edit(name, pairs):
    path = os.path.join(DST, "files", name); text = open(path, encoding="utf-8", newline="").read()
    for old, new in pairs:
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), name, old[:80]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8", newline="").write(text)
shutil.rmtree(DST, ignore_errors=True); shutil.copytree(SRC, DST)
edit("07.ts", [
 ("/** 📥️ Admits the sources record Cargo published beside the executable",
  "/** 🌀️ The first file of a freshness basis that was written at or after its build started (its digest is unknown, `\"\"`), or nothing when the basis held still: an artifact with such a file is stale at birth, and as the cacheable output of `build` it would come back on every action-cache hit of the same input bytes, so it is never exported. */\n"
  "export function movedDashboardSourceV1(compiled:DashboardCompiledSources):string|undefined{return compiled.sources.find((_,index)=>compiled.digests[index]===\"\");}\n\n"
  "/** 📥️ Admits the sources record Cargo published beside the executable"),
 ("  const compiled=compiledDashboardSourcesV1(packageRoot,root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=join(directory,name),staged=`${directory}.${process.pid}.bin`;\n",
  "  const compiled=compiledDashboardSourcesV1(packageRoot,root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=join(directory,name),staged=`${directory}.${process.pid}.bin`,moved=movedDashboardSourceV1(compiled);\n"
  "  if(moved!==undefined)throw refuseBootstrapV1(\"stale\",\"dashboard.artifact-sources-moved\",{mode,path:moved});\n"),
 ("/** 📤️ Copies the published executable into this mode's artifact directory (the cacheable output of `build`) with a record of what it was compiled from, under the exclusive artifact lease:",
  "/** 📤️ Copies the published executable into this mode's artifact directory (the cacheable output of `build`) with a record of what it was compiled from, under the exclusive artifact lease; a build whose freshness basis moved while it compiled is refused before anything is written (`movedDashboardSourceV1`), so the failed action caches nothing and the next build compiles again;"),
])
edit("11.ts", [("changedDashboardSourceV1, ", "changedDashboardSourceV1, movedDashboardSourceV1, ")])
edit("09.json", [
 ('    "dashboard.artifact-behind-sources": {',
  '    "dashboard.artifact-sources-moved": {\n'
  '      "en": "[dashboard] {path} changed while the {mode} build compiled it, so this build is not kept: run repo run @semio-tech/repo-dashboard-rs:build again",\n'
  '      "de": "[dashboard] {path} hat sich geändert, während der {mode}-Build es kompilierte, daher wird dieser Build nicht behalten: repo run @semio-tech/repo-dashboard-rs:build erneut ausführen"\n'
  '    },\n'
  '    "dashboard.artifact-behind-sources": {'),
])
edit("05.ts", [
 ("import {captureDashboardSources,changedDashboardSourceV1,currentDashboardArtifactV1,",
  "import {captureDashboardSources,changedDashboardSourceV1,movedDashboardSourceV1,currentDashboardArtifactV1,"),
 ("/** ✏️ Applies one authored mutation to a fixture tree. */",
  "test(\"an artifact whose freshness basis moved while it compiled is never exported: its unknown digest keeps it stale even after the bytes the build was keyed on come back\",async()=>{\n"
  " const tree=await recordedTree(\"mv\"),path=fixture.compiled[0]!,poisoned={...tree.artifact.compiled,digests:tree.artifact.compiled.digests.map((digest,index)=>index===0?\"\":digest)};\n"
  " expect(movedDashboardSourceV1(tree.artifact.compiled)).toBeUndefined();expect(movedDashboardSourceV1(poisoned)).toBe(path);\n"
  " writeFileSync(join(tree.root,\".🧬semio/🦑️repo/⚡️cache/🎛️dashboard\",tree.selected,protocol.directory,protocol.record),JSON.stringify({...tree.artifact,compiled:poisoned}));mutate(tree.root,\"rewrite-same\",path);\n"
  " expect(changedDashboardSourceV1(tree.root,poisoned)).toBe(path);expect(currentDashboardArtifactV1(tree.root,tree.selected)).toBeUndefined();\n"
  " const source=readFileSync(join(import.meta.dir,\"../../../../📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts\"),\"utf8\"),body=source.slice(source.indexOf(\"export async function exportDashboardArtifactV1\"),source.indexOf(\"export async function installDashboardArtifactV1\")),refusal=body.indexOf('refuseBootstrapV1(\"stale\",\"dashboard.artifact-sources-moved\"');\n"
  " expect(body.indexOf(\"moved=movedDashboardSourceV1(compiled)\")).toBeGreaterThan(0);expect(refusal).toBeGreaterThan(0);expect(refusal).toBeLessThan(body.indexOf(\"mkdirSync(directory\"));expect(refusal).toBeLessThan(body.indexOf(\"writeRecord(\"));\n"
  "});\n\n"
  "/** ✏️ Applies one authored mutation to a fixture tree. */"),
])
manifest = json.load(open(os.path.join(DST, "manifest.json"), encoding="utf-8"))
for rel, row in manifest.items():
    row["new"] = hashlib.sha256(open(os.path.join(DST, "files", row["file"]), "rb").read()).hexdigest()
json.dump(manifest, open(os.path.join(DST, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("audit10 split built:", DST)

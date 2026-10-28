"""🧫 [isolate] batch `helper`: the optional graph cache suite takes its private repository guard from the shared helper; its static source scan moves into the repo-wide test-store-isolation law (anchored on the live bytes). usage: helper_optional.py <src-root> <dst-root>"""
import os, sys
src, dst = sys.argv[1:3]
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts"
s = open(os.path.join(src, rel), encoding="utf-8").read()
pairs = [
 ('import {graph,CommandSourceControlV1,privateGraphWorkspaceV1,requirePrivateGraphWorkspaceV1,liveStoreReferencesV1,testToolchainDirectoryV1} from "../🧰️support/🟦️.ts";',
  'import {graph,CommandSourceControlV1,privateGraphWorkspaceV1} from "../🧰️support/🟦️.ts";\nimport {requirePrivateRepositoryWorkspaceV1,testToolchainDirectoryV1} from "../../../../🧪️testing/🧫️private-repository/🟦️.ts";'),
 ('SEMIO_PRIVATE_GRAPH_WORKSPACE:root', 'SEMIO_PRIVATE_REPOSITORY:root'),
 ('''test("optional graph cache suite touches no live repository store, cache or lease root",async()=>{
 const ws=await privateGraphWorkspaceV1(),own=await readFile(fileURLToPath(import.meta.url),"utf8"),store=[".cache","/b/l"].join(""),offender=["const root=process.","cwd();const live=join(root,'"+store+"');"].join("");
 expect(liveStoreReferencesV1(own)).toEqual([]);
 expect(liveStoreReferencesV1(offender)).toHaveLength(2);
 expect(liveStoreReferencesV1("const root=resolve(directory);")).toEqual([]);
 expect(await ws.requireIsolated()).toBe(ws.root);
 await expect(requirePrivateGraphWorkspaceV1(testToolchainDirectoryV1())).rejects.toThrow("Private graph workspace required");
 await expect(requirePrivateGraphWorkspaceV1(resolve(ws.root,".."))).rejects.toThrow("Private graph workspace required");
});''',
 '''test("optional graph cache suite runs in a private repository, never the checkout",async()=>{
 const ws=await privateGraphWorkspaceV1();
 expect(await ws.requireIsolated()).toBe(ws.root);
 expect(()=>requirePrivateRepositoryWorkspaceV1(testToolchainDirectoryV1())).toThrow("Private repository workspace required");
 expect(()=>requirePrivateRepositoryWorkspaceV1(resolve(ws.root,".."))).toThrow("Private repository workspace required");
});'''),
]
for old, new in pairs:
    if s.count(old) != 1: sys.stderr.write("ANCHOR %d: %s\n" % (s.count(old), old[:100])); sys.exit(2)
    s = s.replace(old, new)
open(os.path.join(dst, rel), "w", encoding="utf-8").write(s); print("optional-cache ->", dst)

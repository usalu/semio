"""🧫 [isolate] batch `helper`: the graph test support delegates its private repository to the shared `🧪️testing/🧫️private-repository` (anchored on the live bytes; exit 2 names a lost anchor). usage: helper_support.py <src-root> <dst-root>"""
import os, sys
src, dst = sys.argv[1:3]
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts"
s = open(os.path.join(src, rel), encoding="utf-8").read()
def need(anchor):
    if s.count(anchor) != 1: sys.stderr.write("ANCHOR %d: %s\n" % (s.count(anchor), anchor[:100])); sys.exit(2)
marker = "/** 🔒️ Whether one directory lies strictly below another. */"; need(marker)
tail = s[s.index(marker):]
if not tail.rstrip().endswith("},170000);"): sys.exit("ANCHOR tail")
s = s[:s.index(marker)] + '''/** 🧫️ The private repository of this test process with its graph store provisioned once (graph profile: it also holds the observation staging files): one fact-store writer enters the private lease and graph images before the first timed test; an adopted repository was provisioned by its parent. @see ../../../../🧪️testing/🧫️private-repository/🟦️.ts */
let privateGraphV1:Promise<PrivateRepositoryWorkspaceV1>|undefined;
export const privateGraphWorkspaceV1=():Promise<PrivateRepositoryWorkspaceV1>=>privateGraphV1??=privateRepositoryWorkspaceV1("graph").then(async workspace=>{if(workspace.owned){const api=await import("../../../📥️inputs/💾️storage/🟨️.mjs");await workspace.invoke(original=>api.withRepositoryBuildGraphFactStoreV1(workspace.root,original,createIssuedResourceControlV1(original,"graph:private-open",0),async()=>{}),{owner:"graph-fixture-provisioning",maximumElapsedMilliseconds:150000});}return workspace;});
'''
for old, new in [('import {mkdir,mkdtemp,readdir,rm} from "node:fs/promises";', 'import {mkdtemp,rm} from "node:fs/promises";'), ('import {existsSync,readFileSync,realpathSync,writeFileSync} from "node:fs";', 'import {readFileSync} from "node:fs";'), ('import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";', 'import {dirname,join,resolve} from "node:path";'), ('import {spyOn,beforeAll,afterAll} from "bun:test";', 'import {spyOn,beforeAll} from "bun:test";'), ('import {readBuildGraphV1} from "../../../🕸️graph/🟦️.ts";\n', 'import {readBuildGraphV1} from "../../../🕸️graph/🟦️.ts";\nimport {privateRepositoryWorkspaceV1,type PrivateRepositoryWorkspaceV1} from "../../../../🧪️testing/🧫️private-repository/🟦️.ts";\n')]:
    need(old); s = s.replace(old, new)
open(os.path.join(dst, rel), "w", encoding="utf-8").write(s); print("support ->", dst)

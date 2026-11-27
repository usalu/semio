import os, sys, time, hashlib
ROOT = "/Users/ueli/Documents/semio"
T = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
SD = T + "/🧰️scripts/F120"
L = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
N = L + "/🧹️normalization"
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()[:8]
def atomic(path, text):
    tmp = path + ".f120.tmp"
    open(tmp, "w", encoding="utf8").write(text); os.replace(tmp, path)
def edit(path, pairs):
    before = sha(path); s = open(path, encoding="utf8").read()
    for a, b in pairs:
        assert s.count(a) == 1, (path, a[:60], s.count(a))
        s = s.replace(a, b)
    atomic(path, s); print(path.split("/")[-2], before, "->", sha(path))
def newfile(path, src):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    assert not os.path.exists(path), path
    atomic(path, open(src, encoding="utf8").read()); print("new", path.split("/")[-2], sha(path))

step = sys.argv[1]
if step == "writer":
    lock = T + "/landing.lock.graph"
    os.mkdir(lock); open(lock + "/owner", "w").write("F120 %s\n" % time.strftime("%H:%M"))
    try:
        W = L + "/🏗️build/🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts"
        edit(W, [
          ('import { linkSync, mkdirSync,', 'import { chmodSync, linkSync, mkdirSync,'),
          ('/** 🪝️ Test seams of the shared writer: the rename that publishes the staged file. */\nexport type OwnerRegistryWriteHooksV1 = Readonly<{ rename?: (from: string, to: string) => void }>;',
           '/** 🪝️ Options of the shared writer: the rename that publishes the staged file (a test seam) and the permission mode the published file takes (default: the process default). */\nexport type OwnerRegistryWriteHooksV1 = Readonly<{ rename?: (from: string, to: string) => void; mode?: number }>;'),
          ('function stageOwnerRegistryV1<T>(file: string, text: string, publish: (pending: string) => T): T {',
           'function stageOwnerRegistryV1<T>(file: string, text: string, publish: (pending: string) => T, mode?: number): T {'),
          ('    writeFileSync(pending, text, { flag: "wx" });\n', '    writeFileSync(pending, text, { flag: "wx" });\n    if (mode !== undefined) chmodSync(pending, mode);\n'),
          ('    (hooks.rename ?? renameSync)(pending, file);\n  });', '    (hooks.rename ?? renameSync)(pending, file);\n  }, hooks.mode);'),
        ])
    finally:
        import shutil; shutil.rmtree(lock)
elif step == "module":
    base = N + "/✍️own-file-writers"
    newfile(base + "/🧬️schema/🔣️.json", SD + "/new-schema.json")
    newfile(base + "/🔣️.json", SD + "/new-policy.json")
    newfile(base + "/🟦️.ts", SD + "/new-module.ts")
elif step == "engine":
    edit(N + "/🛟️recovery/🟦️.ts", [
      ('writePreparationName: (pid: number, token: string) => string, writeCandidateName: string): void {\n  const target = absolutePath(repoRoot, path), identity = sha256(path).slice(0, 24), result = referenceEditResult(repoRoot, plan, journal, path);\n  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage changed: ${path}`);\n',
       'writePreparationName: (pid: number, token: string) => string, writeCandidateName: string, ownWriter?: (target: string, expected: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>, bytes: Buffer) => void): void {\n  const target = absolutePath(repoRoot, path), identity = sha256(path).slice(0, 24), result = referenceEditResult(repoRoot, plan, journal, path);\n  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage changed: ${path}`);\n  if (ownWriter) {\n    ownWriter(target, result.preimage, result.bytes);\n    journal.probe?.("transaction-edit-canonical-exchange", path);\n    return;\n  }\n'),
    ])
elif step == "steps":
    edit(N + "/🚀️apply/🏁️transaction-run/🔧️transaction-steps/🟦️.ts", [
      ('import { applyReferenceEditAtomically, backupPath, validateResumeTuples } from "../../../🛟️recovery/🟦️.ts";\n',
       'import { applyReferenceEditAtomically, backupPath, validateResumeTuples } from "../../../🛟️recovery/🟦️.ts";\nimport { ownFileWriterFor } from "../../../✍️own-file-writers/🟦️.ts";\n'),
      ('editPreparationName, editWritePreparationName, editWriteCandidateName);\n      journal.appliedEditPaths.push(path);', 'editPreparationName, editWritePreparationName, editWriteCandidateName, ownFileWriterFor(path));\n      journal.appliedEditPaths.push(path);'),
    ])

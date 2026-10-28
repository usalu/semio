"""🧹 [audit7] Anchored edits of batch `reclaim`: every temporary directory of the graph farm (snapshot copy, staging, session) is named with the id of its process, and the first one a process creates removes those of processes that are gone (a graph child killed before its farm closed leaves them behind; nothing else removes them). usage: edit_core.py <root>  (exit 1 = an anchor is not found exactly once)"""
import os, sys
root = sys.argv[1]
F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
HELPER = '''/** 🧹️ Creates a private directory of this process for the farm (`kind`: `staging`, `image` or `session`) under the temporary directory, named with the id of this process; the first call of a process for a directory first removes the farm directories there whose process is gone, since a parent killed before its farm closed (out of memory, a budget kill, a terminated build) leaves its snapshot copy, staging and session files behind and nothing else removes them. A directory whose process still runs, whose process id answers anything but "no such process", or whose name is not one the farm writes is kept. */
export function createFarmTemporaryDirectoryV1(kind, { directory = realpathSync(tmpdir()), alive = farmProcessAliveV1, policy = P } = {}) {
  const prefix = policy.temporaryDirectories.prefixes[kind];
  if (typeof prefix !== "string") throw Error("Declared graph farm temporary directory kind required: " + kind);
  if (!reclaimedDirectoriesV1.has(directory)) {
    reclaimedDirectoriesV1.add(directory);
    const pattern = new RegExp(policy.temporaryDirectories.pattern, "u");
    let names = [];
    try { names = readdirSync(directory); } catch {}
    for (const name of names) { const pid = Number(pattern.exec(name)?.groups?.pid); if (Number.isSafeInteger(pid) && pid > 0 && pid !== process.pid && !alive(pid)) { try { rmSync(join(directory, name), { recursive: true, force: true }); } catch {} } }
  }
  return mkdtempSync(join(directory, prefix + process.pid + "-"));
}

/** 🫀️ Whether a process id still names a process: only the answer "no such process" counts as gone, so a platform that cannot tell keeps the directory. */
export function farmProcessAliveV1(pid) { try { process.kill(pid, 0); return true; } catch (error) { return error?.code !== "ESRCH"; } }

const reclaimedDirectoriesV1 = new Set();

/**
 * 🏊️ A pool of worker processes,'''
EDITS = {
 F + "🏊️pool/🟨️.mjs": [
  ('import { closeSync, constants, mkdtempSync, openSync, rmSync, writeFileSync } from "node:fs";', 'import { closeSync, constants, mkdtempSync, openSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";'),
  ('/**\n * 🏊️ A pool of worker processes,', HELPER),
  ('sessionDirectory = mkdtempSync(join(tmpdir(), "semio-farm-session-"));', 'sessionDirectory = createFarmTemporaryDirectoryV1("session", { policy });'),
 ],
 F + "🏊️pool/🟨️.d.mts": [
  ('readonly shutdownGraceMilliseconds:number;readonly runtime:', 'readonly shutdownGraceMilliseconds:number;readonly temporaryDirectories:{readonly prefixes:Readonly<Record<"staging"|"image"|"session",string>>;readonly pattern:string};readonly runtime:'),
  ('export function graphFarmRuntimeV1():string;', 'export function graphFarmRuntimeV1():string;\n/** 🧹️ Creates a private farm directory of this process named with its process id; the first call per directory removes the farm directories of processes that are gone. */\nexport function createFarmTemporaryDirectoryV1(kind:"staging"|"image"|"session",options?:{directory?:string;alive?:(pid:number)=>boolean;policy?:GraphFarmPolicyV1}):string;\n/** 🫀️ Whether a process id still names a process (only "no such process" counts as gone). */\nexport function farmProcessAliveV1(pid:number):boolean;'),
 ],
 F + "🏊️pool/🧬️schema/🔣️.json": [
  ('        "shutdownGraceMilliseconds": 3000,\n', '        "shutdownGraceMilliseconds": 3000,\n        "temporaryDirectories": {\n          "prefixes": {\n            "staging": "semio-graph-farm-",\n            "image": "semio-graph-farm-image-",\n            "session": "semio-farm-session-"\n          },\n          "pattern": "^semio-(?:graph-farm|graph-farm-image|farm-session)-(?<pid>[1-9][0-9]{0,9})-[A-Za-z0-9]{6}$"\n        },\n'),
 ],
 F + "🟨️.mjs": [
  ('import { mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";\nimport { availableParallelism, tmpdir } from "node:os";', 'import { rmSync, writeFileSync } from "node:fs";\nimport { availableParallelism } from "node:os";'),
  ('import { choosePoolV1, createFarmV1,', 'import { choosePoolV1, createFarmTemporaryDirectoryV1, createFarmV1,'),
  ('const directory = mkdtempSync(join(realpathSync(tmpdir()), "semio-graph-farm-image-")), file = join(directory, "snapshot.json");', 'const directory = createFarmTemporaryDirectoryV1("image"), file = join(directory, "snapshot.json");'),
  ('const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-graph-farm-")), pool =', 'const base = createFarmTemporaryDirectoryV1("staging"), pool ='),
 ],
}
bad = 0
for rel, edits in EDITS.items():
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    for old, new in edits:
        n = text.count(old)
        if n != 1:
            if new in text: continue
            print(f"[audit7] anchor found {n} times in {rel}: {old[:90]!r}"); bad = 1; continue
        text = text.replace(old, new)
    mode = os.stat(path).st_mode & 0o7777
    with open(path, "w", encoding="utf8") as handle: handle.write(text)
    os.chmod(path, mode)
sys.exit(bad)

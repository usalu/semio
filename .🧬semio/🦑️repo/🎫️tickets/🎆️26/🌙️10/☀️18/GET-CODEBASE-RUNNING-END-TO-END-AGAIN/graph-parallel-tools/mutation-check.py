#!/usr/bin/env python3
"""🧬 Mutation proof of the graph farm laws: every mutation breaks exactly one rule of the implementation inside a scratch overlay and must make the named law fail; the original must pass first.
usage: mutation-check.py [--root /private/tmp/graph-parallel/ws-farm] [--only SUBSTRING]
Each mutation is a literal replacement that must occur exactly once; the file is restored after every run. Stdlib only."""
import pathlib, subprocess, sys, time

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1] if "--root" in args else "/private/tmp/graph-parallel/ws-farm")
only = args[args.index("--only") + 1] if "--only" in args else ""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
G = B + "/🕸️graph"
INF = G + "/📥️inference"
NODES = INF + "/📦️project-nodes"
FARM, FARM_TEST = B + "/🏊️farm/🟨️.mjs", B + "/🏊️farm/🧪️tests/🟦️.ts"
OBS, OBS_TEST = B + "/👁️observation/🟨️.mjs", B + "/👁️observation/🧪️tests/🛰️detached/🟦️.ts"
STO, INP, STAGING_TEST = B + "/📥️inputs/💾️storage/🟨️.mjs", B + "/📥️inputs/🟨️.mjs", B + "/📥️inputs/💾️storage/🧪️tests/🛰️staging/🟦️.ts"
NODES_FILE, GLUE, DISPATCH = NODES + "/🟨️.mjs", NODES + "/🏊️project-farm/🟨️.mjs", NODES + "/🧪️tests/🏊️farm-dispatch/🟦️.ts"
EDGES, MODULE_TEST = INF + "/🕸️dependency-edges/🟨️.mjs", INF + "/🕸️dependency-edges/🧪️tests/🏊️module-farm/🟦️.ts"
HOST, HOST_TEST = B + "/🌡️host-capacity/🟦️.ts", B + "/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts"
MERGE_TEST = G + "/🕰️generations/🧪️tests/🏊️farm-merge/🟦️.ts"

MUTATIONS = [
    ("encoder forgets the objects it sent", FARM, "          if (object) byObject.set(item, id);\n", "", FARM_TEST),
    ("decoder accepts an unknown identifier", FARM, 'if (item === undefined) throw Error("Known intern identifier required: " + ids[index]);', "", FARM_TEST),
    ("sizing ignores the memory bound", FARM, '["memory", Math.floor((budget - Math.max(parentMegabytes, policy.parentReserveMegabytes)) / workerMegabytes)],', '["memory", Infinity],', FARM_TEST),
    ("sizing ignores the processors", FARM, '["cpu", host.cores - policy.cpuReserve],', '["cpu", Infinity],', FARM_TEST),
    ("a worker of another implementation may serve", FARM, 'if (!accept(message.identity)) { disable("implementation differs"); return; }', "", FARM_TEST),
    ("a worker past its budget is kept", FARM, "message.type === \"result\" && message.rssMegabytes > workerMegabytes", "false", FARM_TEST),
    ("the heap switch of the runtime is not set", FARM, 'if (runtime === "bun") environment.BUN_JSC_forceRAMSize', 'if (false) environment.BUN_JSC_forceRAMSize', FARM_TEST),
    ("a failing job is reported as an answer", FARM, "else { stats.failures++; stats.fallbacks++; settle(job, { failure: message }); }", "else { stats.failures++; stats.fallbacks++; settle(job, { reply: message }); }", FARM_TEST),
    ("replay forgets the facts", OBS, "retainObservationsV1(scopes.getStore(),rows);retainSourceFactsV1(rows);\n const scope=recordScopes.getStore();", "const scope=recordScopes.getStore();", OBS_TEST),
    ("replay forgets the parser records", OBS, "if(scope?.records)for(const[key,bytes]of records)scope.records.put(key,bytes);", "", OBS_TEST),
    ("a sealed record of another digest is written", OBS, "sealing.sealed.digest===digest&&", "", OBS_TEST),
    ("a sealed record of other facts is written", OBS, "sealing.sealed.rowCount===rows.length&&", "", OBS_TEST),
    ("a block the parent lacks is invented from nothing", OBS, "if(block.only===true||(blocksRefused", "if((blocksRefused", OBS_TEST),
    ("a changed block leaf is still trusted", OBS, '&&observationEpochV1(row)===entry.epoch){entry.recheck??=()=>{requireRepositoryBuildGraphObservationPathV1(path,"read");const actual=fs.lstatSync(path,{bigint:true});if(!actual.isFile()||actual.isSymbolicLink()||observationEpochV1(actual)!==entry.epoch)throw Error("Observation leaf changed before delivery");};return entry;}', '){entry.recheck??=()=>{};return entry;}', OBS_TEST),
    ("stored parser records are not counted", OBS, "for(const[key,bytes]of entries)store.put(key,bytes);\n return entries.length;", "for(const[key,bytes]of entries)store.put(key,bytes);\n return 0;", OBS_TEST),
    ("a closure may differ from its name", INP, '||createHash("sha256").update(contract).digest("hex")!==leaf.slice(0,-5)', "", STAGING_TEST),
    ("a staged name may leave its namespace", INP, '||rest.some(part=>part===""||part==="..")', "", STAGING_TEST),
    ("a staged publication may leave the staging root", STO, 'if(local===""||local.startsWith("..")||isAbsolute(local))throw Error("Private graph staging path required");', "", STAGING_TEST),
    ("the staging directory is not private", STO, "if(staged){const path=join(staged.root,namespace);mkdirSync(path,{recursive:true});return path;}", "", STAGING_TEST),
    ("the farm is not closed after the loop", NODES_FILE, ".finally(()=>farm?.close())", "", DISPATCH),
    ("an owner admission may use the farm", NODES_FILE, "options?.ownerAdmission?undefined:await openProjectFarmV1(", "await openProjectFarmV1(", DISPATCH),
    ("the project body is inferred twice", NODES_FILE, "return inferProjectBodyV1(projectEnvironment,", "return await projectWithDefaults(0) ?? inferProjectBodyV1(projectEnvironment,", DISPATCH),
    ("staged files are not adopted", GLUE, "      adoptStagedGraphFilesV1(workspaceRoot, files);\n", "", DISPATCH),
    ("the farm grant is never given back", GLUE, "      opened.capacity.release();\n", "", DISPATCH),
    ("the host is asked before any project needs a worker", GLUE, "  let opening;\n", "  let opening;\n  void acquireBuildFarmCapacityV1({ preferredJobs: 1, waitMilliseconds: 1, environment });\n", DISPATCH),
    ("the dependency pass parses before the records are prepared", EDGES, "await prefillModuleAnalysesV1(", "void prefillModuleAnalysesV1(", MODULE_TEST),
    ("the inherited grant is kept for the farm", HOST, 'delete environment["SEMIO_HOST_CAPACITY_GRANT"];', "", HOST_TEST),
    ("the farm waits for ever for a grant", HOST, "const timer=setTimeout(()=>controller.abort(Error(\"Farm capacity wait elapsed\")),input.waitMilliseconds);", "const timer=setTimeout(()=>{},input.waitMilliseconds);", HOST_TEST),
    ("rows come back in another order", FARM, "items[index] = item;", "items[ids.length - 1 - index] = item;", MERGE_TEST),
]


def run(test, timeout=240):
    done = subprocess.run(["bun", "test", "./" + test], cwd=root, capture_output=True, text=True, timeout=timeout)
    return done.returncode, (done.stdout + done.stderr)[-1500:]


tests = sorted({row[4] for row in MUTATIONS if only in row[0]})
print("baseline")
for test in tests:
    code, output = run(test)
    print("  %s %s" % ("pass" if code == 0 else "FAIL", test.split("/")[-2]))
    if code != 0:
        print(output)
        raise SystemExit("baseline must pass")
killed = survived = 0
for name, file, old, new, test in MUTATIONS:
    if only not in name:
        continue
    path = root / file
    text = path.read_text(encoding="utf8")
    if text.count(old) != 1:
        print("  ANCHOR", name, "occurs", text.count(old))
        survived += 1
        continue
    path.write_text(text.replace(old, new), encoding="utf8")
    started = time.time()
    try:
        code, _ = run(test)
    except subprocess.TimeoutExpired:
        code = 1
    finally:
        path.write_text(text, encoding="utf8")
    verdict = "killed" if code != 0 else "SURVIVED"
    killed += code != 0
    survived += code == 0
    print("  %-8s %-62s %4.0fs" % (verdict, name, time.time() - started))
print("killed %d, survived %d" % (killed, survived))
raise SystemExit(1 if survived else 0)

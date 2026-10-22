#!/usr/bin/env python3
"""🧬 Mutation proof of the round 2 laws of the graph farm set: every mutation breaks exactly one rule of the implementation inside a scratch overlay (round 1 and round 2 applied, see overlay2.py) and must make the named law fail; the original must pass first.
usage: mutation-check2.py [--root /private/tmp/graph-parallel/ws2r] [--only SUBSTRING]
Each mutation is a literal replacement that must occur exactly once; the file is restored after every run. Stdlib only."""
import pathlib, subprocess, sys, time

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1] if "--root" in args else "/private/tmp/graph-parallel/ws2r")
only = args[args.index("--only") + 1] if "--only" in args else ""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
G = B + "/🕸️graph"
INF = G + "/📥️inference"
NODES = INF + "/📦️project-nodes"
STO = B + "/📥️inputs/💾️storage"
LEAVES, LEAVES_TEST = STO + "/🏊️leaves/🟨️.mjs", STO + "/🏊️leaves/🧪️tests/🟦️.ts"
STORAGE = STO + "/🟨️.mjs"
FARM, FARM_TEST = B + "/🏊️farm/🟨️.mjs", B + "/🏊️farm/🧪️tests/🟦️.ts"
POOL, POOL_TEST = B + "/🏊️farm/🧰️pool/🟨️.mjs", B + "/🏊️farm/🧰️pool/🧪️tests/🟦️.ts"
OBS, OBS_TEST = B + "/👁️observation/🟨️.mjs", B + "/👁️observation/🧪️tests/🛰️detached/🟦️.ts"
INP, RC, RS = B + "/📥️inputs/🟨️.mjs", B + "/📥️inputs/🦀️resolution/💾️receipt/🟨️.mjs", B + "/📥️inputs/🦀️resolution/🟦️.ts"
DUPLICATE_TEST = STO + "/🏊️leaves/🧪️tests/🏊️duplicate/🟦️.ts"
RECEIPTS_TEST = B + "/📥️inputs/🦀️resolution/🧪️tests/🏊️receipts/🟦️.ts"
NODES_FILE, GLUE, DISPATCH = NODES + "/🟨️.mjs", NODES + "/🏊️project-farm/🟨️.mjs", NODES + "/🧪️tests/🏊️farm-dispatch/🟦️.ts"
MERGE_TEST = G + "/🕰️generations/🧪️tests/🏊️farm-merge/🟦️.ts"

MUTATIONS = [
    ("encoder forgets the objects it sent", FARM, "          if (object) byObject.set(item, id);\n", "", FARM_TEST),
    ("decoder accepts an unknown identifier", FARM, 'if (item === undefined) throw Error("Known intern identifier required: " + ids[index]);', "", FARM_TEST),
    ("sizing ignores the memory bound", FARM, '["memory", Math.floor((budget - Math.max(parentMegabytes, policy.parentReserveMegabytes) - policy.recordTableMegabytes - policy.seedMegabytes) / workerMegabytes)],', '["memory", Infinity],', FARM_TEST),
    ("sizing ignores the processors", FARM, '["cpu", host.cores - policy.cpuReserve],', '["cpu", Infinity],', FARM_TEST),
    ("a worker of another implementation may serve", FARM, 'if (!(accepts.get(message.kind) ?? accept)(message.kind, message.identity)) { disable("implementation differs"); return; }', "", FARM_TEST),
    ("a worker past its budget is kept", FARM, 'message.type === "result" && message.rssMegabytes > workerMegabytes', "false", FARM_TEST),
    ("the heap switch of the runtime is not set", FARM, 'if (runtime === "bun") environment.BUN_JSC_forceRAMSize', 'if (false) environment.BUN_JSC_forceRAMSize', FARM_TEST),
    ("a failing job is reported as an answer", FARM, "else { stats.failures++; stats.fallbacks++; settle(job, { failure: message }); }", "else { stats.failures++; stats.fallbacks++; settle(job, { reply: message }); }", FARM_TEST),
    ("rows come back in another order", FARM, "items[index] = item;", "items[ids.length - 1 - index] = item;", MERGE_TEST),
    ("prestart boots nothing", FARM, '      for (const slot of slots) if (slot.index < active && slot.state === "empty") launch(slot);\n    },\n    limit', "    },\n    limit", FARM_TEST),
    ("a reopened session keeps its old token", FARM, "const token = (sessions.get(kind)?.token ?? 0) + 1, file", "const token = 1, file", FARM_TEST),
    ("the resident pool is never shared", FARM, "if (held === undefined || held.farm.closed) {", "if (true) {", FARM_TEST),
    ("an idle pool is never closed", FARM, "held.timer = setTimeout(() => { if (pools.get(key) === held && held.leases === 0) { pools.delete(key); void held.farm.close(); } }, policy.residentIdleMilliseconds);\n      held.timer.unref?.();", "held.timer = setTimeout(() => {}, 0);", FARM_TEST),
    ("a worker limit is ignored", FARM, "active = Math.min(count, workers);", "active = workers;", FARM_TEST),
    ("a kind is served before its session is open", FARM, "const wanted = queue.find(job => !job.settled && sessions.has(job.kind));", "const wanted = queue.find(job => !job.settled);", FARM_TEST),
    ("a prestart request is ignored by the pool", POOL, "    if (prestart) lease.farm.prestart();\n", "", POOL_TEST),
    ("the pool of every workspace is the same", POOL, 'key: [workspaceRoot, plan.workerMegabytes].join("\\0")', "key: String(plan.workerMegabytes)", POOL_TEST),
    ("a refusal does not name its reason", POOL, "open: async () => ({ refused: plan.reason })", 'open: async () => ({ refused: "" })', POOL_TEST),
    ("deferred leaves are not owed", LEAVES, "      stats.deferred++;\n      owe(plan.retainedDeltaBytes);\n      maybeFlush();", "      stats.deferred++;\n      maybeFlush();", LEAVES_TEST),
    ("a paid debt stays owed", LEAVES, "{ bytes = 0; leaves = 0; stats.flushes++; return true; }", "{ stats.flushes++; return true; }", LEAVES_TEST),
    ("the byte threshold never pays", LEAVES, "if (bytes >= policy.flushBytes || leaves >= policy.flushLeaves || clock() - since >= policy.flushMilliseconds) flush(false);", "if (leaves >= policy.flushLeaves || clock() - since >= policy.flushMilliseconds) flush(false);", LEAVES_TEST),
    ("the leaf threshold never pays", LEAVES, "if (bytes >= policy.flushBytes || leaves >= policy.flushLeaves || clock() - since >= policy.flushMilliseconds) flush(false);", "if (bytes >= policy.flushBytes || clock() - since >= policy.flushMilliseconds) flush(false);", LEAVES_TEST),
    ("the age threshold never pays", LEAVES, "if (bytes >= policy.flushBytes || leaves >= policy.flushLeaves || clock() - since >= policy.flushMilliseconds) flush(false);", "if (bytes >= policy.flushBytes || leaves >= policy.flushLeaves) flush(false);", LEAVES_TEST),
    ("a final flush tries once", LEAVES, "const attempts = final ? policy.finalFlushAttempts : 1;", "const attempts = 1;", LEAVES_TEST),
    ("a refused adopted leaf stays on disk", LEAVES, "if (!reservation) { if (immutable) declined.add(name); remove(name); refused.push(name); stats.refused++; continue; }", "if (!reservation) { if (immutable) declined.add(name); refused.push(name); stats.refused++; continue; }", LEAVES_TEST),
    ("adopted leaves skip the quota", LEAVES, "const reservation = quota.reserve({ peakBytes: size, retainedDeltaBytes: size });\n        if (!reservation) {", "const reservation = { commit() {}, release() {} };\n        if (!reservation) {", LEAVES_TEST),
    ("a grant reserves nothing", LEAVES, "const reservation = quota.reserve({ peakBytes: size, retainedDeltaBytes: 0 });\n      return", "const reservation = { release() {} };\n      return", LEAVES_TEST),
    ("an adopted name outside the declared paths is accepted", LEAVES, "if (!isOptionalLeafNameV1(name) || !Number.isSafeInteger(size)", "if (!Number.isSafeInteger(size)", LEAVES_TEST),
    ("an immediate publication skips the ledger", LEAVES, "if (!ledger.publish(plan, write)) return false;", "write();", LEAVES_TEST),
    ("a worker may write beyond its grant", LEAVES, "if (used + plan.peakBytes > grantBytes) {", "if (false) {", LEAVES_TEST),
    ("a worker may write anywhere", LEAVES, 'if (name === "" || name.startsWith("..") || isAbsolute(name) || !isOptionalLeafNameV1(name)) throw', "if (false) throw", LEAVES_TEST),
    ("a worker may use any namespace", LEAVES, 'if (typeof root !== "string" || !isAbsolute(root)) throw Error("Declared graph worker namespace required: " + namespace);', "", LEAVES_TEST),
    ("worker leaves are not reported", LEAVES, "written.push({ name, bytes: Math.max(plan.retainedDeltaBytes, 0) });", "", LEAVES_TEST),
    ("a worker deferral is dropped", LEAVES, "defer(manifest) { receipts.add(manifest); },", "defer(manifest) {},", LEAVES_TEST),
    ("optional leaves are never batched", STORAGE, "optionalAccountingOfV1(scope).publish(plan,write,batches.getStore()!==undefined,reservation))return decline();", "optionalAccountingOfV1(scope).publish(plan,write,false,reservation))return decline();", LEAVES_TEST),
    ("the debt is not paid before a retirement", STORAGE, "flushBuildGraphOwedV1(writer.ledger);writer.presence.clear();", "writer.presence.clear();", LEAVES_TEST),
    ("a worker keeps no private directory", STORAGE, 'Declared bounded graph fact namespace required");const worker=workerWriters.getStore();if(worker)return worker.directory(namespace);', 'Declared bounded graph fact namespace required");', LEAVES_TEST),
    ("leaves are adopted without custody", STORAGE, 'requireRepositoryBuildGraphWriterV1(workspace);const scope=writers.getStore(),root=repositoryBuildGraphDataDirectoryV1(workspace,"inference"),outcome', 'const scope=writers.getStore(),root=repositoryBuildGraphDataDirectoryV1(workspace,"inference"),outcome', LEAVES_TEST),
    ("a refused adopted leaf is not removed", STORAGE, "if(!scope.presence.has(path))rmSync(path,{force:true});", "", LEAVES_TEST),
    ("a refused leaf that is present is removed", STORAGE, "if(!scope.presence.has(path))rmSync(path,{force:true});", "rmSync(path,{force:true});", LEAVES_TEST),
    ("replay forgets the facts", OBS, "retainObservationsV1(scopes.getStore(),rows);retainSourceFactsV1(rows);\n const scope=recordScopes.getStore();", "const scope=recordScopes.getStore();", OBS_TEST),
    ("replay forgets the parser records", OBS, "if(scope?.records)for(const[key,bytes]of records)scope.records.put(key,bytes);", "", OBS_TEST),
    ("a sealed record of another digest is written", OBS, "sealing.sealed.digest===digest&&", "", OBS_TEST),
    ("a sealed record of other facts is written", OBS, "sealing.sealed.rowCount===rows.length&&", "", OBS_TEST),
    ("a block the parent lacks is invented from nothing", OBS, "if(block.only===true||(blocksRefused", "if((blocksRefused", OBS_TEST),
    ("a block that is not there still seals", OBS, "if(!persistInferenceBlockV1(directory,{digest,only:true}))return false;", "persistInferenceBlockV1(directory,{digest,only:true});", OBS_TEST),
    ("existing blocks are written again", OBS, "  if(fs.existsSync(path))continue;\n", "", OBS_TEST),
    ("a declined block is not reported", OBS, "fs.renameSync(temporary,path);}))return undefined;", "fs.renameSync(temporary,path);}));", OBS_TEST),
    ("a block another writer finished is replaced", OBS, "if(present){fs.rmSync(temporary,{force:true});return false;}", "", OBS_TEST),
    ("presence is ignored", OBS, "if(presence!==undefined&&presence.has(path))return true;", "", OBS_TEST),
    ("a verified block is not remembered", OBS, "present.recheck();presence?.add(path);return true;}", "present.recheck();return true;}", OBS_TEST),
    ("the record directory is not named", OBS, "const sealing={digest,directory:dirname(cachePath)},result", "const sealing={digest},result", OBS_TEST),
    ("stored parser records are not counted", OBS, "for(const[key,bytes]of entries)store.put(key,bytes);\n return entries.length;", "for(const[key,bytes]of entries)store.put(key,bytes);\n return 0;", OBS_TEST),
    ("the farm is not closed after the loop", NODES_FILE, ".finally(()=>farm?.close())", "", DISPATCH),
    ("an owner admission may use the farm", NODES_FILE, "options?.ownerAdmission?undefined:await openProjectFarmV1(", "await openProjectFarmV1(", DISPATCH),
    ("the project body is inferred twice", NODES_FILE, "return inferProjectBodyV1(projectEnvironment,", "return await projectWithDefaults(0) ?? inferProjectBodyV1(projectEnvironment,", DISPATCH),
    ("the projects are not batched", NODES_FILE, "withBuildGraphOptionalBatchV1(async()=>{", "(async()=>{", DISPATCH),
    ("the receipts are never refreshed", NODES_FILE, "await refreshCargoReceiptsV1(workspaceRoot,receipts,context);", "", DISPATCH),
    ("the workers start late", NODES_FILE, "readRoot:workspaceDataDirectory(workspaceRoot),started});", "readRoot:workspaceDataDirectory(workspaceRoot)});", DISPATCH),
    ("what a worker wrote is not accounted", GLUE, "      adopt(reply);\n", "", DISPATCH),
    ("a duplicate block is accounted twice", LEAVES, "if (immutable && (accepted.has(name) || declined.has(name))) { stats.duplicates++; continue; }", "", DUPLICATE_TEST),
    ("an accepted block is not remembered", LEAVES, "if (immutable) accepted.add(name);", "", DUPLICATE_TEST),
    ("a refused block is retried by its twin", LEAVES, "if (!reservation) { if (immutable) declined.add(name); remove(name);", "if (!reservation) { remove(name);", DUPLICATE_TEST),
    ("the grant is released before the adoption", GLUE, "      const release = grants.get(reply);\n      if (release !== undefined) { outstanding.delete(release); release(); }\n", "", DISPATCH),
    ("a job without an answer is silent", GLUE, 'say("fallback", fields);', "", DISPATCH),
    ("the fallbacks are not capped", GLUE, "if (traffic.fallbacks > policy.maximumFallbacks) throw", "if (false) throw", DISPATCH),
    ("the seed is not capped", GLUE, "exportSourceAnalysisRecordsV1(policy.seedMegabytes * MEGABYTE)", "exportSourceAnalysisRecordsV1()", DISPATCH),
    ("the record tables are not capped", GLUE, "recordTotal + row.bytes.byteLength > policy.recordTableMegabytes * MEGABYTE", "false", DISPATCH),
    ("a crashed job comes back as an answer", FARM, "if (job) { stats.fallbacks++; settle(job, { fallback: why }); }\n    if (++restarts", "if (job) { stats.fallbacks++; settle(job, undefined); }\n    if (++restarts", FARM_TEST),
    ("a reason is not translated", FARM, "return text === undefined ? reason : colon < 0 ? text : text + reason.slice(colon);", "return reason;", FARM_TEST),
    ("the table memory is not planned", FARM, "- policy.recordTableMegabytes - policy.seedMegabytes) / workerMegabytes)", ") / workerMegabytes)", FARM_TEST),
    ("a refused block counts as present", GLUE, "for (const name of adoption.accepted) if", "for (const name of fresh.map(leaf => leaf.name)) if", DISPATCH),
    ("the leftover grants are never given back", GLUE, "      for (const release of outstanding) release();\n", "", DISPATCH),
    ("the receipts of a worker are not deferred", GLUE, "for (const manifest of reply.receipts) deferBuildGraphReceiptV1(manifest);", "", DISPATCH),
    ("the pool is released never", GLUE, "      opened.release();\n", "", DISPATCH),
    ("the host is asked before any project needs a worker", GLUE, "  let opening, width = plan.workers, recordTotal = 0;\n", "  let opening, width = plan.workers, recordTotal = 0;\n  void pool.open();\n", DISPATCH),
    ("receipt bytes depend on the order of the parts", RC, 'brotliCompressSync(Buffer.from(canonicalJsonV1(sealed),"utf8"),', 'brotliCompressSync(Buffer.from(JSON.stringify(sealed),"utf8"),', RECEIPTS_TEST),
    ("a flush forgets what turned dirty while it wrote", RS, "  const crates = [...context.dirty].sort(bytes);\n  context.dirty.clear();\n  if (context.store?.write) for (const crate of crates)", "  const crates = [...context.dirty].sort(bytes);\n  if (context.store?.write) for (const crate of crates)", RECEIPTS_TEST),
    ("the receipts are refreshed in any order", INP, "[...manifests].sort()", "[...manifests]", RECEIPTS_TEST),
    ("a failing crate loses what it derived", INP, "try { await cargoResolutionPartsV1(run, manifest, { tolerant: true }); } finally { await flushCargoReceiptsV1(run); }", "await cargoResolutionPartsV1(run, manifest, { tolerant: true }); await flushCargoReceiptsV1(run);", RECEIPTS_TEST),
    ("a project does not defer its receipts", INP, "  if (deferBuildGraphReceiptV1(manifest)) return;\n", "", RECEIPTS_TEST),
]


def run(test, timeout=300):
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

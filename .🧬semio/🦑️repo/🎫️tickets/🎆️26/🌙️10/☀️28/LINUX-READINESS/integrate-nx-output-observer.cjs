const fs = require("node:fs"), path = require("node:path"), diff = require("diff"), assert = require("node:assert/strict");
const root = process.cwd(), patchPath = path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch");
const sdk = path.join(root, ".cache/b/t/m/nx/80e05b73ff7c42b89e14c0c24ad8da978910f9805a8421498b80afadc432456a/node_modules/nx");
let patch = fs.readFileSync(patchPath, "utf8");
const originalPatch = patch;
const sdkRoot = path.resolve(sdk, "../.."), baselineManifest = JSON.parse(fs.readFileSync(path.join(sdkRoot, "package.json"), "utf8"));
const baselinePatch = fs.readFileSync(path.join(sdkRoot, baselineManifest.patchedDependencies["nx@23.2.0"]), "utf8");
const baselineSpecs = diff.parsePatch(baselinePatch);
const changed = [];
function edit(file, transform) {
  const header = "diff --git a/" + file + " b/" + file, start = originalPatch.indexOf(header), end = start < 0 ? -1 : originalPatch.indexOf("\ndiff --git ", start + header.length);
  const oldSection = start < 0 ? undefined : originalPatch.slice(start, end < 0 ? undefined : end + 1), current = fs.readFileSync(path.join(sdk, file), "utf8");
  const baseline = baselineSpecs.find(spec => spec.oldFileName === "a/" + file);
  const pristine = baseline ? diff.applyPatch(current, diff.reversePatch(baseline)) : current;
  assert.notEqual(pristine, false, "Current SDK differs from the owned patch: " + file);
  const next = transform(current);
  const generated = diff.createTwoFilesPatch("a/" + file, "b/" + file, pristine, next, undefined, undefined, { context: 3 }).replace(/^=+\n/u, "");
  const section = header + "\n" + generated;
  assert.equal(diff.applyPatch(pristine, diff.parsePatch(section)[0]), next);
  if (oldSection) patch = patch.replace(oldSection, section);
  else patch += (patch.endsWith("\n") ? "" : "\n") + section;
  changed.push({ file, priorBytes: current.length, bytes: next.length });
}
edit("dist/src/daemon/server/watcher.js", source => {
  assert(source.includes("function dispatchWorkspaceChanges(events, observe = true) {"));
  source = source.replace("exports.watchOutputFiles = watchOutputFiles;", "exports.watchOutputFiles = watchOutputFiles;\nexports.registerSemioOutputExpressionsV1 = registerSemioOutputExpressionsV1;");
  source = source.replace("let activeServer;", "let semioOutputObserver, semioOutputReady, semioMarkerObserver;\nasync function semioOutputObservationV1() {\n    if (!semioOutputReady) semioOutputReady = (async () => {\n        const loaded = await require('../../utils/workspace-context').loadSemioGraphAuthorizersV1();\n        if (!loaded) throw Error('Original daemon output observation owner required');\n        const { Watcher } = await (0, handle_import_1.handleImport)('../../native/index.js', __dirname);\n        const observer = loaded.authorizers.createNxDaemonOutputObservationV1(async (scope, receive) => {\n            const prefix = (0, path_2.normalizePath)((0, path_1.relative)(workspace_root_1.workspaceRoot, scope.path));\n            const project = events => (events || []).map(event => { if (typeof event.path !== 'string' || (0, path_1.isAbsolute)(event.path)) throw Error('Relative Native output event required'); const path = (0, path_2.normalizePath)(event.path); return { type: event.type, path: prefix ? prefix + '/' + path : path }; });\n            const watcher = new Watcher(scope.path, [], false);\n            let stopped = false;\n            try { watcher.watch((error, events) => { if (stopped) return; try { receive(error, project(events)); } catch (failure) { receive(failure, []); } }); }\n            catch (error) { stopped = true; try { await watcher.stop(); } catch (stop) { throw new AggregateError([error, stop], 'Daemon output watcher startup and stop failed'); } throw error; }\n            return { flush: () => project(watcher.forceFlushPending()), stop: async () => { stopped = true; await watcher.stop(); } };\n        }, (error, events) => require('./handle-outputs-changes').handleOutputsChanges(error, events));\n        semioOutputObserver = observer;\n        return observer;\n    })();\n    return semioOutputReady;\n}\nasync function registerSemioOutputExpressionsV1(expressions) {\n    await (await semioOutputObservationV1()).register(expressions);\n}\nasync function flushSemioOutputChangesV1() {\n    if (semioOutputReady) await (await semioOutputReady).flush();\n    if (semioMarkerObserver) await semioMarkerObserver.flush();\n    const error = require('./handle-outputs-changes').getOutputsWatcherTerminalError();\n    if (error) throw error;\n}\n" + "\nlet activeServer;");
  source = source.replace("function dispatchWorkspaceChanges(events, observe = true) {", "function dispatchWorkspaceChanges(events, observe = true) {\n    if (semioOutputObserver) semioOutputObserver.receiveSource(events);");
  const flushStart = source.indexOf("async function flushPendingWorkspaceChanges()"), flushEnd = source.indexOf("\nasync function watchOutputFiles", flushStart);
  assert(flushStart > 0 && flushEnd > flushStart);
  source = source.slice(0, flushStart) + "async function flushPendingWorkspaceChanges() {\n    const watcher = (0, shutdown_utils_1.getWatcherInstance)();\n    if (watcher) {\n        const events = watcher.forceFlushPending();\n        if (events.length) await dispatchWorkspaceChanges(events);\n    }\n    await flushSemioOutputChangesV1();\n}" + source.slice(flushEnd);
  const start = source.indexOf("async function watchOutputFiles("), end = source.indexOf("\n/**", start);
  assert(start > 0 && end > start);
  return source.slice(0, start) + "async function watchOutputFiles(server, cb) {\n    const loaded = await require('../../utils/workspace-context').loadSemioGraphAuthorizersV1();\n    if (!loaded) throw Error('Original daemon output observation owner required');\n    const observer = await semioOutputObservationV1();\n    const marker = await loaded.authorizers.createNxDaemonProcessMarkerObservationV1(cache_1.serverProcessJsonPath, (error, events) => {\n        if (error) {\n            loaded.authorizers.requestNxDaemonShutdownV1('Daemon process marker observation failed');\n            return cb(error, null);\n        }\n        if (events.length && (0, cache_1.getDaemonProcessIdSync)() !== process.pid) {\n            const failure = Error('This process is no longer the current daemon');\n            loaded.authorizers.requestNxDaemonShutdownV1(failure.message);\n            return cb(failure, null);\n        }\n    });\n    semioMarkerObserver = marker;\n    let closing;\n    return { stop: () => closing ??= (async () => {\n        const results = await Promise.allSettled([observer.stop(), marker.stop()]);\n        semioOutputObserver = undefined; semioOutputReady = undefined; semioMarkerObserver = undefined;\n        const errors = results.filter(result => result.status === 'rejected').map(result => result.reason);\n        if (errors.length) throw errors.length === 1 ? errors[0] : new AggregateError(errors, 'Daemon output observations failed to stop');\n    })() };\n}" + source.slice(end);
});
edit("dist/src/daemon/server/outputs-tracking.js", source => {
  source = source.replace("if (now - timestamps[output] > 2000) {", "{").replace("recordedHashes[current] && now - timestamps[current] > 2000", "recordedHashes[current]");
  source = source.replace('const workspace_root_1 = require("../../utils/workspace-root");', 'const workspace_root_1 = require("../../utils/workspace-root");\nconst watcher_1 = require("./watcher");');
  source = source.replace("function outputsHashesMatchBatch(entries) {", "async function outputsHashesMatchBatch(entries) {\n    await (0, watcher_1.flushPendingWorkspaceChanges)();");
  source = source.replace("function recordOutputsHashBatch(entries) {", "async function recordOutputsHashBatch(entries) {\n    await (0, watcher_1.registerSemioOutputExpressionsV1)(entries.flatMap(entry => entry.outputs));\n    await (0, watcher_1.flushPendingWorkspaceChanges)();");
  return source;
});
edit("dist/src/daemon/server/handle-outputs-tracking.js", source => source.replace("(0, outputs_tracking_1.recordOutputsHashBatch)(payload.data);", "await (0, outputs_tracking_1.recordOutputsHashBatch)(payload.data);").replace("const results = (0, outputs_tracking_1.outputsHashesMatchBatch)(payload.data);", "const results = await (0, outputs_tracking_1.outputsHashesMatchBatch)(payload.data);"));
edit("dist/src/daemon/server/shutdown-utils.js", source => {
  source = source.replace("frozen.authorizers.beginNxDaemonShutdownV1();\n        try {", "frozen.authorizers.beginNxDaemonShutdownV1();\n        let primary;\n        try {");
  const before = "        } finally {\n            await frozen.authorizers.completeNxDaemonShutdownV1();\n            process.exit(0);\n        }";
  assert(source.includes(before));
  return source.replace(before, "        } catch (error) {\n            primary = error;\n        } finally {\n            try { await frozen.authorizers.completeNxDaemonShutdownV1(); }\n            catch (error) { if (!primary) primary = error; else if (primary !== error) primary = new AggregateError([primary, error], 'Daemon shutdown and observation stop failed'); }\n            if (primary) { console.error(primary); process.exit(1); }\n            process.exit(0);\n        }");
});
edit("dist/src/daemon/server/outputs-tracking.d.ts", source => source.replace("}[]): boolean[];", "}[]): Promise<boolean[]>;").replace("}[]): void;", "}[]): Promise<void>;"));
edit("dist/src/daemon/server/watcher.d.ts", source => source.replace("export declare function watchOutputFiles(server: Server, cb: FileWatcherCallback): Promise<any>;", "export declare function registerSemioOutputExpressionsV1(expressions: string[]): Promise<void>;\nexport declare function watchOutputFiles(server: Server, cb: FileWatcherCallback): Promise<{ stop(): Promise<void> }>;"));
edit("dist/src/daemon/server/shutdown-utils.d.ts", source => source.replace("storeOutputWatcherInstance(instance: Watcher)", "storeOutputWatcherInstance(instance: { stop(): Promise<void> })").replace("getOutputWatcherInstance(): Watcher", "getOutputWatcherInstance(): { stop(): Promise<void> }"));
fs.writeFileSync(patchPath, patch);
fs.writeFileSync(path.join(__dirname, "🗑️generated", "nx-output-patch-changes.json"), JSON.stringify(changed, null, 2) + "\n");
console.log(JSON.stringify(changed));

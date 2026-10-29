const fs = require("node:fs");
const path = require("node:path");
const { createRequire } = require("node:module");
const assert = require("node:assert/strict");
const root = process.cwd();
const generated = path.join(__dirname, "🗑️generated");
const sdk = path.join(root, ".cache/b/t/m/nx/80e05b73ff7c42b89e14c0c24ad8da978910f9805a8421498b80afadc432456a");
const fixture = fs.mkdtempSync(path.join(generated, "native-watcher-ready-"));
fs.mkdirSync(path.join(fixture, "a", "b"), { recursive: true });
fs.mkdirSync(path.join(fixture, "c"));
process.env.NX_NATIVE_FILE_CACHE_DIRECTORY = path.join(generated, "native-binding-cache");
const native = createRequire(path.join(sdk, "package.json"))(path.join(sdk,"node_modules/nx/dist/src/native/index.js"));
const snapshot = () => fs.readdirSync("/proc/self/fdinfo").flatMap(name => {
  try {
    const bytes = fs.readFileSync("/proc/self/fdinfo/" + name, "utf8");
    const watches = bytes.split("\n").filter(line => line.startsWith("inotify wd:")).length;
    return watches ? [{ fd: Number(name), watches }] : [];
  } catch { return []; }
});
(async () => {
  let watcher;
  try {
    const before = snapshot();
    watcher = new native.Watcher(fs.realpathSync(fixture), [], false);
    const afterConstructor = snapshot();
    const errors = [], received = [];
    watcher.watch((error, events) => { if (error) errors.push(String(error)); if (events) received.push(...events); });
    const immediatelyAfterWatch = snapshot();
    assert.equal(afterConstructor.reduce((n, row) => n + row.watches, 0), before.reduce((n, row) => n + row.watches, 0));
    assert.equal(immediatelyAfterWatch.reduce((n, row) => n + row.watches, 0) - before.reduce((n, row) => n + row.watches, 0), 4);
    fs.writeFileSync(path.join(fixture, "a", "b", "ready.txt"), "fixture");
    const flushed = watcher.forceFlushPending();
    const events = [...received, ...flushed];
    assert.equal(errors.length, 0);
    assert(events.some(event => event.path === "a/b/ready.txt"));
    const result = { platform: process.platform, nxVersion: "23.2.0", before, afterConstructor, immediatelyAfterWatch, events, errors, startupSleepMilliseconds: 0 };
    fs.writeFileSync(path.join(generated, "native-watcher-startup-proof.json"), JSON.stringify(result, null, 2) + "\n");
    console.log(JSON.stringify(result));
  } finally {
    if (watcher) await watcher.stop();
    fs.rmSync(fixture, { recursive: true, force: true });
  }
})().catch(error => { console.error(error); process.exitCode = 1; });

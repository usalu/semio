import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
const require = createRequire(import.meta.url), ticket = dirname(import.meta.dir), directory = mkdtempSync(join(ticket, "🗑️generated/daemon-retention-"));
const original = require.resolve("nx/src/daemon/logger.js"), source = process.env.SEMIO_NX_RETENTION_LOGGER ?? original, timer = process.argv[2] === "timer";
const vector = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/📓️daemon-retention.json", "utf8"));
const code = `
  const fs = require("node:fs"), assert = require("node:assert/strict"), path = require("node:path"), logger = {};
  const load = require("node:module").createRequire(${JSON.stringify(original)});
  new Function("require", "exports", fs.readFileSync(${JSON.stringify(source)}, "utf8"))(load, logger);
  const filename = load("./tmp-dir").DAEMON_OUTPUT_LOG_FILE, vector = ${JSON.stringify(vector)};
  fs.mkdirSync(path.dirname(filename), {recursive:true});
  const descriptor = fs.openSync(filename, "a"), inode = fs.fstatSync(descriptor).ino;
  let maximumRead = 0;
  const read = fs.readSync;
  fs.readSync = (fd, buffer, offset, length, position) => { maximumRead = Math.max(maximumRead, length); return read(fd, buffer, offset, length, position); };
  try {
    for (let cycle = 0; cycle < vector.cycles; cycle++) {
      fs.ftruncateSync(descriptor, vector.limitBytes * 2);
      fs.writeSync(descriptor, "\\n" + vector.tail);
      logger.clientLogger.writeToFile("retention cycle " + cycle);
      assert.ok(fs.statSync(filename).size <= vector.limitBytes, "Nx daemon log remains unbounded");
      assert.equal(fs.statSync(filename).ino, inode, "The daemon's open descriptor must keep its inode");
      fs.writeSync(descriptor, vector.append);
      const content = fs.readFileSync(filename, "utf8");
      assert.ok(content.includes(vector.tail)); assert.ok(content.endsWith(vector.append));
    }
    assert.ok(maximumRead > 0 && maximumRead <= vector.retainBytes);
  } finally { fs.closeSync(descriptor); }
  fs.rmSync(filename); fs.mkdirSync(filename); logger.pruneDaemonLog(); assert.ok(fs.statSync(filename).isDirectory()); fs.rmdirSync(filename);
  logger.pruneDaemonLog();
  const external = path.join(path.dirname(filename), "outside.log");
  fs.writeFileSync(external, "unowned bytes"); const externalFd = fs.openSync(external,"r+"); fs.ftruncateSync(externalFd,vector.limitBytes*2); fs.closeSync(externalFd);
  fs.linkSync(external,filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(external).size,vector.limitBytes*2); fs.rmSync(filename);
  if (process.platform !== "win32") { fs.symlinkSync(external,filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(external).size,vector.limitBytes*2); fs.rmSync(filename); }
  const actualDirectory = path.dirname(filename), savedDirectory = actualDirectory+"-original";
  fs.renameSync(actualDirectory,savedDirectory); fs.symlinkSync(savedDirectory,actualDirectory,process.platform === "win32" ? "junction" : "dir");
  fs.copyFileSync(path.join(savedDirectory,"outside.log"),filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(filename).size,vector.limitBytes*2);
  fs.rmSync(actualDirectory); fs.renameSync(savedDirectory,actualDirectory);
  if (${timer}) {
    global.NX_DAEMON = true;
    const native = fs.openSync(filename, "a"); fs.ftruncateSync(native, vector.limitBytes*2); fs.writeSync(native, vector.tail);
    setTimeout(() => { assert.ok(fs.statSync(filename).size <= vector.limitBytes, "Direct native writes need periodic retention"); fs.writeSync(native,vector.append); fs.closeSync(native); assert.ok(fs.readFileSync(filename,"utf8").endsWith(vector.append)); console.log("[DEBUG] Real 30-second daemon timer bounded direct native writes PASS"); }, 32000);
  }

  console.log("[DEBUG] Nx daemon retention: repeated native writes are bounded, tail preserved and existing append descriptor stays live PASS");
`;
try {
  const child = Bun.spawnSync(["node", "--eval", code], {cwd: process.cwd(), env:{...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(directory,"workspace-data")}, stdout:"pipe",stderr:"pipe",timeout:timer ? 35000 : 10000});
  assert.equal(child.exitCode,0,child.stdout.toString()+child.stderr.toString()); process.stdout.write(child.stdout);
} finally { rmSync(directory,{recursive:true,force:true}); }

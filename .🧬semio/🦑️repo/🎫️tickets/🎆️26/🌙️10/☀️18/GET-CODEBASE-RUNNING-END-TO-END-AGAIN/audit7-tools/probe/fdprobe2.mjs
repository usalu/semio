import { spawn, execFileSync } from "node:child_process";
import { openSync, closeSync, writeFileSync, readSync, fstatSync, statSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
const role = process.argv[2];
const probe = "const fs=require('fs');const out={};for(let f=3;f<12;f++){try{const s=fs.fstatSync(f);out[f]={ino:s.ino,size:s.size,file:s.isFile(),sock:s.isSocket(),fifo:s.isFIFO()}}catch{}};console.log(JSON.stringify(out))";
if (role === "worker") {
  const facts = fstatSync(3); const buf = Buffer.alloc(facts.size); readSync(3, buf, 0, buf.length, null);
  const viaBun = Bun.spawnSync([process.execPath, "-e", probe], { env: process.env }).stdout.toString().trim();
  const viaNode = String(execFileSync(process.execPath, ["-e", probe], { env: process.env })).trim();
  const viaSpawnIpcLike = await new Promise(r => { const c = spawn(process.execPath, ["-e", probe], { stdio: ["ignore", "pipe", "inherit"], env: process.env }); let o = ""; c.stdout.on("data", d => o += d); c.on("exit", () => r(o.trim())); });
  process.send({ snapshotIno: facts.ino, self: JSON.parse(execFileSync(process.execPath, ["-e", "console.log('{}')"]).toString()), viaBun, viaNode, viaSpawn: viaSpawnIpcLike, envKeys: Object.keys(process.env).filter(k => /NODE|BUN|IPC|CHANNEL/.test(k)) });
  process.disconnect();
} else {
  const dir = mkdtempSync(join(tmpdir(), "a7fd-")), file = join(dir, "snap.json"); writeFileSync(file, "x".repeat(1000), { mode: 0o600 });
  console.log("file ino", statSync(file).ino);
  const fd = openSync(file, "r");
  const child = spawn(process.execPath, [import.meta.path, "worker"], { stdio: ["ignore", "inherit", "inherit", fd, "ipc"], serialization: "json" });
  closeSync(fd);
  child.on("message", m => console.log(JSON.stringify(m, null, 1)));
}

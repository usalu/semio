import { spawn, spawnSync, execFileSync } from "node:child_process";
import { openSync, closeSync, writeFileSync, readSync, fstatSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
const role = process.argv[2];
if (role === "worker") {
  const facts = fstatSync(3); const buf = Buffer.alloc(facts.size); readSync(3, buf, 0, buf.length, null);
  const env = { NODE_CHANNEL_FD: process.env.NODE_CHANNEL_FD ?? null, NODE_CHANNEL_SERIALIZATION_MODE: process.env.NODE_CHANNEL_SERIALIZATION_MODE ?? null };
  const fds = cmd => { try { return String(execFileSync("/bin/sh", ["-c", "ls /dev/fd | tr '\\n' ' '; echo; echo NCF=$NODE_CHANNEL_FD"], { env: process.env })).trim(); } catch (e) { return "ERR " + e.message; } };
  const a = fds();
  const b = Bun.spawnSync(["/bin/sh", "-c", "ls /dev/fd | tr '\\n' ' '; echo; echo NCF=$NODE_CHANNEL_FD"], { env: process.env }).stdout.toString().trim();
  const c = Bun.spawnSync([process.execPath, "-e", "const fs=require('fs');const open=[];for(let f=3;f<10;f++){try{fs.fstatSync(f);open.push(f)}catch{}};console.log(JSON.stringify({open,ncf:process.env.NODE_CHANNEL_FD??null}))"], { env: process.env, stderr: "pipe" });
  process.send({ env, execFileSync: a, bunSpawnSync: b, bunChild: c.stdout.toString().trim() + " | err: " + c.stderr.toString().trim().slice(0, 300) });
  process.disconnect();
} else {
  const dir = mkdtempSync(join(tmpdir(), "a7fd-")), file = join(dir, "snap.json"); writeFileSync(file, "x".repeat(1000), { mode: 0o600 });
  const fd = openSync(file, "r");
  const child = spawn(process.execPath, [import.meta.path, "worker"], { stdio: ["ignore", "inherit", "inherit", fd, "ipc"], serialization: "json" });
  closeSync(fd);
  child.on("message", m => console.log(JSON.stringify(m, null, 1)));
}

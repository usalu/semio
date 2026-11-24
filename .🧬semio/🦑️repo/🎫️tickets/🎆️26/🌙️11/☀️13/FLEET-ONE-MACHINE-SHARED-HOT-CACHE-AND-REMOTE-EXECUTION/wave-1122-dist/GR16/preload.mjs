import fs from "node:fs"; import cp from "node:child_process"; import crypto from "node:crypto";
const C = globalThis.__gr16 = { calls: {}, bytes: {}, files: new Set(), spawns: [], hashes: 0, hashBytes: 0 };
const bump = (k, n = 1) => { C.calls[k] = (C.calls[k] ?? 0) + n; };
C.paths = {}; const R = process.cwd(); const where = (k, p) => { if (typeof p !== "string") return; const q = p.startsWith("/Users/ueli/Documents/semio/") ? p.slice(28) : p.replace(/^\/private\/var\/folders\/[^/]+\/[^/]+\/T\/gr16-[A-Za-z0-9]+/, "<staging>").replace(/[0-9a-f]{12,}/g, "H"); const key = k + " " + q.split("/").slice(0, 8).join("/"); C.paths[key] = (C.paths[key] ?? 0) + 1; };
for (const name of ["readFileSync", "openSync", "statSync", "lstatSync", "readdirSync", "existsSync", "realpathSync", "readlinkSync", "writeFileSync", "appendFileSync", "mkdirSync", "renameSync", "rmSync", "accessSync"]) {
  const original = fs[name]; if (typeof original !== "function") continue;
  fs[name] = function (...args) { bump(name); where(name, args[0]); if (name === "readFileSync" && typeof args[0] === "string") { C.files.add(args[0]); const out = original.apply(this, args); C.bytes.readFileSync = (C.bytes.readFileSync ?? 0) + (out?.length ?? 0); return out; } return original.apply(this, args); };
}
for (const name of ["readFile", "stat", "lstat", "readdir", "open", "writeFile", "access", "realpath"]) {
  const original = fs.promises[name]; if (typeof original !== "function") continue;
  fs.promises[name] = function (...args) { bump("p." + name); where("p." + name, args[0]); if (name === "readFile" && typeof args[0] === "string") C.files.add(args[0]); return original.apply(this, args); };
}
for (const name of ["spawn", "spawnSync", "execFile", "execFileSync", "exec", "execSync", "fork"]) {
  const original = cp[name]; cp[name] = function (...args) { bump("cp." + name); C.spawns.push([name, String(args[0]).slice(0, 80), Array.isArray(args[1]) ? args[1].join(" ").slice(0, 100) : ""]); return original.apply(this, args); };
}
const sp = Bun.spawn, sps = Bun.spawnSync; Bun.spawn = function (...a) { bump("Bun.spawn"); C.spawns.push(["Bun.spawn", JSON.stringify(a[0]).slice(0, 160)]); return sp.apply(this, a); };
Bun.spawnSync = function (...a) { bump("Bun.spawnSync"); C.spawns.push(["Bun.spawnSync", JSON.stringify(a[0]).slice(0, 160)]); return sps.apply(this, a); };
const ch = crypto.createHash; crypto.createHash = function (...a) { C.hashes++; const h = ch.apply(this, a); const u = h.update.bind(h); h.update = (d, ...r) => { C.hashBytes += d?.length ?? 0; return u(d, ...r); }; return h; };

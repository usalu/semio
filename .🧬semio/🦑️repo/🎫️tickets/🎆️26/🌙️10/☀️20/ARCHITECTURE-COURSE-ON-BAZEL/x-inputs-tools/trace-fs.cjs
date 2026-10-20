"use strict";
const fs = require("node:fs"), path = require("node:path");
const out = process.env.SEMIO_FS_TRACE_OUT;
if (out) {
  const append = fs.appendFileSync.bind(fs), seen = new Set();
  const note = (op, value) => {
    try {
      let p = value instanceof URL ? value.pathname : Buffer.isBuffer(value) ? value.toString() : typeof value === "string" ? value : undefined;
      if (p === undefined || p.startsWith("node:")) return;
      p = path.resolve(p);
      const key = op + "\t" + p;
      if (seen.has(key)) return;
      seen.add(key);
      append(out, key + "\n");
    } catch {}
  };
  const names = ["readFile", "readdir", "stat", "lstat", "access", "open", "opendir", "realpath", "readlink", "copyFile", "cp", "createReadStream"];
  for (const name of names) {
    for (const variant of [name, name + "Sync"]) {
      const original = fs[variant];
      if (typeof original !== "function") continue;
      fs[variant] = function (first, ...rest) { note(name, first); return original.call(this, first, ...rest); };
      if (original.native) fs[variant].native = original.native;
    }
    const promised = fs.promises[name];
    if (typeof promised === "function") fs.promises[name] = function (first, ...rest) { note(name, first); return promised.call(this, first, ...rest); };
  }
  const exists = fs.existsSync;
  fs.existsSync = function (first) { note("exists", first); return exists.call(this, first); };
  try { require("node:module").syncBuiltinESMExports(); } catch {}
}

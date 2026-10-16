import fs from "node:fs";
import crypto from "node:crypto";
import path from "node:path";
import { run, render } from "./split.mjs";
const [cfgPath, mode, scratch] = process.argv.slice(2);
const res = await run(cfgPath);
const files = new Map();
for (const k of res.out.keys()) files.set(k === res.keep ? res.file : res.fileOf(k), render(res, k));
if (mode === "scratch") {
  for (const [f, t] of files) {
    const dest = path.join(scratch, path.relative(res.P, f));
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.writeFileSync(dest, t);
  }
  console.log("scratch written", files.size);
} else if (mode === "write") {
  const now = crypto.createHash("sha256").update(fs.readFileSync(res.file, "utf8")).digest("hex");
  if (now !== res.hash) throw new Error("source changed during analysis; aborting");
  for (const [f, t] of files) if (f !== res.file && fs.existsSync(f)) throw new Error("refusing to overwrite " + f);
  for (const [f, t] of files) {
    fs.mkdirSync(path.dirname(f), { recursive: true });
    fs.writeFileSync(f, t);
  }
  console.log("written", files.size);
}

if (mode === "rewrite") {
  const old = await run(scratch);
  const oldFiles = new Map();
  for (const k of old.out.keys()) oldFiles.set(k === old.keep ? old.file : old.fileOf(k), render(old, k));
  for (const [f, t] of oldFiles) {
    if (!fs.existsSync(f) || fs.readFileSync(f, "utf8") !== t) throw new Error("disk differs from previous render (peer edit?): " + f);
  }
  let changed = 0;
  for (const [f, t] of files) {
    if (!oldFiles.has(f) || oldFiles.get(f) !== t) { fs.mkdirSync(path.dirname(f), { recursive: true }); fs.writeFileSync(f, t); changed += 1; }
  }
  console.log("rewrite ok; changed files:", changed);
}

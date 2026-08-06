#!/usr/bin/env bun
import { spawnSync } from "child_process";
import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const ticketDir = path.dirname(process.argv[1]);
process.env.DEVELOPER_DIR = "/Library/Developer/CommandLineTools";

const crates = JSON.parse(fs.readFileSync(path.join(ticketDir, "🧪spot-crates.json"), "utf8"));
const out = [];
out.push("=== cargo check spot " + new Date().toISOString() + " ===");
out.push("DEVELOPER_DIR=" + process.env.DEVELOPER_DIR);
out.push("");
const summary = [];
for (const rel of crates) {
  const manifest = path.join(root, rel, "Cargo.toml");
  out.push("--- " + rel + " ---");
  if (!fs.existsSync(manifest)) { out.push("MISSING"); summary.push({rel, ok:false, reason:"missing"}); continue; }
  const r = spawnSync("cargo", ["check", "--manifest-path", manifest], { cwd: root, encoding: "utf8", env: process.env, timeout: 600000 });
  if (r.stdout) out.push(r.stdout);
  if (r.stderr) out.push(r.stderr);
  out.push("exit=" + r.status);
  summary.push({ rel, ok: r.status === 0, status: r.status });
  out.push("");
}
out.push("=== SUMMARY ===");
out.push(JSON.stringify(summary, null, 2));
fs.writeFileSync(path.join(ticketDir, "🧪cargo-check-spot.log"), out.join("\n"));
fs.writeFileSync(path.join(ticketDir, "🧪cargo-check-summary.json"), JSON.stringify(summary, null, 2));
console.log(JSON.stringify(summary, null, 2));
process.exit(summary.every(s => s.ok) ? 0 : 1);
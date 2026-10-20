import fs from "node:fs";
import path from "node:path";
import { entries, sha } from "./lib.mjs";
import { build as comments } from "./b1-comments.mjs";
import { build as host } from "./b2-host.mjs";
import { build as registry } from "./b3-registry.mjs";
import { build as pointers } from "./b4-pointers.mjs";
import { build as debug } from "./b5-debug.mjs";
import { build as legacy } from "./b6-legacy.mjs";

const here = import.meta.dir;
const out = path.join(here, "staged");
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(path.join(out, "after"), { recursive: true });
const groups = { comments, host, registry, pointers, debug, legacy };
const result = {};
for (const [name, run] of Object.entries(groups)) result[name] = run();
const rows = [];
let index = 0;
for (const [file, state] of entries().sort(([a], [b]) => (a < b ? -1 : 1))) {
  index++;
  const ext = path.extname(file) || ".txt";
  const name = `after/${String(index).padStart(4, "0")}${ext}`;
  fs.writeFileSync(path.join(out, name), state.after, "utf8");
  rows.push({ path: file, beforeSha256: state.before === null ? null : sha(state.before), afterSha256: sha(state.after), after: name });
}
fs.writeFileSync(path.join(out, "manifest.json"), JSON.stringify({ version: 1, root: "repository root", files: rows }, null, 1) + "\n");
const longest = Math.max(...rows.map((row) => row.path.length));
console.log(JSON.stringify({ files: rows.length, longestRelativePath: longest, notes: result.debug?.notes ?? [] }));

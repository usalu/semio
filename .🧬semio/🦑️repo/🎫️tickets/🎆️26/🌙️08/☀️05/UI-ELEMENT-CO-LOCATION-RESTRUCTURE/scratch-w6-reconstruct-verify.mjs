import fs from "fs";
import path from "path";

const WGPU = process.env.WGPU;
const LIB = process.env.LIB;
const CRAB = "🦀️";

const lib = fs.readFileSync(LIB, "utf8");
const files = fs.readdirSync(WGPU).filter((f) => f.startsWith(CRAB) && f.endsWith(".rs"));

const results = [];
for (const f of files) {
  const name = f.slice(CRAB.length, -3); // strip emoji prefix and .rs
  const body = fs.readFileSync(path.join(WGPU, f), "utf8");
  let open = 0, close = 0;
  for (const ch of body) {
    if (ch === "{") open++;
    else if (ch === "}") close++;
  }
  const lines = body.split("\n").length;
  // path declared?
  const pathRe = new RegExp(`#\\[path = "${CRAB}${name}\\.rs"\\]`);
  const wired = pathRe.test(lib) || (name === "label" && lib.includes(`${CRAB}label.rs`));
  results.push({ file: f, name, lines, braces: { open, close, ok: open === close }, wired });
}

const allOk = results.every((r) => r.braces.ok && r.wired);
console.log(JSON.stringify({ allOk, libLines: lib.split("\n").length, results }, null, 2));
fs.writeFileSync(path.join(process.env.TICKET, "scratch-w6-reconstruct-verify.json"), JSON.stringify({ allOk, results }, null, 2));

import { readdirSync, readFileSync, writeFileSync, statSync, existsSync } from "node:fs";
import { join, relative } from "node:path";

const root = "/Users/ueli/Documents/semio";
const ticket = join(import.meta.dir);
const pluginsRoot = join(root, "✏️s/🔌️plugins");
const exact = [
  "🏗️fem","🏛️architect","🏭️process","📖️playbook","🌍️gis","📋️forms","📜️imperative","🪐️space"
].map((n) => join(pluginsRoot, n));
const sourcing = readdirSync(pluginsRoot).find((n) => n.includes("sourcing"));
exact.push(join(pluginsRoot, sourcing));

function walk(dir, acc = []) {
  if (!existsSync(dir)) return acc;
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, acc);
    else acc.push(p);
  }
  return acc;
}

const exampleParts = [];
for (const p of exact) {
  for (const f of walk(join(p, "🗿️artifacts"))) {
    if (!f.includes("📚️examples")) continue;
    if (!(f.endsWith(".dsl.semio") || f.endsWith(".op.semio"))) continue;
    const text = readFileSync(f, "utf8");
    const lines = text.split(/\r?\n/);
    const limit = f.endsWith(".op.semio") ? 100 : 160;
    exampleParts.push(`########## ${relative(root, f)} (${lines.length} lines) ##########\n`);
    exampleParts.push(lines.slice(0, limit).join("\n") + "\n\n");
  }
}
writeFileSync(join(ticket, "🧪w4d-examples-dump.txt"), exampleParts.join(""));

const kw = [];
const pat = /dsl\(keyword\s*=\s*"([^"]+)"/g;
for (const p of exact) {
  kw.push(`===== ${relative(root, p)} =====\n`);
  for (const f of walk(p)) {
    if (!f.endsWith(".rs") || f.includes("/target/")) continue;
    const text = readFileSync(f, "utf8");
    let m;
    while ((m = pat.exec(text))) kw.push(`${relative(root, f)}: ${m[1]}\n`);
  }
  kw.push("\n");
}
writeFileSync(join(ticket, "🧪w4d-dsl-keywords.txt"), kw.join(""));

const opParts = [];
for (const p of exact) {
  for (const f of walk(join(p, "🗿️artifacts"))) {
    if (!f.includes("/🔧️op/🦀️component.rs")) continue;
    const text = readFileSync(f, "utf8");
    opParts.push(`===== ${relative(root, f)} =====\n`);
    const matches = text.match(/pub enum \w*Operation[^{]*\{[\s\S]*?\n\}/g) || [];
    if (matches.length) opParts.push(matches[0].slice(0, 6000) + "\n");
    else {
      const m2 = text.match(/enum \w*Operation\w*[^{]*\{[\s\S]*?\n\}/g);
      opParts.push((m2?.[0]?.slice(0, 4000) || "(no Operation enum)") + "\n");
    }
    opParts.push("\n");
  }
}
writeFileSync(join(ticket, "🧪w4d-op-enums.txt"), opParts.join(""));
console.log("[DEBUG] extract done", { examples: exampleParts.length, kw: kw.length, ops: opParts.length });

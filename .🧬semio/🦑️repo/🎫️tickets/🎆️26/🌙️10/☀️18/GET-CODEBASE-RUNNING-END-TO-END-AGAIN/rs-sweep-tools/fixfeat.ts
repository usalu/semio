import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createHash } from "node:crypto";
import { Finding, Package, loadPackage } from "./cargo.ts";

const ROOT = "/Users/ueli/Documents/semio";
const C = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const G = `${C}/🗑️generated/rs-sweep-run`;
const backupDir = `${C}/🗑️generated/rs-sweep-x/backup`;
const apply = process.argv.includes("--apply");
const input = process.argv.find((a) => a.endsWith(".jsonl"))!;
const findings: Finding[] = readFileSync(input, "utf8").trim().split("\n").filter(Boolean).map((l) => JSON.parse(l));
const files = readFileSync(`${G}/files.nul`, "utf8").split("\0").filter(Boolean).map((f) => join(ROOT, f));
const byName = new Map<string, Package>();
for (const f of files) if (f.endsWith("/Cargo.toml") && !f.includes("/🗑️generated/") && !f.includes("/🎫️tickets/") && existsSync(f)) {
  const p = loadPackage(dirname(f));
  if (p) byName.set(p.name, p);
}
const ledgerPath = `${backupDir}/ledger.json`;
const ledger: Record<string, string> = JSON.parse(readFileSync(ledgerPath, "utf8"));
const want = new Map<string, Set<string>>();
for (const f of findings) {
  const m = /^cfg\(feature = "([^"]+)"\) but (\S+) declares no such feature/.exec(f.msg);
  if (f.kind !== "cfg-feature-undeclared" || !m) continue;
  const s = want.get(m[2]) ?? new Set();
  s.add(m[1]);
  want.set(m[2], s);
}
let n = 0;
for (const [pkgName, feats] of want) {
  const p = byName.get(pkgName);
  if (!p) continue;
  let text = readFileSync(p.manifest, "utf8");
  const lines = text.split("\n");
  const add = [...feats].filter((x) => !(x in p.features)).map((x) => `${/^[A-Za-z0-9_-]+$/.test(x) ? x : JSON.stringify(x)} = []`);
  if (!add.length) continue;
  const idx = lines.findIndex((l) => l.trim() === "[features]");
  if (idx < 0) text = text.replace(/\n*$/, "\n") + `\n[features]\n${add.join("\n")}\n`;
  else {
    let at = idx + 1;
    while (at < lines.length && !/^\s*\[/.test(lines[at])) at++;
    while (at > idx + 1 && lines[at - 1].trim() === "") at--;
    lines.splice(at, 0, ...add);
    text = lines.join("\n");
  }
  n += add.length;
  console.log(pkgName, add.join(", "));
  if (apply) {
    if (!ledger[p.manifest]) {
      const h = createHash("sha1").update(p.manifest).digest("hex").slice(0, 16);
      writeFileSync(`${backupDir}/${h}`, readFileSync(p.manifest));
      ledger[p.manifest] = h;
      writeFileSync(ledgerPath, JSON.stringify(ledger, null, 1));
    }
    writeFileSync(p.manifest, text);
  }
}
console.log({ apply, features: n });

import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const C = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lay1-tools/";
const lines = [...readFileSync(C + "renames1.txt", "utf8").split("\n"), ...readFileSync(C + "renames2.txt", "utf8").split("\n")].filter(Boolean);
const out: string[] = [];
for (const l of lines) {
  const [o, n] = l.split("|");
  const p = join(dirname(o), n);
  const ex = existsSync(join(root, p));
  const ent = ex ? readdirSync(join(root, p)).join(",") : "MISSING";
  out.push(`${ex ? "ok" : "MISSING"}\t${p}\t${ent.slice(0, 120)}`);
}
console.log(out.join("\n"));

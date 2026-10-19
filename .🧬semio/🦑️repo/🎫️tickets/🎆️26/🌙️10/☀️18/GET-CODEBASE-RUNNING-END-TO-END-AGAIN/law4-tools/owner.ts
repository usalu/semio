import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const C = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const cargos = readFileSync(C + "/🗑️generated/law4-out/plugin-cargos.txt", "utf8").split("\n").filter(Boolean);
const owner = new Map<string, string[]>();
for (const c of cargos) {
  const f = join(root, c); if (!existsSync(f)) continue;
  for (const m of readFileSync(f, "utf8").matchAll(/^\s*path\s*=\s*"([^"]*\.rs)"/gmu)) {
    const t = resolve(dirname(f), m[1]!);
    (owner.get(t) ?? owner.set(t, []).get(t)!).push(c);
  }
}
for (const arg of process.argv.slice(2)) console.log(arg.slice(-90), "=>", owner.get(join(root, arg)) ?? "NONE");
export { owner };

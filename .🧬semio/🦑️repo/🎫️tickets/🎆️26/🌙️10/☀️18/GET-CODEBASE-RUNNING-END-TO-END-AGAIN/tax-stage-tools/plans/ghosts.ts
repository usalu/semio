// 👻 Leftovers of tax2's script-to-module rename of the Nx bootstrap tools: references to the removed `🛠️tools/📜️script.ts` follow its successor `🟦️.ts`.
import {readFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {C, LIB, REPO, Vfs} from "../vfs.ts";
import {rewriteReferences} from "../refs-lib.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";

export function planGhosts(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  const tools = `${LIB}/${E(0x26A1)}caching/${E(0x1F680)}bootstrap/${E(0x1F6E0)}tools`, from = `${tools}/${E(0x1F4DC)}script.ts`, to = `${tools}/${E(0x1F7E6)}.ts`;
  if (v.exists(from)) throw new Error("the removed tools script reappeared");
  const touched: string[] = [];
  for (const file of readFileSync(`${C}/🗑️generated/tax-stage/grep-files.txt`, "utf8").split("\n")) {
    if (!file.startsWith(tools + "/") || !v.exists(file)) continue;
    const r = rewriteReferences(v.read(file), dirname(join(REPO, file)), from, to, false);
    if (r.count) { v.write(file, r.text); touched.push(file.replace(LIB, "L")); }
  }
  log({kind: "ghost", from: from.replace(LIB, "L"), to: to.replace(LIB, "L"), touched});
}

// 🏷️ Module-scripts (no entry, only exports): the file is a module, so it takes the module name `🟦️.ts` and every referencer follows.
import {dirname, join} from "node:path";
import {LIB, REPO, Vfs} from "../vfs.ts";
import {refsOf, rewriteReferences} from "../refs-lib.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";
const SCRIPT = E(0x1F4DC) + "script.ts", TS = E(0x1F7E6) + ".ts";

export const moduleScripts = (): string[] => [
  `${LIB}/${E(0x26A1)}caching/${E(0x1F680)}bootstrap/${E(0x1F6E0)}tools/${E(0x1F3D7)}bazel/${SCRIPT}`,
  `${LIB}/${E(0x1F3D7)}build/${E(0x1F527)}tools/${E(0x1F3D7)}bazel/${SCRIPT}`,
];

export function planModuleScripts(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  for (const from of moduleScripts()) {
    const to = `${dirname(from)}/${TS}`;
    if (v.exists(to)) throw new Error(`occupied: ${to}`);
    v.write(to, v.read(from));
    v.remove(from);
    const touched: string[] = [];
    for (const file of new Set(refsOf(from).map((r) => r.file))) {
      if (file === from || !v.exists(file)) continue;
      const r = rewriteReferences(v.read(file), dirname(join(REPO, file)), from, to, false);
      if (r.count) { v.write(file, r.text); touched.push(file); }
    }
    log({kind: "module-script", from: from.replace(LIB, "L"), to: to.replace(LIB, "L"), referencers: touched.length, touched: touched.map((t) => t.replace(LIB, "L"))});
  }
}

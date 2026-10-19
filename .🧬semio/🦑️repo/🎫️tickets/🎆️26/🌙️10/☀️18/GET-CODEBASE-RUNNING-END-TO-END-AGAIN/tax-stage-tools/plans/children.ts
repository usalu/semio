// 📦️ Test-spawned child programs are not command scripts: each moves from `<dir>/📜️script.ts` to `<dir>/📦️child/🟦️.ts` and every spawner and referencer follows.
import {readFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {LIB, REPO, TOOLS, Vfs} from "../vfs.ts";
import {rebaseOwn, rewriteReferences} from "../refs-lib.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";
const SCRIPT = E(0x1F4DC) + "script.ts", TS = E(0x1F7E6) + ".ts", CHILD = E(0x1F4E6) + "child";
export const childList = (): string[] => readFileSync(`${TOOLS}/t-children.txt`, "utf8").split("\n").filter(Boolean).map((r) => `${LIB}/${r}`);

export function planChildren(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  for (const dir of childList()) {
    const from = `${dir}/${SCRIPT}`;
    const inChild = dir.endsWith("/" + CHILD);
    const to = inChild ? `${dir}/${TS}` : `${dir}/${CHILD}/${TS}`;
    if (v.exists(to)) throw new Error(`occupied: ${to}`);
    const spawnDir = inChild ? dirname(dir) : dir, newRel = inChild ? `${CHILD}/${TS}` : `${CHILD}/${TS}`;
    v.write(to, rebaseOwn(v.read(from), join(REPO, dir), join(REPO, dirname(to)), v));
    v.remove(from);
    const touched: string[] = [];
    const sibling = `${spawnDir}/${TS}`;
    if (v.exists(sibling)) {
      let text = v.read(sibling), count = 0;
      text = text.replace(/((?:import\.meta\.dir|importMetaDirectory),\s*)(["'])(?:\.\/)?📜️script\.ts\2/gu, (_m, head: string, q: string) => { count++; return `${head}${q}${newRel}${q}`; });
      text = text.replace(/new URL\((["'])\.\/📜️script\.ts\1,\s*import\.meta\.url\)/gu, (_m, q: string) => { count++; return `new URL(${q}./${newRel}${q},import.meta.url)`; });
      if (count) { v.write(sibling, text); touched.push(`${sibling} (${count} spawn sites)`); }
    }
    for (const ref of new Set(refsFor(from).map((r) => r.file))) {
      if (ref === from || !v.exists(ref)) continue;
      const r = rewriteReferences(v.read(ref), dirname(join(REPO, ref)), from, to, false);
      if (r.count) { v.write(ref, r.text); touched.push(`${ref} (${r.count} references)`); }
    }
    if (dir.endsWith("/" + E(0x1F4E4) + "arguments")) {
      const json = `${dir}/${E(0x1F523)}.json`, text = v.read(json), count = text.split("bun ./" + SCRIPT).length - 1;
      v.write(json, text.split("bun ./" + SCRIPT).join("bun ./" + newRel));
      touched.push(`${json} (${count} command literals)`);
    }
    log({kind: "child", from, to, touched});
  }
}

import {refsOf} from "../refs-lib.ts";
const refsFor = refsOf;

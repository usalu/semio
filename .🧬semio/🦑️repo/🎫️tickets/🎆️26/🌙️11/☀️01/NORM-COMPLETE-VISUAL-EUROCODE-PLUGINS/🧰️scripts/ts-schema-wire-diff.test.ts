import { test } from "bun:test";
import { writeFileSync } from "node:fs";
import { encode, setups } from "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts";
test("dump wire diffs", () => {
  const out: string[] = [];
  for (const s of setups) for (const w of s.corpus) {
    if (w.negative) continue;
    let got: string;
    try { got = `${encode(s.schemas, s.twins[w.role](JSON.parse(w.text)) as never, s.roots[w.role], s.roots[w.role])}\n`; } catch (e) { out.push(`${s.artifact}\t${w.path}\tTHROW ${String(e).slice(0,200)}`); continue; }
    if (got === w.text) continue;
    const a = got.split("\n"), b = w.text.split("\n");
    let i = 0; while (i < a.length && a[i] === b[i]) i++;
    out.push(`${s.artifact}\t${w.path}\tline ${i + 1}\n  want: ${b[i]}\n  got:  ${a[i]}`);
  }
  writeFileSync(process.env.OUT!, out.join("\n") + "\n");
});

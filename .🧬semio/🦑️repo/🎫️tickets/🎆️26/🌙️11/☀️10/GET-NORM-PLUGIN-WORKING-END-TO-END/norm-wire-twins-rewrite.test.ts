import { test } from "bun:test";
import { writeFileSync } from "node:fs";
import { setups, encode } from "../../../../../../../✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts";
test("rewrite", () => {
  const apply = process.env.APPLY === "1";
  let n = 0;
  for (const s of setups) for (const w of s.corpus) {
    if (w.negative) continue;
    const value = JSON.parse(w.text);
    const out = `${encode(s.schemas, (s.twins as any)[w.role](value), (s.roots as any)[w.role], (s.roots as any)[w.role])}\n`;
    if (out === w.text) continue;
    n++;
    if (apply) writeFileSync(w.path, out);
    else if (w.path.includes('en1998') || n < 2) { console.log(w.path); writeFileSync(`/private/tmp/claude-501/-Users-ueli--claude-spares-spare-HZaUUD/b5f92f94-d168-46cc-96bc-ef90adb1be75/scratchpad/before${n}.json`, w.text); writeFileSync(`/private/tmp/claude-501/-Users-ueli--claude-spares-spare-HZaUUD/b5f92f94-d168-46cc-96bc-ef90adb1be75/scratchpad/after${n}.json`, out); }
  }
  console.log("differing", n);
});

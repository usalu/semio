import { test } from "bun:test";
import { writeFileSync, appendFileSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";
import { encode, setups } from "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts";
test("canonicalize order-only wire drift", () => {
  const log = process.env.OUT!; writeFileSync(log, "");
  for (const s of setups) for (const w of s.corpus) {
    if (w.negative) continue;
    let got: string, parsed: unknown;
    try { parsed = s.twins[w.role](JSON.parse(w.text)); got = `${encode(s.schemas, parsed as never, s.roots[w.role], s.roots[w.role])}\n`; } catch (e) { appendFileSync(log, `THROW ${w.path} ${String(e).slice(0,160)}\n`); continue; }
    if (got === w.text) continue;
    if (isDeepStrictEqual(JSON.parse(got), JSON.parse(w.text))) { if (process.env.APPLY) writeFileSync(w.path, got); appendFileSync(log, `ORDER ${w.path}\n`); }
    else appendFileSync(log, `SEMANTIC ${w.path}\n`);
  }
});

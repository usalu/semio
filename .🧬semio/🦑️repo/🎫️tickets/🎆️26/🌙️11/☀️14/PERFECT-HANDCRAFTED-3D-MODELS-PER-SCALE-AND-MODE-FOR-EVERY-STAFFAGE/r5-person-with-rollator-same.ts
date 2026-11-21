/** 🪞 Compares the volumes the round-3 scripts of `humans/person-with-rollator` build with the current source, list by list. */
import { readFileSync } from "node:fs";
import { sourcePath } from "./volumes-lib.ts";
import { ENTRY } from "./r5-person-with-rollator-kit.ts";
import { volumes } from "./r5-person-with-rollator-build.ts";
const cur = JSON.parse(readFileSync(sourcePath(ENTRY), "utf8")).volumes;
const built = await volumes();
for (const s of ["50", "100", "200", "500"]) for (const l of ["body", "thermal", "structure", "decisions"]) {
  const a = cur[s][l], b = built[s][l];
  if (l === "decisions") { console.log(s, l, JSON.stringify(a) === JSON.stringify(b)); continue; }
  const ids = new Set([...a.map((x: any) => x.id), ...b.map((x: any) => x.id)]);
  const diff = [...ids].filter((id) => JSON.stringify(a.find((x: any) => x.id === id)) !== JSON.stringify(b.find((x: any) => x.id === id)));
  console.log(s, l, a.length, b.length, diff.length, diff.slice(0, 8).join(" "));
}

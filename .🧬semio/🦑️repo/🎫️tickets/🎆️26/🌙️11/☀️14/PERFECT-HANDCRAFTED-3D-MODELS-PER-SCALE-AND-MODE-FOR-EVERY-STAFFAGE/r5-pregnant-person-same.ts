/** 🪞 Compares the volumes the round-3 scripts of `humans/pregnant-person` build with the current source, list by list and solid by solid. */
import { readFileSync } from "node:fs";
import { SOURCE } from "./r3-pregnant-person-io.ts";
import { volumes } from "./r3-pregnant-person-build.ts";
const cur = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
const built = await volumes(structuredClone(cur), false);
for (const s of ["50", "100", "200", "500"]) for (const l of ["body", "thermal", "structure"]) {
  const a = cur[s][l], b = built[s][l];
  const ids = new Set([...a.map((x: any) => x.id), ...b.map((x: any) => x.id)]);
  const diff = [...ids].filter((id) => JSON.stringify(a.find((x: any) => x.id === id)) !== JSON.stringify(b.find((x: any) => x.id === id)));
  console.log(s, l, a.length, b.length, diff.length, diff.slice(0, 8).join(" "));
}

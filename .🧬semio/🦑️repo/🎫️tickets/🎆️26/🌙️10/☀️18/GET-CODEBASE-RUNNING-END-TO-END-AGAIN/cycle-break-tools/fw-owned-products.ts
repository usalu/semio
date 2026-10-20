import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane } from "../exports-stage-tools/model.ts";
import { listAll } from "../exports-stage-tools/lib.ts";
const policy = loadPolicy();
const by = new Map<string, number>();
for (const f of listAll()) {
  if (!f.startsWith("🧰️framework/🛍️products/")) continue;
  const o = ownerOfFile(policy, f); const n = o ? lane(o.owner)?.name ?? o.names[0]! : "-";
  if (n !== "@semio-tech/framework") continue;
  const m = /^🧰️framework\/🛍️products\/([^/]+)\/(?:🔨️modules\/)?([^/]+)/u.exec(f);
  const k = m ? m[1] + "/" + m[2] : f.split("/").slice(0, 4).join("/");
  by.set(k, (by.get(k) ?? 0) + 1);
}
for (const [k, v] of [...by].sort((a, b) => b[1] - a[1])) console.log(v, k);

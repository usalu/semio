import { build } from "./b5-debug.mjs";
import { entries } from "./lib.mjs";
build();
for (const [p, v] of entries()) {
  if (!p.endsWith(".rs")) continue;
  const a = v.before.split("\n"), b = v.after.split("\n");
  console.log("--", p.split("/").slice(-4).join("/"), a.length, b.length);
  // naive line diff
  let i = 0, j = 0;
  while (i < a.length || j < b.length) {
    if (a[i] === b[j]) { i++; j++; continue; }
    // find resync
    let k = 1, found = false;
    for (; k < 6 && !found; k++) { if (a[i + k] === b[j]) { for (let t = 0; t < k; t++) console.log("  - " + a[i + t].slice(0, 170)); i += k; found = true; } else if (a[i] === b[j + k]) { for (let t = 0; t < k; t++) console.log("  + " + b[j + t].slice(0, 170)); j += k; found = true; } }
    if (!found) { console.log("  ~ " + a[i]?.slice(0, 240)); console.log("  ~> " + b[j]?.slice(0, 240)); i++; j++; }
  }
}

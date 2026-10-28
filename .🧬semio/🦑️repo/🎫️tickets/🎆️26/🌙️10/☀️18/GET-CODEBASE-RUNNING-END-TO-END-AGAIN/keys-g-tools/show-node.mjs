// show-node: bun show-node.mjs <record> [<record>] : namedInputs of an infer-one record (brotli or plain), owner-glob entries only.
import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
for (const file of process.argv.slice(2)) {
  const raw = readFileSync(file);
  let text; try { text = brotliDecompressSync(raw).toString("utf8"); } catch { text = raw.toString("utf8"); }
  const doc = JSON.parse(text), named = doc.node?.namedInputs ?? doc.namedInputs ?? {};
  console.log("##", file.split("/").pop(), Object.keys(named).sort().join(","));
  for (const [name, list] of Object.entries(named)) {
    const hits = (list).filter((item) => typeof item === "string" && /framework\/(\*\*|[^/]+\/\*\*|\*\.)/u.test(item) && !item.startsWith("!"));
    if (hits.length) console.log(" ", name, hits.map((h) => h.replace("{workspaceRoot}/🧰️framework/", "fw/").slice(0, 70)).join(" | ").slice(0, 700));
  }
}

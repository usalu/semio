import { PreviewGeneratedScript } from "/home/deck/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
const root = process.cwd();
const s = Object.create(PreviewGeneratedScript.prototype);
s.repoRoot = "/home/deck/semio"; s.root = root;
let out = ""; const w = process.stdout.write.bind(process.stdout);
(process.stdout as any).write = (c: any) => { out += c; return true; };
await s.run();
(process.stdout as any).write = w;
const j = JSON.parse(out);
let drift = 0, files = 0;
for (const n of j.nodes) if (n.nodeKind === "file") { files++; const p = "/home/deck/semio/" + n.path; if (!existsSync(p) || Buffer.compare(readFileSync(p), Buffer.from(n.bytesBase64, "base64")) !== 0) { drift++; console.log("DRIFT", n.path); } }
console.log(`preview: files=${files} drift=${drift} staleRemovals=${j.staleRemovals.length}`, j.staleRemovals.slice(0,5));
process.exit(drift || j.staleRemovals.length ? 1 : 0);

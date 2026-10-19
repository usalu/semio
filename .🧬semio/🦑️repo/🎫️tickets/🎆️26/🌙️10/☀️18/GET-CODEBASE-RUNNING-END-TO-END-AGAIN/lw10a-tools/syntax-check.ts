import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const t = new Bun.Transpiler({ loader: "ts" });
let bad = 0, mine = 0;
for (const f of files) {
  const text = readFileSync("/Users/ueli/Documents/semio/" + f, "utf8");
  try { t.transformSync(text); } catch (e: any) {
    bad++;
    const m = e?.errors?.[0] ?? e; const line = m?.position?.line;
    const lt = line ? text.split("\n")[line - 1] ?? "" : "";
    const involvesProcess = text.split("\n").some((l) => /^import.*(?:"|')[^"']*🏃️process\/📦️packages[^"']*(?:"|')/.test(l) === false && false);
    console.log("SYNTAX", f.split("/").slice(-4).join("/"), "line", line, JSON.stringify(lt.slice(0, 140)));
  }
}
console.log({ files: files.length, bad });

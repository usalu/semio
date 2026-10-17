// 🔍 bd9: parse every listed TS/TSX file with Bun's transpiler; prints FAIL lines
import {readFileSync} from "node:fs";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const f of files) {
  const loader = f.endsWith(".tsx") ? "tsx" : "ts";
  try { new Bun.Transpiler({loader}).transformSync(readFileSync(f, "utf8")); } catch (e) { bad++; console.log("FAIL\t" + f + "\t" + String((e as Error).message).split("\n")[0]!.slice(0, 160)); }
}
console.log(`PARSED ${files.length} FAILED ${bad}`);

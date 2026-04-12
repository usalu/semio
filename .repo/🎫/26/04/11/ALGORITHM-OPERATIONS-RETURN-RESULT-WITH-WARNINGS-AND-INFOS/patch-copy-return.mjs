import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "../../../../../");
const p = path.join(root, "semio/js/index.ts");
let s = fs.readFileSync(p, "utf8");
const old = `  return { guid: "", name: "", pieces: copyPieces, connections: copyConnections };
};`;
const neu = `  return operationOk(
    { guid: "", name: "", pieces: copyPieces, connections: copyConnections },
    flatRes.warnings,
    [
      ...flatRes.infos,
      {
        code: "copy.summary",
        message: \`Copied \${copyPieces.length} piece(s) and \${copyConnections.length} connection(s) to clipboard design.\`,
      },
    ],
  );
};`;
if (!s.includes(old)) {
  console.error("old snippet not found");
  process.exit(1);
}
s = s.replace(old, neu);
fs.writeFileSync(p, s);
console.log("patched");

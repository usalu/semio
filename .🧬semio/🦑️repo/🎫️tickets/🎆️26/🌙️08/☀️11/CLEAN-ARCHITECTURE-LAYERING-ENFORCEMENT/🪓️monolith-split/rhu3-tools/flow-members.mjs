import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const RT = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const cfgPath = RT + "🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json";
const cfg = ts.readConfigFile(cfgPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, path.dirname(cfgPath));
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options });
const checker = program.getTypeChecker();
const wanted = new Set((process.env.TYPES ?? "FlowSession").split(","));
const members = new Map();
for (const sf of program.getSourceFiles()) {
  if (!sf.fileName.startsWith(RT)) continue;
  const visit = (n) => {
    if (ts.isPropertyAccessExpression(n)) {
      const t = checker.getTypeAtLocation(n.expression);
      const sym = t.getSymbol() ?? t.aliasSymbol;
      const names = [sym?.getName(), t.aliasSymbol?.getName()];
      if (names.some((x) => x && wanted.has(x))) {
        const key = n.name.text;
        const arr = members.get(key) ?? [];
        arr.push(sf.fileName.replace(RT, "RT/"));
        members.set(key, arr);
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
console.log([...members].map(([k, v]) => `${k}\t${new Set(v).size}`).sort().join("\n"));
console.error("members", members.size);

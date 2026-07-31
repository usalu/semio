import { readFileSync, writeFileSync } from "node:fs";

let c = readFileSync("c:/git/compose/spatial/js/renderer-r3f/index.tsx", "utf8");
c = c.split("activePickViewKinds").join("activePickKinds");
c = c.replace(/replPruneSelectionByKind\(prev, pickViewKind,/g, "replPruneSelectionByKind(prev, activeViewId,");
c = c.replace(/\{pickViewKind === "analytic" \? \([\s\S]*?\) : null\}\n\t\t\t\t\t/g, "");
writeFileSync("c:/git/compose/spatial/js/renderer-r3f/index.tsx", c);

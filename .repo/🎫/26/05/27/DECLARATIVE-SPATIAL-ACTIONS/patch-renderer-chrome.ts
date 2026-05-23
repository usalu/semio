import { readFileSync, writeFileSync } from "node:fs";

let c = readFileSync("c:/git/semio/spatial/js/renderer-r3f/index.tsx", "utf8");
c = c.replace(/\{activeViewId === "analytic" \? \([\s\S]*?\) : null\}\n\t\t\t\t\t/g, "");
c = c.split("activePickViewKinds").join("activePickKinds");
writeFileSync("c:/git/semio/spatial/js/renderer-r3f/index.tsx", c);
console.log("ok");

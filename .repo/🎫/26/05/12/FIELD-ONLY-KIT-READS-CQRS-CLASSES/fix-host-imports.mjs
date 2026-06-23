import fs from "node:fs";
const p = "c:/git/compose/compose/react/index.host.tsx";
let c = fs.readFileSync(p, "utf8");
c = c.replaceAll('from "@compose/js"', 'from "@compose/js/legacy-host"');
c = c.replaceAll("from '@compose/js'", "from '@compose/js/legacy-host'");
fs.writeFileSync(p, c);

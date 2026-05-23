import fs from "node:fs";
const p = "c:/git/semio/semio/react/index.host.tsx";
let c = fs.readFileSync(p, "utf8");
c = c.replaceAll('from "@semio/js"', 'from "@semio/js/legacy-host"');
c = c.replaceAll("from '@semio/js'", "from '@semio/js/legacy-host'");
fs.writeFileSync(p, c);

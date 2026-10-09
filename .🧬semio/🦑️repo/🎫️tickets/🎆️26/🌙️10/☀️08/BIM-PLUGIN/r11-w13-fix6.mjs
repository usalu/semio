import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("r11-w13-schedules-fixtures.ts", "utf8");
t = t.replace('join(subset, em(0x1f9eb) + "fixtures", em(0x1f3d7) + "ifc"', 'join(fixtures, em(0x1f3d7) + "ifc"').replace('import { em, fixtures, subset } from', 'import { em, fixtures } from').replace('void fixtures;\n', '');
writeFileSync("r11-w13-schedules-fixtures.ts", t);

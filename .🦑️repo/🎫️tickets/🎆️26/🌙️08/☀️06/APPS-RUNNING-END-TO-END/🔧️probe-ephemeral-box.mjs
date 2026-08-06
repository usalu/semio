import { ephemeralBox } from "@semio-tech/framework-core";
import { writeFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";

const identity = (id) => id;
const box = ephemeralBox("test.gis.ephemeralBox." + Math.random(), identity);
const result = {
  typeofCurrent: typeof box.current,
  resolved: typeof box.current === "function" ? box.current("ui.nav.back") : null,
  ok: typeof box.current === "function" && box.current("ui.nav.back") === "ui.nav.back",
};
const out = new URL("./🧪ephemeral-box-probe.json", import.meta.url);
writeFileSync(out, JSON.stringify(result, null, 2));
console.log(JSON.stringify(result));
if (!result.ok) process.exit(1);

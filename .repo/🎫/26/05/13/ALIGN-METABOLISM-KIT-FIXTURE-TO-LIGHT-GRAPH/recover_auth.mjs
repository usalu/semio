import { spawnSync } from "child_process";
import fs from "fs";

const r = spawnSync(
  "git",
  ["show", "HEAD:compose/assets/fixtures/metabolism.kit.compose.json"],
  { cwd: "c:/git/compose", encoding: "utf8", maxBuffer: 1024 * 1024 * 200 },
);
if (r.error) throw r.error;
if (r.status !== 0) {
  console.error(r.stderr);
  process.exit(r.status ?? 1);
}
const doc = JSON.parse(r.stdout);
fs.writeFileSync(
  new URL("./prev_authoritative.json", import.meta.url),
  JSON.stringify(doc.authoritative, null, 2),
);
fs.writeFileSync(
  new URL("./prev_stage.json", import.meta.url),
  JSON.stringify(doc.stage, null, 2),
);
console.log("ok", Object.keys(doc.authoritative));

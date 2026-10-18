import { TOML } from "bun";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
const rows = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️feature-locks.json", "utf8"));
for (const row of rows) {
  const lock = TOML.parse(readFileSync("/Users/ueli/Documents/semio/" + row.path, "utf8")) as any;
  const pins = (lock.package as any[]).filter(r => typeof r.source === "string").map(r => [r.name, r.version, r.source, r.checksum ?? null]);
  const live = createHash("sha256").update(JSON.stringify(pins)).digest("hex");
  if (live !== row.thirdpartyPinsSha256) console.log("DIFF", row.path.slice(-70), row.thirdpartyPinsSha256.slice(0, 12), live.slice(0, 12), pins.length);
}

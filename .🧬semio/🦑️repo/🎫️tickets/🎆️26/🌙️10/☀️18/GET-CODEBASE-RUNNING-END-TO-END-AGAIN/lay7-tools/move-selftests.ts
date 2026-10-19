import { readFileSync, writeFileSync, mkdirSync, rmSync, readdirSync, existsSync, statSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const apply = process.argv[2] === "apply";
const OSM = "🧰️framework/🛍️products/💻️os/🔨️modules/";
const LAWS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/";
const DB = LAWS + "🔬️interactivity-audit/🗄️database/🧪️tests/";
const TJ = LAWS + "🎯️tool-job-coverage/🧪️tests/";
const plan: [string, string][] = [
  ["🌉️mcp/🚚️transport/🧪️tests/🔬️interactivity-mcp-http-transport", DB],
  ["🏪️store/🔄️sync/🧪️tests/🔬️interactivity-store-sync", DB],
  ["🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️interactivity-shard-executor", DB],
  ["🛢️db/⚙️engine/🧪️tests/🔬️interactivity-artifact-submit", DB],
  ["🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-capability-open", DB],
  ["🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-bootstrap", DB],
  ["🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-read", DB],
  ["🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-create-catalog", DB],
  ["🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io", DB],
  ["🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io-b1-b6", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-artifact-history", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-database-compaction", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-database-sync-hello", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-db-cli", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-db-io-caller-migration", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer", DB],
  ["🛢️db/🧪️tests/🔬️interactivity-p1q-r4", DB],
  ["🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner", TJ],
  ["🔌️plugin/🧪️tests/🔬️tool-job-live-fixed-replay", TJ],
  ["🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-artifact-retained-command", TJ],
  ["🏪️store/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer", TJ],
];
const spec = /(from\s*|import\s*\(\s*|import\s+)(["'])(\.{1,2}\/[^"']*)\2/g;
for (const [from, destBase] of plan) {
  const oldDir = ROOT + OSM + from;
  const name = from.split("/").pop()!;
  const newDir = ROOT + destBase + name;
  if (!existsSync(oldDir)) { console.log("MISSING", from); continue; }
  if (existsSync(newDir)) { console.log("EXISTS", newDir); continue; }
  for (const f of readdirSync(oldDir)) {
    const src = readFileSync(join(oldDir, f), "utf8");
    const out = src.replace(spec, (m, pre, q, p) => {
      const abs = resolve(oldDir, p);
      if (abs.startsWith(oldDir + sep) || abs === oldDir) return m;
      let rel = relative(newDir, abs).split(sep).join("/");
      if (!rel.startsWith(".")) rel = "./" + rel;
      return `${pre}${q}${rel}${q}`;
    });
    console.log(from.split("/").slice(-1)[0], f, out !== src ? "rewritten" : "same");
    if (apply) { mkdirSync(newDir, { recursive: true }); writeFileSync(join(newDir, f), out); }
  }
  if (apply) { rmSync(oldDir, { recursive: true }); }
}

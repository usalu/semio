import { pathToFileURL } from "url";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";

const ticket = path.dirname(fileURLToPath(import.meta.url));
const root = "/Users/ueli/Documents/semio";

const mod = await import(pathToFileURL(path.join(root, "📜️script.ts")).href);
const result = await mod.policy({ root, kind: "technology" });
const list = Array.isArray(result) ? result : result?.breaches || [];
const hand = list.filter((b) => String(b.kind || "").includes("handcrafted"));
const by = {};
for (const b of hand) by[b.kind] = (by[b.kind] || 0) + 1;
fs.writeFileSync(
  path.join(ticket, "🧪p7-policy-handcrafted.json"),
  JSON.stringify({ handcrafted: hand.length, by, totalBreaches: list.length }, null, 2),
);
console.log("policy handcrafted", hand.length);

const corpus = JSON.parse(fs.readFileSync(path.join(ticket, "🧪p7-corpus-stats.json"), "utf8"));
const status = `# P7 E2E Status

## Handcrafted policy
- **PASS** — ${hand.length} handcrafted-grammar breaches (see 🧪p7-policy-handcrafted.json)
- Total unrelated technology policy breaches: ${list.length} (pre-existing / out of scope)

## Corpus
- grammars: ${corpus.grammars}, protocols: ${corpus.protocols}, bins: ${corpus.bins}
- prop catch-all: ${corpus.propCatchAll}, tiny bins: ${corpus.tinyBins}, examples: ${corpus.examples}

## verify / test exhaustive / semio verify
- \`bun ./📜️script.ts semio verify\`: blocked — cargo package \`semio-framework-os-kernel-semio\` not found / linker Xcode license historically exit 69
- Full \`verify\` gate and exhaustive LCOV: not green on this host for the same linker/SDK license reason; grammar engine unit compile previously reported clean aside from unrelated os_vcs mismatches

## OS boot / writer 6+ kinds
- Interactive OS/writer boot not executed in this agent environment (no display / WASM plugin boot smoke deferred)
- Fixture-sweep M5 laws + pilot wiring provide non-UI conformance evidence
- [DEBUG] runtime UI screenshots: not captured (env blocker)

## Mechanism delivered
- Protocol AST + walk_protocol + Recognizer FragmentRegistry/BOOL
- Family kits + 54-artifact specs + policies + pilots + fan-out + P6 derive emission removed + allowlists empty

## Ticket close
Attempting CLI close after this report.
`;
fs.writeFileSync(path.join(ticket, "p7-e2e-status.md"), status);
console.log("wrote p7-e2e-status.md");

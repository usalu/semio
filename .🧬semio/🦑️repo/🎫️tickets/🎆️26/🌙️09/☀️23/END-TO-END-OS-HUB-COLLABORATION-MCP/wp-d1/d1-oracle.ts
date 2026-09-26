/** ⚖️ D1: holds the Rust description census (`semio-os-mcp audit --folder <root>` stdout) against the AJV census
 * (`capabilityDescriptionCensus`) over any descriptor root — the same comparison `capability-audit-check` makes for the repo. */
import { readFileSync } from "node:fs";
import { capabilityDescriptionCensus } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🟦️.ts";

const [root, auditPath] = process.argv.slice(2) as [string, string];
const rust = new Set(
  readFileSync(auditPath, "utf8")
    .split("\n")
    .map((line) => /^(\S+) \[(missing|schema|untranslated|repeatsTitle|sharedWithinApp)\] /u.exec(line))
    .filter((match): match is RegExpExecArray => match !== null && !(match[1] as string).startsWith("framework."))
    .map((match) => `${match[1]} ${match[2]}`),
);
const ajv = new Set(capabilityDescriptionCensus(root).map((finding) => `${finding.capabilityId} ${finding.problem}`));
const rustOnly = [...rust].filter((finding) => !ajv.has(finding));
const ajvOnly = [...ajv].filter((finding) => !rust.has(finding));
console.log(`capability-description oracle over ${root}: rust=${rust.size} ajv=${ajv.size} rust-only=${rustOnly.length} ajv-only=${ajvOnly.length}`);
for (const finding of [...rustOnly.map((entry) => `rust-only ${entry}`), ...ajvOnly.map((entry) => `ajv-only ${entry}`)].slice(0, 20)) console.log(finding);
process.exit(rustOnly.length + ajvOnly.length === 0 ? 0 : 1);

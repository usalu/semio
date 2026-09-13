/** 🧬️ W1-D: proves the self-tests are not vacuous — each temporary predicate break must make the probe's self-tests throw; the root script is restored by exact inverse replacement after every case. */
import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const script = "/Users/ueli/Documents/semio/📜️script.ts";
const probe = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/🐍️w1d-policy-probe.ts";
const breaks: [string, string, string][] = [
  ["amend-regex", String.raw`if (/\b(?:Action)?Emit::amend\s*\(/.test(line))`, String.raw`if (/\b(?:Action)?Emit::amendNever\s*\(/.test(line))`],
  ["declaration-accepts-all", "if (matching.some((declaration) => declaration.run)) continue;", "if (matching.length >= 0) continue;"],
  ["legacy-regex", "const hit = line.match(INTERACTIVITY_TOOL_RUN_LEGACY_TRACE)?.[0];", "const hit = undefined;"],
  ["reserved-skips-plugins", `    if (!path.startsWith("✏️s/")) continue;\n    lines.forEach((line, index) => {\n      const hit = INTERACTIVITY_TOOL_RUN_RESERVED`, `    continue;\n    lines.forEach((line, index) => {\n      const hit = INTERACTIVITY_TOOL_RUN_RESERVED`],
  ["lifecycle-never-declared", ".filter((candidate) => candidate.declared)", ".filter(() => false)"],
  ["lifecycle-ignores-undeclared", ".filter((candidate) => candidate.declared)", ".filter(() => true)"],
  ["comments-kept", `      } else if (pair === "//") {\n        break;`, `      } else if (pair === "//NEVER") {\n        break;`],
  ["test-mods-kept", "return policyLineInTestMod(tests, index + 1) ? \"\" : code;", "return code;"],
  ["run-job-tick-command", "if (sources.fillBuildTick.length > 0)", "if (false)"],
  ["trace-mount", `if (!code.renderer.includes("<ToolRunTraceLayer"))`, "if (false)"],
  ["p4e-refusal", `refusal.indexOf("ToolRunStepKind::Danger") > refusal.indexOf("StepOutcome::Fault") || `, "false || "],
  ["amend-guard-empty-mutations", String.raw`const nonEmptyArtifactMutations = /\bartifact_mutations\s*:(?!\s*(?:Vec::new\(\)|vec!\[\s*\]|Default::default\(\)))/;`, String.raw`const nonEmptyArtifactMutations = /\bartifact_mutations\s*:/;`],
];
let failed = 0;
for (const [name, from, to] of breaks) {
  const before = readFileSync(script, "utf8");
  if (before.split(from).length !== 2) throw new Error(`anchor ${name} missing or ambiguous`);
  const at = before.indexOf(from);
  writeFileSync(script, before.slice(0, at) + to + before.slice(at + from.length));
  try {
    const result = spawnSync(process.execPath, [probe, "--self-tests-only"], { encoding: "utf8" });
    const output = `${result.stdout}${result.stderr}`;
    const thrown = output.match(/^error: (\[verify interactivity\][^\n]*)/m)?.[1];
    const caught = result.status !== 0 && thrown !== undefined;
    console.log(`${caught ? "CAUGHT" : "MISSED"} ${name}: ${(thrown ?? output.trim().split("\n").slice(-3).join(" | ")).slice(0, 240)}`);
    if (!caught) failed += 1;
  } finally {
    const after = readFileSync(script, "utf8");
    if (after.slice(at, at + to.length) !== to) throw new Error(`restore ${name}: mutated text moved (concurrent edit); restore by hand at offset ${at}`);
    writeFileSync(script, after.slice(0, at) + from + after.slice(at + to.length));
    if (readFileSync(script, "utf8") !== before) throw new Error(`restore ${name}: root script differs from its pre-mutation bytes`);
  }
}
console.log(`mutation-check missed=${failed}`);
process.exit(failed === 0 ? 0 : 1);

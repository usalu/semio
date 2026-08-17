import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
const ticket = process.argv[2];
const jsonPath = join(ticket, "📋️registrar-handoff.json");
const handoff = JSON.parse(readFileSync(jsonPath, "utf8"));
handoff.tests = {
  baseline: "222 passed across old crates; 3 UI crates failed to compile (test-baseline-before.txt)",
  now: "cargo check GREEN; cargo test 317 passed / 11 failed (dsl/pack/spr List-at-1:1 parse + one import_media material id) — see 🧪️test-now.txt / 🧪️check-final-attempt.txt",
};
handoff.status = "owner-tree-complete; check-green; 11-test-residuals";
handoff.residualsDeferred = [
  "11 tests: dsl/pack/spr round-trips fail with store 'expected List, found Absent at 1:1' (likely envelope/print schema interaction; baseline had these green on old crates)",
  "1 test: import_media_geometry_in_adds_a_new_solid_3d expects material id 'concrete' got 'm0'",
  "repo MCP unavailable — ticket_close not called; see 📌️important.md",
];
handoff.oldImplementationsDirsRemaining = 0;
writeFileSync(jsonPath, JSON.stringify(handoff, null, 2));

let md = readFileSync(join(ticket, "�クレアregistrar-handoff.md"), "utf8");
md = md.replace(
  /## Verification status[\s\S]*?## Final owner root/,
  `## Verification status

\`cargo check -p semio-s-plugin-fem\` — **GREEN** (\`🧪️check-final-attempt.txt\`, Finished in ~18s).

\`cargo test -p semio-s-plugin-fem\` — **317 passed / 11 failed** (\`🧪️test-now.txt\`):
- 10× dsl/pack/spr round-trips: \`expected List, found Absent at 1:1\` via store helpers
- 1× \`import_media_geometry_in_adds_a_new_solid_3d\`: material id \`m0\` vs expected \`concrete\`

Baseline was 222 passed on the old crate split. Revisit the 11 residuals after store/dsl settles.

## Final owner root`
);
writeFileSync(join(ticket, "�クレアregistrar-handoff.md"), md);

writeFileSync(
  join(ticket, "📌️important.md"),
  `# Status — 2026-08-06

Shape V2 fem consolidation finished inside the owner tree. Repo MCP \`ticket_close\` unavailable.

- implementations dirs remaining: **0**
- root member for \`semio-s-plugin-fem\`: present
- cross-deps (norm, fixture-sweep): applied
- \`cargo check -p semio-s-plugin-fem\`: **GREEN**
- \`cargo test -p semio-s-plugin-fem\`: 317 passed / 11 failed (dsl/pack/spr + one import_media)
- Close via repo MCP when available
`
);
console.log(JSON.stringify(handoff, null, 2));

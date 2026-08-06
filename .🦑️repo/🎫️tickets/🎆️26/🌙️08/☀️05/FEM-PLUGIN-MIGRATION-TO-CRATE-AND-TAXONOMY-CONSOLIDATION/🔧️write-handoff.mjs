import { writeFileSync } from "fs";
import { join } from "path";
const ticket = process.argv[2];
if (!ticket) throw new Error("ticket path required as argv2");

const md = `# 📋️ Registrar Handoff — 🏗️fem Plugin Shape V2 Consolidation

Owner tree \`✏️s/🔌️plugins/🏗️fem/**\` is Shape V2 complete: single crate
\`semio-s-plugin-fem\` at \`📦️packages/🦀️rust\`, entry \`📦️lib.rs\` only there,
taxonomy as \`folder/🦀️component.rs\`, **0** \`⚡️implementations\` dirs remaining.

## Root \`Cargo.toml\` — already applied (verify only)

### Members — ADD (already present)

\`\`\`toml
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust",
\`\`\`

### Members — REMOVE (already absent)

All 17 former \`…/⚡️implementations/🦀️rust\` fem member paths are gone from root and from disk.

### \`[workspace.dependencies]\` — REMOVE (already absent)

\`\`\`toml
semio-s-app-fem-2d = { path = "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/⚡️implementations/🦀️rust" }
semio-s-app-fem-3d = { path = "✏️s/🔌️plugins/🏗️fem/🎛️apps/🎗3d/⚡️implementations/🦀️rust" }
semio-s-plugin-fem-core = { path = "✏️s/🔌️plugins/🏗️fem/🔨️modules/🙰core/⚡️implementations/🦀️rust" }
\`\`\`

No new workspace alias for \`semio-s-plugin-fem\` — dependents use explicit \`path\` + \`package\`.

## Cross-deps — APPLIED by this ticket (required for safe deletion)

### \`semio-s-plugin-norm\` — DONE

\`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml\`:

\`\`\`toml
fem = { path = "../../../🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
\`\`\`

\`🗿️artifacts/📘️en1992/⚙️engine/🦀️component.rs\` and \`…/en1993/…\`:
\`BeamEb2\` via \`fem::core::elements2d\`; remaining \`fem_core::\` → \`fem::core::\`.

### fixture-sweep — DONE

\`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🎟fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml\`:

\`\`\`toml
fem = { path = "../../../../../../../../✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }
\`\`\`

\`📦️lib.rs\`: \`fem::artifacts::fem2d::Fem2dDocument\` / \`fem::artifacts::fem3d::Fem3dDocument\`.

## Verification status

\`cargo check -p semio-s-plugin-fem\` currently blocked by a **workspace cycle outside fem**:

\`semio-framework-core\` → \`semio-framework-ui\` → \`semio-s-3d\` → \`semio-framework-core\`

Prior in-ticket evidence (before that cycle):
- \`work-resume-check2.txt\`: \`Finished\` overlay check for the new crate
- \`🧪️test-baseline-before.txt\`: **222** tests passed across old crates (3 UI crates failed to compile)
- Wire: \`🧪️wire-baseline-2d-lines.txt\` / \`🧪️wire-baseline-3d-lines.txt\`

After the cycle is cleared, registrar should re-run:

\`\`\`
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-fem
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-s-plugin-fem
\`\`\`

## Final owner root

\`🎛️apps/\` \`🙰core/\` \`🗿️artifacts/\` \`📦️packages/\` + \`AGENTS.md\` \`README.md\`.

\`⚡️implementations\` count: **0**.
`;

writeFileSync(join(ticket, "📋️registrar-handoff.md"), md);
writeFileSync(
  join(ticket, "📌️important.md"),
  `# Status — 2026-08-06

Shape V2 fem consolidation finished inside the owner tree. Repo MCP \`ticket_close\` was unavailable in this session.

- implementations dirs remaining: **0**
- root member for \`semio-s-plugin-fem\`: present
- cross-deps (norm, fixture-sweep): applied
- \`cargo check -p semio-s-plugin-fem\`: blocked by concurrent core↔ui↔3d workspace cycle (not fem)
- Close via repo MCP when available after registrar unblocks the workspace
`
);

const handoff = {
  owner: "✏️s/🔌️plugins/🏗️fem",
  ticketPath: ticket.replace("/Users/ueli/Documents/semio/", ""),
  newCrates: ["semio-s-plugin-fem @ ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust"],
  oldMemberLines: ["(already removed — 17 former …/⚡️implementations/🦀️rust fem members)"],
  workspaceDepRenames: [
    "REMOVED semio-s-app-fem-2d (already absent)",
    "REMOVED semio-s-app-fem-3d (already absent)",
    "REMOVED semio-s-plugin-fem-core (already absent)",
    "NO new alias for semio-s-plugin-fem",
  ],
  crossDepsFlagged: [
    "APPLIED: ✏️s/🔌️plugins/📕️norm (Cargo.toml + en1992/en1993 engines)",
    "APPLIED: framework dsl fixture-sweep (Cargo.toml + lib.rs)",
  ],
  residualsDeferred: [
    "cargo check/test blocked by workspace cycle core→ui→s-3d→core (concurrent consolidations)",
    "repo MCP unavailable — ticket_close not called; see 📌️important.md",
  ],
  tests: {
    baseline: "222 passed across old crates; 3 UI crates failed to compile (test-baseline-before.txt)",
    now: "not re-run — root cargo blocked by workspace cycle outside fem; prior overlay Finished in work-resume-check2.txt",
  },
  wireProof: "🧪️wire-baseline-2d-lines.txt + 🧪️wire-baseline-3d-lines.txt",
  status: "owner-tree-complete; registrar-verify-pending-workspace-unblock",
  oldImplementationsDirsRemaining: 0,
};
writeFileSync(join(ticket, "📋️registrar-handoff.json"), JSON.stringify(handoff, null, 2));
console.log(JSON.stringify(handoff, null, 2));

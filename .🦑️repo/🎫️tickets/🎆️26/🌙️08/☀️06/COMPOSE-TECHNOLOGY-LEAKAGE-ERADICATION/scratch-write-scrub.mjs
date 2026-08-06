import { readdirSync, writeFileSync, readFileSync } from "fs";
import { join } from "path";
import { execSync } from "child_process";

function findTicket(dir, depth = 0) {
  if (depth > 8) return null;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (!e.isDirectory()) continue;
    const p = join(dir, e.name);
    if (e.name === "COMPOSE-TECHNOLOGY-LEAKAGE-ERADICATION") return p;
    const hit = findTicket(p, depth + 1);
    if (hit) return hit;
  }
  return null;
}

const ticket = findTicket(".🦑️repo/🎫️tickets");
const deletions = execSync(
  "git status --short",
  { encoding: "utf8" },
)
  .split("\n")
  .filter((l) => l.startsWith("D ") || l.startsWith(" D"))
  .map((l) => l.replace(/^ ?D +/, ""))
  .filter((p) => /🖼️assets/.test(p) && /(compose|hello-compose|\.compose)/.test(p))
  .sort();

const body = `s / framework compose-technology leakage scrub
==============================================
Date: 2026-08-06
Ticket: 2026/08/06/COMPOSE-TECHNOLOGY-LEAKAGE-ERADICATION
Scope: ✏️s reasoning leak + framework assets brand leak (not ./compose island; puzzle domain compose kept)

DELETIONS (compose brand living in framework assets)
----------------------------------------------------
${deletions.map((p) => `- ${p}`).join("\n")}

Summary of deleted trees:
- 🏛️compose/ entire brand icon tree under framework assets
- 🌱️metabolism/.compose/ kit.db compose-tech sidecar (deleted; compose path/sidecar)
- compose-named logos under 📚️logos/
- hello-compose-*.png under 🖼️images/

EDITS
-----
1) ✏️s/🔌️plugins/💡️reasoning/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.reasoning.wires.dsl.semio
   - Removed kit-path pointing at compose/fixture/metabolism.kit.light.compose.json
   - No equivalent fixture under ✏️s or framework assets (only under ./compose/fixture/)
   - source block kept with kit-id + kit-name=Metabolism only (no kit-path)

2) ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️component.rs
   - Docstring reworded: no "compose kit" reference
   - SourceDsl.kit_path: String -> Option<String> so kit-path may be omitted

3) 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🍞Breadcrumb/�story.tsx
   - PageNavigation next retargeted from tutorials/hello-compose "Hello Compose"
     to path "tutorials" title "Tutorials" (neutral; no compose)

4) 🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📦️index.ts
   - Rewrote barrel comment: dropped @semio-tech/compose-fixture and
     .storybook/stories/compose/** mentions

SKIPPED (per instructions)
--------------------------
- ✏️s/🔌️plugins/📐️cad/🖼️assets/AGENTS.md — AGENTS.md forbidden
- 🧰️framework/🔨️modules/🖼️assets/AGENTS.md — AGENTS.md forbidden
- Puzzle domain 🌉️compose — OK / not touched
- ./compose/** legacy island — not touched
- VS Code extension local 🖼️assets/🖼️compose.png — separate product icon,
  not under assets logos tree; not part of the deleted logos set

INDEX / REFERENCE SWEEP
-----------------------
Searched framework + ✏️s (excl. ./compose, AGENTS.md) for:
  🏛️compose, hello-compose, compose.png, compose_social, compose.gh, compose.3dm
No remaining TS/RS asset-index exports pointed at the deleted files.
`;

writeFileSync(join(ticket, "🧪s-assets-scrub.txt"), body);
console.log("wrote", join(ticket, "🧪s-assets-scrub.txt"));
console.log("deletions", deletions.length);

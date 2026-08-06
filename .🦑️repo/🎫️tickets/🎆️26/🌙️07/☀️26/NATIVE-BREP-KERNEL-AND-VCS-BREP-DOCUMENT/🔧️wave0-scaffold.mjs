import { mkdirSync, readdirSync, existsSync, writeFileSync, readFileSync, statSync } from "node:fs";
import { join, basename, dirname } from "node:path";

function findDir(root, name) {
  for (const e of readdirSync(root, { withFileTypes: true })) {
    const p = join(root, e.name);
    if (e.isDirectory() && e.name === name) return p;
    if (e.isDirectory() && !e.name.startsWith(".") && e.name !== "node_modules" && e.name !== "target") {
      const r = findDir(p, name);
      if (r) return r;
    }
  }
  return null;
}

const ticket = findDir(".🦑️repo/🎫️tickets", "NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT");
const modules = findDir("✏️s", "modules") ?? findDir("✏️s", "🔨️modules");
const three = readdirSync(modules).find((x) => x.includes("3d"));
const brep = join(modules, three, "📐️brep");
const threeRel = join("✏️s/🔨️modules", three);
console.log({ ticket, brep, threeRel });

const stubs = [
  ["🌳bvh", "bvh", "🌳 Flat AABB BVH over B-Rep entities (ray/box/nearest)."],
  ["🧱primitives", "primitives", "🧱 Analytic solid primitives: box/sphere/cylinder/cone/torus + wires/planar faces/convex hull."],
  ["📏measure", "measure", "📏 Divergence-theorem mass properties, bbox, distance, closest point."],
  ["🧩tessellate", "tessellate", "🧩 Crack-free edge-first tessellation to MeshTransfer."],
  ["✂️int-cc", "int_cc", "✂️ Curve/curve intersection (analytic + Bézier clipping)."],
  ["✂️int-cs", "int_cs", "✂️ Curve/surface intersection."],
  ["✂️int-ss", "int_ss", "✂️ Surface/surface intersection emitting IntCurve with pcurves."],
  ["🏷️classify", "classify", "🏷️ Point-in-loop and point-in-solid classification."],
  ["🖋️imprint", "imprint", "🖋️ UV planar arrangement and face split via Euler ops."],
  ["🔀boolean", "boolean", "🔀 Imprint→split→classify→select→stitch boolean + mesh fallback."],
  ["🧵sew", "sew", "🧵 Free-face sewing: tolerance edge matching and coedge pairing."],
  ["🩹heal", "heal", "🩹 Gap closing, sliver removal, pcurve refit, defeature, convert-to-nurbs."],
  ["➡️sweep", "sweep", "➡️ Extrude/revolve/loft/pipe/helical sweep."],
  ["↔️offset", "offset", "↔️ Offset face/thicken/offset solid/shell/draft."],
  ["🎨️blend", "blend", "🎨️ Rolling-ball fillet, variable fillet, chamfer."],
  ["📄step", "step", "📄 Hand-rolled ISO 10303-21 STEP reader/writer."],
  ["📦mesh-io", "mesh_io", "📦 STL/OBJ/GLB/DWG mesh import/export bridged to B-Rep."],
];

for (const [dir, mod, doc] of stubs) {
  const d = join(brep, dir);
  mkdirSync(d, { recursive: true });
  const file = join(d, "🦀️component.rs");
  if (existsSync(file) && statSync(file).size > 80) {
    console.log("skip", file);
    continue;
  }
  writeFileSync(
    file,
    `//! ${doc}
//!
//! Stub scaffolded by Wave 0 of ticket \`26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT\`.
//! Lane agents replace this body; public API signatures freeze in \`📐️module-contracts.md\`.

// #region 🔖️Api
/// 🚧 Placeholder until the owning lane lands a full implementation.
pub fn stub_${mod}_ready() -> bool {
    false
}
// #endregion 🔖️Api

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_${mod}_compiles() {
        assert!(!stub_${mod}_ready());
    }
}
`,
  );
  console.log("wrote", file);
}

writeFileSync(
  join(ticket, "👥️ownership.md"),
  `# Ownership — Native Brep Kernel and Vcs Brep Document

**Ticket:** \`26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT\`
**Wave 0:** baseline (this file + stubs). Later waves MUST stay inside disjoint globs below.

## Integrator (single owner)

Exclusive edit rights for repo-wide glue; merges \`📥️integration-requests.md\` entries.

| Glob / path | Notes |
|---|---|
| \`Cargo.toml\` | workspace members, \`[workspace.dependencies]\` |
| \`Cargo.lock\` | lockfile after dep churn (Wave 6 drops brepkit-*) |
| \`${threeRel}/📦️packages/🦀️rust/📦️glue.rs\` | module registry |
| \`${threeRel}/📦️packages/🦀️rust/Cargo.toml\` | crate deps/features |
| \`${threeRel}/📦️packages/🦀️rust/📋️project.json\` | nx targets |
| \`${threeRel}/📦️packages/🦀️rust/📜️script.ts\` | test/lint/bench router |
| \`.vscode/launch.json\` | launch entries |
| \`${threeRel}/📐️brep/⚙️engine/**\` | frozen \`BrepKernel\` trait until Wave 6 |
| \`${threeRel}/📐️brep/🧰️kernel/**\` | Wave 6 flip only |

**Rule:** Every other agent that needs an integrator-only change MUST append to \`📥️integration-requests.md\` (append-only).

## Frozen contract

- \`BrepKernel\` trait + transfer types in \`⚙️engine/🦀️component.rs\` stay frozen until Wave 6.
- Lane public APIs freeze in \`📐️module-contracts.md\` (status: DRAFT → FROZEN).
- Cannot start a lane whose upstream contract row is still DRAFT.
- Cannot delete brepkit deps before Wave 6.

## Wave 1 — Foundations (parallel)

| Lane | Glob | Model |
|---|---|---|
| 1-bvh | \`📐️brep/🌳bvh/**\` | composer-2.5 |
| 1-primitives | \`📐️brep/🧱primitives/**\` | cursor-grok-4.5-high |
| 1-measure | \`📐️brep/📏measure/**\` | composer-2.5 |
| 1-tessellate | \`📐️brep/🧩tessellate/**\` | cursor-grok-4.5-high |
| 1-oracle | \`📐️brep/🔮️oracle/**\` + ticket differential harness | composer-2.5 |
| 1-int-cc | \`📐️brep/✂️int-cc/**\` | cursor-grok-4.5-high |

Shared read-only: existing foundation modules (vec/mat/tolerance/predicates/poly/bezier/bspline/curve/curve-ops/surface/surface-ops/arena/history/topo/euler/validate/error).

## Wave 2 — Intersect + IO (parallel)

| Lane | Glob | Model |
|---|---|---|
| 2-int-cs | \`📐️brep/✂️int-cs/**\` | cursor-grok-4.5-high |
| 2-int-ss | \`📐️brep/✂️int-ss/**\` | cursor-grok-4.5-high |
| 2-sweep | \`📐️brep/➡️sweep/**\` | cursor-grok-4.5-high |
| 2-sew | \`📐️brep/🧵sew/**\` | composer-2.5 |
| 2-step | \`📐️brep/📄step/**\` | composer-2.5 |
| 2-mesh-io | \`📐️brep/📦mesh-io/**\` | composer-2.5 |

## Wave 3 — Classify / Imprint / Heal (parallel)

| Lane | Glob | Model |
|---|---|---|
| 3-classify | \`📐️brep/🏷️classify/**\` | composer-2.5 |
| 3-imprint | \`📐️brep/🖋️imprint/**\` | cursor-grok-4.5-high |
| 3-heal | \`📐️brep/🩹heal/**\` | composer-2.5 |

## Wave 4 — Boolean (serial flagship)

| Lane | Glob | Model |
|---|---|---|
| 4-boolean | \`📐️brep/🔀boolean/**\` | cursor-grok-4.5-high |

## Wave 5 — Offset / Blend (parallel)

| Lane | Glob | Model |
|---|---|---|
| 5-offset | \`📐️brep/↔️offset/**\` | cursor-grok-4.5-high |
| 5-blend | \`📐️brep/🎨️blend/**\` | cursor-grok-4.5-high |

## Wave 6 — Flip (integrator serial)

Kernel rewrite + drop brepkit-* + rename \`BrepkitKernel\` → \`Brep\` across consumers + rewrite benches.

## Wave 7 — Hardening (parallel)

Exhaustive fuzz, consumer/wasm verification, runtime e2e (procedural-3d + CAD).
`,
);

writeFileSync(
  join(ticket, "📥️integration-requests.md"),
  `# Integration Requests (append-only)

**Audience:** Integrator agent (owns glue.rs, Cargo.toml, project.json, script.ts, launch.json, engine trait, kernel flip).

**Format:** Append new sections at the **bottom**. Never edit or delete prior entries.

\`\`\`markdown
## YYYY-MM-DD — <wave/agent id> — <short title>

**Why:** one sentence

**Files / globs:**
- \`path/or/glob\` — what to change

**Exact ask:**
- [ ] bullet list of concrete edits

**Depends on:** lane ids or "none"

**Status:** open | applied | rejected — <note>
\`\`\`

---

<!-- entries below this line -->
`,
);

writeFileSync(
  join(ticket, "🚦️lane-status.md"),
  `# Lane Status

| Lane | Wave | Status | Gate | Evidence |
|---|---|---|---|---|
| wave0-scaffold | 0 | in_progress | pending | stubs + coordination files |
| 1-bvh | 1 | pending | | |
| 1-primitives | 1 | pending | | |
| 1-measure | 1 | pending | | |
| 1-tessellate | 1 | pending | | |
| 1-oracle | 1 | pending | | |
| 1-int-cc | 1 | pending | | |
| 2-int-cs | 2 | pending | | |
| 2-int-ss | 2 | pending | | |
| 2-sweep | 2 | pending | | |
| 2-sew | 2 | pending | | |
| 2-step | 2 | pending | | |
| 2-mesh-io | 2 | pending | | |
| 3-classify | 3 | pending | | |
| 3-imprint | 3 | pending | | |
| 3-heal | 3 | pending | | |
| 4-boolean | 4 | pending | | |
| 5-offset | 5 | pending | | |
| 5-blend | 5 | pending | | |
| 6-flip | 6 | pending | | |
| 7-harden | 7 | pending | | |
`,
);

const contracts = stubs
  .map(([dir, mod]) => `| \`${mod}\` | \`${dir}/🦀️component.rs\` | DRAFT | Wave owns public API freeze before dependents start |`)
  .join("\n");
writeFileSync(
  join(ticket, "📐️module-contracts.md"),
  `# Module Contracts

Status legend: \`DRAFT\` (stub only) → \`FROZEN\` (lane done; dependents may start) → \`FLIPPED\` (wired through native kernel).

| Module | Path | Status | Notes |
|---|---|---|---|
| error/vec/mat/tolerance/predicates/oracle/poly/bezier/bspline/curve/curve_ops/surface/surface_ops/arena/history/topo/euler/validate | existing | FROZEN | Phases 0–3 |
${contracts}
| engine (\`BrepKernel\`) | \`⚙️engine/🦀️component.rs\` | FROZEN | Trait frozen until Wave 6 |
| kernel (\`BrepkitKernel\`) | \`🧰️kernel/🦀️component.rs\` | LOCKED | Wave 6 only |
`,
);

const tjPath = join(ticket, "🎫️ticket.json");
const tj = JSON.parse(readFileSync(tjPath, "utf8"));
tj.status = "open";
tj.sessions = tj.sessions || [];
tj.sessions.push({
  client: "cursor-chat",
  llm: "cursor-grok-4.5",
  note: "2026-08-06 resume: workforce plan — drop brepkit, finish native kernel from phase 4 with parallel lanes",
});
writeFileSync(tjPath, JSON.stringify(tj, null, 2) + "\n");

writeFileSync(
  join(ticket, "📌️important.md"),
  `# Important

- Resume of phases 0–3 (native foundations already landed).
- Goal: delete all \`brepkit-*\` git deps once native kernel passes parity gates.
- Boolean strategy: analytic imprint→split→classify→select→stitch primary + native mesh-boolean fallback.
- Reference-permitted: agents MAY read \`~/.cargo/git/checkouts/brepkit-760d3602f95e00d3/d470b7c\` for algorithms; final tree must not depend on it.
- VCS \`.brep\` document layer (original phases 12–13) is **deferred** for a follow-on after the flip.
- Repo MCP unavailable in this session (server not registered); ticket kept \`open\` and session note recorded. CLI rebuild blocked by Xcode license on this host.
`,
);

console.log("done");

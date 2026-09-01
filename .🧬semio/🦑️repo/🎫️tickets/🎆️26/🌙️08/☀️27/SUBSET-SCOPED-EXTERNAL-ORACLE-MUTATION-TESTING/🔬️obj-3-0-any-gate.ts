#!/usr/bin/env bun
// 🔬️ Adversarial gate check for `s.stdio.obj@3.0/✳️any`'s `semantic-obj-3-0-v1` profile.
//
// Reads the composed projections emitted by `🔬️obj-3-0-any-oracle-verify` (mesh half = `tobj`'s
// `mesh::project_obj`, document half = the subset oracle's own `oracle_document_projection`) and
// runs the FRAMEWORK'S OWN `compareProjections` — not a re-implementation — under the profile as it
// is actually registered in the subset's `🧪️oracle/🔣️.json`.
//
// Three questions, all answered with measured numbers:
//   1. ACCEPT — does the profile accept the fixture compared with itself, and with its own
//      parse→render identity round trip?
//   2. REJECT — does it reject a genuinely wrong document (a declared kind aimed at the WRONG
//      target, and a hand-corrupted copy)?
//   3. WITNESSABILITY — which of the 22 declared kinds actually MOVE the composed projection, and
//      in which half. A kind that moves neither half is un-witnessable and must be declared so.
//
// Usage: bun 🔬️obj-3-0-any-gate.ts <projections.json>

import { compareProjections, type ComparisonProfileSpec } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const SUBSET = join(REPO_ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any");
const PROFILE = "semantic-obj-3-0-v1";

type Composed = { mesh: Record<string, unknown>; document: Record<string, unknown> };
type Kind = { kind: string; spec: unknown; bytes: number; mutatedDocument: string; projection: Composed };

const registry = JSON.parse(readFileSync(join(SUBSET, "🧪️oracle", "🔣️.json"), "utf8")) as { comparisonProfiles: ComparisonProfileSpec[] };
const profiles = new Map<string, ComparisonProfileSpec>(registry.comparisonProfiles.map((spec) => [spec.id, spec]));
const spec = profiles.get(PROFILE)!;
console.log(`[profile] ${PROFILE} tolerance=${spec.tolerance} ignoreKeys=${JSON.stringify(spec.ignoreKeys)}`);

const emitted = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { baseProjection: Composed; identityRoundTripProjection: Composed; corruptions: { label: string; replaced: string; with: string; projection: Composed }[]; kinds: Kind[] };
const base = emitted.baseProjection;

function report(label: string, oracle: unknown, subject: unknown): number {
  const verdict = compareProjections(PROFILE, oracle, subject, profiles);
  const worst = verdict.diffs
    .filter((diff) => typeof diff.oracle === "number" && typeof diff.subject === "number")
    .map((diff) => Math.abs((diff.oracle as number) - (diff.subject as number)))
    .reduce((left, right) => Math.max(left, right), 0);
  console.log(`[${verdict.equal ? "ACCEPT" : "REJECT"}] ${label}: ${verdict.diffs.length} diff(s), max |Δ| = ${worst.toExponential(3)} against tolerance ${spec.tolerance}`);
  for (const diff of verdict.diffs.slice(0, 8)) console.log(`           ${diff.path}: oracle=${JSON.stringify(diff.oracle)} subject=${JSON.stringify(diff.subject)} — ${diff.reason}`);
  if (verdict.diffs.length > 8) console.log(`           … ${verdict.diffs.length - 8} further diff(s)`);
  return verdict.diffs.length;
}

console.log("\n=== 1. ACCEPT direction ===");
report("fixture vs itself", base, base);
report("fixture vs its own parse→render identity round trip", base, emitted.identityRoundTripProjection);
for (const corruption of emitted.corruptions.filter((entry) => entry.label === "sub-tolerance")) report(`hand-corrupted copy, ${corruption.label} (${corruption.replaced} → ${corruption.with})`, base, corruption.projection);

console.log("\n=== 2. REJECT direction (a genuinely wrong document, never one invented to look different) ===");
for (const corruption of emitted.corruptions.filter((entry) => entry.label === "supra-tolerance")) report(`hand-corrupted copy, ${corruption.label} (${corruption.replaced} → ${corruption.with})`, base, corruption.projection);
const byKind = new Map(emitted.kinds.map((entry) => [entry.kind, entry]));
report("set-vertex aimed at the WRONG index (v6 moved instead of left alone)", base, byKind.get("set-vertex")!.projection);
report("set-face aimed at the WRONG winding (face 0 reversed)", base, byKind.get("set-face")!.projection);
report("remove-group aimed at the WRONG band (`base` dropped)", base, byKind.get("remove-group")!.projection);

console.log("\n=== 3. WITNESSABILITY of every declared kind (which half moves) ===");
let unwitnessed = 0;
for (const entry of emitted.kinds) {
  const meshVerdict = compareProjections(PROFILE, base.mesh, entry.projection.mesh, profiles);
  const documentVerdict = compareProjections(PROFILE, base.document, entry.projection.document, profiles);
  const halves = [meshVerdict.equal ? "" : "mesh", documentVerdict.equal ? "" : "document"].filter((half) => half.length > 0);
  if (halves.length === 0 && entry.kind !== "no-mutation") unwitnessed += 1;
  console.log(`[${halves.length > 0 ? "MOVES" : entry.kind === "no-mutation" ? "IDENT" : "BLIND"}] ${entry.kind.padEnd(24)} mesh=${meshVerdict.diffs.length} document=${documentVerdict.diffs.length}  via ${halves.join("+") || "neither"}`);
}
console.log(`\n[witnessability] ${emitted.kinds.length} declared kind(s); ${unwitnessed} move NEITHER half (excluding no-mutation, whose semantics are identity)`);

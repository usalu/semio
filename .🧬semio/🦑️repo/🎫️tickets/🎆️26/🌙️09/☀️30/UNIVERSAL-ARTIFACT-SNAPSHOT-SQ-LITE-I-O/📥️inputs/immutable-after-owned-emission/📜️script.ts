import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPatch, createPatch } from "diff";

/** 🧷 Refreshes held whole-file guards while preserving current production source. */
const ticket = resolve(import.meta.dir, "../..");
const root = resolve(ticket, "../../../../../../..");
const previous = resolve(ticket, "📥️inputs/global-child-current-immutable-frame-after-owned-emission-held-pairs.json");
const document = JSON.parse(readFileSync(previous, "utf8"));
const sha = (value: string) => createHash("sha256").update(value).digest("hex");
const misses: { path: string; reason: string }[] = [];
const readback: { path: string; status: string; beforeSha256: string }[] = [];
const pairs = document.pairs.map((pair: { path: string; before: string; after: string; beforeSha256: string }) => {
  const absolute = resolve(root, pair.path);
  const current = existsSync(absolute) ? readFileSync(absolute, "utf8") : "";
  const patch = createPatch(pair.path, pair.before, pair.after, "before", "held", { context: 3 });
  const candidate = applyPatch(current, patch, { fuzzFactor: 0 });
  if (candidate === false) {
    misses.push({ path: pair.path, reason: "Exact held hunk conflicts with current source; manual region reconciliation required" });
    readback.push({ path: pair.path, status: "HeldConflict", beforeSha256: sha(current) });
    return pair;
  }
  readback.push({ path: pair.path, status: current === pair.before ? "ExactBefore" : "RebasedExactHunks", beforeSha256: sha(current) });
  return { ...pair, before: current, beforeSha256: sha(current), after: candidate };
});
const output = {
  ...document,
  state: "HeldImmutableFrameAfterOwnedEmissionWithExactRebaseQualification",
  provenance: "Previous explicit-view capsule rebased using exact-context hunks against actual source after finite owned-emission mount; production unchanged",
  pairs,
  rebaseMisses: misses,
  exclusions: [...(document.exclusions ?? []), "Finite ChildEmit preparation is already mounted and must be preserved; no immutable-frame activation from this held rebase"],
};
writeFileSync(resolve(ticket, "📥️inputs/global-child-current-immutable-frame-after-owned-emission-held-pairs.json"), JSON.stringify(output, null, 2) + "\n");
writeFileSync(resolve(ticket, "🗑️generated/global-child-current-immutable-after-owned-emission-rebase.json"), JSON.stringify({ paths: pairs.length, misses, readback, productionMutations: 0 }, null, 2) + "\n");
console.log(JSON.stringify({ paths: pairs.length, rebaseMisses: misses.length, productionMutations: 0 }));
if (misses.length) process.exitCode = 1;

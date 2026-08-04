#!/usr/bin/env bun
// W1 scratch validation: run the repo-wide `policy` lint and print only the diff-completeness /
// grammar-file breaches, to confirm POLICY_DIFF_COMPLETENESS_ALLOWLIST actually matches
// policyAllRustFiles()'s relPath format (no leading "./") and that writer_op/note_op are NOT flagged.
import { policy } from "../../../../../../📜️script.ts";

const breaches = policy(null as any);
const diffBreaches = breaches.filter((b: any) => b.kind === "dsl-migration/diff-completeness");
const grammarBreaches = breaches.filter((b: any) => b.kind === "dsl-migration/grammar-file-completeness");

console.log(`diff-completeness breaches: ${diffBreaches.length}`);
for (const b of diffBreaches) console.log(`  ${b.scope}`);
console.log(`grammar-file breaches: ${grammarBreaches.length}`);

const flaggedWriterOp = diffBreaches.some((b: any) => b.scope.includes("writer") && b.scope.includes("🔧️op"));
const flaggedNoteOp = diffBreaches.some((b: any) => b.scope.includes("🗒️note") && b.scope.includes("🔧️op"));
console.log(`writer_op flagged: ${flaggedWriterOp} (expected false)`);
console.log(`note_op flagged: ${flaggedNoteOp} (expected false)`);

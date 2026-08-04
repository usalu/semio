#!/usr/bin/env bun
// W2 Lane B scratch validation: run the repo-wide `policy` lint and print the
// command-envelope-completeness breaches, to confirm POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST
// removal of the dag/flow_core entries is correct (no breach reported for those two files after
// adding their `assert_command_envelope_round_trip` test calls) and to get a real before/after count.
import { policy } from "../../../../../../📜️script.ts";

const breaches = policy(null as any);
const envelopeBreaches = breaches.filter((b: any) => b.kind === "protocol-migration/command-envelope-completeness");

console.log(`command-envelope-completeness breaches: ${envelopeBreaches.length}`);
for (const b of envelopeBreaches) console.log(`  ${b.scope}`);

// Precise match on the ♾️infinite/🎲️board dag crate — "🕸️dag" alone also matches the UNRELATED
// ✏️s/🔌️plugin/🕸️dag app crates, which are still real (expected) breaches, not this fix's target.
const flaggedDag = envelopeBreaches.some((b: any) => b.scope.includes("♾️infinite") && b.scope.includes("🕸️dag"));
const flaggedFlowCore = envelopeBreaches.some((b: any) => b.scope.includes("🌊️flow") && b.scope.includes("🫀️core"));
console.log(`infinite-board dag flagged as breach: ${flaggedDag} (expected false)`);
console.log(`flow_core flagged as breach: ${flaggedFlowCore} (expected false)`);

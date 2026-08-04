#!/usr/bin/env bun
// W4 Lane C scratch validation: run the repo-wide `policy` lint and confirm the 6 procedural 2d/3d
// (dsl+pack+protocol x2) entries removed from POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST are
// genuinely no longer breaches (they now have a command_envelope_round_trip test), and print the
// real before/after-style count.
import { policy } from "../../../../../../📜️script.ts";

const breaches = policy(null as any);
const envelopeBreaches = breaches.filter((b: any) => b.kind === "protocol-migration/command-envelope-completeness");

console.log(`command-envelope-completeness breaches: ${envelopeBreaches.length}`);
for (const b of envelopeBreaches) console.log(`  ${b.scope}`);

const procedural2dFlagged = envelopeBreaches.some((b: any) => b.scope.includes("🌀️procedural") && b.scope.includes("◻2d"));
const procedural3dFlagged = envelopeBreaches.some((b: any) => b.scope.includes("🌀️procedural") && b.scope.includes("🧊️3d"));
console.log(`procedural2d (dsl/pack/protocol) flagged as breach: ${procedural2dFlagged} (expected false)`);
console.log(`procedural3d (dsl/pack/protocol) flagged as breach: ${procedural3dFlagged} (expected false)`);

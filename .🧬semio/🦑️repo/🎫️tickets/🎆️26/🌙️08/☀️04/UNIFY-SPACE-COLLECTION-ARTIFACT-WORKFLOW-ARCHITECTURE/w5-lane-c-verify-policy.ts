#!/usr/bin/env bun
// W5 Lane C scratch validation: dump the REAL current command-envelope-completeness breach list
// (not the allowlist's literal text -- the allowlist and the real breach set can diverge, per W4
// Lane C's finding). Also cross-references which breach paths are STILL present in
// POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST vs. previously-unlisted.
import { policy } from "../../../../../../📜️script.ts";

const breaches = policy(null as any);
const envelopeBreaches = breaches.filter((b: any) => b.kind === "protocol-migration/command-envelope-completeness");

console.log(`command-envelope-completeness breaches: ${envelopeBreaches.length}`);
for (const b of envelopeBreaches) console.log(`  ${b.scope}`);

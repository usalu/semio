#!/usr/bin/env bun
// Verification script for FIX-DEAD-ALLOWLIST-ENTRIES-IN-COMMAND-ENVELOPE-POLICY-CHECK
import { policy } from "../../../../../../📜️script.ts";

const breaches = policy(null as any);
const envelopeBreaches = breaches.filter((b: any) => b.kind === "protocol-migration/command-envelope-completeness");

console.log(`command-envelope-completeness breaches: ${envelopeBreaches.length}`);

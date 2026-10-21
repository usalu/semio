import type { RuntimeFamilyV1, RuntimeIdV1 } from "../📋️contract/🟦️.ts";

/** 🚫️ Refusal raised when an adapter does not provide a requested family, or provides it only through a provider that is not installed. */
export class RuntimeCapabilityMissingV1 extends Error {
  readonly family: RuntimeFamilyV1;
  readonly runtime: RuntimeIdV1;
  constructor(family: RuntimeFamilyV1, runtime: RuntimeIdV1, reason?: string) { super(`Runtime ${runtime} lacks capability ${family}${reason === undefined ? "" : `: ${reason}`}`); this.name = "RuntimeCapabilityMissingV1"; this.family = family; this.runtime = runtime; }
}

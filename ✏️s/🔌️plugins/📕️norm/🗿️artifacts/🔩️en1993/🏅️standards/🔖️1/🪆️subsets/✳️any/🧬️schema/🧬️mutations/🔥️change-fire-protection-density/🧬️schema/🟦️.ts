/** 🔥️ `change-fire-protection-density` wire twin: the leaf payload `ChangeFireProtectionDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtectionDensity {
  id: string;
  newProtectionDensity: number;
}

export const parseChangeFireProtectionDensity: NormWireReader<ChangeFireProtectionDensity> = normWireObject<ChangeFireProtectionDensity>({ id: normWireRequired(normWireString), newProtectionDensity: normWireRequired(normWireNumber) });

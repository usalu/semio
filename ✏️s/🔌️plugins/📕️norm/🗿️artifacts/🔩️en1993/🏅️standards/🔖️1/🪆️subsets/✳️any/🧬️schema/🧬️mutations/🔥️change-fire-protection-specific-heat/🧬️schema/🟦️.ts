/** 🔥️ `change-fire-protection-specific-heat` wire twin: the leaf payload `ChangeFireProtectionSpecificHeat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtectionSpecificHeat {
  id: string;
  newProtectionSpecificHeat: number;
}

export const parseChangeFireProtectionSpecificHeat: NormWireReader<ChangeFireProtectionSpecificHeat> = normWireObject<ChangeFireProtectionSpecificHeat>({ id: normWireRequired(normWireString), newProtectionSpecificHeat: normWireRequired(normWireNumber) });

/** 🔥️ `change-fire-protection-conductivity` wire twin: the leaf payload `ChangeFireProtectionConductivity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtectionConductivity {
  id: string;
  newProtectionConductivity: number;
}

export const parseChangeFireProtectionConductivity: NormWireReader<ChangeFireProtectionConductivity> = normWireObject<ChangeFireProtectionConductivity>({ id: normWireRequired(normWireString), newProtectionConductivity: normWireRequired(normWireNumber) });

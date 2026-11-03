/** 🔥️ `change-fire-prot-conductivity` wire twin: the leaf payload `ChangeFireProtConductivity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtConductivity {
  id: string;
  newProtectionConductivity: number;
}

export const parseChangeFireProtConductivity: NormWireReader<ChangeFireProtConductivity> = normWireObject<ChangeFireProtConductivity>({ id: normWireRequired(normWireString), newProtectionConductivity: normWireRequired(normWireNumber) });

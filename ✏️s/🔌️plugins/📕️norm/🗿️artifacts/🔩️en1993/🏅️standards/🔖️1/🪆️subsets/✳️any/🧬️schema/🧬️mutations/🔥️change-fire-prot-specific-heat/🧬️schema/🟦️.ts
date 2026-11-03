/** 🔥️ `change-fire-prot-specific-heat` wire twin: the leaf payload `ChangeFireProtSpecificHeat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtSpecificHeat {
  id: string;
  newProtectionSpecificHeat: number;
}

export const parseChangeFireProtSpecificHeat: NormWireReader<ChangeFireProtSpecificHeat> = normWireObject<ChangeFireProtSpecificHeat>({ id: normWireRequired(normWireString), newProtectionSpecificHeat: normWireRequired(normWireNumber) });

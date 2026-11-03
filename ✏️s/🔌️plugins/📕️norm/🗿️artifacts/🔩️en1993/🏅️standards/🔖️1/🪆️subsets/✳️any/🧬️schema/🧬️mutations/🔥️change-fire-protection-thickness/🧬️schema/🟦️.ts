/** 🔥️ `change-fire-protection-thickness` wire twin: the leaf payload `ChangeFireProtectionThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireProtectionThickness {
  id: string;
  newProtectionThickness: number;
}

export const parseChangeFireProtectionThickness: NormWireReader<ChangeFireProtectionThickness> = normWireObject<ChangeFireProtectionThickness>({ id: normWireRequired(normWireString), newProtectionThickness: normWireRequired(normWireNumber) });

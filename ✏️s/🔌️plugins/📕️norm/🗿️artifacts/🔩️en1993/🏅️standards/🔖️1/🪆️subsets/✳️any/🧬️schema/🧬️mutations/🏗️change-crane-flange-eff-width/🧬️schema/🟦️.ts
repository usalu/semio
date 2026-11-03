/** 🏗️ `change-crane-flange-eff-width` wire twin: the leaf payload `ChangeCraneFlangeEffWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneFlangeEffWidth {
  id: string;
  newFlangeEffectiveWidth: number;
}

export const parseChangeCraneFlangeEffWidth: NormWireReader<ChangeCraneFlangeEffWidth> = normWireObject<ChangeCraneFlangeEffWidth>({ id: normWireRequired(normWireString), newFlangeEffectiveWidth: normWireRequired(normWireNumber) });

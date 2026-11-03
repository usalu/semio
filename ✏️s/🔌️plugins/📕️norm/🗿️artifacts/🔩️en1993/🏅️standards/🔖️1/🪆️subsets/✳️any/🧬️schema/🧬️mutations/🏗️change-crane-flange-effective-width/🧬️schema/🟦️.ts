/** 🏗️ `change-crane-flange-effective-width` wire twin: the leaf payload `ChangeCraneFlangeEffectiveWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneFlangeEffectiveWidth {
  id: string;
  newFlangeEffectiveWidth: number;
}

export const parseChangeCraneFlangeEffectiveWidth: NormWireReader<ChangeCraneFlangeEffectiveWidth> = normWireObject<ChangeCraneFlangeEffectiveWidth>({ id: normWireRequired(normWireString), newFlangeEffectiveWidth: normWireRequired(normWireNumber) });

/** 🧱️ `change-plated-stiffener-flange-width` wire twin: the leaf payload `ChangePlatedStiffenerFlangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerFlangeWidth {
  id: string;
  newStiffenerFlangeWidth: number;
}

export const parseChangePlatedStiffenerFlangeWidth: NormWireReader<ChangePlatedStiffenerFlangeWidth> = normWireObject<ChangePlatedStiffenerFlangeWidth>({ id: normWireRequired(normWireString), newStiffenerFlangeWidth: normWireRequired(normWireNumber) });

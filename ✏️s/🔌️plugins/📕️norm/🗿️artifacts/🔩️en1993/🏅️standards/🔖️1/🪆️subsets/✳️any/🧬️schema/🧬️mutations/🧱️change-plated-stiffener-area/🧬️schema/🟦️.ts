/** 🧱️ `change-plated-stiffener-area` wire twin: the leaf payload `ChangePlatedStiffenerArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerArea {
  id: string;
  newStiffenerArea: number;
}

export const parseChangePlatedStiffenerArea: NormWireReader<ChangePlatedStiffenerArea> = normWireObject<ChangePlatedStiffenerArea>({ id: normWireRequired(normWireString), newStiffenerArea: normWireRequired(normWireNumber) });

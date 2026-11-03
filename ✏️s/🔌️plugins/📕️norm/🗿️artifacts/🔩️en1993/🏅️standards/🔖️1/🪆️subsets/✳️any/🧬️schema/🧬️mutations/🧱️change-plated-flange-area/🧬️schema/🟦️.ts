/** 🧱️ `change-plated-flange-area` wire twin: the leaf payload `ChangePlatedFlangeArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeArea {
  id: string;
  newFlangeArea: number;
}

export const parseChangePlatedFlangeArea: NormWireReader<ChangePlatedFlangeArea> = normWireObject<ChangePlatedFlangeArea>({ id: normWireRequired(normWireString), newFlangeArea: normWireRequired(normWireNumber) });

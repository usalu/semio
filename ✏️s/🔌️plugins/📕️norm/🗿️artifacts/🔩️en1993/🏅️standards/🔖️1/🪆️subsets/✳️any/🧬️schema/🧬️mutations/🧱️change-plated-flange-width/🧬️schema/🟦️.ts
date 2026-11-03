/** 🧱️ `change-plated-flange-width` wire twin: the leaf payload `ChangePlatedFlangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeWidth {
  id: string;
  newFlangeWidth: number;
}

export const parseChangePlatedFlangeWidth: NormWireReader<ChangePlatedFlangeWidth> = normWireObject<ChangePlatedFlangeWidth>({ id: normWireRequired(normWireString), newFlangeWidth: normWireRequired(normWireNumber) });

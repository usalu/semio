/** 🧱️ `change-plated-stiff-flange-width` wire twin: the leaf payload `ChangePlatedStiffFlangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffFlangeWidth {
  id: string;
  newStiffenerFlangeWidth: number;
}

export const parseChangePlatedStiffFlangeWidth: NormWireReader<ChangePlatedStiffFlangeWidth> = normWireObject<ChangePlatedStiffFlangeWidth>({ id: normWireRequired(normWireString), newStiffenerFlangeWidth: normWireRequired(normWireNumber) });

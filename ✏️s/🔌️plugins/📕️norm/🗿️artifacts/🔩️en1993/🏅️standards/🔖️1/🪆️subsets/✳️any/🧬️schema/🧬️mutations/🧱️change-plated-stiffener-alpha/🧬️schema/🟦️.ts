/** 🧱️ `change-plated-stiffener-alpha` wire twin: the leaf payload `ChangePlatedStiffenerAlpha`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerAlpha {
  id: string;
  newStiffenerAlpha: number;
}

export const parseChangePlatedStiffenerAlpha: NormWireReader<ChangePlatedStiffenerAlpha> = normWireObject<ChangePlatedStiffenerAlpha>({ id: normWireRequired(normWireString), newStiffenerAlpha: normWireRequired(normWireNumber) });

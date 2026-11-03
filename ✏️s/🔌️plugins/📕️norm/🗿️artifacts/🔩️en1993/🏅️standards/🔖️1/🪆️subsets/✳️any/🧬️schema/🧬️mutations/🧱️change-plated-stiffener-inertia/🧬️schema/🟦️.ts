/** 🧱️ `change-plated-stiffener-inertia` wire twin: the leaf payload `ChangePlatedStiffenerInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerInertia {
  id: string;
  newStiffenerInertia: number;
}

export const parseChangePlatedStiffenerInertia: NormWireReader<ChangePlatedStiffenerInertia> = normWireObject<ChangePlatedStiffenerInertia>({ id: normWireRequired(normWireString), newStiffenerInertia: normWireRequired(normWireNumber) });

/** 🧱️ `change-plated-longitudinal-inertia` wire twin: the leaf payload `ChangePlatedLongitudinalInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedLongitudinalInertia {
  id: string;
  newLongitudinalInertia: number;
}

export const parseChangePlatedLongitudinalInertia: NormWireReader<ChangePlatedLongitudinalInertia> = normWireObject<ChangePlatedLongitudinalInertia>({ id: normWireRequired(normWireString), newLongitudinalInertia: normWireRequired(normWireNumber) });

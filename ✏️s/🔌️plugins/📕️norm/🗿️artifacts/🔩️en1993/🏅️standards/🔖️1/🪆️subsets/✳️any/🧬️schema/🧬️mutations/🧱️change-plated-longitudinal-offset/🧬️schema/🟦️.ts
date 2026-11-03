/** 🧱️ `change-plated-longitudinal-offset` wire twin: the leaf payload `ChangePlatedLongitudinalOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedLongitudinalOffset {
  id: string;
  newLongitudinalOffset: number;
}

export const parseChangePlatedLongitudinalOffset: NormWireReader<ChangePlatedLongitudinalOffset> = normWireObject<ChangePlatedLongitudinalOffset>({ id: normWireRequired(normWireString), newLongitudinalOffset: normWireRequired(normWireNumber) });

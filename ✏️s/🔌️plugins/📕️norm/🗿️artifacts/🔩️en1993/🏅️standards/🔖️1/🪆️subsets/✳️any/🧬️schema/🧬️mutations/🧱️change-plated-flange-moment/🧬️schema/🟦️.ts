/** 🧱️ `change-plated-flange-moment` wire twin: the leaf payload `ChangePlatedFlangeMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeMoment {
  id: string;
  newFlangeMoment: number;
}

export const parseChangePlatedFlangeMoment: NormWireReader<ChangePlatedFlangeMoment> = normWireObject<ChangePlatedFlangeMoment>({ id: normWireRequired(normWireString), newFlangeMoment: normWireRequired(normWireNumber) });

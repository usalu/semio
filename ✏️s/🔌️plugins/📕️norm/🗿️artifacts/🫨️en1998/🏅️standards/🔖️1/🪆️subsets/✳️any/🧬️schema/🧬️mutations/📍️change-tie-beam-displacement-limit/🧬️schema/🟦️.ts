/** 📍️ `change-tie-beam-displacement-limit` wire twin: the leaf payload `ChangeTieBeamDisplacementLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamDisplacementLimit {
  mutation: "changeTieBeamDisplacementLimit";
  id: string;
  newDisplacementLimit: number;
}

export const parseChangeTieBeamDisplacementLimit: NormWireReader<ChangeTieBeamDisplacementLimit> = normWireObject<ChangeTieBeamDisplacementLimit>({ mutation: normWireRequired(normWireLiteral("changeTieBeamDisplacementLimit")), id: normWireRequired(normWireString), newDisplacementLimit: normWireRequired(normWireNumber) });

/** 📍️ `change-tie-beam-depth` wire twin: the leaf payload `ChangeTieBeamDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamDepth {
  mutation: "changeTieBeamDepth";
  id: string;
  newDepth: number;
}

export const parseChangeTieBeamDepth: NormWireReader<ChangeTieBeamDepth> = normWireObject<ChangeTieBeamDepth>({ mutation: normWireRequired(normWireLiteral("changeTieBeamDepth")), id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });

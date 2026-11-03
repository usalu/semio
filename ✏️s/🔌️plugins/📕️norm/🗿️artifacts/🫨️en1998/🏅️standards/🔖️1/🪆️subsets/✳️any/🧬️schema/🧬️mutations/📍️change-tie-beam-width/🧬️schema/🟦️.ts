/** 📍️ `change-tie-beam-width` wire twin: the leaf payload `ChangeTieBeamWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamWidth {
  mutation: "changeTieBeamWidth";
  id: string;
  newWidth: number;
}

export const parseChangeTieBeamWidth: NormWireReader<ChangeTieBeamWidth> = normWireObject<ChangeTieBeamWidth>({ mutation: normWireRequired(normWireLiteral("changeTieBeamWidth")), id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });

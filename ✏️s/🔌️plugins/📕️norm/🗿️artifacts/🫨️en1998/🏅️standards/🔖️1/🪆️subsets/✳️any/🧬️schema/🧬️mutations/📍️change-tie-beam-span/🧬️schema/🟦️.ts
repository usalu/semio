/** 📍️ `change-tie-beam-span` wire twin: the leaf payload `ChangeTieBeamSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamSpan {
  mutation: "changeTieBeamSpan";
  id: string;
  newSpan: number;
}

export const parseChangeTieBeamSpan: NormWireReader<ChangeTieBeamSpan> = normWireObject<ChangeTieBeamSpan>({ mutation: normWireRequired(normWireLiteral("changeTieBeamSpan")), id: normWireRequired(normWireString), newSpan: normWireRequired(normWireNumber) });

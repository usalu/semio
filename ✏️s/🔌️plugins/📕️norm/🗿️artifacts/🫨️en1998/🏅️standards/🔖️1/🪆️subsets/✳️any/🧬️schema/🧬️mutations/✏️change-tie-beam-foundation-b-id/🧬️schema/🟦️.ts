/** ✏️ `change-tie-beam-foundation-b-id` wire twin: the leaf payload `ChangeTieBeamFoundationBId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamFoundationBId {
  mutation: "changeTieBeamFoundationBId";
  id: string;
  newFoundationBId: string;
}

export const parseChangeTieBeamFoundationBId: NormWireReader<ChangeTieBeamFoundationBId> = normWireObject<ChangeTieBeamFoundationBId>({ mutation: normWireRequired(normWireLiteral("changeTieBeamFoundationBId")), id: normWireRequired(normWireString), newFoundationBId: normWireRequired(normWireString) });

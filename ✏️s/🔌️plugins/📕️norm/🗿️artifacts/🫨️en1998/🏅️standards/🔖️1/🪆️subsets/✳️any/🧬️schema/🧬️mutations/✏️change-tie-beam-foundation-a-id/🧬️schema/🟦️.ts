/** ✏️ `change-tie-beam-foundation-a-id` wire twin: the leaf payload `ChangeTieBeamFoundationAId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamFoundationAId {
  mutation: "changeTieBeamFoundationAId";
  id: string;
  newFoundationAId: string;
}

export const parseChangeTieBeamFoundationAId: NormWireReader<ChangeTieBeamFoundationAId> = normWireObject<ChangeTieBeamFoundationAId>({ mutation: normWireRequired(normWireLiteral("changeTieBeamFoundationAId")), id: normWireRequired(normWireString), newFoundationAId: normWireRequired(normWireString) });

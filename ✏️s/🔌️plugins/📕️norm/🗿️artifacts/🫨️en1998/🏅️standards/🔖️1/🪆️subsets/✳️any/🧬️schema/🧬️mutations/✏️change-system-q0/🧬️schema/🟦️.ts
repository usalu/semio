/** ✏️ `change-system-q0` wire twin: the leaf payload `ChangeSystemQ0`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSystemQ0 {
  mutation: "changeSystemQ0";
  buildingId: string;
  id: string;
  newQ0: number;
}

export const parseChangeSystemQ0: NormWireReader<ChangeSystemQ0> = normWireObject<ChangeSystemQ0>({ mutation: normWireRequired(normWireLiteral("changeSystemQ0")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQ0: normWireRequired(normWireNumber) });

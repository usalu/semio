/** ✏️ `change-system-system-type` wire twin: the leaf payload `ChangeSystemSystemType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SystemType, parseSystemType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSystemSystemType {
  mutation: "changeSystemSystemType";
  buildingId: string;
  id: string;
  newSystemType: SystemType;
}

export const parseChangeSystemSystemType: NormWireReader<ChangeSystemSystemType> = normWireObject<ChangeSystemSystemType>({ mutation: normWireRequired(normWireLiteral("changeSystemSystemType")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newSystemType: normWireRequired(parseSystemType) });

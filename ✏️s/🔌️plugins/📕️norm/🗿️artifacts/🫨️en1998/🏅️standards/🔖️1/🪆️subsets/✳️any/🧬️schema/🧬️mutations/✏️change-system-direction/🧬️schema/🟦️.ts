/** ✏️ `change-system-direction` wire twin: the leaf payload `ChangeSystemDirection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AnalysisDirection, parseAnalysisDirection } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSystemDirection {
  mutation: "changeSystemDirection";
  buildingId: string;
  id: string;
  newDirection: AnalysisDirection;
}

export const parseChangeSystemDirection: NormWireReader<ChangeSystemDirection> = normWireObject<ChangeSystemDirection>({ mutation: normWireRequired(normWireLiteral("changeSystemDirection")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDirection: normWireRequired(parseAnalysisDirection) });

/** ⚖️ `change-storey-permanent-gk-n` wire twin: the leaf payload `ChangeStoreyPermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyPermanentGkN {
  mutation: "changeStoreyPermanentGkN";
  buildingId: string;
  id: string;
  newPermanentGkN: number;
}

export const parseChangeStoreyPermanentGkN: NormWireReader<ChangeStoreyPermanentGkN> = normWireObject<ChangeStoreyPermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeStoreyPermanentGkN")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newPermanentGkN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

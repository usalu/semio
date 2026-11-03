/** ✏️ `change-diaphragm-storey-id` wire twin: the leaf payload `ChangeDiaphragmStoreyId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmStoreyId {
  mutation: "changeDiaphragmStoreyId";
  id: string;
  newStoreyId: string;
}

export const parseChangeDiaphragmStoreyId: NormWireReader<ChangeDiaphragmStoreyId> = normWireObject<ChangeDiaphragmStoreyId>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmStoreyId")), id: normWireRequired(normWireString), newStoreyId: normWireRequired(normWireString) });

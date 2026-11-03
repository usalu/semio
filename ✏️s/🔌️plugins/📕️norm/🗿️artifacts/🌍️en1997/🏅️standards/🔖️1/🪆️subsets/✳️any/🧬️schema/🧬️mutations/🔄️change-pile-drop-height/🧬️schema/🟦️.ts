/** 🔄️ `change-pile-drop-height` wire twin: the leaf payload `ChangePileDropHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDropHeight {
  mutation: "changePileDropHeight";
  id: string;
  newDropHeight: number;
}

export const parseChangePileDropHeight: NormWireReader<ChangePileDropHeight> = normWireObject<ChangePileDropHeight>({ mutation: normWireRequired(normWireLiteral("changePileDropHeight")), id: normWireRequired(normWireString), newDropHeight: normWireRequired(normWireNumber) });

/** 🏗️ `change-pile-group-cap-vertical` wire twin: the leaf payload `ChangePileGroupCapVertical`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupCapVertical {
  mutation: "changePileGroupCapVertical";
  id: string;
  newCapVertical: number;
}

export const parseChangePileGroupCapVertical: NormWireReader<ChangePileGroupCapVertical> = normWireObject<ChangePileGroupCapVertical>({ mutation: normWireRequired(normWireLiteral("changePileGroupCapVertical")), id: normWireRequired(normWireString), newCapVertical: normWireRequired(normWireNumber) });

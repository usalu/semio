/** 🏗️ `change-pile-group-pile-id` wire twin: the leaf payload `ChangePileGroupPileId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupPileId {
  mutation: "changePileGroupPileId";
  id: string;
  newPileId: string;
}

export const parseChangePileGroupPileId: NormWireReader<ChangePileGroupPileId> = normWireObject<ChangePileGroupPileId>({ mutation: normWireRequired(normWireLiteral("changePileGroupPileId")), id: normWireRequired(normWireString), newPileId: normWireRequired(normWireString) });

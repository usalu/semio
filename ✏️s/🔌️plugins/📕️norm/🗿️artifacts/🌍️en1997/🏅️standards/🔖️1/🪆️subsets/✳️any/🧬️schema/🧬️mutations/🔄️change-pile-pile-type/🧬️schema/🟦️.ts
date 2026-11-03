/** 🔄️ `change-pile-pile-type` wire twin: the leaf payload `ChangePilePileType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PileType, parsePileType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePilePileType {
  mutation: "changePilePileType";
  id: string;
  newPileType: PileType;
}

export const parseChangePilePileType: NormWireReader<ChangePilePileType> = normWireObject<ChangePilePileType>({ mutation: normWireRequired(normWireLiteral("changePilePileType")), id: normWireRequired(normWireString), newPileType: normWireRequired(parsePileType) });

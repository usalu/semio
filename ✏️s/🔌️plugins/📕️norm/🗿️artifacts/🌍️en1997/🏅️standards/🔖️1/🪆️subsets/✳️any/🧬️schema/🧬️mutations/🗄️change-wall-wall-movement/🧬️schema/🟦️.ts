/** 🗄️ `change-wall-wall-movement` wire twin: the leaf payload `ChangeWallWallMovement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallMovement, parseWallMovement } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWallWallMovement {
  mutation: "changeWallWallMovement";
  id: string;
  newWallMovement: WallMovement;
}

export const parseChangeWallWallMovement: NormWireReader<ChangeWallWallMovement> = normWireObject<ChangeWallWallMovement>({ mutation: normWireRequired(normWireLiteral("changeWallWallMovement")), id: normWireRequired(normWireString), newWallMovement: normWireRequired(parseWallMovement) });

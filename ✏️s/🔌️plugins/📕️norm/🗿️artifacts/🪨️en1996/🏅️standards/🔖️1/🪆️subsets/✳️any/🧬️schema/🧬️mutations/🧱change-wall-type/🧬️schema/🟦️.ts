/** 🧱 `change-wall-type` wire twin: the leaf payload `ChangeWallType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996WallType, parseEn1996WallType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWallType {
  id: string;
  newWallType: En1996WallType;
}

export const parseChangeWallType: NormWireReader<ChangeWallType> = normWireObject<ChangeWallType>({ id: normWireRequired(normWireString), newWallType: normWireRequired(parseEn1996WallType) });

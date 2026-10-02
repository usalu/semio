/** 🧱 `change-wall-type` wire twin: the leaf payload `ChangeWallType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996WallType, parseEn1996WallType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWallType {
  index: number;
  newWallType: En1996WallType;
}

export const parseChangeWallType: NormWireReader<ChangeWallType> = normWireObject<ChangeWallType>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newWallType: normWireRequired(parseEn1996WallType) });

/** 🗄️ `change-wall-height` wire twin: the leaf payload `ChangeWallHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallHeight {
  mutation: "changeWallHeight";
  id: string;
  newHeight: number;
}

export const parseChangeWallHeight: NormWireReader<ChangeWallHeight> = normWireObject<ChangeWallHeight>({ mutation: normWireRequired(normWireLiteral("changeWallHeight")), id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });

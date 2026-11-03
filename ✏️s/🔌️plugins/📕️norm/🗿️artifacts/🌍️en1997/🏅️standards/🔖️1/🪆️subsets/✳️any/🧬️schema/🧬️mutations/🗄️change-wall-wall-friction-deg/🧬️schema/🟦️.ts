/** 🗄️ `change-wall-wall-friction-deg` wire twin: the leaf payload `ChangeWallWallFrictionDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallWallFrictionDeg {
  mutation: "changeWallWallFrictionDeg";
  id: string;
  newWallFrictionDeg: number;
}

export const parseChangeWallWallFrictionDeg: NormWireReader<ChangeWallWallFrictionDeg> = normWireObject<ChangeWallWallFrictionDeg>({ mutation: normWireRequired(normWireLiteral("changeWallWallFrictionDeg")), id: normWireRequired(normWireString), newWallFrictionDeg: normWireRequired(normWireNumber) });

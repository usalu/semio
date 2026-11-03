/** 🧱 `change-emb-wall-phi-deg` wire twin: the leaf payload `ChangeEmbWallPhiDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallPhiDeg {
  mutation: "changeEmbWallPhiDeg";
  id: string;
  newPhiDeg: number;
}

export const parseChangeEmbWallPhiDeg: NormWireReader<ChangeEmbWallPhiDeg> = normWireObject<ChangeEmbWallPhiDeg>({ mutation: normWireRequired(normWireLiteral("changeEmbWallPhiDeg")), id: normWireRequired(normWireString), newPhiDeg: normWireRequired(normWireNumber) });

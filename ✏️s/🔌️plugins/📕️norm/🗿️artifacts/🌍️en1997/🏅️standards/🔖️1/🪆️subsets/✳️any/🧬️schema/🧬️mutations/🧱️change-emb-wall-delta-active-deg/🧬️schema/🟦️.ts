/** 🧱 `change-emb-wall-delta-active-deg` wire twin: the leaf payload `ChangeEmbWallDeltaActiveDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallDeltaActiveDeg {
  mutation: "changeEmbWallDeltaActiveDeg";
  id: string;
  newDeltaActiveDeg: number;
}

export const parseChangeEmbWallDeltaActiveDeg: NormWireReader<ChangeEmbWallDeltaActiveDeg> = normWireObject<ChangeEmbWallDeltaActiveDeg>({ mutation: normWireRequired(normWireLiteral("changeEmbWallDeltaActiveDeg")), id: normWireRequired(normWireString), newDeltaActiveDeg: normWireRequired(normWireNumber) });

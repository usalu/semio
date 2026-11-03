/** 🧱 `change-emb-wall-delta-passive-deg` wire twin: the leaf payload `ChangeEmbWallDeltaPassiveDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallDeltaPassiveDeg {
  mutation: "changeEmbWallDeltaPassiveDeg";
  id: string;
  newDeltaPassiveDeg: number;
}

export const parseChangeEmbWallDeltaPassiveDeg: NormWireReader<ChangeEmbWallDeltaPassiveDeg> = normWireObject<ChangeEmbWallDeltaPassiveDeg>({ mutation: normWireRequired(normWireLiteral("changeEmbWallDeltaPassiveDeg")), id: normWireRequired(normWireString), newDeltaPassiveDeg: normWireRequired(normWireNumber) });

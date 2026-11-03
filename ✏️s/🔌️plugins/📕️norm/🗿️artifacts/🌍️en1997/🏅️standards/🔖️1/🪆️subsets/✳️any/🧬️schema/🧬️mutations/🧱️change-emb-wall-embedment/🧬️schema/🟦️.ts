/** 🧱 `change-emb-wall-embedment` wire twin: the leaf payload `ChangeEmbWallEmbedment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallEmbedment {
  mutation: "changeEmbWallEmbedment";
  id: string;
  newEmbedment: number;
}

export const parseChangeEmbWallEmbedment: NormWireReader<ChangeEmbWallEmbedment> = normWireObject<ChangeEmbWallEmbedment>({ mutation: normWireRequired(normWireLiteral("changeEmbWallEmbedment")), id: normWireRequired(normWireString), newEmbedment: normWireRequired(normWireNumber) });

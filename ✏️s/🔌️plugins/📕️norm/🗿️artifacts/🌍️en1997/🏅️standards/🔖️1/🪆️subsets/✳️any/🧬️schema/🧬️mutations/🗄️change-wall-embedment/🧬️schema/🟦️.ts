/** 🗄️ `change-wall-embedment` wire twin: the leaf payload `ChangeWallEmbedment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallEmbedment {
  mutation: "changeWallEmbedment";
  id: string;
  newEmbedment: number;
}

export const parseChangeWallEmbedment: NormWireReader<ChangeWallEmbedment> = normWireObject<ChangeWallEmbedment>({ mutation: normWireRequired(normWireLiteral("changeWallEmbedment")), id: normWireRequired(normWireString), newEmbedment: normWireRequired(normWireNumber) });

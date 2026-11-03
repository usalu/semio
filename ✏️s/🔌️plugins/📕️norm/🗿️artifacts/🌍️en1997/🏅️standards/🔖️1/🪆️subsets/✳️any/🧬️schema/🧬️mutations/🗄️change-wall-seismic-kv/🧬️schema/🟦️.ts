/** 🗄️ `change-wall-seismic-kv` wire twin: the leaf payload `ChangeWallSeismicKv`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallSeismicKv {
  mutation: "changeWallSeismicKv";
  id: string;
  newSeismicKv: number;
}

export const parseChangeWallSeismicKv: NormWireReader<ChangeWallSeismicKv> = normWireObject<ChangeWallSeismicKv>({ mutation: normWireRequired(normWireLiteral("changeWallSeismicKv")), id: normWireRequired(normWireString), newSeismicKv: normWireRequired(normWireNumber) });

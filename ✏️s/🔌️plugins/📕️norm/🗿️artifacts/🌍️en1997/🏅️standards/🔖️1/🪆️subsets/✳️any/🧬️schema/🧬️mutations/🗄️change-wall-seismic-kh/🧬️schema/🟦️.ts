/** 🗄️ `change-wall-seismic-kh` wire twin: the leaf payload `ChangeWallSeismicKh`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallSeismicKh {
  mutation: "changeWallSeismicKh";
  id: string;
  newSeismicKh: number;
}

export const parseChangeWallSeismicKh: NormWireReader<ChangeWallSeismicKh> = normWireObject<ChangeWallSeismicKh>({ mutation: normWireRequired(normWireLiteral("changeWallSeismicKh")), id: normWireRequired(normWireString), newSeismicKh: normWireRequired(normWireNumber) });

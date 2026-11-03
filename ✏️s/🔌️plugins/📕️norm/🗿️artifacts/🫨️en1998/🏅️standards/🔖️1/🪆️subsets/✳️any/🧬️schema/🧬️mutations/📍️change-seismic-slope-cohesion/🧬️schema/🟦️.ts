/** 📍️ `change-seismic-slope-cohesion` wire twin: the leaf payload `ChangeSeismicSlopeCohesion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeCohesion {
  mutation: "changeSeismicSlopeCohesion";
  id: string;
  newCohesion: number;
}

export const parseChangeSeismicSlopeCohesion: NormWireReader<ChangeSeismicSlopeCohesion> = normWireObject<ChangeSeismicSlopeCohesion>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeCohesion")), id: normWireRequired(normWireString), newCohesion: normWireRequired(normWireNumber) });

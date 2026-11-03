/** 📍️ `change-seismic-slope-friction-angle` wire twin: the leaf payload `ChangeSeismicSlopeFrictionAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeFrictionAngle {
  mutation: "changeSeismicSlopeFrictionAngle";
  id: string;
  newFrictionAngle: number;
}

export const parseChangeSeismicSlopeFrictionAngle: NormWireReader<ChangeSeismicSlopeFrictionAngle> = normWireObject<ChangeSeismicSlopeFrictionAngle>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeFrictionAngle")), id: normWireRequired(normWireString), newFrictionAngle: normWireRequired(normWireNumber) });

/** ✏️ `change-foundation-soil-phi-deg` wire twin: the leaf payload `ChangeFoundationSoilPhiDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationSoilPhiDeg {
  mutation: "changeFoundationSoilPhiDeg";
  id: string;
  newSoilFrictionAngleDeg: number;
}

export const parseChangeFoundationSoilPhiDeg: NormWireReader<ChangeFoundationSoilPhiDeg> = normWireObject<ChangeFoundationSoilPhiDeg>({ mutation: normWireRequired(normWireLiteral("changeFoundationSoilPhiDeg")), id: normWireRequired(normWireString), newSoilFrictionAngleDeg: normWireRequired(normWireNumber) });

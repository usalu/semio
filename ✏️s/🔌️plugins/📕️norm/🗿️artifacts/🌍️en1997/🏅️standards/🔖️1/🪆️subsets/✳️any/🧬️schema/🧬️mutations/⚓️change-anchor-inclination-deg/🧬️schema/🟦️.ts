/** ⚓️ `change-anchor-inclination-deg` wire twin: the leaf payload `ChangeAnchorInclinationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorInclinationDeg {
  mutation: "changeAnchorInclinationDeg";
  id: string;
  newInclinationDeg: number;
}

export const parseChangeAnchorInclinationDeg: NormWireReader<ChangeAnchorInclinationDeg> = normWireObject<ChangeAnchorInclinationDeg>({ mutation: normWireRequired(normWireLiteral("changeAnchorInclinationDeg")), id: normWireRequired(normWireString), newInclinationDeg: normWireRequired(normWireNumber) });

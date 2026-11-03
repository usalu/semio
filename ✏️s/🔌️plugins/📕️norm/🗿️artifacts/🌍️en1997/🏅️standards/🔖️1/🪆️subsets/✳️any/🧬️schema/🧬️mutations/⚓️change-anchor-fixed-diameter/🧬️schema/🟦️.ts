/** ⚓️ `change-anchor-fixed-diameter` wire twin: the leaf payload `ChangeAnchorFixedDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFixedDiameter {
  mutation: "changeAnchorFixedDiameter";
  id: string;
  newFixedDiameter: number;
}

export const parseChangeAnchorFixedDiameter: NormWireReader<ChangeAnchorFixedDiameter> = normWireObject<ChangeAnchorFixedDiameter>({ mutation: normWireRequired(normWireLiteral("changeAnchorFixedDiameter")), id: normWireRequired(normWireString), newFixedDiameter: normWireRequired(normWireNumber) });

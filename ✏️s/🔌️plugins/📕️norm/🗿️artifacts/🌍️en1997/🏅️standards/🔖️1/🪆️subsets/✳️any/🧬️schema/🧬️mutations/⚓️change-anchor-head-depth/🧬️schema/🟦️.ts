/** ⚓️ `change-anchor-head-depth` wire twin: the leaf payload `ChangeAnchorHeadDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorHeadDepth {
  mutation: "changeAnchorHeadDepth";
  id: string;
  newHeadDepth: number;
}

export const parseChangeAnchorHeadDepth: NormWireReader<ChangeAnchorHeadDepth> = normWireObject<ChangeAnchorHeadDepth>({ mutation: normWireRequired(normWireLiteral("changeAnchorHeadDepth")), id: normWireRequired(normWireString), newHeadDepth: normWireRequired(normWireNumber) });

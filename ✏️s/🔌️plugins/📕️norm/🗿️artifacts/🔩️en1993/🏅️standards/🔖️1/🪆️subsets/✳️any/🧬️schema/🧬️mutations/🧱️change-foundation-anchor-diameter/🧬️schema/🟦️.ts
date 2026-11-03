/** 🧱️ `change-foundation-anchor-diameter` wire twin: the leaf payload `ChangeFoundationAnchorDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorDiameter {
  id: string;
  newAnchorDiameter: number;
}

export const parseChangeFoundationAnchorDiameter: NormWireReader<ChangeFoundationAnchorDiameter> = normWireObject<ChangeFoundationAnchorDiameter>({ id: normWireRequired(normWireString), newAnchorDiameter: normWireRequired(normWireNumber) });

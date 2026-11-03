/** 🧱️ `change-foundation-anchor-count` wire twin: the leaf payload `ChangeFoundationAnchorCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorCount {
  id: string;
  newAnchorCount: number;
}

export const parseChangeFoundationAnchorCount: NormWireReader<ChangeFoundationAnchorCount> = normWireObject<ChangeFoundationAnchorCount>({ id: normWireRequired(normWireString), newAnchorCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });

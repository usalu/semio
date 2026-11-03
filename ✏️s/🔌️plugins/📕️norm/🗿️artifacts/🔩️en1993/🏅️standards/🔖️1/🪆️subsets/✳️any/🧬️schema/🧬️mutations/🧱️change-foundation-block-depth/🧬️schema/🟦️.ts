/** 🧱️ `change-foundation-block-depth` wire twin: the leaf payload `ChangeFoundationBlockDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBlockDepth {
  id: string;
  newBlockDepth: number;
}

export const parseChangeFoundationBlockDepth: NormWireReader<ChangeFoundationBlockDepth> = normWireObject<ChangeFoundationBlockDepth>({ id: normWireRequired(normWireString), newBlockDepth: normWireRequired(normWireNumber) });

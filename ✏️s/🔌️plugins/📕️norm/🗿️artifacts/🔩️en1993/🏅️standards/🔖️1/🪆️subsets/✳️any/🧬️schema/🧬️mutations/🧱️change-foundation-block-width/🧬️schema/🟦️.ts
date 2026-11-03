/** 🧱️ `change-foundation-block-width` wire twin: the leaf payload `ChangeFoundationBlockWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBlockWidth {
  id: string;
  newBlockWidth: number;
}

export const parseChangeFoundationBlockWidth: NormWireReader<ChangeFoundationBlockWidth> = normWireObject<ChangeFoundationBlockWidth>({ id: normWireRequired(normWireString), newBlockWidth: normWireRequired(normWireNumber) });

/** 🧱️ `change-foundation-block-length` wire twin: the leaf payload `ChangeFoundationBlockLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBlockLength {
  id: string;
  newBlockLength: number;
}

export const parseChangeFoundationBlockLength: NormWireReader<ChangeFoundationBlockLength> = normWireObject<ChangeFoundationBlockLength>({ id: normWireRequired(normWireString), newBlockLength: normWireRequired(normWireNumber) });

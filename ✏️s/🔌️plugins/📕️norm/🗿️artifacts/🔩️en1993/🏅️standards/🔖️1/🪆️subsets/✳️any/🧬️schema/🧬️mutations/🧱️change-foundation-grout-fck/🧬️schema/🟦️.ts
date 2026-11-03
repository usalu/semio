/** 🧱️ `change-foundation-grout-fck` wire twin: the leaf payload `ChangeFoundationGroutFck`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationGroutFck {
  id: string;
  newGroutFck: number;
}

export const parseChangeFoundationGroutFck: NormWireReader<ChangeFoundationGroutFck> = normWireObject<ChangeFoundationGroutFck>({ id: normWireRequired(normWireString), newGroutFck: normWireRequired(normWireNumber) });

/** 🧱️ `change-foundation-concrete-fck` wire twin: the leaf payload `ChangeFoundationConcreteFck`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationConcreteFck {
  id: string;
  newConcreteFck: number;
}

export const parseChangeFoundationConcreteFck: NormWireReader<ChangeFoundationConcreteFck> = normWireObject<ChangeFoundationConcreteFck>({ id: normWireRequired(normWireString), newConcreteFck: normWireRequired(normWireNumber) });

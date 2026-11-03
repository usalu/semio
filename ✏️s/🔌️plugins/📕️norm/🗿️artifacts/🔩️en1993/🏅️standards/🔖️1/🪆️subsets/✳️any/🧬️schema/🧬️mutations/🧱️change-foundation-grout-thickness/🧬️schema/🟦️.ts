/** 🧱️ `change-foundation-grout-thickness` wire twin: the leaf payload `ChangeFoundationGroutThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationGroutThickness {
  id: string;
  newGroutThickness: number;
}

export const parseChangeFoundationGroutThickness: NormWireReader<ChangeFoundationGroutThickness> = normWireObject<ChangeFoundationGroutThickness>({ id: normWireRequired(normWireString), newGroutThickness: normWireRequired(normWireNumber) });

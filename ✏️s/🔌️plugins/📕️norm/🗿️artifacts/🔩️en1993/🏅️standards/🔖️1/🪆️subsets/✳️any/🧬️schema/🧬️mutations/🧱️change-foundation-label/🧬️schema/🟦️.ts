/** 🧱️ `change-foundation-label` wire twin: the leaf payload `ChangeFoundationLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLabel {
  id: string;
  newLabel: string;
}

export const parseChangeFoundationLabel: NormWireReader<ChangeFoundationLabel> = normWireObject<ChangeFoundationLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });

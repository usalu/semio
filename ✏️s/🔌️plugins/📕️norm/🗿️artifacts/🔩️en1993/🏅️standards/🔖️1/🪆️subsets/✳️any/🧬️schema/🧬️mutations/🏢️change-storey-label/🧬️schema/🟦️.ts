/** 🏢️ `change-storey-label` wire twin: the leaf payload `ChangeStoreyLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyLabel {
  id: string;
  newLabel: string;
}

export const parseChangeStoreyLabel: NormWireReader<ChangeStoreyLabel> = normWireObject<ChangeStoreyLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });

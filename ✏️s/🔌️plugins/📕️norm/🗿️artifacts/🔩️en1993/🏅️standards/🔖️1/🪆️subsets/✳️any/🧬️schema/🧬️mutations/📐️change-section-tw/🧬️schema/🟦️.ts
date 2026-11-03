/** 📐️ `change-section-tw` wire twin: the leaf payload `ChangeSectionTw`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionTw {
  id: string;
  newTw: number;
}

export const parseChangeSectionTw: NormWireReader<ChangeSectionTw> = normWireObject<ChangeSectionTw>({ id: normWireRequired(normWireString), newTw: normWireRequired(normWireNumber) });

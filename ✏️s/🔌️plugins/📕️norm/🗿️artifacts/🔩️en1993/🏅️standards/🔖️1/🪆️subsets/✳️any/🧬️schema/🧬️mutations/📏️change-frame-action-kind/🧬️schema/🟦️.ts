/** 📏️ `change-frame-action-kind` wire twin: the leaf payload `ChangeFrameActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameActionKind, parseFrameActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrameActionKind {
  id: string;
  newKind: FrameActionKind;
}

export const parseChangeFrameActionKind: NormWireReader<ChangeFrameActionKind> = normWireObject<ChangeFrameActionKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseFrameActionKind) });

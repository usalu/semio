/** 🌉 `change-wind-bridge-parapet-kind` wire twin: the leaf payload `ChangeWindBridgeParapetKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeParapetKind {
  id: string;
  newParapetKind: number;
}

export const parseChangeWindBridgeParapetKind: NormWireReader<ChangeWindBridgeParapetKind> = normWireObject<ChangeWindBridgeParapetKind>({ id: normWireRequired(normWireString), newParapetKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });

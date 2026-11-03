/** 🌉 `change-wind-bridge-traffic-kind` wire twin: the leaf payload `ChangeWindBridgeTrafficKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeTrafficKind {
  id: string;
  newTrafficKind: number;
}

export const parseChangeWindBridgeTrafficKind: NormWireReader<ChangeWindBridgeTrafficKind> = normWireObject<ChangeWindBridgeTrafficKind>({ id: normWireRequired(normWireString), newTrafficKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });

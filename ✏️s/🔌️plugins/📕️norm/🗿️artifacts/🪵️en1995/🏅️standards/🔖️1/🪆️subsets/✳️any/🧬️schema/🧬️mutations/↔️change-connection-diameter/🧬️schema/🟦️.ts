/** ↔️ `change-connection-diameter` wire twin: the leaf payload `ChangeConnectionDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionDiameter {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionDiameter: NormWireReader<ChangeConnectionDiameter> = normWireObject<ChangeConnectionDiameter>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });

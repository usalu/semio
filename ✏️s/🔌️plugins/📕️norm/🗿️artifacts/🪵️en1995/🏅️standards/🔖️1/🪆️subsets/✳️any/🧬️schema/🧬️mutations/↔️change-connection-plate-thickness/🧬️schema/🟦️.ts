/** ↔️ `change-connection-plate-thickness` wire twin: the leaf payload `ChangeConnectionPlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionPlateThickness {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionPlateThickness: NormWireReader<ChangeConnectionPlateThickness> = normWireObject<ChangeConnectionPlateThickness>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

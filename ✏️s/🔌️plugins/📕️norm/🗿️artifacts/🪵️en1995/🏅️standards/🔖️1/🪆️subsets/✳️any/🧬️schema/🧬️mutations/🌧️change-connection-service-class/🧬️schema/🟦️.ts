/** 🌧️ `change-connection-service-class` wire twin: the leaf payload `ChangeConnectionServiceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionServiceClass {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionServiceClass: NormWireReader<ChangeConnectionServiceClass> = normWireObject<ChangeConnectionServiceClass>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":3})) });

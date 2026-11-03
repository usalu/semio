/** ↔️ `change-connection-spacing` wire twin: the leaf payload `ChangeConnectionSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionSpacing {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionSpacing: NormWireReader<ChangeConnectionSpacing> = normWireObject<ChangeConnectionSpacing>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

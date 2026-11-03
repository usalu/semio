/** 🔩️ `change-connection-steel-plate` wire twin: the leaf payload `ChangeConnectionSteelPlate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionSteelPlate {
  connectionId: string;
  newValue: boolean;
}

export const parseChangeConnectionSteelPlate: NormWireReader<ChangeConnectionSteelPlate> = normWireObject<ChangeConnectionSteelPlate>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireBoolean) });

/** 🔩️ `change-connection-fastener-type` wire twin: the leaf payload `ChangeConnectionFastenerType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FastenerType, parseFastenerType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnectionFastenerType {
  connectionId: string;
  newValue: FastenerType;
}

export const parseChangeConnectionFastenerType: NormWireReader<ChangeConnectionFastenerType> = normWireObject<ChangeConnectionFastenerType>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(parseFastenerType) });

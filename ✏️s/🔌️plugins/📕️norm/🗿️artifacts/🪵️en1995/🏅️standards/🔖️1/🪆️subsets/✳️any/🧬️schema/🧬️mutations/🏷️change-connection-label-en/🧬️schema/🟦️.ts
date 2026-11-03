/** 🏷️ `change-connection-label-en` wire twin: the leaf payload `ChangeConnectionLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionLabelEn {
  connectionId: string;
  newValue: string;
}

export const parseChangeConnectionLabelEn: NormWireReader<ChangeConnectionLabelEn> = normWireObject<ChangeConnectionLabelEn>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });

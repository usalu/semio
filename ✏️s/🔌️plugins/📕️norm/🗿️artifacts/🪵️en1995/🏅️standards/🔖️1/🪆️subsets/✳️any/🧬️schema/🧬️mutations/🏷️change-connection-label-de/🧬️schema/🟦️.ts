/** 🏷️ `change-connection-label-de` wire twin: the leaf payload `ChangeConnectionLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionLabelDe {
  connectionId: string;
  newValue: string;
}

export const parseChangeConnectionLabelDe: NormWireReader<ChangeConnectionLabelDe> = normWireObject<ChangeConnectionLabelDe>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });

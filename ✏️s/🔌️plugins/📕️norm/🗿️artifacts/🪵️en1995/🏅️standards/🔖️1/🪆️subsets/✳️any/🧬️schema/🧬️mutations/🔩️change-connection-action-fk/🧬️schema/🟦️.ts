/** 🔩️ `change-connection-action-fk` wire twin: the leaf payload `ChangeConnectionActionFK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionActionFK {
  connectionId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeConnectionActionFK: NormWireReader<ChangeConnectionActionFK> = normWireObject<ChangeConnectionActionFK>({ connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });

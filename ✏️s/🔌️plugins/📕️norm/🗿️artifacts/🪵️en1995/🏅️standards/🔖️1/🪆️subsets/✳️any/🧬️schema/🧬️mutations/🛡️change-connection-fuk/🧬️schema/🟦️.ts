/** 🛡️ `change-connection-fuk` wire twin: the leaf payload `ChangeConnectionFUK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionFUK {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionFUK: NormWireReader<ChangeConnectionFUK> = normWireObject<ChangeConnectionFUK>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });

/** 🔩️ `change-fastener-connection` wire twin: the leaf payload `ChangeFastenerConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerConnection {
  fastenerId: string;
  newValue: string;
}

export const parseChangeFastenerConnection: NormWireReader<ChangeFastenerConnection> = normWireObject<ChangeFastenerConnection>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });

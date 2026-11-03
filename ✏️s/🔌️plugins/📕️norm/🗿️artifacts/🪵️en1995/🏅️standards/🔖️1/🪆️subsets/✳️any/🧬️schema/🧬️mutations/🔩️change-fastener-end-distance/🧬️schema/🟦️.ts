/** 🔩️ `change-fastener-end-distance` wire twin: the leaf payload `ChangeFastenerEndDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerEndDistance {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerEndDistance: NormWireReader<ChangeFastenerEndDistance> = normWireObject<ChangeFastenerEndDistance>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });

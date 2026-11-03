/** 🛡️ `change-diaphragm-fastener-capacity` wire twin: the leaf payload `ChangeDiaphragmFastenerCapacity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmFastenerCapacity {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmFastenerCapacity: NormWireReader<ChangeDiaphragmFastenerCapacity> = normWireObject<ChangeDiaphragmFastenerCapacity>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });

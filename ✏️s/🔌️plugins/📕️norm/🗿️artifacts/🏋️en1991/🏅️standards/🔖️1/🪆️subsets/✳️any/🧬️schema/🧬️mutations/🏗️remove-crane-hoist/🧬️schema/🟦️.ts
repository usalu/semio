/** 🏗 `remove-crane-hoist` wire twin: the leaf payload `RemoveCraneHoist`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCraneHoist {
  id: string;
}

export const parseRemoveCraneHoist: NormWireReader<RemoveCraneHoist> = normWireObject<RemoveCraneHoist>({ id: normWireRequired(normWireString) });

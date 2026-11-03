/** 🏗️ `change-crane-stress-range` wire twin: the leaf payload `ChangeCraneStressRange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneStressRange {
  id: string;
  newStressRange: number;
}

export const parseChangeCraneStressRange: NormWireReader<ChangeCraneStressRange> = normWireObject<ChangeCraneStressRange>({ id: normWireRequired(normWireString), newStressRange: normWireRequired(normWireNumber) });

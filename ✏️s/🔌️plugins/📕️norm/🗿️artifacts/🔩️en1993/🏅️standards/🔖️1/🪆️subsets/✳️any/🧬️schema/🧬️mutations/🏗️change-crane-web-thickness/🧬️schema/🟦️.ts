/** 🏗️ `change-crane-web-thickness` wire twin: the leaf payload `ChangeCraneWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneWebThickness {
  id: string;
  newWebThickness: number;
}

export const parseChangeCraneWebThickness: NormWireReader<ChangeCraneWebThickness> = normWireObject<ChangeCraneWebThickness>({ id: normWireRequired(normWireString), newWebThickness: normWireRequired(normWireNumber) });

/** 🔧️ `change-param-tolerance` wire twin: the leaf payload `ChangeParamTolerance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamTolerance {
  id: string;
  newTolerance: number;
}

export const parseChangeParamTolerance: NormWireReader<ChangeParamTolerance> = normWireObject<ChangeParamTolerance>({ id: normWireRequired(normWireString), newTolerance: normWireRequired(normWireNumber) });

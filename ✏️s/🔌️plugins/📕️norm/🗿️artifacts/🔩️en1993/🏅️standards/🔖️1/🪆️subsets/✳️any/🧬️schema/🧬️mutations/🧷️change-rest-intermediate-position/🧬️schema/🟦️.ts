/** 🧷️ `change-rest-intermediate-position` wire twin: the leaf payload `ChangeRestIntermediatePosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestIntermediatePosition {
  id: string;
  newIntermediatePosition: number;
}

export const parseChangeRestIntermediatePosition: NormWireReader<ChangeRestIntermediatePosition> = normWireObject<ChangeRestIntermediatePosition>({ id: normWireRequired(normWireString), newIntermediatePosition: normWireRequired(normWireNumber) });

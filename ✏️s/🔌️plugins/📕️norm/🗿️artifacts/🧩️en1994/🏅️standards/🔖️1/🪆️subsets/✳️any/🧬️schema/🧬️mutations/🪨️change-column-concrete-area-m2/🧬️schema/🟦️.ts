/** 🪨️ `change-column-concrete-area-m2` wire twin: the leaf payload `ChangeColumnConcreteAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnConcreteAreaM2 {
  id: string;
  newConcreteAM2: number;
}

export const parseChangeColumnConcreteAreaM2: NormWireReader<ChangeColumnConcreteAreaM2> = normWireObject<ChangeColumnConcreteAreaM2>({ id: normWireRequired(normWireString), newConcreteAM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

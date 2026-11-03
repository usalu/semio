/** 🪜️ `change-built-up-batten-inertia` wire twin: the leaf payload `ChangeBuiltUpBattenInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpBattenInertia {
  id: string;
  newBattenInertia: number;
}

export const parseChangeBuiltUpBattenInertia: NormWireReader<ChangeBuiltUpBattenInertia> = normWireObject<ChangeBuiltUpBattenInertia>({ id: normWireRequired(normWireString), newBattenInertia: normWireRequired(normWireNumber) });

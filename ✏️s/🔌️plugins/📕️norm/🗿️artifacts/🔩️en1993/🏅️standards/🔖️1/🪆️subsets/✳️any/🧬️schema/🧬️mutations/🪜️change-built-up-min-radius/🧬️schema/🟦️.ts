/** 🪜️ `change-built-up-min-radius` wire twin: the leaf payload `ChangeBuiltUpMinRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpMinRadius {
  id: string;
  newMinRadius: number;
}

export const parseChangeBuiltUpMinRadius: NormWireReader<ChangeBuiltUpMinRadius> = normWireObject<ChangeBuiltUpMinRadius>({ id: normWireRequired(normWireString), newMinRadius: normWireRequired(normWireNumber) });

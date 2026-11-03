/** 🏠 `change-roof-has-parapet` wire twin: the leaf payload `ChangeRoofHasParapet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofHasParapet {
  id: string;
  newHasParapet: boolean;
}

export const parseChangeRoofHasParapet: NormWireReader<ChangeRoofHasParapet> = normWireObject<ChangeRoofHasParapet>({ id: normWireRequired(normWireString), newHasParapet: normWireRequired(normWireBoolean) });

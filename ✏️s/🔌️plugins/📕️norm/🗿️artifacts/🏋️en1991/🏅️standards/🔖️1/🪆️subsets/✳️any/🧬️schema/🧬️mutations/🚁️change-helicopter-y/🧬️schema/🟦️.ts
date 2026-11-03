/** 🚁 `change-helicopter-y` wire twin: the leaf payload `ChangeHelicopterY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHelicopterY {
  id: string;
  newY: number;
}

export const parseChangeHelicopterY: NormWireReader<ChangeHelicopterY> = normWireObject<ChangeHelicopterY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });

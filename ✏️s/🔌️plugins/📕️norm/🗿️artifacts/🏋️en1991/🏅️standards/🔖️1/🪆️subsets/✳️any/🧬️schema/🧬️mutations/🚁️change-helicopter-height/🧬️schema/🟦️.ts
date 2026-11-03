/** 🚁 `change-helicopter-height` wire twin: the leaf payload `ChangeHelicopterHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHelicopterHeight {
  id: string;
  newHeight: number;
}

export const parseChangeHelicopterHeight: NormWireReader<ChangeHelicopterHeight> = normWireObject<ChangeHelicopterHeight>({ id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });

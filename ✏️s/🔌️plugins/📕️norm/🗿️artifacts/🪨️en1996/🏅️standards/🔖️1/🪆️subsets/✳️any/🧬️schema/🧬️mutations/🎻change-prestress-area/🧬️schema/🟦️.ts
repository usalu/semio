/** 🎻 `change-prestress-area` wire twin: the leaf payload `ChangePrestressArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressArea {
  wallId: string;
  id: string;
  newAreaM2: number;
}

export const parseChangePrestressArea: NormWireReader<ChangePrestressArea> = normWireObject<ChangePrestressArea>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });

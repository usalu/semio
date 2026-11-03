/** 🪵️ `change-pile-shear-area` wire twin: the leaf payload `ChangePileShearArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileShearArea {
  id: string;
  newShearArea: number;
}

export const parseChangePileShearArea: NormWireReader<ChangePileShearArea> = normWireObject<ChangePileShearArea>({ id: normWireRequired(normWireString), newShearArea: normWireRequired(normWireNumber) });

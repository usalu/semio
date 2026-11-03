/** 🗼️ `change-tower-slenderness` wire twin: the leaf payload `ChangeTowerSlenderness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerSlenderness {
  id: string;
  newSlenderness: number;
}

export const parseChangeTowerSlenderness: NormWireReader<ChangeTowerSlenderness> = normWireObject<ChangeTowerSlenderness>({ id: normWireRequired(normWireString), newSlenderness: normWireRequired(normWireNumber) });

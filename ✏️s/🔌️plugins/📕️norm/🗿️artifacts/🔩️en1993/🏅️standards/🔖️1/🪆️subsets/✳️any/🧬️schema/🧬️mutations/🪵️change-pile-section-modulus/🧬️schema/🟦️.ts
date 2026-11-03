/** 🪵️ `change-pile-section-modulus` wire twin: the leaf payload `ChangePileSectionModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileSectionModulus {
  id: string;
  newSectionModulus: number;
}

export const parseChangePileSectionModulus: NormWireReader<ChangePileSectionModulus> = normWireObject<ChangePileSectionModulus>({ id: normWireRequired(normWireString), newSectionModulus: normWireRequired(normWireNumber) });

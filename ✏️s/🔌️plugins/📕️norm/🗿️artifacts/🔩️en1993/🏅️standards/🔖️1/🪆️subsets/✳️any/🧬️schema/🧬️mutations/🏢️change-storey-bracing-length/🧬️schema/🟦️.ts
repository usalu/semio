/** 🏢️ `change-storey-bracing-length` wire twin: the leaf payload `ChangeStoreyBracingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyBracingLength {
  id: string;
  newBracingLength: number;
}

export const parseChangeStoreyBracingLength: NormWireReader<ChangeStoreyBracingLength> = normWireObject<ChangeStoreyBracingLength>({ id: normWireRequired(normWireString), newBracingLength: normWireRequired(normWireNumber) });

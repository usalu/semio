/** 🏢️ `change-storey-bracing-deflection` wire twin: the leaf payload `ChangeStoreyBracingDeflection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyBracingDeflection {
  id: string;
  newBracingDeflection: number;
}

export const parseChangeStoreyBracingDeflection: NormWireReader<ChangeStoreyBracingDeflection> = normWireObject<ChangeStoreyBracingDeflection>({ id: normWireRequired(normWireString), newBracingDeflection: normWireRequired(normWireNumber) });

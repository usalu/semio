/** 🏢️ `change-storey-plastic-analysis` wire twin: the leaf payload `ChangeStoreyPlasticAnalysis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyPlasticAnalysis {
  id: string;
  newPlasticAnalysis: number;
}

export const parseChangeStoreyPlasticAnalysis: NormWireReader<ChangeStoreyPlasticAnalysis> = normWireObject<ChangeStoreyPlasticAnalysis>({ id: normWireRequired(normWireString), newPlasticAnalysis: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });

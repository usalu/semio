/** ✖️ `change-bracing-resistance` wire twin: the leaf payload `ChangeBracingResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingResistance {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingResistance: NormWireReader<ChangeBracingResistance> = normWireObject<ChangeBracingResistance>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });

/** 🧱 `change-unit-fb` wire twin: the leaf payload `ChangeUnitFb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUnitFb {
  id: string;
  newFBPa: number;
}

export const parseChangeUnitFb: NormWireReader<ChangeUnitFb> = normWireObject<ChangeUnitFb>({ id: normWireRequired(normWireString), newFBPa: normWireRequired(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0})) });

/** ⛽ `change-fire-load-density-qf` wire twin: the leaf payload `ChangeFireLoadDensityQf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireLoadDensityQf {
  newFireLoadDensityQf: number;
}

export const parseChangeFireLoadDensityQf: NormWireReader<ChangeFireLoadDensityQf> = normWireObject<ChangeFireLoadDensityQf>({ newFireLoadDensityQf: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

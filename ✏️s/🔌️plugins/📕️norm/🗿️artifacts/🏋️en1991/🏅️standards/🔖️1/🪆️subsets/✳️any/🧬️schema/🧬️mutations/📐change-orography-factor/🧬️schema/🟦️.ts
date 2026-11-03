/** 📐 `change-orography-factor` wire twin: the leaf payload `ChangeOrographyFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOrographyFactor {
  newOrographyFactor: number;
}

export const parseChangeOrographyFactor: NormWireReader<ChangeOrographyFactor> = normWireObject<ChangeOrographyFactor>({ newOrographyFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

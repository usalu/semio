/** 🪟 `change-fire-opening-factor` wire twin: the leaf payload `ChangeFireOpeningFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireOpeningFactor {
  newFireOpeningFactor: number;
}

export const parseChangeFireOpeningFactor: NormWireReader<ChangeFireOpeningFactor> = normWireObject<ChangeFireOpeningFactor>({ newFireOpeningFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });

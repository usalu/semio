/** 📏 `change-crane-runway-length` wire twin: the leaf payload `ChangeCraneRunwayLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneRunwayLength {
  newCraneRunwayLength: number;
}

export const parseChangeCraneRunwayLength: NormWireReader<ChangeCraneRunwayLength> = normWireObject<ChangeCraneRunwayLength>({ newCraneRunwayLength: normWireRequired(normWireRange(normWireNumber, {"minimum":1})) });

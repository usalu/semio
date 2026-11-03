/** ❄️ `change-altitude` wire twin: the leaf payload `ChangeAltitude`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAltitude {
  newAltitude: number;
}

export const parseChangeAltitude: NormWireReader<ChangeAltitude> = normWireObject<ChangeAltitude>({ newAltitude: normWireRequired(normWireNumber) });

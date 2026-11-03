/** ⚙ `change-rail-load-group-factor-vertical` wire twin: the leaf payload `ChangeRailLoadGroupFactorVertical`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupFactorVertical {
  id: string;
  newFactorVertical: number;
}

export const parseChangeRailLoadGroupFactorVertical: NormWireReader<ChangeRailLoadGroupFactorVertical> = normWireObject<ChangeRailLoadGroupFactorVertical>({ id: normWireRequired(normWireString), newFactorVertical: normWireRequired(normWireNumber) });

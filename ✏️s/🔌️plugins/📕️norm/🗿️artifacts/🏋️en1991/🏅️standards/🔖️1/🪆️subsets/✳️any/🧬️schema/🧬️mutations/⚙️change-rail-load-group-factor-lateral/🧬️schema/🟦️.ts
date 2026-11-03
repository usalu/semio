/** ⚙ `change-rail-load-group-factor-lateral` wire twin: the leaf payload `ChangeRailLoadGroupFactorLateral`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupFactorLateral {
  id: string;
  newFactorLateral: number;
}

export const parseChangeRailLoadGroupFactorLateral: NormWireReader<ChangeRailLoadGroupFactorLateral> = normWireObject<ChangeRailLoadGroupFactorLateral>({ id: normWireRequired(normWireString), newFactorLateral: normWireRequired(normWireNumber) });

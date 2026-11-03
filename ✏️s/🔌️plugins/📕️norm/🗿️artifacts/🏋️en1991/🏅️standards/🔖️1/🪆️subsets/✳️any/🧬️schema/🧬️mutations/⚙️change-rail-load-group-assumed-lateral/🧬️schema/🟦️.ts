/** ⚙ `change-rail-load-group-assumed-lateral` wire twin: the leaf payload `ChangeRailLoadGroupAssumedLateral`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupAssumedLateral {
  id: string;
  newAssumedLateral: number;
}

export const parseChangeRailLoadGroupAssumedLateral: NormWireReader<ChangeRailLoadGroupAssumedLateral> = normWireObject<ChangeRailLoadGroupAssumedLateral>({ id: normWireRequired(normWireString), newAssumedLateral: normWireRequired(normWireNumber) });

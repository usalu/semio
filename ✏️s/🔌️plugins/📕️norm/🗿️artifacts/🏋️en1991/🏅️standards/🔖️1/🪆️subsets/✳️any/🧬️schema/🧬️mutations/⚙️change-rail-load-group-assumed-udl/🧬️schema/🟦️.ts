/** ⚙ `change-rail-load-group-assumed-udl` wire twin: the leaf payload `ChangeRailLoadGroupAssumedUdl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupAssumedUdl {
  id: string;
  newAssumedUdl: number;
}

export const parseChangeRailLoadGroupAssumedUdl: NormWireReader<ChangeRailLoadGroupAssumedUdl> = normWireObject<ChangeRailLoadGroupAssumedUdl>({ id: normWireRequired(normWireString), newAssumedUdl: normWireRequired(normWireNumber) });

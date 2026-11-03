/** ⚙ `change-rail-derailment-assumed-line-load` wire twin: the leaf payload `ChangeRailDerailmentAssumedLineLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDerailmentAssumedLineLoad {
  id: string;
  newAssumedLineLoad: number;
}

export const parseChangeRailDerailmentAssumedLineLoad: NormWireReader<ChangeRailDerailmentAssumedLineLoad> = normWireObject<ChangeRailDerailmentAssumedLineLoad>({ id: normWireRequired(normWireString), newAssumedLineLoad: normWireRequired(normWireNumber) });

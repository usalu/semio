/** ⚙ `change-rail-derailment-offset` wire twin: the leaf payload `ChangeRailDerailmentOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDerailmentOffset {
  id: string;
  newOffset: number;
}

export const parseChangeRailDerailmentOffset: NormWireReader<ChangeRailDerailmentOffset> = normWireObject<ChangeRailDerailmentOffset>({ id: normWireRequired(normWireString), newOffset: normWireRequired(normWireNumber) });

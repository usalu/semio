/** ⚙ `change-rail-derailment-loaded-length` wire twin: the leaf payload `ChangeRailDerailmentLoadedLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDerailmentLoadedLength {
  id: string;
  newLoadedLength: number;
}

export const parseChangeRailDerailmentLoadedLength: NormWireReader<ChangeRailDerailmentLoadedLength> = normWireObject<ChangeRailDerailmentLoadedLength>({ id: normWireRequired(normWireString), newLoadedLength: normWireRequired(normWireNumber) });

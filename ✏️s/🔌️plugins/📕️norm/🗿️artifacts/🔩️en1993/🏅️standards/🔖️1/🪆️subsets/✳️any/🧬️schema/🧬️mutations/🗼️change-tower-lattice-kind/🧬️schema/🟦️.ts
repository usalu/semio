/** 🗼️ `change-tower-lattice-kind` wire twin: the leaf payload `ChangeTowerLatticeKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LatticeKind, parseLatticeKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTowerLatticeKind {
  id: string;
  newLatticeKind: LatticeKind;
}

export const parseChangeTowerLatticeKind: NormWireReader<ChangeTowerLatticeKind> = normWireObject<ChangeTowerLatticeKind>({ id: normWireRequired(normWireString), newLatticeKind: normWireRequired(parseLatticeKind) });

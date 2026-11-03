/** 🕳️ `change-opening-y` wire twin: the leaf payload `ChangeOpeningY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningY {
  memberId: string;
  openingId: string;
  newY: number;
}

export const parseChangeOpeningY: NormWireReader<ChangeOpeningY> = normWireObject<ChangeOpeningY>({ memberId: normWireRequired(normWireString), openingId: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });

/** 🕳️ `change-opening-x` wire twin: the leaf payload `ChangeOpeningX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningX {
  memberId: string;
  openingId: string;
  newX: number;
}

export const parseChangeOpeningX: NormWireReader<ChangeOpeningX> = normWireObject<ChangeOpeningX>({ memberId: normWireRequired(normWireString), openingId: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });

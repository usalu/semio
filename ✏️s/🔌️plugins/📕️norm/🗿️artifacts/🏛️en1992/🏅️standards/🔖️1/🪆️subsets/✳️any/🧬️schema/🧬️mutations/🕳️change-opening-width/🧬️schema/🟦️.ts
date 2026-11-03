/** 🕳️ `change-opening-width` wire twin: the leaf payload `ChangeOpeningWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningWidth {
  memberId: string;
  openingId: string;
  newWidth: number;
}

export const parseChangeOpeningWidth: NormWireReader<ChangeOpeningWidth> = normWireObject<ChangeOpeningWidth>({ memberId: normWireRequired(normWireString), openingId: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });

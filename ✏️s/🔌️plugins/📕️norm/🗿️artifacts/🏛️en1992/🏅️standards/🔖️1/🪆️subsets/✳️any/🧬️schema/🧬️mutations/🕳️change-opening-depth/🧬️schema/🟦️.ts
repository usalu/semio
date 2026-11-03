/** 🕳️ `change-opening-depth` wire twin: the leaf payload `ChangeOpeningDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningDepth {
  memberId: string;
  openingId: string;
  newDepth: number;
}

export const parseChangeOpeningDepth: NormWireReader<ChangeOpeningDepth> = normWireObject<ChangeOpeningDepth>({ memberId: normWireRequired(normWireString), openingId: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });

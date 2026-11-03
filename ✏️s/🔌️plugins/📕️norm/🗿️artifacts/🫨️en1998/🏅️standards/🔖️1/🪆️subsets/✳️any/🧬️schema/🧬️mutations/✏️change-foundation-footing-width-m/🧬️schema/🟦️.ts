/** ✏️ `change-foundation-footing-width-m` wire twin: the leaf payload `ChangeFoundationFootingWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationFootingWidthM {
  mutation: "changeFoundationFootingWidthM";
  id: string;
  newFootingWidthM: number;
}

export const parseChangeFoundationFootingWidthM: NormWireReader<ChangeFoundationFootingWidthM> = normWireObject<ChangeFoundationFootingWidthM>({ mutation: normWireRequired(normWireLiteral("changeFoundationFootingWidthM")), id: normWireRequired(normWireString), newFootingWidthM: normWireRequired(normWireNumber) });

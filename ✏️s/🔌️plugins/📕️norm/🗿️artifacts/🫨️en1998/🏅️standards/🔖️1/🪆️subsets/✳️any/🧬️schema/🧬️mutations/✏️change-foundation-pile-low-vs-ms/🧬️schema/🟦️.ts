/** ✏️ `change-foundation-pile-low-vs-ms` wire twin: the leaf payload `ChangeFoundationPileLowVsMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileLowVsMS {
  mutation: "changeFoundationPileLowVsMS";
  id: string;
  newPileLowerVsMS: number;
}

export const parseChangeFoundationPileLowVsMS: NormWireReader<ChangeFoundationPileLowVsMS> = normWireObject<ChangeFoundationPileLowVsMS>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileLowVsMS")), id: normWireRequired(normWireString), newPileLowerVsMS: normWireRequired(normWireNumber) });

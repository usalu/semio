/** 🏠 `change-roof-multi-span` wire twin: the leaf payload `ChangeRoofMultiSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofMultiSpan {
  id: string;
  newMultiSpan: boolean;
}

export const parseChangeRoofMultiSpan: NormWireReader<ChangeRoofMultiSpan> = normWireObject<ChangeRoofMultiSpan>({ id: normWireRequired(normWireString), newMultiSpan: normWireRequired(normWireBoolean) });

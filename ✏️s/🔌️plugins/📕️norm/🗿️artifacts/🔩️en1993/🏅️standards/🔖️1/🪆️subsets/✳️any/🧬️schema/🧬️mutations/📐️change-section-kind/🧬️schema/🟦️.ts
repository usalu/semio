/** 📐️ `change-section-kind` wire twin: the leaf payload `ChangeSectionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SectionKind, parseSectionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSectionKind {
  id: string;
  newKind: SectionKind;
}

export const parseChangeSectionKind: NormWireReader<ChangeSectionKind> = normWireObject<ChangeSectionKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseSectionKind) });

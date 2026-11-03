/** 🔤️ `change-case-text` wire twin: the leaf payload `ChangeCaseText`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNullable, normWireObject, normWireOptional, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseText {
  caseId: string;
  key: string;
  value?: string | null;
}

export const parseChangeCaseText: NormWireReader<ChangeCaseText> = normWireObject<ChangeCaseText>({ caseId: normWireRequired(normWireString), key: normWireRequired(normWireString), value: normWireOptional(normWireNullable(normWireString)) });

/** 🔢️ `change-case-number` wire twin: the leaf payload `ChangeCaseNumber`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseNumber {
  caseId: string;
  key: string;
  value?: number | null;
}

export const parseChangeCaseNumber: NormWireReader<ChangeCaseNumber> = normWireObject<ChangeCaseNumber>({ caseId: normWireRequired(normWireString), key: normWireRequired(normWireString), value: normWireOptional(normWireNullable(normWireNumber)) });

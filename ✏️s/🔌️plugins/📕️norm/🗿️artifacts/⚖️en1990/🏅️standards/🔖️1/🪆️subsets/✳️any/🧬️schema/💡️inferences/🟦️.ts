/** 💡️ `En1990Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireNumber, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1990Inference {
  outline: En1990Outline;
}

export interface En1990Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
  checkCount: number;
  passCount: number;
  allPass: boolean;
  governingClause: string;
  governingUtilization: number;
}

export const parseEn1990Inference: NormWireReader<En1990Inference> = normWireObject<En1990Inference>({ outline: normWireRequired(normWireRef(() => parseEn1990Outline)) });
export const parseEn1990Outline: NormWireReader<En1990Outline> = normWireObject<En1990Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger), checkCount: normWireRequired(normWireInteger), passCount: normWireRequired(normWireInteger), allPass: normWireRequired(normWireBoolean), governingClause: normWireRequired(normWireString), governingUtilization: normWireRequired(normWireNumber) });

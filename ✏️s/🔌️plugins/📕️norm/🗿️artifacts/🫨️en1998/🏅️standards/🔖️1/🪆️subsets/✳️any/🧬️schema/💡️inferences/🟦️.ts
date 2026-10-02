/** 💡️ `En1998Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1998Inference {
  outline: En1998Outline;
}

export interface En1998Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1998Inference: NormWireReader<En1998Inference> = normWireObject<En1998Inference>({ outline: normWireRequired(normWireRef(() => parseEn1998Outline)) });
export const parseEn1998Outline: NormWireReader<En1998Outline> = normWireObject<En1998Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });

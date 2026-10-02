/** 🏭️ `change-manufacturer-file` wire twin: the leaf payload `ChangeManufacturerFile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ManufacturerFile, parseManufacturerFile } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeManufacturerFile {
  newManufacturerFile: ManufacturerFile;
}

export const parseChangeManufacturerFile: NormWireReader<ChangeManufacturerFile> = normWireObject<ChangeManufacturerFile>({ newManufacturerFile: normWireRequired(parseManufacturerFile) });

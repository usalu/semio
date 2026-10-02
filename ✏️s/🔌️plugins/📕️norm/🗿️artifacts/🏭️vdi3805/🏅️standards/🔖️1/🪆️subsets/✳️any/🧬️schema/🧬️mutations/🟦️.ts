/** 🧺️ `Vdi3805Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseRemoveGeometryConnection, type RemoveGeometryConnection } from "./✂️remove-geometry-connection/🧬️schema/🟦️.ts";
import { type ChangeProductConfiguration, parseChangeProductConfiguration } from "./🎛️change-product-configuration/🧬️schema/🟦️.ts";
import { type ChangeManufacturerFile, parseChangeManufacturerFile } from "./🏭️change-manufacturer-file/🧬️schema/🟦️.ts";
import { parseRenameProduct, type RenameProduct } from "./🏷️rename-product/🧬️schema/🟦️.ts";
import { type ChangeCorrectionAsOf, parseChangeCorrectionAsOf } from "./📅️change-correction-as-of/🧬️schema/🟦️.ts";
import { type AddCurve, parseAddCurve } from "./📈️add-curve/🧬️schema/🟦️.ts";
import { parseRemoveCurve, type RemoveCurve } from "./📉️remove-curve/🧬️schema/🟦️.ts";
import { type ChangeCurvePoints, parseChangeCurvePoints } from "./📍️change-curve-points/🧬️schema/🟦️.ts";
import { parseResizeGeometry, type ResizeGeometry } from "./📐️resize-geometry/🧬️schema/🟦️.ts";
import { type AddProduct, parseAddProduct } from "./📦️add-product/🧬️schema/🟦️.ts";
import { type AddGeometryConnection, parseAddGeometryConnection } from "./🔌️add-geometry-connection/🧬️schema/🟦️.ts";
import { type ChangeStrictMode, parseChangeStrictMode } from "./🔒️change-strict-mode/🧬️schema/🟦️.ts";
import { type ChangeEditionProfile, parseChangeEditionProfile } from "./🔖️change-edition-profile/🧬️schema/🟦️.ts";
import { parseRemoveProduct, type RemoveProduct } from "./🗑️remove-product/🧬️schema/🟦️.ts";
import { type AddGeometry, parseAddGeometry } from "./🧊️add-geometry/🧬️schema/🟦️.ts";
import { type ChangeGeometryParameters, parseChangeGeometryParameters } from "./🧮️change-geometry-parameters/🧬️schema/🟦️.ts";
import { parseRemoveEditionProfile, type RemoveEditionProfile } from "./🧹️remove-edition-profile/🧬️schema/🟦️.ts";
import { type ChangeLimits, parseChangeLimits } from "./🚧️change-limits/🧬️schema/🟦️.ts";
import { parseRemoveGeometry, type RemoveGeometry } from "./🚮️remove-geometry/🧬️schema/🟦️.ts";

export type Vdi3805Mutation =
  | { ChangeManufacturerFile: ChangeManufacturerFile }
  | { ChangeLimits: ChangeLimits }
  | { ChangeCorrectionAsOf: ChangeCorrectionAsOf }
  | { ChangeStrictMode: ChangeStrictMode }
  | { ChangeEditionProfile: ChangeEditionProfile }
  | { RemoveEditionProfile: RemoveEditionProfile }
  | { AddProduct: AddProduct }
  | { RemoveProduct: RemoveProduct }
  | { RenameProduct: RenameProduct }
  | { ChangeProductConfiguration: ChangeProductConfiguration }
  | { AddGeometry: AddGeometry }
  | { RemoveGeometry: RemoveGeometry }
  | { ResizeGeometry: ResizeGeometry }
  | { AddGeometryConnection: AddGeometryConnection }
  | { RemoveGeometryConnection: RemoveGeometryConnection }
  | { ChangeGeometryParameters: ChangeGeometryParameters }
  | { AddCurve: AddCurve }
  | { RemoveCurve: RemoveCurve }
  | { ChangeCurvePoints: ChangeCurvePoints };

export const parseVdi3805Mutation: NormWireReader<Vdi3805Mutation> = normWireExternal<Vdi3805Mutation>({
  ChangeManufacturerFile: parseChangeManufacturerFile,
  ChangeLimits: parseChangeLimits,
  ChangeCorrectionAsOf: parseChangeCorrectionAsOf,
  ChangeStrictMode: parseChangeStrictMode,
  ChangeEditionProfile: parseChangeEditionProfile,
  RemoveEditionProfile: parseRemoveEditionProfile,
  AddProduct: parseAddProduct,
  RemoveProduct: parseRemoveProduct,
  RenameProduct: parseRenameProduct,
  ChangeProductConfiguration: parseChangeProductConfiguration,
  AddGeometry: parseAddGeometry,
  RemoveGeometry: parseRemoveGeometry,
  ResizeGeometry: parseResizeGeometry,
  AddGeometryConnection: parseAddGeometryConnection,
  RemoveGeometryConnection: parseRemoveGeometryConnection,
  ChangeGeometryParameters: parseChangeGeometryParameters,
  AddCurve: parseAddCurve,
  RemoveCurve: parseRemoveCurve,
  ChangeCurvePoints: parseChangeCurvePoints,
});

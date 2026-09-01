/** 🧬️ Din4108Mutation — mirrors `Din4108Mutation` in `🦀️component.rs` (22 variants: one
 * `change-<field>` leaf per document-root scalar, plus `insert-layer`/`remove-layer`/
 * `reorder-layers`/`change-layer-thickness`/`change-layer-lambda` over the id-less, index-addressed
 * `layers` construction build-up). `Din4108Mutation` carries only `#[derive(dsl::Mutations)]` — no
 * `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by every committed
 * `🧪️tests/*​/🦠️mutation/🔣️component.json` fixture (e.g. `{"ChangeMoistureMuInterior":
 * {"new_moisture_mu_interior":2.5}}`). None of the 22 leaf structs carry
 * `#[serde(rename_all = ...)]`, so every leaf's own field names are the literal Rust snake_case
 * names verbatim. `LayerDocument` itself DOES carry `#[serde(rename_all = "camelCase")]`, so its
 * own fields stay camelCase where it is embedded as a payload value. `ClimateZoneDe` (imported from
 * `📕️norm/📄️artifact/🦀️component.rs`) has no `#[serde(rename_all)]` either, so it serializes as the
 * literal PascalCase Rust variant name (`"Zone1"`…`"Zone4"`) — its `#[dsl(key = "zone1")]`
 * annotations are a DSL-engine binding key, not a serde rename, and don't affect the JSON wire form
 * (confirmed by the `change-climate` fixture: `{"ChangeClimate":{"new_climate":"Zone4"}}`). */

export type ClimateZoneDe = "Zone1" | "Zone2" | "Zone3" | "Zone4";

export interface LayerDocument {
  thicknessM: number;
  lambdaWMk: number;
}

export interface ChangeCategory {
  new_category: string;
}

export interface ChangeClimate {
  new_climate: ClimateZoneDe;
}

export interface ChangeAirtightnessN50 {
  new_airtightness_n50: number;
}

export interface ChangePsiTimesLSum {
  new_psi_times_l_sum: number;
}

export interface ChangeRhInt {
  new_rh_int: number;
}

export interface ChangeCatalogId {
  new_catalog_id: string;
}

export interface ChangeMaterialId {
  new_material_id: string;
}

export interface ChangeAirtightnessClass {
  new_airtightness_class: string;
}

export interface ChangeTIntC {
  new_t_int_c: number;
}

export interface ChangeSolarAbsorptance {
  new_solar_absorptance: number;
}

export interface ChangeIrradianceWM2 {
  new_irradiance_w_m2: number;
}

export interface ChangeMoistureMuExterior {
  new_moisture_mu_exterior: number;
}

export interface ChangeMoistureMuInterior {
  new_moisture_mu_interior: number;
}

export interface ChangeEnvelopeAreaM2 {
  new_envelope_area_m2: number;
}

export interface ChangeBb2DetailsConform {
  new_bb2_details_conform: boolean;
}

export interface ChangeApplicationType {
  new_application_type: string;
}

export interface ChangeDeclaredApplicationClass {
  new_declared_application_class: string;
}

export interface InsertLayer {
  index: number;
  layer: LayerDocument;
}

export interface RemoveLayer {
  index: number;
}

export interface ReorderLayers {
  from: number;
  to: number;
}

export interface ChangeLayerThickness {
  index: number;
  new_thickness_m: number;
}

export interface ChangeLayerLambda {
  index: number;
  new_lambda_w_mk: number;
}

export type Din4108Mutation =
  | { ChangeCategory: ChangeCategory }
  | { ChangeClimate: ChangeClimate }
  | { ChangeAirtightnessN50: ChangeAirtightnessN50 }
  | { ChangePsiTimesLSum: ChangePsiTimesLSum }
  | { ChangeRhInt: ChangeRhInt }
  | { ChangeCatalogId: ChangeCatalogId }
  | { ChangeMaterialId: ChangeMaterialId }
  | { ChangeAirtightnessClass: ChangeAirtightnessClass }
  | { ChangeTIntC: ChangeTIntC }
  | { ChangeSolarAbsorptance: ChangeSolarAbsorptance }
  | { ChangeIrradianceWM2: ChangeIrradianceWM2 }
  | { ChangeMoistureMuExterior: ChangeMoistureMuExterior }
  | { ChangeMoistureMuInterior: ChangeMoistureMuInterior }
  | { ChangeEnvelopeAreaM2: ChangeEnvelopeAreaM2 }
  | { ChangeBb2DetailsConform: ChangeBb2DetailsConform }
  | { ChangeApplicationType: ChangeApplicationType }
  | { ChangeDeclaredApplicationClass: ChangeDeclaredApplicationClass }
  | { InsertLayer: InsertLayer }
  | { RemoveLayer: RemoveLayer }
  | { ReorderLayers: ReorderLayers }
  | { ChangeLayerThickness: ChangeLayerThickness }
  | { ChangeLayerLambda: ChangeLayerLambda };

/** 🧬️ Din4108 snapshot schema — artifact-lane fields only. */

export interface Din4108Snapshot {
  /** @state artifact */
  category: string;
  /** @state artifact */
  layers: Din4108LayerDocument[];
  /** @state artifact */
  climate: string;
  /** @state artifact */
  airtightnessN50: number;
  /** @state artifact */
  psiTimesLSum: number;
  /** @state artifact */
  rhInt: number;
  /** @state artifact */
  catalogId: string;
  /** @state artifact */
  materialId: string;
  /** @state artifact */
  airtightnessClass: string;
  /** @state artifact */
  tIntC: number;
  /** @state artifact */
  solarAbsorptance: number;
  /** @state artifact */
  irradianceWM2: number;
  /** @state artifact */
  moistureMuExterior: number;
  /** @state artifact */
  moistureMuInterior: number;
  /** @state artifact */
  envelopeAreaM2: number;
  /** @state artifact */
  bb2DetailsConform: boolean;
  /** @state artifact */
  applicationType: string;
  /** @state artifact */
  declaredApplicationClass: string;
}
export interface Din4108LayerDocument { thicknessM: number; lambdaWMk: number; }

/** 🧬️ Din4108 snapshot schema — persistent fields only. */

export interface Din4108Snapshot {
  /** @state persistent */
  category: string;
  /** @state persistent */
  layers: Din4108LayerDocument[];
  /** @state persistent */
  climate: string;
  /** @state persistent */
  airtightnessN50: number;
  /** @state persistent */
  psiTimesLSum: number;
  /** @state persistent */
  rhInt: number;
  /** @state persistent */
  catalogId: string;
  /** @state persistent */
  materialId: string;
  /** @state persistent */
  airtightnessClass: string;
  /** @state persistent */
  tIntC: number;
  /** @state persistent */
  solarAbsorptance: number;
  /** @state persistent */
  irradianceWM2: number;
  /** @state persistent */
  moistureMuExterior: number;
  /** @state persistent */
  moistureMuInterior: number;
  /** @state persistent */
  envelopeAreaM2: number;
  /** @state persistent */
  bb2DetailsConform: boolean;
  /** @state persistent */
  applicationType: string;
  /** @state persistent */
  declaredApplicationClass: string;
}
export interface Din4108LayerDocument { thicknessM: number; lambdaWMk: number; }

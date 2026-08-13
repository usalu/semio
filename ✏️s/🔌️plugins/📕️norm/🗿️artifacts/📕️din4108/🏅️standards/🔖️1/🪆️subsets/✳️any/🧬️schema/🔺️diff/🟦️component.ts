/** 🧬️ Din4108 diff schema — sparse field delta. */

export interface Din4108Diff {
  /** @state artifact */
  artifact?: Din4108Artifact;
  /** @state artifact */
  category?: string;
  /** @state artifact */
  layers?: Din4108LayerList;
  /** @state artifact */
  climate?: string;
  /** @state artifact */
  airtightnessN50?: number;
  /** @state artifact */
  psiTimesLSum?: number;
  /** @state artifact */
  rhInt?: number;
  /** @state artifact */
  catalogId?: string;
  /** @state artifact */
  materialId?: string;
  /** @state artifact */
  airtightnessClass?: string;
  /** @state artifact */
  tIntC?: number;
  /** @state artifact */
  solarAbsorptance?: number;
  /** @state artifact */
  irradianceWM2?: number;
  /** @state artifact */
  moistureMuExterior?: number;
  /** @state artifact */
  moistureMuInterior?: number;
  /** @state artifact */
  envelopeAreaM2?: number;
  /** @state artifact */
  bb2DetailsConform?: boolean;
  /** @state artifact */
  applicationType?: string;
  /** @state artifact */
  declaredApplicationClass?: string;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface Din4108Artifact {
  category: string;
  layers: Din4108LayerDocument[];
  climate: string;
  airtightnessN50: number;
  psiTimesLSum: number;
  rhInt: number;
  catalogId: string;
  materialId: string;
  airtightnessClass: string;
  tIntC: number;
  solarAbsorptance: number;
  irradianceWM2: number;
  moistureMuExterior: number;
  moistureMuInterior: number;
  envelopeAreaM2: number;
  bb2DetailsConform: boolean;
  applicationType: string;
  declaredApplicationClass: string;
  selectedCheckIndex?: number | null;
}
export interface Din4108LayerDocument { thicknessM: number; lambdaWMk: number; }

export interface Din4108LayerList { values: Din4108LayerDocument[]; }

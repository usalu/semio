/** 🧬️ Din4108 diff schema — sparse field delta. */

export interface Din4108Diff {
  /** @state persistent */
  artifact?: Din4108Artifact;
  /** @state persistent */
  category?: string;
  /** @state persistent */
  layers?: Din4108StringList;
  /** @state persistent */
  climate?: string;
  /** @state persistent */
  airtightnessN50?: number;
  /** @state persistent */
  psiTimesLSum?: number;
  /** @state persistent */
  rhInt?: number;
  /** @state persistent */
  catalogId?: string;
  /** @state persistent */
  materialId?: string;
  /** @state persistent */
  airtightnessClass?: string;
  /** @state persistent */
  tIntC?: number;
  /** @state persistent */
  solarAbsorptance?: number;
  /** @state persistent */
  irradianceWM2?: number;
  /** @state persistent */
  moistureMuExterior?: number;
  /** @state persistent */
  moistureMuInterior?: number;
  /** @state persistent */
  envelopeAreaM2?: number;
  /** @state persistent */
  bb2DetailsConform?: boolean;
  /** @state persistent */
  applicationType?: string;
  /** @state persistent */
  declaredApplicationClass?: string;
  /** @state shared-ui */
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

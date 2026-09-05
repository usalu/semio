/** 🧬️ En1995 document mutations — discriminated union mirroring `En1995Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeMEdKnm {
  newMEdKnm: number;
}

export interface ChangeNEdKn {
  newNEdKn: number;
}

export interface ChangeVEdKn {
  newVEdKn: number;
}

export interface ChangeWMm3 {
  newWMm3: number;
}

export interface ChangeAMm2 {
  newAMm2: number;
}

export interface ChangeBMm {
  newBMm: number;
}

export interface ChangeHMm {
  newHMm: number;
}

export interface ChangeFMK {
  newFMK: number;
}

export interface ChangeFC0K {
  newFC0K: number;
}

export interface ChangeServiceClass {
  newServiceClass: string;
}

export interface ChangeLoadDuration {
  newLoadDuration: string;
}

export interface ChangeMCritKnm {
  newMCritKnm: number;
}

export interface ChangeFEdKn {
  newFEdKn: number;
}

export interface ChangeAEfMm2 {
  newAEfMm2: number;
}

export interface ChangeFVK {
  newFVK: number;
}

export interface ChangeFireDurationMin {
  newFireDurationMin: number;
}

export interface ChangeSectionDepthMm {
  newSectionDepthMm: number;
}

export interface ChangeAVertMS2 {
  newAVertMS2: number;
}

export interface ChangeNCyclesBridge {
  newNCyclesBridge: number;
}

export type En1995Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeMEdKnm: ChangeMEdKnm }
  | { ChangeNEdKn: ChangeNEdKn }
  | { ChangeVEdKn: ChangeVEdKn }
  | { ChangeWMm3: ChangeWMm3 }
  | { ChangeAMm2: ChangeAMm2 }
  | { ChangeBMm: ChangeBMm }
  | { ChangeHMm: ChangeHMm }
  | { ChangeFMK: ChangeFMK }
  | { ChangeFC0K: ChangeFC0K }
  | { ChangeServiceClass: ChangeServiceClass }
  | { ChangeLoadDuration: ChangeLoadDuration }
  | { ChangeMCritKnm: ChangeMCritKnm }
  | { ChangeFEdKn: ChangeFEdKn }
  | { ChangeAEfMm2: ChangeAEfMm2 }
  | { ChangeFVK: ChangeFVK }
  | { ChangeFireDurationMin: ChangeFireDurationMin }
  | { ChangeSectionDepthMm: ChangeSectionDepthMm }
  | { ChangeAVertMS2: ChangeAVertMS2 }
  | { ChangeNCyclesBridge: ChangeNCyclesBridge };

export type { Din16798Snapshot, Din16798Zone, Din16798VentSystem } from "./📸️snapshot/🟦️.ts";

import type { Din16798Zone, Din16798VentSystem } from "./📸️snapshot/🟦️.ts";

export type Din16798Artifact = {
  annex: string;
  thetaRmC: number;
  outdoorCo2Ppm: number;
  zones: Din16798Zone[];
  ventSystems: Din16798VentSystem[];
  envelopeN50HInv: number;
  envelopeVolumeM3: number;
  cellarAreaM2: number;
  cellarVentilationM3H: number;
  nightSetbackK: number;
};

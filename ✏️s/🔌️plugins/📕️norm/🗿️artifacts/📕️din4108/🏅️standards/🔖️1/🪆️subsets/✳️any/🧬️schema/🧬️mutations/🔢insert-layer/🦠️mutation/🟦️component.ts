/** mutation payload — mirrors `InsertLayer`. */
export interface InsertLayer {
  index: number;
  layer: { thicknessM: number; lambdaWMk: number };
}

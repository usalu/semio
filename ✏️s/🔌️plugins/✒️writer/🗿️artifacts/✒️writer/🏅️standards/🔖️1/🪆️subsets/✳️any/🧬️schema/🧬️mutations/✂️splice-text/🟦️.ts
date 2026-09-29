/** ✂️ Direct `splice-text` payload — one range-text operation (`semio.ui.scene.text-splice.v1`). */
export interface SpliceText {
  start: number;
  deleted: string;
  insert: string;
  before: string;
  after: string;
}

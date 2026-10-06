/** 🪟️ Browser attachment and presentation belong to one concrete Canvas owner. */
export interface CanvasSessionPortV1 {
  attachCanvas(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
}

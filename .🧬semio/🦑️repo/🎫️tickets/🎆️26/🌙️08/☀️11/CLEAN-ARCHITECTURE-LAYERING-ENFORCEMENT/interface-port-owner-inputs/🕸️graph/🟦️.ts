import type {CanvasSessionPortV1} from "CANVAS_PORT";

/** 🕸️ Required capabilities of the actual retained Graph browser session. */
export interface GraphWasmSession extends CanvasSessionPortV1 {
  synchronizeScene(bytes: Uint8Array, format: 0 | 1, maximumInputBytes: number, maximumOwnedBytes: number, progress: (completed: number, total: number, ownedBytes: number) => boolean): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerCancelScreen(): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectionPreviewMethod(): string;
  selectedNodeIdsJson(): string;
  hoveredNodeId(): string | null | undefined;
  hoveredChannelJson(): string;
  viewport(): unknown;
  takePendingOpenInstanceId(): string | null | undefined;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  entityScreenJson(domain: string, id: string): string;
  syncInteraction(selectedIdsJson: string, hoveredId?: string | null): void;
  alignSelection(mode: string): void;
  takeGraphEditsJson(): string;
  setCanvasThemeJson(json: string): void;
}

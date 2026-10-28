import type {DrawingPlanOracle} from '../../🧪️tests/🟦️.ts';
import type {CanvasPublicationOracle} from '../📤️canvas/🧪️tests/🟦️.ts';
export function canvasPublicationOracle():CanvasPublicationOracle;
export type DrawingCompositionOracle=DrawingPlanOracle & {canvas:DrawingPlanOracle['plan']};
export function drawingExportOracle(root:string):DrawingCompositionOracle;

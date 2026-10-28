import {checkDrawingExport} from './🟦️.ts';import {drawingExportOracle,canvasPublicationOracle} from './oracle.mjs';
import {checkCanvasPublication} from '../📤️canvas/🧪️tests/🟦️.ts';
/** 🧪️ Executes actual composed Drawing/paragraph source laws using independent font, scene and byte oracles. */
export async function runLayoutDrawingExportTests(root:string):Promise<void>{checkCanvasPublication(canvasPublicationOracle());await checkDrawingExport(root,drawingExportOracle(root));}
if(import.meta.main)await runLayoutDrawingExportTests(process.cwd());

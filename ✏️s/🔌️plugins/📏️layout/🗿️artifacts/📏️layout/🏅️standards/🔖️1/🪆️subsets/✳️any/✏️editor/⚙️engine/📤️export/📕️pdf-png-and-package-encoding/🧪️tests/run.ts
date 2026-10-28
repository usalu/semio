import{checkPackageZip}from'../📦️zip/🧪️tests/🟦️.ts';
import{packageZipOracle}from'../📦️zip/🧪️tests/oracle.mjs';
import {checkExportMilestones} from '../../📍️checkpoint/🧪️tests/🟦️.ts';
import {milestoneOracle} from '../../📍️checkpoint/🧪️tests/oracle.mjs';
import {checkTextFragments,checkOutputTextPublication} from '../../📤️output/📝️text/🧪️tests/🟦️.ts';
import {fragmentOracle} from '../../📤️output/📝️text/🧪️tests/oracle.mjs';
import {checkOutputStorage} from '../../📤️output/🧪️tests/🟦️.ts';
import {outputOracle} from '../../📤️output/🧪️tests/oracle.mjs';
import {checkSvgPaint} from '../../🎨️faithful-primitives/🎨️svg-paint/🧪️tests/🟦️.ts';
import {svgPaintOracle} from '../../🎨️faithful-primitives/🎨️svg-paint/🧪️tests/oracle.mjs';
import {checkPdfProjection} from '../🧭️projection/🧪️tests/🟦️.ts';
import {testPdfRasterPlanes,testPdfIntegrationPlanes} from '../🖼️raster/🧪️tests/🟦️.ts';
import {testPdfUnicodeCmap} from '../🔤️unicode/🧪️tests/🟦️.ts';
import {testPdfSemanticText} from '../📝️text/🧪️tests/🟦️.ts';
import {rasterPlane,unicodeText,markedText,fontFamilyHex} from './oracles.mjs';
import {checkPhysicalBufferWitnesses} from '../../🧪️tests/♻️buffers/🟦️.ts';
import {physicalBufferOracle} from '../../🧪️tests/♻️buffers/oracle.mjs';
/** 🧪️ Executes real raster/Unicode stream helpers through independent file/image readers. */
export async function runLayoutPdfStreamTests(root:string){return{packageZip:checkPackageZip(packageZipOracle()),checkpointMilestones:checkExportMilestones(milestoneOracle()),textFragments:checkTextFragments(fragmentOracle()),outputText:checkOutputTextPublication(fragmentOracle()),outputStorage:checkOutputStorage(outputOracle()),svgPaint:await checkSvgPaint(svgPaintOracle(root)),physicalClose:checkPhysicalBufferWitnesses(physicalBufferOracle),projection:checkPdfProjection(),raster:await testPdfRasterPlanes(rasterPlane),largeImage:await testPdfIntegrationPlanes(rasterPlane),unicode:await testPdfUnicodeCmap(unicodeText),semanticText:await testPdfSemanticText(fontFamilyHex,markedText)};}

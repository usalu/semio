from tt2_edit import *
edit(ANY+'🚪️io/🖼️image/🧪️tests/🟦️.ts',[
 ('import {RasterSceneJob} from "../../../🧬️schema/🎬️scene/📷️raster/🟦️.ts";\n','import {RasterSceneJob} from "../../../🧬️schema/🎬️scene/📷️raster/🟦️.ts";\nimport {parseDrawingImageAsset} from "../../../🧬️schema/🟦️.ts";\n'),
 ('const foreign=PNG.sync.read(Buffer.from(row.png));','const foreign=PNG.sync.read(Buffer.from(row.png)),asset=parseDrawingImageAsset(row.expected);'),
 ('drawingImageDataUri(row.expected,','drawingImageDataUri(asset,',2),
 ('const original=structuredClone(row.expected);','const original=structuredClone(asset);',2),
 ('refused=new DrawingImageEmissionJob(row.expected,4096,8)','refused=new DrawingImageEmissionJob(asset,4096,8)'),
])

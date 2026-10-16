from tt2_edit import *
S=ANY+'🧬️schema/'
edit(S+'🧮️geometry/📷️framing/🟦️.ts',[
 ('import { drawingTextFallbackExtent } from "@semio-tech/framework/2d/text";\n','import { drawingTextFallbackExtent } from "@semio-tech/framework/2d/text";\nimport { binary64Value } from "@semio-tech/framework/value/ieee754";\n'),
 ('  let bounds: [number,number,number,number] | undefined = artboard && artboard.width>0 && artboard.height>0 ? [0,0,artboard.width,artboard.height] : undefined;\n','  const width = artboard ? binary64Value(artboard.width) : 0,height = artboard ? binary64Value(artboard.height) : 0;\n  let bounds: [number,number,number,number] | undefined = width>0 && height>0 ? [0,0,width,height] : undefined;\n'),
])
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(S+'🧮️geometry/📷️framing/🧪️tests/🔬️unit/🟦️.ts',[
 ('import { drawingSceneBounds,type FramingNode } from "../../🟦️.ts";\n','import { parseDrawingArtboard } from "../../../../🟦️.ts";\nimport { drawingSceneBounds } from "../../🟦️.ts";\nimport { drawingWords,framingNodesFixture } from '+FX+';\n'),
 ('  const nodes = item.nodes as FramingNode[];\n  const artboard = "artboard" in item ? item.artboard : undefined;\n  const actual = drawingSceneBounds(artboard,nodes);\n  const box = new Box2();\n  if (artboard) box.set(new Vector2(0,0),new Vector2(artboard.width,artboard.height));','  const nodes = framingNodesFixture(item.nodes);\n  const authored = "artboard" in item ? item.artboard : undefined;\n  const actual = drawingSceneBounds(authored && parseDrawingArtboard(drawingWords(authored)),nodes);\n  const box = new Box2();\n  if (authored) box.set(new Vector2(0,0),new Vector2(authored.width,authored.height));'),
])

from tt2_edit import *
A=ANY+'🧬️schema/🧮️geometry/↗️affine/🟦️.ts'
edit(A,[
 ('import type {DrawingTransform} from "../../🧬️mutations/🟦️.ts";\n','import {binary64,binary64Value} from "@semio-tech/framework/value/ieee754";\nimport type {DrawingTransform as DrawingTransformWords} from "../../🟦️.ts";\nimport type {DrawingTransform} from "../../🧬️mutations/🟦️.ts";\n'),
])
s=open(A,encoding='utf8').read().rstrip('\n')+'''

/** 📥️ Capture a numeric transform as the persisted binary64 words, the single place a computed transform enters the model. */
export function drawingTransformToWords(transform: DrawingTransform): DrawingTransformWords {
  return {x:binary64(transform.x),y:binary64(transform.y),scaleX:binary64(transform.scaleX),scaleY:binary64(transform.scaleY),shear:binary64(transform.shear),rotation:binary64(transform.rotation)};
}

/** 📤️ Derive the numeric transform of a persisted one, the single computational read of its words. */
export function drawingTransformFromWords(words: DrawingTransformWords): DrawingTransform {
  return {x:binary64Value(words.x),y:binary64Value(words.y),scaleX:binary64Value(words.scaleX),scaleY:binary64Value(words.scaleY),shear:binary64Value(words.shear),rotation:binary64Value(words.rotation)};
}
'''
open(A,'w',encoding='utf8').write(s)
B=ANY+'🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/'
edit(B+'📄️document/🟦️.ts',[
 ('import {drawingMatrixToTransform,drawingTransformToMatrix} from','import {drawingMatrixToTransform,drawingTransformToMatrix,drawingTransformToWords} from'),
 ('blendMode:blendMode(style["mix-blend-mode"]),transform,attributes:','blendMode:blendMode(style["mix-blend-mode"]),transform:drawingTransformToWords(transform),attributes:'),
])

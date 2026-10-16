from tt2_edit import *
S=ANY+'🧬️schema/'
edit(S+'🧬️mutations/🧪️tests/🔬️kinds-catalog/🟦️.ts',[
 ('expect(parseDrawingLayerNode(root)).toEqual(root);','expect<unknown>(parseDrawingLayerNode(root)).toEqual(root);',2),
 ('expect(parseDrawingLayerPatch({blendMode:patch.value}).blendMode).toBe(patch.value);','expect<unknown>(parseDrawingLayerPatch({blendMode:patch.value}).blendMode).toBe(patch.value);'),
 ('expect(blendDiff(mutation as never).layers.modified[0]!.patch.blendMode).toBe(patch.value);','expect<unknown>(blendDiff(mutation as never).layers.modified[0]!.patch.blendMode).toBe(patch.value);'),
])

from tt2_edit import *
S=ANY+'🧬️schema/🎬️scene/'
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(S+'📍️placement/🧪️tests/🔬️unit/🟦️.ts',[
 ('rootVisible:index=>row.selected.includes(layers[index]!.id)}','rootVisible:index=>new Set<string>(row.selected).has(layers[index]!.id)}'),
 ('expect(output.basis).toBe(row.expected.basis);expect(output.bounds).toEqual(row.expected.bounds);','expect<unknown>(output.basis).toBe(row.expected.basis);expect<unknown>(output.bounds).toEqual(row.expected.bounds);'),
 ('if(previous)expect(output).toEqual(previous);','if(previous)expect<unknown>(output).toEqual(previous);'),
])
edit(S+'🪪️identity/🧪️tests/🔬️unit/🟦️.ts',[
 ('expect(status,row.name).toBe(row.status);','expect<unknown>(status,row.name).toBe(row.status);'),
])

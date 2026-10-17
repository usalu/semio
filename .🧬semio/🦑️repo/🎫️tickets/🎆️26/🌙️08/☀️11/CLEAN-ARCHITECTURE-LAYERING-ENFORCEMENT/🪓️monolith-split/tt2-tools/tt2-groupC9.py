from tt2_edit import *
edit(ANY+'🧬️schema/🧮️geometry/✏️editing/🧪️tests/🔬️unit/🟦️.ts',[
 ('import type { PathGeometrySegment } from "../../../../🟦️.ts";\n','import type { PathGeometrySegment } from "../../../../🟦️.ts";\nimport { admitted } from "../../../../../🧪️tests/🧫️fixtures/🟦️.ts";\n'),
 ('test("multi-point translation is atomic, order-independent and agrees with Three vectors", () => {\n  const validate=new Ajv({strict:true}).compile(schema);','test("multi-point translation is atomic, order-independent and agrees with Three vectors", () => {\n  const validate=new Ajv({strict:true}).compile<PathEdit>(schema);'),
 ('expect(editPath(actual,{...row.operation,delta:row.operation.delta.map(value=>-value)} as PathEdit)).toEqual(source);','expect(editPath(actual,admitted(validate,{...row.operation,delta:row.operation.delta.map(value=>-value)},"path edit"))).toEqual(source);'),
 ('expect(editPath(coincident.before as PathGeometrySegment[],coincident.operation as PathEdit)).toEqual(','expect<unknown>(editPath(coincident.before as PathGeometrySegment[],coincident.operation as PathEdit)).toEqual('),
 ('expect(dragPathPoint(segment,point,sample.matrix as Matrix,sample.start as Point,sample.start as Point,false)).toEqual(local);','expect<unknown>(dragPathPoint(segment,point,sample.matrix as Matrix,sample.start as Point,sample.start as Point,false)).toEqual(local);'),
 ('    expect(preview).toEqual(row.after);\n','    expect<unknown>(preview).toEqual(row.after);\n'),
 ('expect(result?.point??null).toBe(row.point);','expect<unknown>(result?.point??null).toBe(row.point);'),
 ('expect(result?.point??null).toBe(oracle?.point??null);','expect<unknown>(result?.point??null).toBe(oracle?.point??null);'),
 ('const actual=editPath(source,operation);\n    expect(actual).toEqual(row.after);','const actual=editPath(source,operation);\n    expect<unknown>(actual).toEqual(row.after);'),
 ('expect(actual).toEqual(row.after);expect(source).toEqual(row.segments);','expect<unknown>(actual).toEqual(row.after);expect<unknown>(source).toEqual(row.segments);'),
])

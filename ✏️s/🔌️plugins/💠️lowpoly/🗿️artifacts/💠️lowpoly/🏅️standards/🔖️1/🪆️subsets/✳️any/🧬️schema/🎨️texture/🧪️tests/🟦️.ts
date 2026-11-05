import{test,expect}from"bun:test";
import Ajv from"ajv";
import{validatePaintPixels,PaintTextureError,PAINT_TEXTURE_BYTES}from"../🟦️.ts";
import schema from"../🔣️.json";
import documentSchema from"../../🔣️.json";
import fixture from"./🔣️.json";
test("canonical document paint extent refuses every malformed nonempty body without crop pad or replacement",()=>{
 const projection=(documentSchema as any).definitions?.LowpolyPaintLayer??(documentSchema as any).$defs.LowpolyPaintLayer;const validateWire=new Ajv({strict:true}).compile({type:"string",anyOf:projection.properties.pixels.anyOf});
 const validate=new Ajv({strict:true}).compile(schema);expect(PAINT_TEXTURE_BYTES).toBe(schema.definitions.Layout.const.explicitBytes);
 for(const row of fixture.cases){
  const bytes=new Uint8Array(row.bytes).fill(17);const before=Buffer.from(bytes).toString('base64');const independent=Buffer.from(before,'base64');expect(independent.byteLength).toBe(row.bytes);expect(validate(Array.from(independent))).toBe(row.valid);expect(validateWire(before)).toBe(row.valid);
  if(row.valid)expect(validatePaintPixels(bytes)).toBeUndefined();else{let failure:unknown;try{validatePaintPixels(bytes);}catch(error){failure=error;}expect(failure).toBeInstanceOf(PaintTextureError);expect((failure as PaintTextureError).code).toBe(fixture.refusal);expect((failure as PaintTextureError).actualBytes).toBe(row.bytes);}
  expect(Buffer.from(bytes).toString('base64')).toBe(before);
 }
});

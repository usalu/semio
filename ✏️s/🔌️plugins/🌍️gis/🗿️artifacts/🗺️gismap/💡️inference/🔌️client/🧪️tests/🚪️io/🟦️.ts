/** 🚪️ GIS service declarations preserve the installed protocol through the framework's explicit typed and wire IO. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {admitDocumentServiceDeclarationV1} from "@semio-tech/framework/inference/service";
import {admitDocumentServiceWireDeclarationV1,encodeDocumentServiceWireDeclarationV1} from "@semio-tech/framework/inference/service/io";
import {gisMapDocumentServiceDeclarationV1} from "../../🟦️.ts";
import {gisMapDocumentServiceWireDeclarationV1} from "../../🚪️io/🟦️.ts";

test("GIS native declarations preserve the installed service protocol through explicit IO",()=>{
  const typed=gisMapDocumentServiceDeclarationV1(),wire=gisMapDocumentServiceWireDeclarationV1();
  expect(typed.operations.length).toBe(6);expect(typeof typed.operations[0]!.inputSchema).toBe("object");expect(typeof wire.operations[0]!.inputSchema).toBe("string");
  expect(admitDocumentServiceWireDeclarationV1("gis",wire)).toEqual(typed);expect(encodeDocumentServiceWireDeclarationV1(typed)).toEqual(wire);
  expect(()=>admitDocumentServiceDeclarationV1("gis",wire)).toThrow();expect(()=>admitDocumentServiceWireDeclarationV1("gis",typed)).toThrow();
  for(const operation of wire.operations){const schema=JSON.parse(operation.inputSchema);expect(schema.$id.startsWith("gis:")).toBe(true);new Ajv({strict:true}).compile(schema);}
});

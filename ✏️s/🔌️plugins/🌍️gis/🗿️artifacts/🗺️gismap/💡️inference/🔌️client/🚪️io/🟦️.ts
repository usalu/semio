import {gisMapDocumentServiceDeclarationV1} from "../🟦️.ts";
import {encodeDocumentServiceWireDeclarationV1} from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🚪️io/🟦️.ts";

/** 📤 Emits GIS's typed declaration into the actual native DocumentHttpPort topic grammar. */
export function gisMapDocumentServiceWireDeclarationV1() {
  return encodeDocumentServiceWireDeclarationV1(gisMapDocumentServiceDeclarationV1());
}

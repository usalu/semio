import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import texts from "./🔣️.json" with {type:"json"};

/** 📖️ The texts of the broken crates board and its sweep in English and German. */
export const BROKEN_TEXTS_V1=bindCatalogV1("broken",texts);

/** 🔑️ Every message key of this catalogue. */
export type BrokenTextKeyV1=keyof typeof texts.messages;

/** 🔤️ Renders one message in the chosen language. */
export const brokenTextV1=(language:"en"|"de",key:BrokenTextKeyV1,values:TextValuesV1={}):string=>textV1(BROKEN_TEXTS_V1.catalog,key,language,values);

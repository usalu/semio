import{parseFormsDiff,type FormsDiff}from"../../../../🧬️schema/🔺️diff/🟦️.ts";
import {decodeDocument} from "../../📸️snapshot/🔣️json/🟦️.ts";

/** 🔺️ Decode a sparse Forms JSON transport delta. */
export function parseFormsJsonDiff(value:unknown):FormsDiff{return parseFormsDiff(decodeDocument(value));}

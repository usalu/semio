import {strictEqual,deepStrictEqual,throws} from 'node:assert/strict';
import corpus from './cases.json' with {type:'json'};
import {PDF_LAYOUT_TEXT_PREFIX,pdfFontFamilyScalar,pdfTextProperties} from '../🟦️.ts';
import {PDF_UNICODE_PREFIX,PDF_UNICODE_SUFFIX,PDF_ENCODING_CMAP,pdfUnicodeRange,pdfUnicodeScalar} from '../../🔤️unicode/🟦️.ts';
/** 🏷️ Checks authored properties against an independent encoder and a real marked-content PDF reader. */
export async function testPdfSemanticText(fontHex:(family:string)=>string,reader:(unicode:string,encoding:string,encoded:string,scope:string)=>Promise<{text:string;tags:string[]}>):Promise<{cases:number;refusals:number}> {
  const cmap=PDF_UNICODE_PREFIX+Array.from({length:1026},(_,at)=>pdfUnicodeRange(at)).join('')+PDF_UNICODE_SUFFIX;
  for(const vector of corpus.cases){
    const hex=[...vector.family].map(pdfFontFamilyScalar).join('');strictEqual(hex,vector.hex);strictEqual(hex,fontHex(vector.family));
    const scope=PDF_LAYOUT_TEXT_PREFIX+hex+pdfTextProperties(vector.weight,vector.italic);
    const observed=await reader(cmap,PDF_ENCODING_CMAP,pdfUnicodeScalar('é'),scope);strictEqual(observed.text,'é');deepStrictEqual([...observed.tags],['SemioLayoutText']);
  }
  throws(()=>pdfFontFamilyScalar('\ud800'));throws(()=>pdfFontFamilyScalar('ab'));throws(()=>pdfFontFamilyScalar(''));throws(()=>pdfTextProperties(-1,false));throws(()=>pdfTextProperties(4294967296,false));throws(()=>pdfTextProperties(1.5,false));
  return{cases:corpus.cases.length,refusals:6};
}

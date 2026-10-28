export const PDF_LAYOUT_TEXT_PREFIX = '/SemioLayoutText << /SemioFontFamily <FEFF';
export const PDF_LAYOUT_TEXT_SUFFIX = ' EMC\n';
/** 🔠️ Emits one font-family scalar as bounded UTF-16BE PDF string bytes. */
export function pdfFontFamilyScalar(character: string): string {
  const point = character.codePointAt(0);
  if (point === undefined || character.length !== (point > 0xffff ? 2 : 1) || point >= 0xd800 && point <= 0xdfff) throw new Error('layout-export-unicode-scalar');
  return Array.from({length: character.length}, (_, at) => character.charCodeAt(at).toString(16).padStart(4, '0').toUpperCase()).join('');
}
/** 🏷️ Completes bounded application properties using standard PDF marked content. */
export function pdfTextProperties(weight: number, italic: boolean): string {
  if (!Number.isInteger(weight) || weight < 0 || weight > 0xffffffff || typeof italic !== 'boolean') throw new Error('layout-export-text-properties');
  return '> /SemioFontWeight ' + weight + ' /SemioItalic ' + italic + ' >> BDC\n';
}

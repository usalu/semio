export const PDF_UNICODE_PREFIX = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n/CMapName /LayoutUnicode def\n/CMapType 2 def\n1 begincodespacerange\n<00000000> <0010FFFF>\nendcodespacerange\n";
export const PDF_UNICODE_SUFFIX = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
export const PDF_ENCODING_CMAP = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> def\n/CMapName /LayoutInvisibleUnicode def\n/CMapType 1 def\n/WMode 0 def\n1 begincodespacerange\n<00000000> <0010FFFF>\nendcodespacerange\n1 beginnotdefrange\n<00000000> <0010FFFF> 0\nendnotdefrange\nendcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
/** 🔠️ Emits one bounded UTF-32 to UTF-16BE CMap range, excluding surrogate scalar codes. */
export function pdfUnicodeRange(at: number): string {
  if (!Number.isInteger(at) || at < 0 || at >= 1026) throw new Error('layout-export-unicode-range');
  const first = at === 0 ? 0 : at === 1 ? 0xe000 : 0x10000 + (at - 2) * 1024;
  const last = at === 0 ? 0xd7ff : at === 1 ? 0xffff : first + 1023;
  const target = at === 0 ? '0000' : at === 1 ? 'E000' : ((0xd800 + at - 2) * 65536 + 0xdc00).toString(16).padStart(8, '0').toUpperCase();
  return '1 beginbfrange\n<' + first.toString(16).padStart(8, '0').toUpperCase() + '> <' + last.toString(16).padStart(8, '0').toUpperCase() + '> <' + target + '>\nendbfrange\n';
}
/** 🧷️ Emits one scalar as a fixed-width character code for streamed invisible semantic text. */
export function pdfUnicodeScalar(character: string): string {
  const point = character.codePointAt(0);
  if (point === undefined || character.length !== (point > 0xffff ? 2 : 1) || point >= 0xd800 && point <= 0xdfff) throw new Error('layout-export-unicode-scalar');
  return point.toString(16).padStart(8, '0').toUpperCase();
}

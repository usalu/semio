import vector from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp/🧪️tests/📍️position/🔣️.json" with { type: "json" };
function lspPosition(text: string, byteOffset: number) {
  const prefix = new TextDecoder().decode(new TextEncoder().encode(text).subarray(0, byteOffset));
  const rows = prefix.split(/\r\n|\r|\n/);
  return { line: rows.length - 1, character: rows[rows.length - 1]!.length };
}
let bad = 0;
for (const row of vector.cases.filter(row => row.oracle)) {
  const got = lspPosition(row.text, row.offset);
  const ok = got.line === row.line && got.character === row.character;
  if (!ok) { bad += 1; console.log("MISMATCH", row.name, got, row); }
}
console.log("oracle cases", vector.cases.filter(row => row.oracle).length, "mismatches", bad);

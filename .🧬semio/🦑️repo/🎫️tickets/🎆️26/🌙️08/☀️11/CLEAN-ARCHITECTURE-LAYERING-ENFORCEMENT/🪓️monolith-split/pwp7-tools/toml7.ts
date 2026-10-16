const files = process.argv.slice(2);
let bad = 0;
for (const f of files) {
  try { Bun.TOML.parse(await Bun.file(f).text()); } catch (e) { bad++; console.log('TOML FAIL', f, String(e).slice(0, 200)); }
}
console.log('toml checked', files.length, 'bad', bad);

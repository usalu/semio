// Loads every post-component release module so missing files, aliases and exports surface as import errors.
const root = process.argv[2]!;
const files = process.argv.slice(3);
let failed = 0;
for (const file of files) {
  try { await import(`${root}/${file}`); console.log(`ok   ${file}`); } catch (error) { failed++; console.log(`FAIL ${file}\n  ${String((error as Error).message ?? error).split("\n").slice(0, 4).join("\n  ")}`); }
}
process.exit(failed ? 1 : 0);

const folder = "✏️s/🔌️plugins/🏭️process";
const stale = process.argv.includes("--stale");
const matcher = process.argv.includes("--limits") ? /const PROCESS3D_MODEL_(FILE|DATA_URL|IMPORT)/ :
  stale ? /solid_handle|solidHandle|solid-handle|deep-solid-handle/ : /importModelFile|ImportedSolid\s*\{|solid_handle: String/;
const pattern = stale ? "**/*.{rs,ts,json,semio,py,sql}" : "**/*.rs";
for (const relative of new Bun.Glob(pattern).scanSync({ cwd: folder })) {
  if (!stale && relative.includes("/🧪️tests/")) continue;
  const content = await Bun.file(`${folder}/${relative}`).text();
  const matches = content.split("\n").flatMap((line, index) => matcher.test(line) ? [`${index + 1} ${line.trim()}`] : []);
  if (matches.length) console.log(relative, matches.join("\n"));
}

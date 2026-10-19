const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const [name, cache, ink, outk, lang, n] = Bun.argv.slice(2);
let c=0;
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s" && t.name===name && (!cache||t.cache===cache) && (!ink||t.inputs.kind===ink) && (!outk||t.outputs.kind===outk) && (!lang||t.file.includes(lang)))) { console.log(t.file); if(++c>=Number(n??3)) break; }

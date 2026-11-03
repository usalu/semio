import { fileOf, patchView } from "./patch.ts";
const [cat, slug, band, view, filt] = process.argv.slice(2);
patchView(fileOf(cat!, slug!), band!, view!, (s) => { for (const x of s) if (!filt || JSON.stringify(x).includes(filt)) console.log(JSON.stringify(x)); return s; });

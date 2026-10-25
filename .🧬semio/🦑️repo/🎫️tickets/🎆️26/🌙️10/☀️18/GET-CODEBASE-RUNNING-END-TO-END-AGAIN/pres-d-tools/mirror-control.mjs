const m = await import("/Users/ueli/Documents/semio/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🪞️zukunft-bau-mirror/🟦️.ts");
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public";
const orig = readFileSync(process.argv[2], "utf8");
const d = m.danglingTargets(m.fontFaceUrls(orig), `${root}/🧰️zukunft-bau-entwerfen-mit-bestand_files`, root);
console.log("dangling font urls in ORIGINAL css:", d.length);
const preload = '<link rel="preload" href="https://www.zukunftbau.de/_assets/x/Fonts/OpenSans-Bold.woff2" as="font" crossorigin="" />';
console.log("remote in original preload:", m.remoteSubresources(m.htmlReferences(preload)).length);

import { assetPathsFromFiles } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const l = await assetPathsFromFiles(); console.log(l.length, JSON.stringify(l.filter((e: any) => /bollard|loader|crane|swing|scissor|diesel/.test(JSON.stringify(e)))));

/** 🧾️ Writes the `validateAsset` issues of an asset file as JSON lines (code, path, message, data): bun issues.ts <asset.json> */
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
for (const i of validateAsset(asset)) console.log(JSON.stringify({ code: i.code, path: (i as { path?: string }).path, message: i.message.en, data: (i as { data?: unknown }).data }));

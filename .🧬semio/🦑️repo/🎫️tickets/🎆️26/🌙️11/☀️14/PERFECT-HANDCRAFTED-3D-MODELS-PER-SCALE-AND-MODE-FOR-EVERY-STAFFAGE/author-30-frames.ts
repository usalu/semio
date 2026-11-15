import { writeFileSync } from "node:fs";
import { entryOf, poseState } from "./volumes-lib.ts";
const [spec] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const st = poseState(asset, 50);
writeFileSync(`🗑️generated/author-30/${entry.slug}.frames.json`, JSON.stringify(st.frames));

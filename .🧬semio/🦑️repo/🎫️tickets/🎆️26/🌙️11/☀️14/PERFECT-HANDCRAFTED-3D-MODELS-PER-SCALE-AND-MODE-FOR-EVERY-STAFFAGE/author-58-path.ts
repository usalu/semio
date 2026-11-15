import { entryOf } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
console.log(entry.path);
const f = (asset as any).representations["200"].geometry.front.filter((s: any) => ["wheel-r", "frame-column"].includes(s.id));
console.log(JSON.stringify(f.map((s: any) => [s.id, s.x, s.y, s.w, s.h])));

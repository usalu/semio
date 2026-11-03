import { readFileSync, writeFileSync } from "node:fs";
export const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const file = (slug: string) => `${L}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const load = (slug: string): any => JSON.parse(readFileSync(file(slug), "utf8"));
export const save = (slug: string, doc: any): void => writeFileSync(file(slug), JSON.stringify(doc, null, 2) + "\n");
export const reps = (a: any) => Object.values<any>(a.representations).filter((r) => r.status === "authored");

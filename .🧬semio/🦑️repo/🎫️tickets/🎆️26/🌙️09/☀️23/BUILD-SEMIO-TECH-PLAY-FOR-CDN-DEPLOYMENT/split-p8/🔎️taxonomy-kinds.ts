/** 🔎️ Resolves the semantic directory kind of every segment of the Lane C semio split's new directories, parent by parent. */
import { semanticDirectoryKindId } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const paths = process.argv.slice(2);
let unresolved = 0;
for (const path of paths) {
  let parent: string | undefined;
  const kinds = path.split("/").map((segment) => {
    const kind = semanticDirectoryKindId(segment, undefined, parent ? { parentKindId: parent } : {});
    parent = kind ?? undefined;
    return `${segment}=${kind ?? "?"}`;
  });
  const last = kinds[kinds.length - 1]!;
  if (last.endsWith("=?")) unresolved++;
  console.log(kinds.slice(-3).join(" / "));
}
console.log(`[DEBUG] unresolved-leaves=${unresolved}`);

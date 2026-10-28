// oracle2-g: prints the plan rules of corpus rows (stdin: JSON rows with tree/allowed/ambient/names/refusals) with the staged policy module.
import {planV1} from "./src/🔭️read-oracle/🪢️policy/🟦️.ts";
const rows = JSON.parse(await Bun.stdin.text()) as {id: string; tree: string[]; allowed: string[]; ambient: string[]; names: string[]; refusals: boolean}[];
const listerOf = (paths: readonly string[]) => {
  const directories = new Set(paths.flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
  return (directory: string) => {
    if (directory !== "" && !directories.has(directory)) return undefined;
    const prefix = directory ? `${directory}/` : "";
    return [...new Set(paths.filter((path) => path.startsWith(prefix)).map((path) => path.slice(prefix.length).split("/")[0]!))];
  };
};
console.log(JSON.stringify(Object.fromEntries(rows.map((row) => [row.id, planV1("/w/ws", row, listerOf(row.tree), {refusals: row.refusals}).rules]))));

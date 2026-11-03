import { load } from "./lib.ts"; console.log(load(process.argv[2]!).asset.symbols.map((s: any) => s.id).join(" "));

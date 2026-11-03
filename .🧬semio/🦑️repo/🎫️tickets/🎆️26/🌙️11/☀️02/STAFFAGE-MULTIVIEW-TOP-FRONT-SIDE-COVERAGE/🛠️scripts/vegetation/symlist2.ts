import { load } from "./lib.ts"; console.log(load(process.argv[2]!).asset.symbols.map((s: any) => `${s.id}:${s.denominators.includes(200) ? 200 : s.denominators[0]}`).join(" "));

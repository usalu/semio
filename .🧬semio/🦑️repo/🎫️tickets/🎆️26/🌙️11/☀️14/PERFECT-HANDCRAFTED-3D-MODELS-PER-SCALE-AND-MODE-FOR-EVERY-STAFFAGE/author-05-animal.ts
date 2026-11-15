import { bodyFromShapes, detailsFromShapes, run, stdDecisions, type Cfg, type Rule } from "./author-05-gen.ts";

export const animal = (slug: string, nameEn: string, nameDe: string, rules: Rule[], extra: (c: Parameters<Cfg["body"]>[0]) => void = () => {}, noteEn = "", noteDe = "", emit: Cfg["emitterRegion"] = (em) => (em.kind === "breath" ? "coat-head" : "coat-trunk"), over: Partial<Cfg> = {}): void => {
  const cfg: Cfg = {
    slug,
    rules,
    body: (c) => {
      bodyFromShapes(c, cfg);
      detailsFromShapes(c);
      extra(c);
    },
    decisions: stdDecisions(nameEn, nameDe, noteEn, noteDe),
    emitterRegion: emit,
    ...over,
  };
  run(cfg);
};

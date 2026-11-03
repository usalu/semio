import { fileOf, patchArray } from "./patch.ts";

const sides: Record<string, [string, string]> = { "contact-front": ["contact-front-left", "contact-front-right"], "contact-rear": ["contact-rear-left", "contact-rear-right"] };
const titles: Record<string, Record<string, { en: string; de: string }>> = {
  "contact-off": { "contact-front": { en: "Front feet lift", de: "Vordere Füße heben ab" } },
  "contact-on": { "contact-front": { en: "Front feet touch down", de: "Vordere Füße setzen auf" } },
};
patchArray(fileOf("🛋️furniture", "cafe-chair"), '"clips"', /\n( *)"clips": \[/, (clips) =>
  clips.map((c: any) => ({
    ...c,
    events: (c.events ?? []).flatMap((e: any) => {
      const pair = typeof e.target === "string" ? sides[e.target] : undefined;
      if (pair === undefined || !String(e.kind).startsWith("contact-")) return [e];
      const side = (id: string) => (id.endsWith("left") ? { en: "left", de: "links" } : { en: "right", de: "rechts" });
      return pair.map((id) => ({ ...e, target: id, title: { en: `${titles[e.kind]?.[e.target]?.en ?? e.title.en} (${side(id).en})`, de: `${titles[e.kind]?.[e.target]?.de ?? e.title.de} (${side(id).de})` } }));
    }),
  })),
);

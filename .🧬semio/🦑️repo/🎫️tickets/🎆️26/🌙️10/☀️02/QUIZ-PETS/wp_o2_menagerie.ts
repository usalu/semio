/** 🧪️ Ticket tool (work package O2): the ten species of the work package as a menagerie of their own, so the stories gallery shows exactly them — scene `envelope` (housy, waly, windowy, roofy, insuly), scene `technology` (solary, battery, shady, venty, servy) and scene `home` (all ten in turn).
 *
 * Usage (from `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript`):
 *   PETS_STORIES_PORT=6347 PETS_MENAGERIE=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_menagerie.ts" bun ./📜️script.ts dev
 */
import housy from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json";
import waly from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🧱️waly/🔣️.json";
import windowy from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🪟️windowy/🔣️.json";
import roofy from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🛖️roofy/🔣️.json";
import insuly from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🧶️insuly/🔣️.json";
import solary from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🔆️solary/🔣️.json";
import battery from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🔋️battery/🔣️.json";
import shady from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/😎️shady/🔣️.json";
import venty from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🌬️venty/🔣️.json";
import servy from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🖥️servy/🔣️.json";

const strip = <Document extends { readonly $schema?: string }>(document: Document): Omit<Document, "$schema"> => {
  const { $schema: _, ...rest } = document;
  return rest;
};

export default {
  schema: "semio.pets.menagerie/v1",
  id: "polish",
  title: { en: "Work package O2", de: "Arbeitspaket O2" },
  species: [housy, waly, windowy, roofy, insuly, solary, battery, shady, venty, servy].map(strip),
  bonds: [
    { between: ["housy", "insuly"], affinity: 0.8 },
    { between: ["waly", "windowy"], affinity: -0.5 },
    { between: ["roofy", "housy"], affinity: 0.6 },
    { between: ["solary", "battery"], affinity: 0.8 },
    { between: ["shady", "solary"], affinity: -0.5 },
    { between: ["venty", "servy"], affinity: 0.5 },
  ],
  casts: [
    { scene: "envelope", core: ["housy", "waly", "windowy", "roofy", "insuly"], rotation: [] },
    { scene: "technology", core: ["solary", "battery", "shady", "venty", "servy"], rotation: [] },
    { scene: "home", core: ["housy", "waly", "windowy", "roofy", "insuly", "solary", "battery", "shady", "venty", "servy"], rotation: [] },
  ],
};

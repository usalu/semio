/** 🧪️ Ticket tool: the reference species wrapped into a one-species menagerie, for the stories gallery before a real menagerie exists.
 *
 * Usage (from the repository root): `PETS_MENAGERIE=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/reference_menagerie.ts" bun nx run @semio-tech/pets-react:dev`
 */
import species from "./reference-species.json";

export default {
  schema: "semio.pets.menagerie/v1",
  id: "reference",
  title: { en: "Reference rig", de: "Referenzgerüst" },
  species: [species],
  bonds: [],
  casts: [{ scene: "home", core: [species.id], rotation: [] }],
  chemistry: [],
};

/** 🧮️ Ticket tool of work package P1: per species of the architecture menagerie, how many parts, bones, eyes and emitters it has, how many element groups the depiction builds for its parts and face and how many of them follow a bone that its clips move — the elements a frame can write to.
 *
 * Usage (from the repository root): bun ".../p1_species_shape.ts"
 */
import { ARCHITECTURE_MENAGERIE } from "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts";
import { depict } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts";
import { JSDOM } from "jsdom";

const document = new JSDOM("<!doctype html><html><body></body></html>").window.document;
let total = 0;
for (const species of ARCHITECTURE_MENAGERIE.species) {
  const depiction = depict(species, document);
  const elements = depiction.element.querySelectorAll("*").length + 1;
  const groups = depiction.followers.reduce((sum, list) => sum + list.length, 0);
  const runs = species.parts.reduce((sum, part, index) => sum + (index === 0 || species.parts[index - 1]!.bone !== part.bone ? 1 : 0), 0);
  total += elements;
  process.stdout.write(`${species.id}: parts ${species.parts.length}, bones ${species.bones.length}, eyes ${species.face.eyes.length}, groups ${groups}, runs of one bone ${runs}, elements ${elements}\n`);
}
process.stdout.write(`elements of all species: ${total}\n`);

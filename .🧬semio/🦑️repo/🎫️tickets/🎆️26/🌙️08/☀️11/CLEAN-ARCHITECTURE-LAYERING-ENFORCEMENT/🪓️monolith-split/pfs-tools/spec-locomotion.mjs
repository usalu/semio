import fs from "node:fs";
const T = process.env.T, G = process.env.G;
const file = "🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts";
const orig = fs.readFileSync(G + "/pfs-orig/locomotion.ts.txt", "utf8");
const headerBody = orig.slice(orig.indexOf("/**"), orig.indexOf("*/") + 2).split("\n").slice(1, -1).join("\n");
const see = (list) => list.map(([p, d]) => ` * @see ${p} — ${d}`).join("\n");
const doc = (emoji, text, refs = []) => `/** ${emoji} ${text}\n *\n${see([...refs, ["../🦀️.rs", "the Rust twin"], ["../../🎪️stage/🟦️.ts", "the façade of the stage and the normative order of a tick"]])}\n */`;
const footingDoc = `/** ⚓️ Locomotion, footing: what carries a pet and how it loses its ground — the plan of a course, the puff in which a pet vanishes and arrives anew, dropping a pet whose footing went away and installing a planned course. The module as a whole (this folder; the children sit in the sibling folders \`📐️constants\`, \`🛬️courses\`, \`🛫️departure\`, \`🏃️motion\`, \`✋️hand\`, \`🪜️outings\`, \`🧳️trips\`, \`🪄️mischief\` and \`🚪️leaving\`):\n${headerBody.split("\n").slice(1).join("\n").replace(/@see \.\.\//g, "@see ../../").replace("@see ./🦀️.rs", "@see ../🦀️.rs")}\n */`;
const spec = {
  file,
  label: "locomotion",
  childRoot: "🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion",
  deleteParent: true,
  children: [
    { folder: "📐️constants", doc: doc("📐️", "Locomotion, constants: the waiting times, spans, speeds and shares of the motions, and the bounce and puff durations the rest of the stage reads.", [["../⚓️footing/🟦️.ts", "the footing that uses them first"]]), stmts: ["43-72"] },
    { folder: "⚓️footing", doc: footingDoc, stmts: ["77-229", "642", "828-832", "1589-1607"] },
    { folder: "🛬️courses", doc: doc("🛬️", "Locomotion, courses: how a pet whose ground went away plans its way down — tick by tick, landing included, with the air control of the steerings, the parachute and the one-way platform of another pet's head.", [["../⚓️footing/🟦️.ts", "the plan and the installed course"]]), stmts: ["234-415"] },
    { folder: "🛫️departure", doc: doc("🛫️", "Locomotion, departure: setting out for a goal on the pet's perch, strolling, attending and the hop or glide that launches a pet from one perch to the next.", [["../🛬️courses/🟦️.ts", "the plan of a launch"]]), stmts: ["420-523"] },
    { folder: "🏃️motion", doc: doc("🏃️", "Locomotion, motion: the guarded stride of a walker, the scoot of a scooter, the slip of a rider on a head and the flight and plunge of a pet in the air.", [["../🛬️courses/🟦️.ts", "the ways down"]]), stmts: ["528-641", "650-693"] },
    { folder: "✋️hand", doc: doc("✋️", "Locomotion, hand: a pet lifted into the learner's hand, held, let go, given back and tossed.", [["../🧳️trips/🟦️.ts", "the halt of a trip a lift interrupts"]]), stmts: ["698-812"] },
    { folder: "🪜️outings", doc: doc("🪜️", "Locomotion, outings: every outing a pet with gear can take from a stand — the climb onto a wall, the ladder and the rope, the slide and the rests on the way — and the legs of a trip assembled from them.", [["../⚓️footing/🟦️.ts", "the release of gear"]]), stmts: ["817-826", "835-1328"] },
    { folder: "🧳️trips", doc: doc("🧳️", "Locomotion, trips: setting out on a trip with gear, embarking, travelling leg by leg, alighting, resting, clinging and halting where the actor is.", [["../🪜️outings/🟦️.ts", "the outings a trip is made of"]]), stmts: ["1331-1586"] },
    { folder: "🪄️mischief", doc: doc("🪄️", "Locomotion, mischief: posting a pet to a spot, popping it back, pushing and unpushing it and what is left unlifted.", [["../🧳️trips/🟦️.ts", "the trips a post starts"]]), stmts: ["1612-1704"] },
    { folder: "🚪️leaving", doc: doc("🚪️", "Locomotion, leaving: a pet that leaves the stage after halting wherever it is.", [["../🧳️trips/🟦️.ts", "the halt that comes first"]]), stmts: ["1709-1731"] },
  ],
};
fs.writeFileSync(T + "/spec-locomotion.json", JSON.stringify(spec, null, 1));

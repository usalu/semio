import fs from "node:fs";
const T = process.env.T;
const file = "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts";
const see = (list) => list.map(([p, d]) => ` * @see ${p} — ${d}`).join("\n");
const doc = (emoji, text, refs = []) => `/** ${emoji} ${text}\n *\n${see([...refs, ["../🟦️.ts", "the package entry that publishes this module"]])}\n */`;
const spec = {
  file,
  label: "presentation",
  childRoot: "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript",
  deleteParent: true,
  children: [
    { folder: "🧱️vocabulary", doc: doc("🧱️", "Presentation vocabulary: how strongly a participant is emphasised, the transition into a slide, the participant itself, the metadata of an arrangement, the dispositions that place and style a participant and the language kinds a presentation is written in."), stmts: ["7", "12", "355", "363", "409-456", "876"] },
    { folder: "🎭️embodiments", doc: doc("🎭️", "Presentation embodiments: the text morph roots and every embodiment a participant takes — text, figure, video, pdf, iframe, markdown, json, bullets, authors and affiliations — with the media scroll origin and the morph labels of the affiliation steps.", [["../🧱️vocabulary/🟦️.ts", "emphasis and dispositions"]]), stmts: ["19-350"] },
    { folder: "🔭️scope", doc: doc("🔭️", "Presentation scope: the participants and embodiments an artifact scope holds and the resolution scope built from thoughts.", [["../🎭️embodiments/🟦️.ts", "the embodiments a scope holds"]]), stmts: ["368-393"] },
    { folder: "🎞️structure", doc: doc("🎞️", "Presentation structure: arrangements, slides, thoughts, sequences, chapters and the presentation, and the scope an arrangement resolves in.", [["../🔭️scope/🟦️.ts", "the scopes the structure resolves in"]]), stmts: ["396", "826-871", "879-891"] },
    { folder: "🧩️tiles", doc: doc("🧩️", "Presentation tiles: tiling figures, splitting a figure grid into tiles and the tile play that seeds, moves and resizes tile drafts and builds their morph prompt.", [["../🎭️embodiments/🟦️.ts", "the figure embodiment"], ["../🧱️vocabulary/🟦️.ts", "the dispositions"]]), stmts: ["461-821"] },
    { folder: "📐️resolution", doc: doc("📐️", "Presentation resolution: resolved dispositions with their bounds and centring, the expansion of thoughts into render slides and the resolution of embodiments and arrangements in a scope.", [["../🎞️structure/🟦️.ts", "thoughts, slides and arrangements"], ["../🔭️scope/🟦️.ts", "the resolution scope"]]), stmts: ["898-1159"] },
    { folder: "🔖️bookmarks", doc: doc("🔖️", "Presentation bookmarks: the query parameters and hash of a slide url, the bookmark names of entities and the collection of every slide of a presentation.", [["../📐️resolution/🟦️.ts", "the expansion into slides"]]), stmts: ["1162-1301"] },
    { folder: "📂️slide-files", doc: doc("📂️", "Presentation slide files: the types and file paths of slide and thought files, the assembly of thoughts and chapters from them and the intro specification a thought file may carry.", [["../🎞️structure/🟦️.ts", "the structure assembled"], ["../🔭️scope/🟦️.ts", "the artifact scope merged"]]), stmts: ["1306-1476", "1538"] },
    { folder: "🎬️intro", doc: doc("🎬️", "Presentation intro: the template that turns title, description, goal, authors and affiliations into the slides and thought file of an introduction.", [["../📂️slide-files/🟦️.ts", "the slide files the intro writes"]]), stmts: ["1563-1878"] },
    { folder: "🧭️slide-glob", doc: doc("🧭️", "Presentation slide glob: loading a whole presentation from a glob of slide modules and thought files.", [["../📂️slide-files/🟦️.ts", "the slide files"], ["../🎬️intro/🟦️.ts", "the intro thought files that expand to slide modules"]]), stmts: ["1478-1483"] },
    { folder: "🔄️analogy-and-play", doc: doc("🔄️", "Presentation analogy and play: the analogy template between a source and a target and the definitions of the play apps.", [["../🎞️structure/🟦️.ts", "the structure an analogy builds"]]), stmts: ["1882-2010"] },
  ],
};
fs.writeFileSync(T + "/spec-presentation.json", JSON.stringify(spec, null, 1));

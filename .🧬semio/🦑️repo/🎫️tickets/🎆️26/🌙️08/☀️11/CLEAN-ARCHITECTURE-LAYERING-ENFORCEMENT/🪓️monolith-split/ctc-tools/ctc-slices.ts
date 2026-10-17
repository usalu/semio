// 🗺️ Maps a repo-relative crate or file path to the rollout slice that is carving it right now (null = finished).
const S = "✏️s/🔌️plugins/";
const ST = S + "🗄️stdio/🗿️artifacts/";
const SUB = ST + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/";
const rules: [string, string[]][] = [
  ["r1", ["🗜️deflate", "📰️xml", "🧾️json", "🎒️zip"].map(d => ST + d + "/")],
  ["r3", ["🖼️image", "📑️document", "📦️object", "🧰️kit", "🌊️flow"].map(d => SUB + d + "/")],
  ["r4", ["🎞️animation", "🕸️graph", "📐️cad", "📽️presentation"].map(d => SUB + d + "/").concat([ST + "🎨️svg/", ST + "🧿️semio/📦️packages/"])],
  ["r5a", ["🖋️dxf", "📸️jpg", "🖼️tiff", "📷️png"].map(d => ST + d + "/")],
  ["r5b", [ST + "📖️pdf/"]],
  ["r5c", [ST + "🧊️gltf/"]],
  ["r6", [S + "🀄️wfc/", S + "🌍️gis/"]],
  ["r7", [S + "🔱️trinity/", S + "✒️writer/", S + "🪵️sourcing/", S + "🎬️sequence/"]],
  ["r8b", [S + "📋️forms/", S + "🎞️animate/", S + "🗒️note/", S + "🎥️shooting/"]],
  ["r9b", [S + "🎪️demonstrator/", S + "🏛️architect/", S + "🔋️energy/"]],
  ["r2done", ["🏛️model", "🔺️mesh", "🖊️drawing", "🧊️brep"].map(d => SUB + d + "/")],
];
export const sliceOf = (path: string): string | null => {
  for (const [name, prefixes] of rules) for (const p of prefixes) if (path.startsWith(p)) return name;
  return null;
};

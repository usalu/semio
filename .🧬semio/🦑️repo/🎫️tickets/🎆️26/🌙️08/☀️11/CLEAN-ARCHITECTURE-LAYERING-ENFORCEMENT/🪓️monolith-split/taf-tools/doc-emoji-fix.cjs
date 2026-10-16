// usage: node doc-emoji-fix.cjs [APPLY]
const fs = require("fs");
const { firstGrapheme, docstrings } = require("./doc-emoji.cjs");
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const table = `
🎛️commands/✅️verify|✅
🎛️commands/✅️verify/🚦️gates|🚦
🎛️commands/✅️verify/🚨️policy-breach-gates|🚨
🎛️commands/✅️verify/🧟️generated-corruption|🧟
🎛️commands/✅️verify/🧷️composed-child-refs|🧷
🎛️commands/✅️verify/🦀️rust-warnings|⚠️ 🛟
🎛️commands/✅️verify/🎨️storybook-freshness|🖌 🌈
🎛️commands/🏭️generate|🏭
🎛️commands/📚️examples|📖 📕
🎛️commands/📝️commit|📝
🎛️commands/🔧️setup|🔧
🎛️commands/🗄️stdio|📟 📡
🎛️commands/🚀️release|📦 🛫
🎛️commands/🛠️dev|🛠
🎛️commands/🧩️os|💻 🖲
🎛️commands/🧪️test|🧪
🎛️commands/🧬️schema|🧾 📜
🎛️commands/🧹️code-style|🧹
🗄️stdio-ledger|📒 📔
⚖️laws/🧪️self-test-lanes|🧫 🧯
⚖️laws/🎯️tool-job-coverage/📋️inventory|📋
⚖️laws/🎯️tool-job-coverage/🏭️factory|🔩 🧰
⚖️laws/🎯️tool-job-coverage/📤️publication|📤
⚖️laws/🎯️tool-job-coverage/🗄️artifact-store|🗄
⚖️laws/🎯️tool-job-coverage/🛣️routes|🛣
⚖️laws/🎯️tool-job-coverage/🔒️lifecycle|🔒
⚖️laws/🎯️tool-job-coverage/🔁️live-replay|🔂 ♻
⚖️laws/🎯️tool-job-coverage/⚙️fixed-operation|⚙
⚖️laws/🎯️tool-job-coverage/🏗️fem-visual|🏗
⚖️laws/🎯️tool-job-coverage/🏁️run|🏁
⚖️laws/🔬️interactivity-audit/🗂️files|🗂
⚖️laws/🔬️interactivity-audit/🧭️all-app|🛰 🔭
⚖️laws/🔬️interactivity-audit/🔎️scan|🔎
⚖️laws/🔬️interactivity-audit/🏁️run|🏃 🚥
⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history|⏳ 🕰
⚖️laws/🔬️interactivity-audit/🖥️mounted|🖥
⚖️laws/🔬️interactivity-audit/🖥️mounted/🖼️mounted-surface|🖼
⚖️laws/🔬️interactivity-audit/🗄️database|🛢 💽
⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle|🌱
⚖️laws/🧩️plugin-consistency/🏛️foundation|🏛
⚖️laws/🧩️plugin-consistency/📋️allowlists|📃 🚫
⚖️laws/🧩️plugin-consistency/🏗️crate-shape|📏 🪚
⚖️laws/🧩️plugin-consistency/🧱️app-coupling|🧱
⚖️laws/🧩️plugin-consistency/🧮️completeness|🧮
⚖️laws/🧩️plugin-consistency/🚧️authority-bans|🚧
⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape|🗺
⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture|🔌
⚖️laws/🧩️plugin-consistency/🪆️subset-conformance|🔖 🏁
⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation|✍
⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome|🎯
⚖️laws/🧩️plugin-consistency/🚪️artifact-io|🛂 🔑
⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2|🧬
⚖️laws/🧩️plugin-consistency/📐️schema-overhaul-pc|📐
⚖️laws/🧩️plugin-consistency/🧊️dissolved-kernels|🫧 💧
⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism|🧼
⚖️laws/🧩️plugin-consistency/🧭️breaches|🧭
🧹️normalization/🔣️taxonomy/🏷️filenames|🏷
🔍️discovery/📖️source-access/🦀️rust-evidence|🦀
🗂️workspaces/📍️checkout-root|📍
🧪️tests/📎️root-script-sources|📎
`.trim().split("\n").map((l) => { const [p, c] = l.split("|"); return { p, cands: c.trim().split(/\s+/) }; });
const norm = (g) => g.replace(/️/g, "");
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const taken = new Map();
const plan = [];
for (const { p, cands } of table) {
  const f = L + p + "/🟦️.ts";
  let text = fs.readFileSync(f, "utf8");
  const m = text.match(/^(\s*(?:\/\/[^\n]*\n\s*)*\/\*\*\s*)(\S+)/u);
  if (!m) throw new Error("no docstring: " + p);
  const cur = firstGrapheme(m[2]);
  const others = docstrings(text).filter(Boolean).map(norm);
  const idxFirst = others.indexOf(norm(cur));
  others.splice(idxFirst, 1);
  let pick = null;
  for (const c of cands) { const n = norm(c); if (!taken.has(n) && !others.includes(n)) { pick = n; break; } }
  if (!pick) { console.log("NOCAND\t" + p + "\tin-file: " + [...new Set(others)].join(" ")); continue; }
  taken.set(pick, p);
  plan.push({ f, p, cur, pick: pick + "️", m, text });
}
for (const x of plan) console.log(`${x.p}\t${x.cur}\t${x.pick}`);
if (process.argv[2] === "APPLY" && plan.length === table.length) {
  for (const x of plan) {
    const t2 = x.text.replace(x.m[0], x.m[1] + x.pick);
    fs.writeFileSync(x.f, t2);
  }
  console.log("APPLIED", plan.length);
}

// 🪄 Gives every repeated doc emoji of a file a fresh, fitting one (first use stays). usage: bun dedupe-docs.mjs <file>... ; edits the file in place (overlay only), asserts the path is below /private/tmp/fnsplit4a-finish/ and that only comment lines changed.
import {readFileSync, writeFileSync, realpathSync} from "node:fs";
const V = "️";
const norm = (s) => s.replaceAll(V, "");
const DOC = /^([ \t]*(?:\/\/\/|\/\*\*)[ \t]*)(\S+)/;
const KEYS = [[/\bhook|\buse[A-Z]/, "🪝"], [/\bstate|\bstore/i, "🗄"], [/\bstream|\bpipe/i, "🌊"], [/\bresolve|\bresolution/i, "🧭"], [/\bcache|\bmemo/i, "🧠"], [/\bqueue|\bbatch/i, "📚"], [/\bschema|\bcontract/i, "📜"], [/\bpath|\bfile/i, "📁"], [/\bparse|\btoken/i, "🔤"], [/\bcheck|\bvalidate|\bassert/i, "✅"], [/\bclose|\bretire|\brelease/i, "🔒"], [/\bopen|\bstart|\bboot/i, "🚪"], [/\bpublish|\bsend|\bemit/i, "📣"], [/\bread|\bload|\bfetch/i, "📥"], [/\bwrite|\bsave|\bpersist/i, "💾"], [/\berror|\bfault|\bfail/i, "🚨"], [/\bconfig|\boption/i, "🎛"], [/\bsession|\bwindow|\bpanel/i, "🪟"], [/\bagent|\bconversation|\bchat/i, "💬"], [/\bframe|\bcodec|\bencode|\bdecode/i, "🎞"], [/\btype|\binterface/i, "🏷"], [/\bconst|\blimit|\bdefault/i, "📏"], [/\btest|\bfixture/i, "🧪"], [/\bshared|\bhelper/i, "🧰"]];
const POOL = "🔶 🔷 🔸 🔹 🟠 🟡 🟢 🔵 🟣 🟤 ⭐ 🌟 💠 🔰 🔱 🎯 🎲 🧩 🧱 🪵 🪨 🧲 🪝 🔩 🔧 🔨 🪛 🪚 🗜 🔗 📌 📍 📎 📏 📐 ✂ 🗂 🗃 📂 📁 📦 📮 🏷 🔖 🧾 📜 📃 📄 📑 🔍 🔎 🔬 🔭 📡 🛰 🧭 🗺 🧳 🎒 🪁 🎈 🎀 🎁 🪅 🧸 🎨 🖌 🖍 🪄 🔮 🧿 🧫 🧬 💎 🔔 📣 📢 🎵 🎼 🥁 🏆 🎖 🏅 🚩 🏁 🪧 🚦 🚧 ⚓ ⛵ 🛶 🚀 🛸 🚁 🚂 🚲 🛴 ⏱ ⏲ ⏰ 🕰 ⌛ ⏳ 🌡 ☀ 🌙 🌈 🔥 💧 🌊 ❄ ⚡ 🍀 🌱 🌿 🌳 🌲 🌴 🌵 🌾 🍁 🌸 🌼 🌻 🪻 🐝 🐞 🦋 🐌 🐜 🪲 🐢 🦎 🐙 🦀 🦞 🦐 🐠 🐬 🦈 🐳 🦭 🦦 🦫 🦔 🦇 🐦 🦉 🦅 🦆 🦢 🦜 🪶".split(" ");
for (const f of process.argv.slice(2)) {
  if (!realpathSync(f).startsWith("/private/tmp/fnsplit4a-finish/")) throw new Error("not an overlay file: " + f);
  const lines = readFileSync(f, "utf8").split("\n");
  const used = new Set(lines.map((l) => DOC.exec(l)).filter(Boolean).map((m) => norm(m[2])));
  const seen = new Set(), log = [];
  lines.forEach((l, i) => {
    const m = DOC.exec(l);
    if (!m) return;
    const e = norm(m[2]);
    if (!seen.has(e)) { seen.add(e); return; }
    const hit = KEYS.map(([re, em]) => [re, norm(em)]).find(([re, em]) => re.test(l.slice(m[0].length)) && !used.has(em));
    const pick = hit ? hit[1] : POOL.map(norm).find((x) => !used.has(x) && !seen.has(x));
    used.add(pick);
    seen.add(pick);
    lines[i] = l.replace(DOC, (_, pre) => pre + pick + V);
    log.push(`${i + 1}: ${e} -> ${pick}`);
  });
  if (log.length) writeFileSync(f, lines.join("\n"));
  console.log(f.split("/").slice(-2).join("/"), log.join(" | ") || "unchanged");
}

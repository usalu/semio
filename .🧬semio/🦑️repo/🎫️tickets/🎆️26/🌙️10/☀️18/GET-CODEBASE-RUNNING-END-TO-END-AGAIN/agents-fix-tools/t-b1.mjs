import { build } from "./b1-comments.mjs";
import { entries } from "./lib.mjs";
import { scan } from "./scan-comments.mjs";
const c = build();
console.log(c);
for (const [p, v] of entries()) { const left = scan(v.after, p).filter((h) => !h.directive && !h.body.startsWith("/**")); console.log(p.split("/").slice(-3).join("/"), v.before.length, v.after.length, "left:", left.length); }
import { syntaxErrors, docEmojiDuplicates } from "./check-docs.mjs";
for (const [p, v] of entries()) {
  const err = syntaxErrors(p, v.after);
  const a = docEmojiDuplicates(v.before), b = docEmojiDuplicates(v.after);
  const worse = [...b].filter(([e, n]) => n > (a.get(e) ?? 0) && n > 1).map(([e, n]) => e + n);
  console.log(p.split("/").slice(-2).join("/"), "syntax:", err.length, err.join(";"), "dupEmoji:", worse.join(","));
}

#!/usr/bin/env bun
/**
 * 🏷️ R10 item 4 codemod (lands LAST in window 3, on the coordinator's word): removes the `@emoji` residue token from the
 * front of every docstring (Rust `///` / `//!`, TS/JS `/**` and continuation ` * `, the generators that emit such lines,
 * the two `indexOf` anchors that find them, PowerShell `#`, Python `"""`) so the docstring starts with its emoji, as
 * AGENTS.md asks and TypeScript's own JSDoc reader needs (R9 item 5: `/** @emoji 🧹️ …` has an EMPTY summary).
 * A docstring whose marker is a symbol glyph (`⊕`, `√`, `⛶️` …) keeps it; the eleven that carry no symbol after the token
 * (a bullet, mojibake, or plain text) get the hand-picked emoji in `PICKS` (checked unused in their file). Excluded: Markdown prose (`.cursor/plans`), the ticket tree, and the frozen
 * projection asset whose sha256 the taxonomy pins (`📽️nested-cargo-package-projection`: historical source text), and the
 * docstring census fixture whose `@emoji` openers are the census law's test inputs (`🧮️source-census`).
 * Usage: bun at-emoji-strip.ts [--apply] [--diff <out.diff>] — dry run by default; idempotent.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const EXCLUDED = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json"];
const TOKEN = /(\/\/\/|\/\/!|\/\*\*|\/\*|\*|\/\/|#|""")([ \t]*)@emoji[ \t]+/gu;
const EMOJI_START = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)/u;

/** 🎨️ Docstrings whose text after `@emoji` is not an emoji: `<path>\t<exact text after the token>` → replacement text. */
const PICKS: Readonly<Record<string, string>> = {
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts\t• Bulleted list body. */": "🔘️ Bulleted list body. */",
  "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts\t• Bulleted list body. */": "🔘️ Bulleted list body. */",
  "🧰️framework/🔨️modules/📚️compiler/🧮️math/🦀️.rs\tˆ Accent (`hat`/`bar`/`vec`/`dot`/`ddot`/`tilde`): places a combining accent glyph over": "🎩️ Accent (`hat`/`bar`/`vec`/`dot`/`ddot`/`tilde`): places a combining accent glyph over",
  "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx\tCombined passive drop-zone treatment (fill + emphasized text/icons). */": "📥️ Combined passive drop-zone treatment (fill + emphasized text/icons). */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx\tFlow host surface for diagram runtime. */": "🌊️ Flow host surface for diagram runtime. */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx\tDefault diagram host port wired to @xyflow/react. */": "🔗️ Default diagram host port wired to @xyflow/react. */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx\tESM-safe setter for flowHostPort. */": "🪝️ ESM-safe setter for flowHostPort. */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx\tJSX alias for diagram flow host. */": "🏷️ JSX alias for diagram flow host. */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx\tJSX alias for diagram flow provider. */": "🔖️ JSX alias for diagram flow provider. */",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧱️DragHandle/🟦️.tsx\tdata-hover-scope attr for DragHandle hover exclusion. */": "🫥️ data-hover-scope attr for DragHandle hover exclusion. */",
};
const CAD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx";
const CAD_TEXT = "Canvas-only {@link InteractionRepl} (no model-definition aside)";

const listed = Bun.spawnSync(["git", "grep", "-lz", "@emoji", "--", ":!.🧬semio", ":!*.md"], { cwd: ROOT, stdout: "pipe" }).stdout.toString().split("\0").filter((path) => path && !EXCLUDED.includes(path));
const apply = process.argv.includes("--apply");
const diffAt = process.argv.indexOf("--diff");
const diffPath = diffAt >= 0 ? process.argv[diffAt + 1] : undefined;
let tokens = 0;
const unresolved: string[] = [];
const picked: string[] = [];
const diff: string[] = [];
const perFile: [string, number][] = [];
for (const path of listed) {
  const before = readFileSync(join(ROOT, path), "utf8");
  let count = 0;
  const lines = before.split("\n").map((line, index) => {
    let cut = false;
    const replaced = line.replace(TOKEN, (_match, opener: string, space: string, offset: number, whole: string) => {
    count += 1;
    const rest = whole.slice(offset + _match.length);
    const gap = space || (opener === '"""' ? "" : " ");
    if (EMOJI_START.test(rest)) return `${opener}${gap}`;
    const pick = PICKS[`${path}\t${rest}`];
    if (pick !== undefined) {
      picked.push(`${path}:${index + 1}`);
      cut = true;
      return `${opener}${gap}${pick}\u0000`;
    }
    if (path === CAD && rest.includes(CAD_TEXT)) {
      picked.push(`${path}:${index + 1}`);
      cut = true;
      return `${opener}${gap}🪟️ ${rest.slice(rest.indexOf(CAD_TEXT))}\u0000`;
    }
    unresolved.push(`${path}:${index + 1} ${rest.slice(0, 80)}`);
    return _match;
    });
    return cut ? replaced.slice(0, replaced.indexOf("\u0000")) : replaced;
  });
  const after = lines.join("\n");
  tokens += count;
  if (after === before) continue;
  perFile.push([path, count]);
  if (diffPath) before.split("\n").forEach((line, index) => { if (line !== lines[index]) diff.push(`--- ${path}:${index + 1}\n-${line}\n+${lines[index]}`); });
  if (apply) writeFileSync(join(ROOT, path), after);
}
if (diffPath) writeFileSync(diffPath, `${diff.join("\n")}\n`);
console.log(JSON.stringify({ mode: apply ? "apply" : "dry-run", files: listed.length, changedFiles: perFile.length, tokens, picked: picked.length, unresolved: unresolved.length }));
for (const line of unresolved) console.log(`unresolved ${line}`);
process.exit(unresolved.length ? 1 : 0);

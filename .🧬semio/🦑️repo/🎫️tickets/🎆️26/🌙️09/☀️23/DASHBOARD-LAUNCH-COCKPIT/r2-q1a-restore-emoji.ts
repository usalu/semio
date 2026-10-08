import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const root = "C:/git/semio";
const ui = "🧰️framework/🔨️modules/🖱️ui";
const oldRevision = "506c4f39d5e";
const oldPath = `${ui}/⌨️tui/🦀️component.rs`;
const dry = process.argv.includes("--dry");

const excluded = ["🔡️ansi", "🔌️backend", "🪟️windows", "🏃️host", "🚇️pty"];
const files = [
  "⌨️tui/🦀️.rs",
  "⌨️tui/📡️event/🦀️.rs",
  "⌨️tui/🪀️widget/🦀️.rs",
  "⌨️tui/📏️layout/🦀️.rs",
  "⌨️tui/🎬️scene/🦀️.rs",
  "⌨️tui/📐️geometry/🦀️.rs",
  "⌨️tui/🧪️tests/🔬️unit/🦀️.rs",
].filter((file) => !excluded.some((name) => file.includes(name)));

const manual: Record<string, string> = {
  "A decoded terminal key.": "⌨️",
  "Modifier bitflags.": "🎹️",
  "Any input the terminal can report to the retained-mode engine.": "📡️",
  "Pseudo-terminal child process spawn and byte I/O for the native TUI host.": "🚇️",
  "Removes `window_kind_id` from the layout tree.": "🗑️",
  "Appends a window tab to the stack containing `host_window_kind_id`.": "📎️",
  "A generational-arena retained scene tree, renderer-agnostic; the overlay root stacks above the main root.": "🗂️",
  "Moves `id` under `new_parent`, preserving subtree state for layout remounts.": "🚚️",
  "The deepest visible, hittable node whose rect contains `pos`; overlays win over the main tree.": "🎯️",
};

const git = spawnSync("git", ["-c", "core.quotepath=false", "show", `${oldRevision}:${oldPath}`], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
if (git.status !== 0) throw new Error(git.stderr);
const hasEmoji = (token: string) => /[^\x00-\x7f]/.test(token);
const docLine = /^(\s*)(\/\/\/|\/\/!)\s+(\S+)\s+(.*)$/;
const regionLine = /^(\s*)\/\/\s*#(region|endregion)\s+(\S+)$/;
const byText = new Map<string, string[]>();
const byRegion = new Map<string, string>();
for (const line of git.stdout.split(/\r?\n/)) {
  const doc = docLine.exec(line);
  if (doc && hasEmoji(doc[3])) {
    const list = byText.get(doc[4]) ?? [];
    list.push(doc[3]);
    byText.set(doc[4], list);
  }
  const region = regionLine.exec(line);
  if (region && hasEmoji(region[3])) {
    const match = /^(\P{L}+)(\p{L}.*)$/u.exec(region[3]);
    if (match) byRegion.set(match[2], match[1]);
  }
}

const placeholderDoc = /^(\s*)(\/\/\/|\/\/!)\s*\?+\s*(.*)$/;
const placeholderRegion = /^(\s*)\/\/\s*#(region|endregion)\s*\?+\s*(\S+)$/;
const consumed = new Map<string, number>();
const unmatched: string[] = [];
let restored = 0;

for (const file of files) {
  const path = `${root}/${ui}/${file}`;
  const lines = readFileSync(path, "utf8").split("\n");
  let changed = false;
  const out = lines.map((line, index) => {
    const cr = line.endsWith("\r") ? "\r" : "";
    const body = cr ? line.slice(0, -1) : line;
    const doc = placeholderDoc.exec(body);
    if (doc) {
      const candidates = byText.get(doc[3]);
      let emoji = manual[doc[3]];
      if (candidates) {
        const used = consumed.get(doc[3]) ?? 0;
        emoji = candidates[Math.min(used, candidates.length - 1)];
        consumed.set(doc[3], used + 1);
      }
      if (!emoji) {
        unmatched.push(`${file}:${index + 1}: ${doc[3]}`);
        return line;
      }
      restored++;
      changed = true;
      return `${doc[1]}${doc[2]} ${emoji} ${doc[3]}${cr}`;
    }
    const region = placeholderRegion.exec(body);
    if (region) {
      const emoji = byRegion.get(region[3]) ?? "🔖️";
      restored++;
      changed = true;
      return `${region[1]}//#${region[2]} ${emoji}${region[3]}${cr}`;
    }
    return line;
  });
  if (changed && !dry) writeFileSync(path, out.join("\n"), "utf8");
}

console.log(JSON.stringify({ dry, restored, unmatched }, null, 2));

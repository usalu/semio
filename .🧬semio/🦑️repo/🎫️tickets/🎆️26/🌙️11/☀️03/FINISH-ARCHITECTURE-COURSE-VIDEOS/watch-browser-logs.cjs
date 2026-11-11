const fs = require("fs");
const path = require("path");
const root = ".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered";
const target = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/🗑️generated";
function scan(folder, depth) {
  if (!fs.existsSync(folder) || depth < 0) return;
  for (const entry of fs.readdirSync(folder, { withFileTypes: true })) {
    const file = path.join(folder, entry.name);
    if (entry.isDirectory()) scan(file, depth - 1);
    else if (entry.name === "run.log" && Date.now() - fs.statSync(file).mtimeMs < 300000) {
      const id = path.basename(path.dirname(file));
      try { fs.copyFileSync(file, path.join(target, "browser-" + id + ".log")); } catch {}
    }
  }
}
setInterval(() => {
  try {
    for (const folder of [root, path.join(root, "chapters")])
      for (const name of fs.readdirSync(folder)) if (name.startsWith("p-")) scan(path.join(folder, name), 4);
  } catch {}
}, 200);

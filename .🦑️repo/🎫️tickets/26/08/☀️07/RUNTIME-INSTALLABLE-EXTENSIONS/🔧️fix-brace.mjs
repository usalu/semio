import fs from "fs";
import path from "path";

function find(pred) {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = path.join(dir, e.name);
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (pred(p, e.name)) return p;
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const shellPath = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx") && n.includes("component"));
let text = fs.readFileSync(shellPath, "utf8");

const bad = `            }
          }
          }
        }
      }
      if (appRegistrationsJson) {`;

const good = `            }
          }
        }
      }
      if (appRegistrationsJson) {`;

if (!text.includes(bad)) {
  console.log("pattern not found, dumping nearby");
  const idx = text.indexOf("[DEBUG] setContributions push skipped");
  console.log(JSON.stringify(text.slice(idx, idx + 350)));
  throw new Error("extra brace pattern missing");
}
text = text.replace(bad, good);
fs.writeFileSync(shellPath, text);
console.log("removed extra brace");

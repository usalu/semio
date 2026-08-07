import fs from "fs";

const file = "🧰️framework/🛍️products/💻️os/🦀️component.rs";
const lines = fs.readFileSync(file, "utf8").split("\n");
for (let i = 0; i < lines.length; i++) {
  if (/pub use.*space|SpaceProjection|InstalledExtension|empty_space/.test(lines[i])) {
    console.log(`${i + 1}|${lines[i]}`);
  }
}

import { readdirSync, readFileSync } from "fs";
import { join } from "path";

const texDir = "/Users/ueli/Documents/semio/print/tex";
const files = readdirSync(texDir);

for (const file of files) {
  if (file.endsWith(".sty") || file.endsWith(".cls")) {
    const filePath = join(texDir, file);
    const content = readFileSync(filePath, "utf-8");
    const lines = content.split("\n");
    lines.forEach((line, index) => {
      if (line.includes("colspec")) {
        console.log(`${file}:${index + 1}: ${line.trim()}`);
      }
    });
  }
}

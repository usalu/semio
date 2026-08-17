import { readFileSync } from "fs";
const file = readFileSync("framework/product/playground/renderer/react/index.tsx", "utf8");
const lines = file.split("\n");
lines.forEach((line, i) => {
  if (line.includes("framework-platform-renderer-react") || line.includes("platform/renderer/react")) {
    console.log(`${i + 1}: ${line}`);
  }
});

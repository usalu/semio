import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const repo = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const reactDir = join(repo, "elements", "client", "lib", "react");
const stylingDir = join(repo, "elements", "client", "lib", "styling");
mkdirSync(stylingDir, { recursive: true });

const globalsPath = join(reactDir, "globals.css");
const text = readFileSync(globalsPath, "utf8");
const lines = text.split(/\r?\n/);
const shared = lines.slice(0, 1168).join("\n").replace('@import "./theme.css";', '@import "./palette.css";');
writeFileSync(join(stylingDir, "elements.css"), shared + "\n", "utf8");
const tail = lines.slice(1174).join("\n");
writeFileSync(join(reactDir, "globals-ui.css"), tail + "\n", "utf8");
copyFileSync(join(reactDir, "theme.css"), join(stylingDir, "palette.css"));

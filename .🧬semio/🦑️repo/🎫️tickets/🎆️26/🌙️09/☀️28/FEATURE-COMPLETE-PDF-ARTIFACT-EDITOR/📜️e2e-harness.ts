/** 🧰 Single source of the dev-host port, oracle pins and Python launcher shared by every mounted PDF E2E script. */
import {readFileSync} from "node:fs";
import {join} from "node:path";

export const ticket = import.meta.dir;
const variant = "✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route/🧫️fixtures/🔣️.json";
export const devPort: number = JSON.parse(readFileSync(join(process.cwd(), variant), "utf8")).reactPort;
export const devUrl = `http://127.0.0.1:${devPort}/`;
export const oraclePins = ["pymupdf==1.26.5", "pillow==13.0.0"];
export const python = (tool: string, ...args: string[]) => ["repo", "tools", "exec", "--light", "uv", "--", "run", "--no-project", "--python", "3.12",
  ...oraclePins.flatMap(pin => ["--with", pin]), "--", "python", join(ticket, tool), ...args];

import { join } from "node:path";
import { pathToFileURL } from "node:url";

const { runCmd } = await import(pathToFileURL(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const ticket = process.env.SEMIO_TICKET_DIR!;
runCmd("powershell.exe", ["-NoProfile", "-NonInteractive", "-File", join(ticket, "inspect-sdk-cabinets.ps1"), join(ticket, "🗑️generated")]);

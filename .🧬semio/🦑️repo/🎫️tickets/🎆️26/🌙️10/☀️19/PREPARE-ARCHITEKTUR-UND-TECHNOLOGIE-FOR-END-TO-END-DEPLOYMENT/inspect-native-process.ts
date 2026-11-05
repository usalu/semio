import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const { runCmd } = await import(pathToFileURL(join(process.cwd(), '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe', ['-NoProfile', '-NonInteractive', '-Command', "Get-Process -Name vs_BuildTools,vs_setup_bootstrapper,vs_installershell,vs_installer,setup,consent,winget -ErrorAction SilentlyContinue | Select-Object ProcessName,Id,MainWindowTitle | ConvertTo-Json -Compress; exit 0"]);

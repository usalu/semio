import { spawn } from "node:child_process";
import { mkdirSync, openSync, closeSync, writeFileSync, existsSync } from "node:fs";
import { resolve, join } from "node:path";
import { homedir } from "node:os";
import { fileURLToPath } from "node:url";

const folder = fileURLToPath(new URL(".", import.meta.url));
const generated = resolve(folder, "🗑️generated");
const cache = resolve(".cache/b/p/m/storybook-ui");
const port = Number(process.env.STORYBOOK_PORT ?? 6011);
mkdirSync(generated, { recursive: true });
mkdirSync(cache, { recursive: true });
const logPath = resolve(generated, "storybook.log");
const log = openSync(logPath, "w");
const launcher = join(homedir(), ".local/share/semio/launcher/bin/repo");
const child = spawn(existsSync(launcher) ? launcher : "repo", ["tools", "exec", "bun", "--", "./node_modules/storybook/dist/bin/dispatcher.js", "dev", "--port", String(port), "--host", "127.0.0.1", "--no-open", "--ci"], {
  cwd: process.cwd(),
  env: { ...process.env, STORYBOOK_SCOPE: "ui", SEMIO_BUILD_MODE: "prototype", SEMIO_VITE_CACHE_DIRECTORY: cache },
  detached: true,
  stdio: ["ignore", log, log],
});
child.unref();
closeSync(log);
writeFileSync(resolve(folder, "storybook.md"), `# Storybook\n\n- URL: http://localhost:${port}\n- Launcher PID: ${child.pid}\n- Log: \`🗑️generated/storybook.log\`\n- UI scope, live dev server, canonical bounded Vite cache, localhost binding.\n\nPort 6010 belongs to a static inspection server in the separate fix-storybook checkout; it is not this branch. The previous launch failed its compiler-cache requirement and the initial HTTP check mistakenly reached that static server. Startup verification for this process is pending.\n`);
console.log(JSON.stringify({ pid: child.pid, url: `http://localhost:${port}`, log: logPath }));

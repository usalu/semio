/** 🧭️ Exercises the published PDF shell and verifies the saved downloads with independent tools. */
import {closeSync, copyFileSync, existsSync, mkdirSync, openSync} from "node:fs";
import {join} from "node:path";

const ticket = import.meta.dir;
const generated = join(ticket, "🗑️generated");
const run = process.argv[2];
const selected = new Set(process.argv.slice(3));
const cases = ["rotation", "content", "style", "page-history", "png", "large-vector", "large-text", "large-images", "cancel", "reject-invalid"];
if (!run || !/^[a-z0-9-]+$/u.test(run) || [...selected].some(name => !cases.includes(name))) throw Error("Expected a fresh run name and optional case names");
const output = join(generated, "mounted-" + run);
const url = "http://127.0.0.1:6260/";
mkdirSync(output);
const original = join(process.cwd(), "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const content = join(generated, "pdf-mounted-content-input.pdf");
const history = join(generated, "pdf-page-history-source.pdf");
const png = join(output, "png");
const invalidPdf = join(output, "invalid.pdf");
const oversized = join(generated, "large-vector-pdf.pdf");
const largeText = join(generated, "large-mounted-pdf.pdf");
const largeImages = join(generated, "🎞️controlled-images.pdf");
const bun = (tool: string, ...args: string[]) => {
  const command = ["repo", "tools", "exec", "bun", "--", join(ticket, tool), ...args];
  if (process.env.SEMIO_PDF_TRACE_OPEN !== "1") return command;
  const memory = tool === "📜️pdf-browser-direct.ts" ? 1024 : 1536;
  return ["repo", "capacity", "run", "--memory=" + memory, "--", ...command];
};
const python = (tool: string, ...args: string[]) => ["repo", "tools", "exec", "--light", "uv", "--", "run", "--offline", "--no-project", "--python", "3.12", "--with", "pymupdf==1.26.7", "--with", "pillow==12.0.0", "--", "python", join(ticket, tool), ...args];

function start(name: string, args: string[], env = process.env) {
  const log = openSync(join(output, name + ".log"), "wx");
  try {
    return Bun.spawn(args, {env, stdout: log, stderr: log});
  } finally {
    closeSync(log);
  }
}
let active: ReturnType<typeof start> | undefined;
let server: ReturnType<typeof start> | undefined;
let interrupted = false;
function cancel() {
  interrupted = true;
  active?.kill("SIGINT");
  server?.kill("SIGINT");
}
process.once("SIGINT", cancel);
process.once("SIGTERM", cancel);

async function step(name: string, args: string[]) {
  if (interrupted) throw Error("Mounted PDF suite interrupted");
  active = start(name, args);
  try {
    const status = await active.exited;
    console.log("[DEBUG]", name, "exit", status);
    if (status !== 0) throw Error(name + " exited " + status);
  } finally {
    active = undefined;
  }
}
const has = (name: string) => !selected.size || selected.has(name);
const ready = () => fetch(url,{signal: AbortSignal.timeout(2000)}).then(response => response.ok, () => false);
try {
  if (await ready()) throw Error("Port 6260 already serves another session");
  if ((has("content") || has("style")) && !existsSync(content)) await step("prepare-content", python("📜️mounted-content-edit-oracle.py", content));
  if (has("page-history") && !existsSync(history)) await step("prepare-history", python("📜️pdf-page-history-oracle.py", "build"));
  if ((has("large-vector") || has("cancel")) && !existsSync(oversized)) await step("prepare-large-vector", python("📜️large-vector-pdf-oracle.py", oversized));
  if (has("large-text") && !existsSync(largeText)) await step("prepare-large-text", python("📜️large-mounted-pdf-oracle.py", largeText));
  if (has("large-images") && !existsSync(largeImages)) await step("prepare-large-images", python("🎞️fixture.py"));
  if (has("png")) {
    await step("prepare-png", python("📜️pdf-raw-png-history-oracle.py", png, "--self-test"));
    copyFileSync(join(png,"oracle-proof","image-source.pdf"), join(png,"image-source.pdf"));
  }
  if (has("reject-invalid")) await step("prepare-invalid", python("📜️pdf-invalid-open-oracle.py", invalidPdf));
  server = start("serve", bun("📜️pdf-browser-direct.ts", "serve"), {...process.env, S_LOCAL_ONLY: "1", S_OS_PORT: "6260"});
  const deadline = Date.now() + 300_000;
  while (!(await ready())) {
    if (server.exitCode !== null || interrupted || Date.now() >= deadline) throw Error("Browser activation did not start; see serve.log");
    await Bun.sleep(1000);
  }
  if (server.exitCode !== null) throw Error("Browser activation process exited");
  let failures = 0;
  const check = async (name: string, browser: string[], oracle?: string[]) => {
    if (!has(name)) return;
    try {
      await step(name, browser);
      if (oracle) await step(name + "-oracle", oracle);
    } catch (error) {
      failures++;
      console.error("[DEBUG]", error);
    }
  };
  const smoke = (source: string, ...mode: string[]) => bun("📜️pdf-browser-smoke.ts", url, source, output, ...mode);
  await check("rotation", smoke(original), python("📜️mounted-download-oracle.py", original, join(output,"pdf-mounted.pdf")));
  await check("content", smoke(content, "edit-content"), python("📜️mounted-content-edit-oracle.py", content, join(output,"pdf-mounted-content.pdf")));
  await check("style", smoke(content, "style-properties"), python("📜️pdf-style-properties-oracle.py", join(output,"pdf-mounted-style-properties.pdf")));
  await check("page-history", smoke(history, "page-history"), python("📜️pdf-page-history-oracle.py", "verify", join(output,"pdf-mounted-page-history.pdf")));
  await check("png", bun("image-authoring-browser.ts", url, png, join(png,"large-picked.png")), python("📜️pdf-raw-png-history-oracle.py", png, "--check"));
  await check("large-vector", smoke(oversized, "large-vector"));
  await check("large-text", smoke(largeText, "large-text"));
  await check("large-images", smoke(largeImages, "large-images"), python("🎞️fixture.py", join(output,"pdf-mounted-large-images.pdf")));
  await check("cancel", smoke(oversized, "cancel-large"));
  await check("reject-invalid", smoke(invalidPdf, "reject-invalid"));
  if (failures) process.exitCode = 1;
} finally {
  if (server?.exitCode === null) {
    server.kill("SIGINT");
    await server.exited;
  }
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}

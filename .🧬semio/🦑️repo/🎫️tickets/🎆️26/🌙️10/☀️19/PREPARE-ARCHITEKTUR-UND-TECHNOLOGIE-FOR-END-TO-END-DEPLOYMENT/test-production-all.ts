import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR!;
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const { withNativeTemporaryDirectoryV1, nativeTemporarySystemPortV1 } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts")).href);
const port = nativeTemporarySystemPortV1((args: string[]) => runCmdStatus("subst.exe", args));
withNativeTemporaryDirectoryV1({ target: join(ticket, "🗑️generated"), ticket, candidates: [join(process.env.LOCALAPPDATA!, "semio", "native-temp")] }, port, (alias: string) => {
  const env = { ...process.env, SEMIO_MSVC_FIXTURE_ROOT: alias, TEMP: alias, TMP: alias };
  runCmd("repo", ["tools", "exec", "--cwd=" + join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🪟️msvc/📦️packages/🐹️go"), "go", "--", "test", "./...", "-count=1"], { cwd: root, env });
});

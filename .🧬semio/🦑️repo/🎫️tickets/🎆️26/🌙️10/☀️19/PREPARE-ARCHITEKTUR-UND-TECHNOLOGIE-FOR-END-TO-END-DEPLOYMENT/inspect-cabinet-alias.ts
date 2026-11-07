import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR!;
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const { withNativeTemporaryDirectoryV1, nativeTemporarySystemPortV1 } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts")).href);
const generated = join(ticket, "🗑️generated"), port = nativeTemporarySystemPortV1((args: string[]) => runCmdStatus("subst.exe", args));
withNativeTemporaryDirectoryV1({ target: generated, ticket, candidates: [join(process.env.LOCALAPPDATA!, "semio", "native-temp")] }, port, (alias: string) => {
  runCmd("repo", ["tools", "exec", "go", "--", "run", join(ticket, "inspect-cabinet.go"), join(alias, "26ea25236f12b23db661acf268a70cfa.cab"), join(alias, "cab-sample.json")]);
});

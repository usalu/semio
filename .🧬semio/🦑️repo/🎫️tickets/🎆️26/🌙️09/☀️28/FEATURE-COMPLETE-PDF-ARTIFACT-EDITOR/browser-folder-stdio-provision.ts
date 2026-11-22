/** 🥾 Publishes the native journal helper under the dev server’s admitted build mode. */
import { accessSync, chmodSync, constants, copyFileSync, createReadStream, existsSync, mkdirSync, renameSync, rmSync } from "node:fs";
import { createHash, randomUUID } from "node:crypto";
import { dirname, join } from "node:path";
import { withCapturedCargoArtifactsTaskV1 } from "@semio-tech/process/native-build";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { buildAdmittedCargoArtifactsV1 } from "PROCESS_CARGO_BUILD";
import { decodeCargoProvenanceV1 } from "PROCESS_CARGO_RECEIPT";
import type { ScriptInvocation } from "PROCESS_SCRIPT_ROUTING";

const nativeStore = "NATIVE_STORE_MANIFEST";

/** 🧭 Prepares the managed executable before entering the long-lived Vite phase. */
export async function ensureNativeFolderJournalV1(root: string, invocation: ScriptInvocation): Promise<string> {
  const authority = readNativeCallerAuthorityV1(invocation.capabilities);
  if (authority.repositoryRoot !== root) throw Error("Actual native journal workspace required");
  const name = process.platform === "win32" ? "semio-folder-journal.exe" : "semio-folder-journal";
  const directory = join(root, ".cache", "b", "p", "m", "browser", authority.original.buildMode, "tools", "folder-journal", process.platform);
  let prepared: string | undefined;
  const hash = async (path: string) => {
    const digest = createHash("sha256");
    for await (const bytes of createReadStream(path)) digest.update(bytes);
    return digest.digest("hex");
  };
  await buildAdmittedCargoArtifactsV1(join(root, nativeStore), ["--bin", "semio-folder-journal", "--features", "folder-journal"], root, {
    producer: "artifact", invocation,
    control: scriptCompilerAdmissionControlV1(invocation, "cargo:build", buildBudgetMs()),
    consume: (capture) => withCapturedCargoArtifactsTaskV1(capture, async (view) => {
      const executable = view.files.get(name);
      const observed = decodeCargoProvenanceV1(view.observation);
      if (!executable || observed.status !== 0 || observed.cancelled) throw Error("Cargo did not deliver the native folder journal");
      const content = await hash(executable);
      await view.recheck();
      const destination = join(directory, content, name);
      if (existsSync(destination)) {
        accessSync(destination, constants.X_OK);
        if (await hash(destination) !== content) throw Error("Published native folder journal digest changed");
        prepared = destination;
        return;
      }
      mkdirSync(dirname(destination), { recursive: true });
      const staging = `${destination}.${randomUUID()}.tmp`;
      try {
        copyFileSync(executable, staging);
        if (await hash(staging) !== content) throw Error("Native folder journal changed during publication");
        chmodSync(staging, 0o755);
        try { renameSync(staging, destination); }
        catch (error) {
          if (!existsSync(destination) || await hash(destination) !== content) throw error;
        }
      } finally { rmSync(staging, { force: true }); }
      accessSync(destination, constants.X_OK);
      prepared = destination;
    }),
  });
  if (!prepared) throw Error("Native folder journal publication did not settle");
  return prepared;
}

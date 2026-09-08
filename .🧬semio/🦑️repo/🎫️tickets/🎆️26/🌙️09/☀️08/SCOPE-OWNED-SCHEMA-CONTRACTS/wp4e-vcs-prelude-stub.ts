/** 🧿️ Bun preload that stubs the vcs plugin's own prelude law so `native-openable-catalog-provider-check` reaches the hub laws while `🌿️vcs` is red in another partition (`📓️wp4e-hub.md` §3.2). */
import { plugin } from "bun";

plugin({
  name: "wp4e-vcs-prelude-stub",
  setup(build) {
    build.onLoad({ filter: /🌿️vcs\/📦️packages\/🦀️rust\/📜️script\.ts$/u }, () => ({
      loader: "ts",
      contents: "export async function proveVcsNativeCodecReceipts(): Promise<void> { console.log(\"[DEBUG] wp4e: vcs prelude stubbed; peer-owned law is red\"); }\n",
    }));
  },
});

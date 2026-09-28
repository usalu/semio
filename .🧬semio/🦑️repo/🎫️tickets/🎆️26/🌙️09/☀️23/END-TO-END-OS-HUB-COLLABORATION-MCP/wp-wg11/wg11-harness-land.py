#!/usr/bin/env python3
"""🤝️ WG11 session 14b — window-3 landing of the permanent wgpu collaboration acceptance (preamble 14 rule 17).

1. The staged harness `wp-wg11/harness/🤝️hub-collaboration/🟦️.ts` becomes
   `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts` (NEW dir, taxonomy = R10) with every absolute repo import rewritten relative.
2. `🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`: the native runtime staging duplicated by `hub-live-collaboration-check`
   and `native-guest-journey-check` becomes ONE `stageNativeRuntime(variant, profile)`, and the verb `hub-collaboration-acceptance`
   runs the harness CLI (evidence under the target's gitignored `🤖️generated/🤝️hub-collaboration`, native journeys stage block2d).

Dry run by default; `--apply` writes (every anchor asserted exactly once; refuses when the target file exists).
"""

import difflib
import os
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WGPU = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
STAGED = Path(__file__).resolve().parent / "harness/🤝️hub-collaboration/🟦️.ts"
TARGET = WGPU / "🧪️tests/🤝️hub-collaboration/🟦️.ts"
SCRIPT = WGPU / "📦️packages/🟦️typescript/📜️script.ts"

STAGE_OLD_LIVE = """    const variant = "block2d", profile = "release";
    const publisher = join(import.meta.dir, "../../⌨️native-entrypoint/📦️modules/📜️script.ts");
    const published = spawnSync(process.execPath, [publisher, "publish", variant, profile], { cwd: repoRoot, encoding: "utf8" });
    if (published.status !== 0) throw new Error(`native ${variant} runtime publish failed: ${published.stderr}${published.stdout}`);
    const modules = nativeRuntimeDirectory(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, profile);
    const law = "shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document";"""
STAGE_NEW_LIVE = """    const variant = "block2d", modules = stageNativeRuntime(variant, "release");
    const law = "shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document";"""
STAGE_OLD_GUEST = """    const variant = "block2d", profile = "release";
    const publisher = join(import.meta.dir, "../../⌨️native-entrypoint/📦️modules/📜️script.ts");
    const published = spawnSync(process.execPath, [publisher, "publish", variant, profile], { cwd: repoRoot, encoding: "utf8" });
    if (published.status !== 0) throw new Error(`native ${variant} runtime publish failed: ${published.stderr}${published.stdout}`);
    const modules = nativeRuntimeDirectory(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, profile);
    const laws = ["""
STAGE_NEW_GUEST = """    const variant = "block2d", profile = "release", modules = stageNativeRuntime(variant, profile);
    const laws = ["""

SCRIPT_EDITS = [
    (
        """import { runNativeBinary } from "../../⌨️native-entrypoint/📜️script.ts";
""",
        """import { runNativeBinary } from "../../⌨️native-entrypoint/📜️script.ts";
import { runHubCollaborationCli } from "../../🧪️tests/🤝️hub-collaboration/🟦️.ts";
""",
    ),
    (
        """/** @emoji 🧪️ Runs one ignored live law of this crate, exactly, under the crate's exhaustive budget. */""",
        """/** @emoji 📦️ Stages a completed plugin release component as the native runtime (the `native-entrypoint` publish verb, no
 * compilation; release because the 88 MB dev block2d component exceeds the 64 MiB execution-target component bound) and answers
 * the directory the native live laws mount it from (`SEMIO_PLUGIN_MODULES`). */
function stageNativeRuntime(variant: string, profile: "dev" | "release"): string {
  const publisher = join(import.meta.dir, "../../⌨️native-entrypoint/📦️modules/📜️script.ts");
  const published = spawnSync(process.execPath, [publisher, "publish", variant, profile], { cwd: repoRoot, encoding: "utf8" });
  if (published.status !== 0) throw new Error(`native ${variant} runtime publish failed: ${published.stderr}${published.stdout}`);
  return nativeRuntimeDirectory(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, profile);
}

/** @emoji 🧪️ Runs one ignored live law of this crate, exactly, under the crate's exhaustive budget. */""",
    ),
    (STAGE_OLD_LIVE, STAGE_NEW_LIVE),
    (STAGE_OLD_GUEST, STAGE_NEW_GUEST),
    (
        """/** @emoji ⏯️ One native wgpu shell mounts the staged block2d guest""",
        """/** @emoji 🤝️ Two humans — and a delegated AI agent — collaborate on ONE hub document through the wgpu shells: one journey per
 * run (`--journey wasm32|wasm32-react|wasm32-native|native-react|native-react-cursors|cursors|agent-pixels --hub <url>
 * [--serve <url>] [--react-serve <url>] [--locale en|de]`), one acceptance record `<check>-<locale>`; an omitted serve is started
 * and stopped by the run itself, native journeys mount the staged block2d release runtime. Humans only from
 * `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}`. See {@link runHubCollaborationCli}. */
class HubCollaborationAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runHubCollaborationCli(repoRoot, join(import.meta.dir, "../../🤖️generated/🤝️hub-collaboration"), segments, { nativeModules: () => stageNativeRuntime("block2d", "release") });
  }
}

/** @emoji ⏯️ One native wgpu shell mounts the staged block2d guest""",
    ),
    (
        """  .register("hub-live-collaboration-check", HubLiveCollaborationCheckScript)
""",
        """  .register("hub-live-collaboration-check", HubLiveCollaborationCheckScript)
  .register("hub-collaboration-acceptance", HubCollaborationAcceptanceScript)
""",
    ),
]


def relocated_harness() -> str:
    source = STAGED.read_text(encoding="utf-8")

    def relative(match: re.Match) -> str:
        path = os.path.relpath(match.group(1), TARGET.parent)
        return f'from "{path if path.startswith(".") else "./" + path}"'

    rewritten = re.sub(r'from "(' + re.escape(str(ROOT)) + r'/[^"]+)"', relative, source)
    if str(ROOT) in rewritten:
        sys.exit("an absolute repo path survived the import rewrite")
    return rewritten


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    if TARGET.exists():
        sys.exit(f"{TARGET} exists — landed already")
    harness = relocated_harness()
    before = SCRIPT.read_text(encoding="utf-8")
    after = replaced(SCRIPT, before, SCRIPT_EDITS)
    sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), "📜️script.ts", "📜️script.ts (patched)", n=1))
    imports = [line for line in harness.splitlines() if line.startswith("import ")]
    print("\n+++ NEW " + str(TARGET.relative_to(ROOT)) + f" ({len(harness)} chars)\n" + "\n".join(imports))
    if apply:
        TARGET.parent.mkdir(parents=True, exist_ok=False)
        TARGET.write_text(harness, encoding="utf-8")
        SCRIPT.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: 2 files (1 new)")


if __name__ == "__main__":
    main()

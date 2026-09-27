#!/usr/bin/env python3
"""🔁️ Z4 prepared set for window 3 — the devcontainer lifecycle leaves Bash (AGENTS.md: permanent scripts only in
`📜️script.ts`): `postStartCommand`/`postAttachCommand` run `bun ./📜️script.ts setup devcontainer start|attach`, implemented
by `⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts` (injected host, one argv per program, no shell) and proven by
`🧪️tests/🔁️lifecycle` + fixture (fast-xml-parser and lodash oracles, recording hosts). Retires `.devcontainer/post-start.sh`,
`post-attach.sh`, `gitkraken-launch.sh` (→ `setup devcontainer gitkraken`) and the two laws that parsed them
(`🧩️extension-attach` executed a Bash block, `🔒️persistent-state` grepped the start script: both contracts now hold on the
implementation). Also fixes what the Bash carried stale: the VSIX path (`🔨️modules/🧩️vscode` → `💻️client/🧩️vscode`, so
attach could never find the package), `safe.directory` appended again on every start, an agent with no identity restarted on
every start, and a venv `source` that could never reach the person's shell (dropped).

usage: apply.py <repo root> [--apply]   (dry run by default; idempotent)
"""
import os, shutil, sys

ROOT = os.path.abspath(sys.argv[1])
APPLY = "--apply" in sys.argv[2:]
HERE = os.path.dirname(os.path.abspath(__file__))
C = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers"
COPIES = [
    (os.path.join(HERE, "module", "🟦️.ts"), f"{C}/🔁️lifecycle/🟦️.ts"),
    (os.path.join(HERE, "law", "🟦️.ts"), f"{C}/🧪️tests/🔁️lifecycle/🟦️.ts"),
    (os.path.join(HERE, "fixture", "🔣️.json"), f"{C}/🧫️fixtures/🔁️lifecycle/🔣️.json"),
]
RETIRED = [".devcontainer/post-start.sh", ".devcontainer/post-attach.sh", ".devcontainer/gitkraken-launch.sh", f"{C}/🧪️tests/🧩️extension-attach/🟦️.ts", f"{C}/🧫️fixtures/🧩️extension-attach/🔣️.json", f"{C}/🧪️tests/🔒️persistent-state/🟦️.ts", f"{C}/🧫️fixtures/🔒️persistent-state/🔣️.json"]
CONTRACTS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts"
LIFECYCLE_IMPORT = 'import { devcontainerAttach, devcontainerStart, launchGitKraken, processLifecycleContext, processLifecycleHost } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts";\n'
HUNKS = [
    ("📜️script.ts", 'import { repoCacheDirectory } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";\n', 'import { repoCacheDirectory } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";\n' + LIFECYCLE_IMPORT),
    ("📜️script.ts", '''        prepare: () => console.log("[prepare] Nx prerequisites completed"),
      },
      "bun ./📜️script.ts setup [postinstall|git|native]",
    );
  }
''', '''        prepare: () => console.log("[prepare] Nx prerequisites completed"),
        devcontainer: (rest) => this.runDevcontainer(rest),
      },
      "bun ./📜️script.ts setup [postinstall|git|native|devcontainer <start|attach|gitkraken>]",
    );
  }

  /** @emoji 🔁️ The devcontainer lifecycle `.devcontainer/devcontainer.json` runs: `start` on every container start, `attach` on
   * every editor attach, `gitkraken` to open GitKraken Desktop on the checkout. */
  private async runDevcontainer(segments: string[]): Promise<void> {
    const host = processLifecycleHost(), context = processLifecycleContext(this.root);
    if (segments[0] === "start" && segments.length === 1) return devcontainerStart(host, context);
    if (segments[0] === "attach" && segments.length === 1) return devcontainerAttach(host, context);
    if (segments[0] === "gitkraken" && segments.length === 1) return launchGitKraken(host, context);
    throw new Error("usage: bun ./📜️script.ts setup devcontainer <start|attach|gitkraken>");
  }
'''),
    (".devcontainer/devcontainer.json", '''  "postStartCommand": "bash .devcontainer/post-start.sh",
  "postAttachCommand": "bash .devcontainer/post-attach.sh",''', '''  "postStartCommand": ["bun", "./📜️script.ts", "setup", "devcontainer", "start"],
  "postAttachCommand": ["bun", "./📜️script.ts", "setup", "devcontainer", "attach"],'''),
    (".devcontainer/devcontainer.json", '"SSH_AUTH_SOCK": "/home/vscode/.ssh/compose-ssh-agent.sock"', '"SSH_AUTH_SOCK": "/home/vscode/.ssh/semio-ssh-agent.sock"'),
    (f"{C}/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json", '"retiredScripts": [".devcontainer/post-create.sh"],', '"retiredScripts": [".devcontainer/post-create.sh", ".devcontainer/post-start.sh", ".devcontainer/post-attach.sh", ".devcontainer/gitkraken-launch.sh"],'),
    (CONTRACTS, '''  testContainerPersistentState(workspace);
  testExtensionAttach(workspace, output);
''', '''  await testDevcontainerLifecycle(workspace, output);
'''),
]
IMPORT_FILTER = ("testContainerPersistentState", "testExtensionAttach")
README_OLD = ".devcontainer/README.md"


def main() -> None:
    texts, report, failed = {}, [], False
    for rel, old, new in HUNKS:
        path = os.path.join(ROOT, rel)
        if rel not in texts: texts[rel] = open(path, encoding="utf-8").read()
        text = texts[rel]
        if text.count(new) == 1 and (old in new or old not in text): state = "applied"
        elif text.count(old) == 1 and new not in text: texts[rel] = text.replace(old, new); state = "ready"
        else: state = f"CONFLICT (old x{text.count(old)}, new x{text.count(new)})"; failed = True
        report.append((state, rel))
    contracts = texts.get(CONTRACTS, "")
    imports = [line for line in contracts.split("\n") if line.startswith("import ") and any(name in line for name in IMPORT_FILTER)]
    lifecycle_import = 'import { testDevcontainerLifecycle } from "../../📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts";'
    if lifecycle_import in contracts and not imports: report.append(("applied", f"{CONTRACTS} imports"))
    elif len(imports) == 2 and lifecycle_import not in contracts:
        texts[CONTRACTS] = contracts.replace(imports[0] + "\n", lifecycle_import + "\n").replace(imports[1] + "\n", "")
        report.append(("ready", f"{CONTRACTS} imports"))
    else: report.append((f"CONFLICT ({len(imports)} retired imports)", f"{CONTRACTS} imports")); failed = True
    for source, rel in COPIES:
        target = os.path.join(ROOT, rel)
        state = "applied" if os.path.exists(target) and open(target, "rb").read() == open(source, "rb").read() else "CONFLICT (differs)" if os.path.exists(target) else "ready"
        failed |= state.startswith("CONFLICT")
        report.append((state, f"create {rel}"))
    for rel in RETIRED: report.append(("ready" if os.path.exists(os.path.join(ROOT, rel)) else "applied", f"retire {rel}"))
    for state, what in report: print(f"{state:9} {what}")
    if failed: sys.exit("dry run found conflicts; nothing written")
    if not APPLY: return
    for rel, text in texts.items():
        with open(os.path.join(ROOT, rel), "w", encoding="utf-8") as handle: handle.write(text)
    for source, rel in COPIES:
        os.makedirs(os.path.dirname(os.path.join(ROOT, rel)), exist_ok=True)
        shutil.copyfile(source, os.path.join(ROOT, rel))
    for rel in RETIRED:
        path = os.path.join(ROOT, rel)
        if os.path.exists(path): os.remove(path)
        parent = os.path.dirname(path)
        if os.path.isdir(parent) and not os.listdir(parent): os.rmdir(parent)
    print("applied")


main()

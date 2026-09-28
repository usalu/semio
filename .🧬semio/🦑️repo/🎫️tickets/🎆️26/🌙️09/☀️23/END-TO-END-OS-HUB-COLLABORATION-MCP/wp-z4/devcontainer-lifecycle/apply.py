#!/usr/bin/env python3
"""🔁️ Z4 prepared set for window 3 — the devcontainer lifecycle leaves Bash (AGENTS.md: permanent scripts only in
`📜️script.ts`): `postStartCommand`/`postAttachCommand` run `bun ./📜️script.ts setup devcontainer start|attach`, implemented
by `⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts` (injected host, one argv per program, no shell; the root script
imports it lazily so no other verb loads it) and proven by `🧪️tests/🔁️lifecycle` + fixture (fast-xml-parser and lodash
oracles, recording hosts). Retires `.devcontainer/post-start.sh`, `post-attach.sh`, `gitkraken-launch.sh`
(→ `setup devcontainer gitkraken`), the two laws that parsed them (`🧩️extension-attach` executed a Bash block,
`🔒️persistent-state` grepped the start script: both contracts now hold on the implementation) and the two Go copies of
`TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap` that ran `bash .devcontainer/post-attach.sh` (their
GitKraken create/complete/set cases move into the lifecycle law; their Windsurf/Codex MCP assertions were already red —
MCP client configs are repository files under the root policy since 2026-04-12). Also fixes what the Bash carried stale:
the VSIX path (`🔨️modules/🧩️vscode` → `💻️client/🧩️vscode`), `safe.directory` appended again on every start, an agent with no
identity restarted on every start, a venv `source` that could never reach the person's shell (dropped), and the dead
`SEMIO_GITKRAKEN_AUTO_START`/`SEMIO_F3D_AUTO_START` variables (devcontainer + Windows bootstrap); the README is rewritten
(guarded by the pre-image digest).

usage: apply.py <repo root> [--apply]   (dry run by default; idempotent)
"""
from __future__ import annotations
import hashlib, os, shutil, sys

ROOT = os.path.abspath(sys.argv[1])
APPLY = "--apply" in sys.argv[2:]
HERE = os.path.dirname(os.path.abspath(__file__))
C = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers"
COPIES = [
    (os.path.join(HERE, "module", "🟦️.ts"), f"{C}/🔁️lifecycle/🟦️.ts"),
    (os.path.join(HERE, "law", "🟦️.ts"), f"{C}/🧪️tests/🔁️lifecycle/🟦️.ts"),
    (os.path.join(HERE, "fixture", "🔣️.json"), f"{C}/🧫️fixtures/🔁️lifecycle/🔣️.json"),
]
README = (os.path.join(HERE, "readme", "README.md"), ".devcontainer/README.md", "d5135b350bd6c66240f5531885756ac0e38cac0a62d0e013cbfd3d9eb4818367")
RETIRED = [".devcontainer/post-start.sh", ".devcontainer/post-attach.sh", ".devcontainer/gitkraken-launch.sh", f"{C}/🧪️tests/🧩️extension-attach/🟦️.ts", f"{C}/🧫️fixtures/🧩️extension-attach/🔣️.json", f"{C}/🧪️tests/🔒️persistent-state/🟦️.ts", f"{C}/🧫️fixtures/🔒️persistent-state/🔣️.json"]
CONTRACTS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts"
GO_EXHAUSTIVE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔭️exhaustive_test.go"
GO_COMPONENT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go"
GO_TEST = "func TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap(t *testing.T) {\n"
HUNKS = [
    ("📜️script.ts", '''        prepare: () => console.log("[prepare] Nx prerequisites completed"),
      },
      "bun ./📜️script.ts setup [postinstall|git|native]",
    );
  }
''', '''        prepare: () => console.log("[prepare] Nx prerequisites completed"),
        devcontainer: (rest) => this.runDevcontainer(rest),
      },
      "bun ./📜️script.ts setup [postinstall|git|native|deps|prepare|devcontainer <start|attach|gitkraken>]",
    );
  }

  /** @emoji 🔁️ The devcontainer lifecycle `.devcontainer/devcontainer.json` runs: `start` on every container start, `attach` on
   * every editor attach, `gitkraken` to open GitKraken Desktop on the checkout. Loaded on demand, so no other verb pays for it.
   * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts */
  private async runDevcontainer(segments: string[]): Promise<void> {
    const lifecycle = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts");
    const host = lifecycle.processLifecycleHost(), context = lifecycle.processLifecycleContext(this.root);
    await dispatchSubcommand(
      segments,
      { start: () => lifecycle.devcontainerStart(host, context), attach: () => lifecycle.devcontainerAttach(host, context), gitkraken: () => lifecycle.launchGitKraken(host, context) },
      "bun ./📜️script.ts setup devcontainer <start|attach|gitkraken>",
    );
  }
'''),
    (".devcontainer/devcontainer.json", '''  "postStartCommand": "bash .devcontainer/post-start.sh",
  "postAttachCommand": "bash .devcontainer/post-attach.sh",''', '''  "postStartCommand": ["bun", "./📜️script.ts", "setup", "devcontainer", "start"],
  "postAttachCommand": ["bun", "./📜️script.ts", "setup", "devcontainer", "attach"],'''),
    (".devcontainer/devcontainer.json", '"SSH_AUTH_SOCK": "/home/vscode/.ssh/compose-ssh-agent.sock"', '"SSH_AUTH_SOCK": "/home/vscode/.ssh/semio-ssh-agent.sock"'),
    (".devcontainer/devcontainer.json", '''    "SEMIO_GITKRAKEN_WORKSPACE_NAME": "compose",
    "SEMIO_GITKRAKEN_AUTO_START": "false",
    "SEMIO_F3D_AUTO_START": "true",
''', '''    "SEMIO_GITKRAKEN_WORKSPACE_NAME": "compose",
'''),
    ("🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1", '''Set-UserEnvironmentVariable -Name "SEMIO_GITKRAKEN_WORKSPACE_NAME" -Value "compose"
Set-UserEnvironmentVariable -Name "SEMIO_GITKRAKEN_AUTO_START" -Value "false"
Set-UserEnvironmentVariable -Name "SEMIO_F3D_AUTO_START" -Value "true"
''', '''Set-UserEnvironmentVariable -Name "SEMIO_GITKRAKEN_WORKSPACE_NAME" -Value "compose"
'''),
    (f"{C}/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json", '"retiredScripts": [".devcontainer/post-create.sh"],', '"retiredScripts": [".devcontainer/post-create.sh", ".devcontainer/post-start.sh", ".devcontainer/post-attach.sh", ".devcontainer/gitkraken-launch.sh"],'),
    (CONTRACTS, '''  testContainerPersistentState(workspace);
  testExtensionAttach(workspace, output);
''', '''  await testDevcontainerLifecycle(workspace, output);
'''),
    (GO_EXHAUSTIVE, '''	filepath "path/filepath"
	runtime "runtime"
	strconv "strconv"
''', '''	filepath "path/filepath"
	strconv "strconv"
'''),
]
IMPORT_FILTER = ("testContainerPersistentState", "testExtensionAttach")


def go_function_span(text: str) -> tuple[int, int] | None:
    """🧭️ The retired Go test function plus the blank line after it, as one span (start marker to its column-0 `}`)."""
    if text.count(GO_TEST) != 1: return None
    start = text.index(GO_TEST)
    end = text.index("\n}\n", start) + 3
    return start, end + (1 if text[end:end + 1] == "\n" else 0)


def main() -> None:
    texts, report, failed = {}, [], False
    read = lambda rel: texts.setdefault(rel, open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read())
    for rel, old, new in HUNKS:
        text = read(rel)
        if text.count(new) == 1 and old in new: state = "applied"
        elif text.count(old) == 1: texts[rel] = text.replace(old, new); state = "ready"
        elif old not in text and text.count(new) == 1: state = "applied"
        else: state = f"CONFLICT (old x{text.count(old)}, new x{text.count(new)})"; failed = True
        report.append((state, rel))
    for rel in (GO_EXHAUSTIVE, GO_COMPONENT):
        text = read(rel)
        span = go_function_span(text)
        if span is None and GO_TEST not in text: report.append(("applied", f"retire Go test in {rel}"))
        elif span is None: report.append(("CONFLICT (Go test declared twice)", rel)); failed = True
        else:
            removed = text[span[0]:span[1]]
            if "bash\", \".devcontainer/post-attach.sh\")" not in removed or removed.count("\nfunc ") != 0: report.append(("CONFLICT (span guard)", rel)); failed = True
            else: texts[rel] = text[:span[0]] + text[span[1]:]; report.append((f"ready     ({removed.count(chr(10))} lines)", f"retire Go test in {rel}"))
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
    source, rel, before = README
    live = open(os.path.join(ROOT, rel), "rb").read()
    state = "applied" if live == open(source, "rb").read() else "ready" if hashlib.sha256(live).hexdigest() == before else "CONFLICT (README changed since the patch was prepared)"
    failed |= state.startswith("CONFLICT")
    report.append((state, f"rewrite {rel}"))
    for rel in RETIRED: report.append(("ready" if os.path.exists(os.path.join(ROOT, rel)) else "applied", f"retire {rel}"))
    for state, what in report: print(f"{state:9} {what}")
    if failed: sys.exit("dry run found conflicts; nothing written")
    if not APPLY: return
    for rel, text in texts.items():
        with open(os.path.join(ROOT, rel), "w", encoding="utf-8", newline="") as handle: handle.write(text)
    for source, rel in COPIES + [README[:2]]:
        os.makedirs(os.path.dirname(os.path.join(ROOT, rel)), exist_ok=True)
        shutil.copyfile(source, os.path.join(ROOT, rel))
    for rel in RETIRED:
        path = os.path.join(ROOT, rel)
        if os.path.exists(path): os.remove(path)
        parent = os.path.dirname(path)
        if os.path.isdir(parent) and not os.listdir(parent): os.rmdir(parent)
    print("applied")


main()

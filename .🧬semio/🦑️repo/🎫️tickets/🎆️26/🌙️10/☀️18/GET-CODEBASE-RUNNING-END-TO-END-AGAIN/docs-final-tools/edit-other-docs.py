import sys, os
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import Doc

ROOT = os.environ.get("DOCS_ROOT", "/Users/ueli/Documents/semio")
REPO_README = "🧰️framework/🛍️products/🦑️repo/README.md"

# ================================================================ root README.md
R = Doc(ROOT + "/README.md")
R.replace_in("- **Neutral model and two implementations.**", "Anything not yet run on a real host is marked there as not yet verified.", "Source files reach a Bazel action through shared per-directory `filegroup`s, the import closure of every `repo:script` command is a declared input under both tools, and the first real Bazel runs on a scratch host (2026-10-20) showed a cold run, a warm run without a single spawn and a rerun of exactly the task whose declared input changed; a production `bun repo --tool=bazel run` is not yet verified. Anything not yet run on a real host is marked there as not yet verified.")
R.replace_in("- **Caches** are fixed-capacity partitions", "on Windows too a directory (journaled write-through swap, NTFS per-directory case sensitivity; provisional, the bootstrap script and the Bazel readiness check still demand an elevated terminal).", "on Windows too a directory (journaled write-through swap, NTFS per-directory case sensitivity; not run on Windows, and no elevated terminal is demanded except once for the machine-wide long-path setting and the optional Windows feature). macOS images are created silenced (no FSEvents log, no Spotlight index) and the policy is re-applied whenever one is opened.")
R.insert_after("- **Caches** are fixed-capacity partitions", [
"- **Dependencies and waiting.** `dependencies sync --all` took about 31 minutes on 2026-10-20 (an earlier attempt ran 7,000 s without finishing): the verification passes are no longer quadratic. Ordinary `bun repo` calls check a sealed dependency generation receipt (seconds) instead of walking the workspace several times (36 to 58 s per tree pass), and fall back to the complete walk on any mismatch; a sealed receipt has not been seen in a live run yet. A new workspace package cannot be resolved until `dependencies lock --all` and `dependencies sync --all` ran, and the tree must stay untouched while a sync runs. A wait for a cache image shows a `lease-queue` line with position, holder pid and mode; a `remains active on …` line is a running walk, not a wait. Details and open items are in the [repo docs](🧰️framework/🛍️products/🦑️repo/README.md#-build-and-run).",
"- **CI.** `.github/workflows/build-system.yml` runs on pull requests, pushes to `main` and by hand on Ubuntu, macOS and Windows: pinned Bun with a verified SHA-256, `bun repo --help`, `preflight`, `tools setup all`, `dependencies sync --all`, a cache proof for Nx and Bazel on `@teaching/architecture-course:test`, and that target under both tools. It has never run.",
])
R.append("- **Developer dashboard.**", " Start it with `bun run dashboard` (`AGENTS.md` calls it `repo dashboard`); builds, tests and checks by agents and automation use `bun repo …`, and no editor or IDE configuration registers commands.")
R.append("- **Zero-touch.**", " `bun ./📜️script.ts verify zero-touch` is the law behind that rule: it walks nine closures and fails on a bare workspace specifier or an external package.")
R.save()

# ================================================================ SKILL.md
S = Doc(ROOT + "/.agents/skills/build/SKILL.md")
S.insert_after("bun ./📜️script.ts verify build-owners [sync]", ["bun ./📜️script.ts verify zero-touch                  # CLI import closures stay relative and node_modules-free"])
S.replace_line("- Register human developer controls in the owning `📋️project.json` dashboard metadata and expose them through the dashboard. The dashboard is the sole human developer control surface; do not create IDE launch, task, solution, or workspace configurations. This newer developer request supersedes older launch registration guidance in repository instructions.",
"- Register human developer controls in the owning `📋️project.json` dashboard metadata. The dashboard (`AGENTS.md`: `repo dashboard`; started with `bun run dashboard`, which runs `bun repo run @semio-tech/repo-dashboard-rs:run`) is the sole human control surface and there is no `bun repo dashboard` verb. Agents never start it and use `bun repo …`; do not create IDE launch, task, solution or workspace configurations.")
S.append("- Manifest targets:", " The import closure of a `repo:script` command is derived (relative and workspace-package imports, literal `new URL(\"…\", import.meta.url)` targets, declared test modules) and is a declared input under Nx and Bazel; write no hand lists for imports. A Bazel `cacheable` target without an `outputs` array runs uncached (`unbounded`).")
S.insert_after("- Never call `nx`, `bunx nx`", [
"- A new workspace package (new `package.json`) cannot be resolved until `dependencies lock --all` and `dependencies sync --all` ran; while a sync runs, edit nothing (a provider-file edit fails it). Ordinary commands verify the sealed dependency generation receipt in `.cache/b/d/m/generation`; an edit of a file in its import closure costs the next command one complete walk. Waiting on a cache image prints a `lease-queue` line (position, holder pid, mode); `remains active on dependencies:store-ownership:state:<N>` is a running walk, not a wait.",
"- Bazel: the generated workspace writes no `.bazelignore` (it disabled all caching) and is queried with `//:all`, never `//...`; the adapter has not run a product through `bun repo --tool=bazel run` yet.",
"- Cargo: small `bun repo cargo` invocations one after the other, never beside `dependencies sync` or `show projects`; compile windows over many crates are sequenced by a ticket tool, not by a `bun repo` command.",
"- Script entries: a `📜️script.ts` is either a router (`receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: [\"<command>\"] }))`) or a bare `if (import.meta.main) await runScriptProgram();` for programs that own their process birth (test children under `🧪️tests` and `👷️child`); a router around such a child fails the outer birth.",
"- Start Playwright with Bun, never `node` (`ERR_UNSUPPORTED_ESM_URL_SCHEME`); Vite configs of products that reach the build tool closure need `configLoader: \"native\"`.",
])
S.append("Inspect before touching code:", " Windows partitions are directories too (`fixed.dir`, `v`, `m`) swapped by a journaled rename with an intent receipt `swap.json`; no VHD, no elevation, and an interrupted swap is recovered by the next exclusive opener. macOS images are silenced at creation (`.fseventsd/no_log`, `.metadata_never_index`); a growing `.fseventsd` is pruned at open (item 8 of Recovery).")
S.save()

# ================================================================ .devcontainer/README.md
V = Doc(ROOT + "/.devcontainer/README.md")
V.append("The service declares no `privileged`, capability or device;", " The partitions hold the dependency generation receipt (`.cache/b/d/m/generation`) and the Bazel disk cache (`.cache/b/p/m/bazel/actions`), so a container that keeps the `.cache` volume starts warm; the macOS-only image silencing (`.fseventsd`) does not apply on Linux.")
V.append("The dashboard is the exclusive developer control plane", " It is the `repo dashboard` of `AGENTS.md`; there is no `bun repo dashboard` verb, and agents and automation use `bun repo …` instead.")
V.save()
print("other docs ok")

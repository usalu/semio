#!/usr/bin/env python3
"""🏗️ Builds the staged batch `ci1` of lane cicover from the CURRENT live files: `python3 -I mkbatch.py [--root DIR] [--batch NAME]`.
Every edit is anchored (an anchor that is not found exactly once stops the build with exit 2), new files come from `src/`. Rebuild before applying when a hot file moved."""
import hashlib, json, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "src")
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
BATCH = sys.argv[sys.argv.index("--batch") + 1] if "--batch" in sys.argv else "ci1"
OUT = os.path.join(HERE, "staged", BATCH)
WF = ".github/workflows/"
ACT = ".github/actions/"
CI = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/"
LAW = CI + "🗂️suites/🧪️tests/🔀️workflow/🟦️.ts"
DASH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/"
def sha(data): return hashlib.sha256(data).hexdigest()
def read(rel): return open(os.path.join(ROOT, rel), encoding="utf8").read()
def sub(text, old, new, count=1):
    found = text.count(old)
    if found != count: sys.exit(f"anchor found {found} times, expected {count}: {old[:100]!r}")
    return text.replace(old, new)
def pins(text):
    for label, pin in (("ubuntu-latest", "ubuntu-24.04"), ("macos-latest", "macos-26"), ("windows-latest", "windows-2025")):
        text = text.replace(label, pin)
    return text
def src(name): return open(os.path.join(SRC, name), encoding="utf8").read()
TOOL_CACHE = lambda mode, name, paths, hashing, indent, extra="": (
    f"{indent}- name: {extra}\n{indent}  uses: ./.github/actions/tool-cache\n{indent}  with:\n{indent}    mode: {mode}\n{indent}    name: {name}\n{indent}    paths: {paths}\n{indent}    hash: {hashing}\n")
GO_CACHE_HASH = "${{ hashFiles('go.work', '🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🔣️.json') }}"
BUN_HASH = "${{ hashFiles('bun.lock', '*/bun.lock', 'package.json') }}"
BUN_RESTORE = TOOL_CACHE("restore", "bun-store", ".cache/b/d/m/bun-cache", BUN_HASH, "      ", "Restore the Bun store")
GO_DIR = ".🧬semio/🦑️repo/⚡️cache/go"

def build_system(t):
    t = pins(t)
    t = sub(t, "# request, every push to main and by hand.", "# request, every push to the default branch (`main` and `⛳wip`) and by hand.")
    t = sub(t, "# Language: the entry has no default language", """# Runners and timeouts: every runner image is pinned to a version (ubuntu-24.04, macos-26, windows-2025), never to a `-latest` alias, so that the migration
# of an alias (ubuntu-latest moves to 26.04) is a reviewed change; a law (the workflow tests of the suite sweep) rejects an alias. Timeouts follow the policy of the
# suite sweep: a shard has `maximumShardBudgetMinutes` plus the slack of one suite, a sweep job gets that plus 50 minutes of provisioning and never the hosted
# maximum of 360, and every step that downloads or installs has its own limit of 30 minutes, so a hung download ends one step instead of holding a runner for hours.
#
# Language: the entry has no default language""")
    t = sub(t, "The Bun store always travels.", "The Bun store always travels; the launcher runtimes and the Go toolchain of the installer travel through the `repo-launcher` action and every other tool cache through ./.github/actions/tool-cache (saved only by the default branch).")
    t = sub(t, "  push:\n    branches: [main]\n", '  push:\n    branches: [main, "⛳wip"]\n')
    t = sub(t, "      base:\n        description: Revision to compare against (scope affected)\n        type: string\n        default: origin/main\n",
            '      base:\n        description: Revision to compare against (scope affected); empty means the default branch of the repository\n        type: string\n        default: ""\n')
    t = sub(t, "          DISPATCH_BASE: ${{ inputs.base }}\n", "          DISPATCH_BASE: ${{ inputs.base }}\n          DEFAULT_BRANCH: ${{ github.event.repository.default_branch }}\n")
    t = sub(t, '--base=${DISPATCH_BASE:-origin/main} --head=HEAD', '--base=${DISPATCH_BASE:-origin/$DEFAULT_BRANCH} --head=HEAD')
    t = sub(t, "timeout-minutes: 350", "timeout-minutes: 240", 2)
    t = sub(t, "    name: devcontainer (must-run suites)\n    needs: [smoke]\n    runs-on: ubuntu-24.04\n    timeout-minutes: 120\n", "    name: devcontainer (must-run suites)\n    needs: [smoke]\n    runs-on: ubuntu-24.04\n    timeout-minutes: 90\n")
    t = sub(t, "      - name: Install the authored dependency scopes\n        run: repo dependencies sync --all\n      - name: Native storage laws",
            BUN_RESTORE + "      - name: Install the authored dependency scopes\n        run: repo dependencies sync --all\n      - name: Native storage laws")
    t = sub(t, "        run: repo tools verify\n      - name: Install the authored dependency scopes\n        run: repo dependencies sync --all\n      - name: Run the suites that must really run",
            "        run: repo tools verify\n" + BUN_RESTORE + "      - name: Install the authored dependency scopes\n        run: repo dependencies sync --all\n      - name: Run the suites that must really run")
    t, count = re.subn(r"^(        run: repo (?:tools setup [^\n]*|dependencies sync --all))\n", r"\1\n        timeout-minutes: 30\n", t, flags=re.M)
    if count != 14: sys.exit(f"step timeouts: only {count} provisioning steps found")
    t = sub(t, "          target: ${{ env.PROOF_TARGET }}\n", "          target: ${{ env.PROOF_TARGET }}\n        timeout-minutes: 45\n", 2)
    return t

def dashboard(t):
    t = sub(t, "    branches: [main]\n", '    branches: [main, "⛳wip"]\n')
    t = sub(t, "  SEMIO_BUILD_BUDGET_MS: '21600000'\n", "  SEMIO_BUILD_BUDGET_MS: '9000000'\n")
    t = sub(t, "timeout-minutes: 360", "timeout-minutes: 180", 2)
    t = pins(t)
    go = TOOL_CACHE("restore", "go-build", GO_DIR, GO_CACHE_HASH, "      ", "Restore the Go build cache")
    t = sub(t, "      - uses: ./.github/actions/native-repo\n", go + "      - uses: ./.github/actions/native-repo\n")
    save = TOOL_CACHE("save", "go-build", GO_DIR, GO_CACHE_HASH, "      ", "Save the Go build cache (default branch)")
    t = sub(t, "      - name: Retain Native Dashboard Evidence\n", save + "      - name: Retain Native Dashboard Evidence\n")
    return t

LAUNCHER_PATHS = "|\n          ${{ runner.os == 'Windows' && '~\\AppData\\Local\\semio\\runtimes' || runner.os == 'macOS' && '~/Library/Caches/semio/runtimes' || '~/.cache/semio/runtimes' }}\n          ${{ runner.os == 'Windows' && '~\\AppData\\Local\\semio\\go' || '~/.local/share/semio/go' }}"
LAUNCHER_HASH = "${{ hashFiles('🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json', '🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json') }}"
def launcher_cache(mode, label):
    return (f"    - name: {label}\n      uses: ./.github/actions/tool-cache\n      with:\n        mode: {mode}\n        name: launcher\n        paths: " + LAUNCHER_PATHS.replace("\n          ", "\n          ") + f"\n        hash: {LAUNCHER_HASH}\n")
def repo_launcher(t):
    t = sub(t, "runs:\n  using: composite\n  steps:\n", "runs:\n  using: composite\n  steps:\n" + launcher_cache("restore", "Restore the launcher runtimes and the installer's toolchain"))
    if not t.endswith("run: repo --version\n"): sys.exit("repo-launcher: last step moved")
    return t + launcher_cache("save", "Save the launcher runtimes and the installer's toolchain (default branch)")

def budget(t, minutes_ms, text):
    t = sub(t, "runs:\n  using: composite\n", f"  budget-ms:\n    description: {text}\n    required: false\n    default: \"{minutes_ms}\"\n\nruns:\n  using: composite\n") if "inputs:" in t else sub(t, "runs:\n  using: composite\n", f"inputs:\n  budget-ms:\n    description: {text}\n    required: false\n    default: \"{minutes_ms}\"\n\nruns:\n  using: composite\n")
    return sub(t, "SEMIO_BUILD_BUDGET_MS=21600000", "SEMIO_BUILD_BUDGET_MS=${{ inputs.budget-ms }}")
def devcontainer_suites(t):
    t = sub(t, "\n\nruns:\n  using: composite\n", "\n  budget-ms:\n    description: Build budget in milliseconds handed to the container as SEMIO_BUILD_BUDGET_MS; keep it below the timeout of the calling job (default 210 minutes for the 240 minute sweep jobs).\n    required: false\n    default: \"12600000\"\n\nruns:\n  using: composite\n")
    return sub(t, "SEMIO_BUILD_BUDGET_MS=21600000", "SEMIO_BUILD_BUDGET_MS=${{ inputs.budget-ms }}")
def devcontainer_repo(t):
    t = sub(t, "runs:\n  using: composite\n", "inputs:\n  budget-ms:\n    description: Build budget in milliseconds handed to the container as SEMIO_BUILD_BUDGET_MS; keep it below the timeout of the calling job (default 150 minutes for the 180 minute dashboard job).\n    required: false\n    default: \"9000000\"\n\nruns:\n  using: composite\n")
    return sub(t, "SEMIO_BUILD_BUDGET_MS=21600000", "SEMIO_BUILD_BUDGET_MS=${{ inputs.budget-ms }}")

def dependabot(t):
    return sub(t, '      - "/.github/actions/cache-proof"\n', '      - "/.github/actions/cache-proof"\n      - "/.github/actions/tool-cache"\n')

def law(t):
    t = sub(t, '"continue-on-error"?:boolean|string;"working-directory"?:string};', '"continue-on-error"?:boolean|string;"working-directory"?:string;"timeout-minutes"?:number};')
    t = sub(t, 'toEqual({linux:Array(7).fill("ubuntu-latest"),macos:Array(7).fill("macos-latest"),windows:Array(7).fill("windows-latest")});', 'toEqual({linux:Array(7).fill(RUNNERS.linux),macos:Array(7).fill(RUNNERS.macos),windows:Array(7).fill(RUNNERS.windows)});')
    if not t.endswith("\n"): t += "\n"
    return t + src("law-additions.ts")

def dash_fixture(t):
    return pins(t)

EDITS = [
    (WF + "build-system.yml", build_system), (WF + "dashboard.yml", dashboard), (ACT + "repo-launcher/action.yml", repo_launcher),
    (ACT + "devcontainer-suites/action.yml", devcontainer_suites), (ACT + "devcontainer-repo/action.yml", devcontainer_repo),
    (".github/dependabot.yml", dependabot), (LAW, law), (DASH + "🔣️.json", dash_fixture), (DASH + "🧬️schema/🔣️.json", dash_fixture)]
NEW = [(ACT + "tool-cache/action.yml", "tool-cache.yml"), (WF + "go.yml", "go.yml"), (WF + "products.yml", "products.yml")]

def main():
    if os.path.isdir(OUT): shutil.rmtree(OUT)
    os.makedirs(os.path.join(OUT, "files")); os.makedirs(os.path.join(OUT, "base"))
    manifest = {}
    for number, (rel, fn) in enumerate(EDITS):
        before = open(os.path.join(ROOT, rel), "rb").read()
        after = fn(before.decode("utf8")).encode("utf8")
        name = f"{number:02d}{os.path.splitext(rel)[1]}"
        open(os.path.join(OUT, "base", name), "wb").write(before); open(os.path.join(OUT, "files", name), "wb").write(after)
        manifest[rel] = {"file": name, "base": sha(before), "new": sha(after)}
    for number, (rel, source) in enumerate(NEW, start=len(EDITS)):
        if os.path.exists(os.path.join(ROOT, rel)): sys.exit(f"new file exists already: {rel}")
        data = open(os.path.join(SRC, source), "rb").read()
        name = f"{number:02d}{os.path.splitext(rel)[1]}"
        open(os.path.join(OUT, "files", name), "wb").write(data)
        manifest[rel] = {"file": name, "base": None, "new": sha(data)}
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    shutil.copy(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), os.path.join(OUT, "stage.py"))
    shutil.copy(os.path.join(SRC, "verify.sh"), os.path.join(OUT, "verify.sh"))
    print("built", OUT, len(manifest), "files")
main()

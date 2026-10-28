#!/usr/bin/env python3
"""🧪 Negative controls of the workflow laws: `python3 -I negative.py <MIRROR root with the batch applied>`.
Each control seeds one violation into a mirror file, runs the workflow law, requires the named test to fail (and no other), and restores the file. Exit 1 when a control does not bite."""
import os, shutil, subprocess, sys
root = os.path.abspath(sys.argv[1])
law = "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts"
controls = [
    ("an unpinned runner alias", ".github/workflows/go.yml", "os: ubuntu-24.04}", "os: ubuntu-latest}", "every runner image is pinned"),
    ("an action pinned to a tag", ".github/workflows/products.yml", "actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9", "actions/upload-artifact@v7", "every external action is pinned"),
    ("a push filter without the default branch", ".github/workflows/products.yml", 'branches: [main, "⛳wip"]', "branches: [main]", "a push to the default branch starts"),
    ("a sweep job at the hosted maximum", ".github/workflows/build-system.yml", "    name: devcontainer suites (${{ matrix.table }} ${{ matrix.index }}/${{ matrix.count }})\n    needs: [smoke]\n    runs-on: ubuntu-24.04\n    timeout-minutes: 240", "    name: devcontainer suites (${{ matrix.table }} ${{ matrix.index }}/${{ matrix.count }})\n    needs: [smoke]\n    runs-on: ubuntu-24.04\n    timeout-minutes: 350", "every job is bounded"),
    ("a provisioning step without its own limit", ".github/workflows/go.yml", "      - name: Provision the build tool\n        timeout-minutes: 30\n        run: repo tools setup \"$BUILD_TOOL\"", "      - name: Provision the build tool\n        run: repo tools setup \"$BUILD_TOOL\"", "every job is bounded"),
    ("a Windows evidence step that can fail the job", ".github/workflows/go.yml", "        continue-on-error: true\n        timeout-minutes: 10\n        shell: pwsh\n        run: |\n          $ErrorActionPreference = 'Stop'\n          $directory", "        timeout-minutes: 10\n        shell: pwsh\n        run: |\n          $ErrorActionPreference = 'Stop'\n          $directory", "the launcher laws run on Linux"),
    ("the launcher laws on two operating systems only", ".github/workflows/go.yml", "          - {name: windows, os: windows-2025}\n    steps:\n      - name: Enable long paths before checkout (Windows)\n        if: ${{ runner.os == 'Windows' }}\n        shell: pwsh\n        run: |\n          git config --system core.longpaths true\n          git config --system core.autocrlf false\n          New-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem' -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force | Out-Null\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n        with:\n          persist-credentials: false\n      - uses: ./.github/actions/repo-launcher\n        with:\n          node-version: ${{ env.NODE_VERSION }}\n      - name: Restore the Go build cache", "    steps:\n      - name: Enable long paths before checkout (Windows)\n        if: ${{ runner.os == 'Windows' }}\n        shell: pwsh\n        run: |\n          git config --system core.longpaths true\n          git config --system core.autocrlf false\n          New-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem' -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force | Out-Null\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n        with:\n          persist-credentials: false\n      - uses: ./.github/actions/repo-launcher\n        with:\n          node-version: ${{ env.NODE_VERSION }}\n      - name: Restore the Go build cache", "the launcher laws run on Linux"),
    ("a cache saved by every run", ".github/actions/tool-cache/action.yml", "      if: ${{ inputs.mode == 'save' && success() && github.event_name != 'pull_request' && github.ref == format('refs/heads/{0}', github.event.repository.default_branch) && steps.stored.outputs.cache-hit != 'true' }}", "      if: ${{ inputs.mode == 'save' && success() }}", "a cache is saved only by"),
    ("a duplicate artifact name", ".github/workflows/go.yml", "name: go-modules-${{ matrix.name }}-${{ matrix.group }}", "name: go-launcher-${{ matrix.name }}", "the artifact names of every workflow"),
]
def run():
    result = subprocess.run(["bun", "test", law], cwd=root, capture_output=True, text=True)
    return [line for line in (result.stdout + result.stderr).splitlines() if line.startswith("(fail)")], result.returncode
base, code = run()
if code != 0: sys.exit(f"the law is not green before the controls: {base}")
bad = 0
for label, rel, old, new, expected in controls:
    path = os.path.join(root, rel); original = open(path, encoding="utf8").read()
    if original.count(old) < 1: print(f"CONTROL BROKEN ({label}): anchor not found"); bad += 1; continue
    try:
        open(path, "w", encoding="utf8").write(original.replace(old, new, 1)); failed, code = run()
    finally:
        open(path, "w", encoding="utf8").write(original)
    ok = code != 0 and any(expected in line for line in failed)
    print(("bites " if ok else "MISSED ") + label + (" -> " + "; ".join(failed)[:140] if not ok else ""))
    bad += 0 if ok else 1
sys.exit(1 if bad else 0)

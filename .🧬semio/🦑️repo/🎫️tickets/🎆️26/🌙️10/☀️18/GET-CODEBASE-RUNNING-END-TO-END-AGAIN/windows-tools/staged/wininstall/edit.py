#!/usr/bin/env python3
"""🔁 Edits of batch `wininstall` applied in place to a root: python3 -I edit.py <root>. The Windows launcher installer replaces a running `repo.exe` (on Windows the launcher stays the parent of every build, and a running image can be renamed but not overwritten or deleted): it moves the old file aside under a unique name, puts the new one in place, and removes copies moved aside by earlier installs once nothing runs them; a text law pins the order."""
import os, sys
ROOT = sys.argv[1]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
def edit(rel, pairs):
    path = os.path.join(ROOT, rel); text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text: continue
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), rel, old[:70]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8", newline="").write(text)
edit(L + "/📥️install/🔵️.ps1", [
 ("# Saved as UTF-8 with BOM:",
  "# A running `repo.exe` (the launcher stays the parent of every build on Windows) cannot be overwritten or deleted, only renamed: the old file moves aside as `repo.exe.old-<id>` and the next install removes such copies once nothing runs them.\n# Saved as UTF-8 with BOM:"),
 ('    $target = Join-Path $Destination "repo.exe"\n    Copy-Item -LiteralPath (Join-Path $work "repo.exe") -Destination "$target.partial" -Force\n    Move-Item -LiteralPath "$target.partial" -Destination $target -Force\n',
  '    $target = Join-Path $Destination "repo.exe"\n    Get-ChildItem -LiteralPath $Destination -Filter "repo.exe.old-*" -File | ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force -ErrorAction SilentlyContinue }\n    Copy-Item -LiteralPath (Join-Path $work "repo.exe") -Destination "$target.partial" -Force\n    if (Test-Path -LiteralPath $target) { Move-Item -LiteralPath $target -Destination ("$target.old-" + [Guid]::NewGuid().ToString("N").Substring(0, 12)) }\n    Move-Item -LiteralPath "$target.partial" -Destination $target\n'),
])
edit(L + "/🧪️tests/📥️install-contract/🟦️.ts", [
 ('test("both installers hold the same lock around the toolchain install',
  'test("the Windows installer replaces a running launcher: earlier moved-aside copies go first, the new file is staged, the old one is renamed aside (Windows renames a running image but never overwrites or deletes it), then the new one takes its name",()=>{\n  const windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");\n  const steps=[\'-Filter "repo.exe.old-*" -File\',\'-Destination "$target.partial" -Force\',\'Move-Item -LiteralPath $target -Destination ("$target.old-" +\',\'Move-Item -LiteralPath "$target.partial" -Destination $target\\n\'].map(step=>windows.indexOf(step));\n  expect(steps.every(index=>index>0)).toBe(true);expect([...steps].sort((left,right)=>left-right)).toEqual(steps);\n  expect(windows).not.toContain(\'-Destination $target -Force\');expect(windows.charCodeAt(0)).toBe(0xfeff);\n});\n\ntest("both installers hold the same lock around the toolchain install'),
])
edit(L + "/🧪️tests/📥️install-contract/🥒️.feature", [
 ("    And the cancelled run exits with 130 and leaves no partial directory, lock or launcher",
  "    And the cancelled run exits with 130 and leaves no partial directory, lock or launcher\n\n  Scenario: A running launcher is replaced on Windows\n    Given a repo.exe that a build still runs\n    When the Windows installer installs the new launcher\n    Then the old file is renamed aside, the new one takes its name, and the next install removes the moved-aside copy"),
])
print("wininstall edits applied to", ROOT)

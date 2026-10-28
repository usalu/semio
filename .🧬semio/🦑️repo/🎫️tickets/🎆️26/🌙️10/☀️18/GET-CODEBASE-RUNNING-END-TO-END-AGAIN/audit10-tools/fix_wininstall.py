"""🛟 Builds the audit10 copy of batch `wininstall` from the lane's staged bytes (anchored, exit 1 = anchor lost): a failed final move retries 10…1280 ms like the launcher's own transient rename and then puts the moved-aside launcher back, so an install never leaves the destination without `repo.exe`. usage: python3 -I fix_wininstall.py"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
SRC = os.path.join(C, "windows-tools", "staged", "wininstall"); DST = os.path.join(HERE, "staged", "wininstall")
def edit(name, pairs):
    path = os.path.join(DST, "files", name); text = open(path, encoding="utf-8", newline="").read()
    for old, new in pairs:
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), name, old[:80]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8", newline="").write(text)
shutil.rmtree(DST, ignore_errors=True); shutil.copytree(SRC, DST)
edit("00.ps1", [
 ("and the next install removes such copies once nothing runs them.",
  "and the next install removes such copies once nothing runs them. The new file takes the name with the launcher's own transient-rename retry (10…1280 ms, Defender or the indexer may hold a fresh executable); if it still cannot, the old launcher is moved back, so the destination never stays without `repo.exe`."),
 ('    if (Test-Path -LiteralPath $target) { Move-Item -LiteralPath $target -Destination ("$target.old-" + [Guid]::NewGuid().ToString("N").Substring(0, 12)) }\n    Move-Item -LiteralPath "$target.partial" -Destination $target\n',
  '    $aside = $null\n'
  '    if (Test-Path -LiteralPath $target) { $aside = "$target.old-" + [Guid]::NewGuid().ToString("N").Substring(0, 12); Move-Item -LiteralPath $target -Destination $aside }\n'
  '    for ($attempt = 0; ; $attempt++) {\n'
  '        try { Move-Item -LiteralPath "$target.partial" -Destination $target; break }\n'
  '        catch {\n'
  '            if ($attempt -lt 8) { Start-Sleep -Milliseconds ([int](10 * [Math]::Pow(2, $attempt))); continue }\n'
  '            if ($aside) { Move-Item -LiteralPath $aside -Destination $target }\n'
  '            throw\n'
  '        }\n'
  '    }\n'),
])
edit("01.ts", [
 ("""const steps=['-Filter "repo.exe.old-*" -File','-Destination "$target.partial" -Force','Move-Item -LiteralPath $target -Destination ("$target.old-" +','Move-Item -LiteralPath "$target.partial" -Destination $target\\n'].map(step=>windows.indexOf(step));""",
  """const steps=['-Filter "repo.exe.old-*" -File','-Destination "$target.partial" -Force','$aside = "$target.old-" +','Move-Item -LiteralPath $target -Destination $aside','try { Move-Item -LiteralPath "$target.partial" -Destination $target; break }','Start-Sleep -Milliseconds ([int](10 * [Math]::Pow(2, $attempt)))','if ($aside) { Move-Item -LiteralPath $aside -Destination $target }','throw\\n'].map(step=>windows.indexOf(step));"""),
 ("expect(windows).not.toContain('-Destination $target -Force');",
  "expect(windows).not.toContain('-Destination $target -Force');expect(windows).toContain('if ($attempt -lt 8)');"),
 ("then the new one takes its name\",()=>{",
  "then the new one takes its name, retried 10…1280 ms, and a final failure moves the old one back\",()=>{"),
])
edit("02.feature", [
 ("    Then the old file is renamed aside, the new one takes its name, and the next install removes the moved-aside copy",
  "    Then the old file is renamed aside, the new one takes its name, and the next install removes the moved-aside copy\n\n  Scenario: The new launcher cannot take the name on Windows\n    Given a repo.exe that is renamed aside and a new file that Defender still holds\n    When the new file cannot take the name after the retries of 10 to 1280 ms\n    Then the old launcher is moved back and the install fails, so repo.exe is never missing"),
])
manifest = json.load(open(os.path.join(DST, "manifest.json"), encoding="utf-8"))
for rel, row in manifest.items():
    row["new"] = hashlib.sha256(open(os.path.join(DST, "files", row["file"]), "rb").read()).hexdigest()
json.dump(manifest, open(os.path.join(DST, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("audit10 wininstall built:", DST)

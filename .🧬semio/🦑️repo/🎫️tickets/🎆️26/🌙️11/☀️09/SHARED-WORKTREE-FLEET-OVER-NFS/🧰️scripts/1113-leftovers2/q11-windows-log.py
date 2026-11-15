#!/usr/bin/env python3
"""q11: Windows entry shim captures the boot program output into a size-limited log (X21). usage: q11-windows-log.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
def edit(path,fn):
    p=root+"/"+path
    s=open(p,encoding="utf8").read(); s=fn(s); open(p,"w",encoding="utf8").write(s)
def ps(s):
    s=sub(s,"$env:SEMIO_FLEET_ENTRY = 'boot'\n","$env:SEMIO_FLEET_ENTRY = 'boot'\n$log = Join-Path $dir 'run.log'\n$script:lines = 0\nfunction Limit-Log { if ((Test-Path -LiteralPath $log) -and ((Get-Item -LiteralPath $log).Length -gt 8MB)) { $tail = Get-Content -LiteralPath $log -Tail 20000; Set-Content -LiteralPath $log -Value $tail } }\nfunction Write-Log([string]$Text) { Add-Content -LiteralPath $log -Value $Text -ErrorAction SilentlyContinue; $script:lines++; if ($script:lines % 500 -eq 0) { Limit-Log } }\nLimit-Log\n")
    s=sub(s,"function Write-Note([string]$Text) { $line = \"$((Get-Date).ToUniversalTime().ToString('s'))Z $Text\"; Write-Output $line; Set-Content -Path (Join-Path $dir 'shim.txt') -Value $line }","function Write-Note([string]$Text) { $line = \"$((Get-Date).ToUniversalTime().ToString('s'))Z $Text\"; Write-Output $line; Write-Log $line; Set-Content -Path (Join-Path $dir 'shim.txt') -Value $line }")
    s=sub(s,"    & $Runtime $Program @Rest\n    $code = $LASTEXITCODE","    & $Runtime $Program @Rest 2>&1 | ForEach-Object { $text = \"$_\"; Write-Output $text; Write-Log $text }\n    $code = $LASTEXITCODE")
    s=s.replace("# Service entry shim (installed as boot.ps1): finds a runtime for the boot program (boot.mjs, then boot.prev.mjs) and keeps trying;","# Service entry shim (installed as boot.ps1): finds a runtime for the boot program (boot.mjs, then boot.prev.mjs) and keeps trying; everything the boot program prints is also appended to run.log beside it (cut to its last 20000 lines above 8 MiB), because a scheduled task keeps no output;")
    return s
edit("🔁️replica/🥾️entry/🔵️.ps1",ps)
def tests(s):
    return s.rstrip("\n")+'''

test("the Windows entry shim keeps the output of the boot program in a size-limited log beside it (checked as text, simulated: no PowerShell here)",()=>{
 const text=readFileSync(join(sourceDirectory,"🔵️.ps1"),"utf8");
 expect(text).toContain("$log = Join-Path $dir 'run.log'");
 expect(text).toContain("& $Runtime $Program @Rest 2>&1 | ForEach-Object");
 expect(text).toContain("Write-Log $text");
 expect(text).toContain("-gt 8MB");
 expect(text).toContain("-Tail 20000");
 expect(text.indexOf("Limit-Log\\n")).toBeLessThan(text.indexOf("function Get-CachedBun"));
 expect(text).toContain("exit $code");
});
'''
edit("🔁️replica/🥾️entry/🧪️tests/🟦️.ts",tests)
print("ok")

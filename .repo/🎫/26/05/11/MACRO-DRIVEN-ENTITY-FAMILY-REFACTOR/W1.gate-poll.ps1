# W1 gate: W0.complete + cargo check (compose/rs). Default max 4h = 320 * 45s.
param([int]$MaxIter = 320)
$ticket = Split-Path -Parent $MyInvocation.MyCommand.Path
$maxIter = $MaxIter
$pollLog = Join-Path $ticket 'W1.gate-poll.log'
for ($iter = 1; $iter -le $maxIter; $iter++) {
    $w0path = Join-Path $ticket 'W0.complete'
    $w0 = Test-Path $w0path
    "$(Get-Date -Format o) iter=$iter/$maxIter W0.complete=$w0" | Out-File -Append $pollLog
    if ($w0) {
        Push-Location 'c:\git\compose\compose\rs'
        cargo check *> (Join-Path $ticket 'W1.gate-cargo.log') 2>&1
        $code = $LASTEXITCODE
        Pop-Location
        if ($code -eq 0) {
            'OK' | Out-File (Join-Path $ticket 'W1.gate-passed.marker')
            exit 0
        }
    }
    if ($iter -lt $maxIter) { Start-Sleep -Seconds 45 }
}
exit 1

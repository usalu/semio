<#
.SYNOPSIS
🛰️ Round 31 observer for g1-m05 (Windows): starts no repo command and changes nothing outside this folder. Every 10 minutes it appends to round-31.md the machine's ready.json, the last 30 lines of the running stage's log, the free memory (the Windows stand-in for MemAvailable), the five largest processes and a 10 s file-operation sample of the running stage's process tree. Ends when ready.json says ready, when ROUND.md or next.md changes, or after the given hours. Usage: round-31-observe.ps1 [hours]
.LINK
../../ROUND.md
#>
param([double]$Hours = 6)
$ErrorActionPreference = 'SilentlyContinue'
$ready = 'C:\semio\.cache\f\ready'
$status = 'S:\v1\origins\g1-m05\ready.json'
$report = Join-Path $PSScriptRoot 'round-31.md'
$roundFile = Join-Path $PSScriptRoot '..\..\ROUND.md'
$nextFile = Join-Path $PSScriptRoot 'next.md'
function Cut([string]$Text, [int]$Length) { if ($Text.Length -gt $Length) { $Text.Substring(0, $Length) } else { $Text } }
function Round-Line { Get-Content -LiteralPath $roundFile -TotalCount 1 }
function Next-Mark { if (Test-Path -LiteralPath $nextFile) { (Get-FileHash -LiteralPath $nextFile).Hash } else { '' } }
function Ready-State { Get-Content -LiteralPath $status -Raw | ConvertFrom-Json }
function Stage-Tree([int]$Root) {
    $all = Get-CimInstance Win32_Process
    $ids = [System.Collections.Generic.List[int]]::new()
    $ids.Add($Root)
    for ($i = 0; $i -lt $ids.Count; $i++) { $all | Where-Object { $_.ParentProcessId -eq $ids[$i] -and -not $ids.Contains([int]$_.ProcessId) } | ForEach-Object { $ids.Add([int]$_.ProcessId) } }
    $all | Where-Object { $ids.Contains([int]$_.ProcessId) }
}
function Stage-Sample {
    $running = Get-Content -LiteralPath (Join-Path $ready 'running.json') -Raw | ConvertFrom-Json
    if (-not $running.pid) { return 'no running stage' }
    $before = @{}
    Stage-Tree $running.pid | ForEach-Object { $before[[int]$_.ProcessId] = $_ }
    $defender = (Get-Process MsMpEng).TotalProcessorTime.TotalSeconds
    Start-Sleep -Seconds 10
    $lines = Stage-Tree $running.pid | Where-Object { $before.ContainsKey([int]$_.ProcessId) } | ForEach-Object {
        $old = $before[[int]$_.ProcessId]
        'pid {0} {1} rss {2} MB, in 10 s: cpu {3} s, read ops {4}, write ops {5}, other ops {6}, read {7} KB, written {8} KB' -f $_.ProcessId, $_.Name, [int]($_.WorkingSetSize / 1MB), [math]::Round((($_.KernelModeTime + $_.UserModeTime) - ($old.KernelModeTime + $old.UserModeTime)) / 1e7, 1), ($_.ReadOperationCount - $old.ReadOperationCount), ($_.WriteOperationCount - $old.WriteOperationCount), ($_.OtherOperationCount - $old.OtherOperationCount), [int](($_.ReadTransferCount - $old.ReadTransferCount) / 1KB), [int](($_.WriteTransferCount - $old.WriteTransferCount) / 1KB)
    }
    @("stage $($running.stage), wrapper pid $($running.pid), started $([DateTimeOffset]::FromUnixTimeMilliseconds($running.startedMs).UtcDateTime.ToString('s'))Z") + $lines + "MsMpEng (Defender) cpu in the same 10 s: $([math]::Round((Get-Process MsMpEng).TotalProcessorTime.TotalSeconds - $defender, 1)) s"
}
function Observation {
    $state = Ready-State
    $os = Get-CimInstance Win32_OperatingSystem
    $largest = Get-CimInstance Win32_Process | Sort-Object WorkingSetSize -Descending | Select-Object -First 5 | ForEach-Object { Cut ('{0} {1} {2} {3}' -f [int]($_.WorkingSetSize / 1KB), $_.ProcessId, $_.ParentProcessId, ($_.CommandLine -replace '\s+', ' ')) 400 }
    $tail = Get-Content -LiteralPath (Join-Path $ready "$($state.stage).log") -Tail 30 | ForEach-Object { Cut $_ 300 }
    @(
        '', "## Observation $([DateTime]::UtcNow.ToString('s'))Z", '',
        ('- `ready.json`: state `{0}`, stage `{1}` ({2} of {3}), try (`attempts`) {4}, exit `{5}`, refusal `{6}`, ready `{7}`, reason `{8}`, peak {9} MB' -f $state.state, $state.stage, $state.stageIndex, $state.stageCount, $state.attempts, $state.exit, $state.refusal, $state.ready, $state.reason.code, $state.peakMegabytes),
        "- Memory (Windows has no ``memory_pressure``): $([int]($os.FreePhysicalMemory / 1KB)) MB physical free of $([int]($os.TotalVisibleMemorySize / 1KB)) MB, $([int]($os.FreeVirtualMemory / 1KB)) MB commit free",
        '- Permission dialog on screen: not applicable (Windows has no macOS consent dialog); this session has no view of the screen',
        '', '`ready.json`:', '', '```json', (Cut (Get-Content -LiteralPath $status -Raw) 2500), '```',
        '', "Last 30 lines of ``$($state.stage).log``:", '', '```text'
    ) + $tail + @('```', '', 'File operations of the running stage (10 s sample):', '', '```text') + (Stage-Sample) + @('```', '', 'Five largest processes (rss KB, pid, ppid, command):', '', '```text') + $largest + '```'
}
$round = Round-Line
$next = Next-Mark
$deadline = [DateTime]::UtcNow.AddHours($Hours)
$outcome = 'time limit'
function Ended { if ((Ready-State).ready -eq $true) { 'ready' } elseif ((Round-Line) -ne $round) { 'round changed' } elseif ((Next-Mark) -ne $next) { 'next.md changed' } }
while ([DateTime]::UtcNow -lt $deadline) {
    Add-Content -LiteralPath $report -Value ((Observation) -join "`n") -Encoding utf8NoBOM
    $end = Ended
    for ($i = 0; -not $end -and $i -lt 19; $i++) { Start-Sleep -Seconds 30; $end = Ended }
    if ($end) { $outcome = $end; break }
}
"[DEBUG] round 31 observer ended: $outcome at $([DateTime]::UtcNow.ToString('s'))Z"
Cut (Get-Content -LiteralPath $status -Raw) 1500
Get-Content -LiteralPath $roundFile -TotalCount 3
if (Test-Path -LiteralPath $nextFile) { Get-Content -LiteralPath $nextFile }

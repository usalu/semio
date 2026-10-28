param([switch]$Execute)
$ErrorActionPreference = 'Stop'
$outputPath = [System.IO.Path]::GetFullPath('C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️26\ARCHITECTURAL-ENERGY-CLIPS\🗑️generated\full')
$entrySuffix = '🏗️authoring\🎬️clip\🖌️render\🐍️.py'
function Get-OwnedTree($snapshot, $rootId) {
    $selected = @($rootId)
    do {
        $added = @($snapshot | Where-Object { $_.ParentProcessId -in $selected -and $_.ProcessId -notin $selected } | ForEach-Object { [int]$_.ProcessId })
        $selected += $added
    } while ($added.Count)
    return @($snapshot | Where-Object { $_.ProcessId -in $selected })
}
$snapshot = @(Get-CimInstance Win32_Process)
$candidates = @($snapshot | Where-Object {
    $_.Name -eq 'python.exe' -and $_.CommandLine -and
    $_.CommandLine.Replace('/', '\').Contains($entrySuffix) -and
    $_.CommandLine.Replace('/', '\').Contains($outputPath)
})
$roots = @($candidates | Where-Object { $_.ProcessId -notin @($candidates.ParentProcessId) })
if ($roots.Count -ne 1) { throw ('Expected one owned full-render Python process, found ' + $roots.Count) }
$rootProcess = $roots[0]
$ancestorIds = @([int]$rootProcess.ProcessId)
$cursor = $rootProcess
while ($cursor) {
    $ancestorIds += [int]$cursor.ParentProcessId
    $cursor = $snapshot | Where-Object { $_.ProcessId -eq $cursor.ParentProcessId } | Select-Object -First 1
}
if (@($candidates | Where-Object { $_.ProcessId -notin $ancestorIds }).Count) { throw "Independent full-render process ambiguity" }
$owned = @(Get-OwnedTree $snapshot $rootProcess.ProcessId)
$owned | Select-Object ProcessId, ParentProcessId, Name, CreationDate, CommandLine | ConvertTo-Json -Depth 4
if (-not $Execute) { exit 0 }
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class FullRenderConsoleSignal {
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool AttachConsole(uint processId);
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool FreeConsole();
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool GenerateConsoleCtrlEvent(uint signal, uint groupId);
}
'@
$attached = [FullRenderConsoleSignal]::AttachConsole([uint32]$rootProcess.ProcessId)
$signaled = $false
if ($attached) {
    $signaled = [FullRenderConsoleSignal]::GenerateConsoleCtrlEvent(1, [uint32]$rootProcess.ProcessId)
    [void][FullRenderConsoleSignal]::FreeConsole()
}
Write-Output ('[DEBUG] Targeted graceful Ctrl-Break attempt: attached=' + $attached + ', signaled=' + $signaled)
Start-Sleep -Seconds 3
$now = @(Get-CimInstance Win32_Process)
$owned += @(Get-OwnedTree $now $rootProcess.ProcessId)
$owned = @($owned | Sort-Object ProcessId -Unique)
$liveRoot = @($now | Where-Object { $_.ProcessId -eq $rootProcess.ProcessId -and $_.CreationDate -eq $rootProcess.CreationDate })
if ($liveRoot.Count) {
    Stop-Process -Id $rootProcess.ProcessId -Force
    Write-Output ('[DEBUG] Stopped exact owned render parent ' + $rootProcess.ProcessId)
}
$afterParent = @(Get-CimInstance Win32_Process)
$owned += @(Get-OwnedTree $afterParent $rootProcess.ProcessId)
$owned = @($owned | Sort-Object ProcessId -Unique)
foreach ($candidate in $owned) {
    if ($candidate.ProcessId -eq $rootProcess.ProcessId) { continue }
    $current = Get-CimInstance Win32_Process -Filter ('ProcessId=' + $candidate.ProcessId)
    if ($current -and $current.CreationDate -eq $candidate.CreationDate) {
        Stop-Process -Id $candidate.ProcessId -Force
        Write-Output ('[DEBUG] Stopped verified owned descendant ' + $candidate.ProcessId + ' ' + $candidate.Name)
    }
}
Write-Output '[DEBUG] Owned full-render cancellation completed; published cache remains.'

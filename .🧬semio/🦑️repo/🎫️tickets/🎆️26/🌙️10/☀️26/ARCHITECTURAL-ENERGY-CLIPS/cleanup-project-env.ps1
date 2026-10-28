$ErrorActionPreference = 'Stop'
$workspacePath = [System.IO.Path]::GetFullPath('C:\git\semio')
$expectedTarget = [System.IO.Path]::GetFullPath((Join-Path $workspacePath '🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv'))
if (-not (Test-Path -LiteralPath $expectedTarget)) {
    Write-Output '[DEBUG] Project environment already absent.'
    exit 0
}
$resolvedTarget = [System.IO.Path]::GetFullPath((Resolve-Path -LiteralPath $expectedTarget).ProviderPath)
if (-not $resolvedTarget.Equals($expectedTarget, [System.StringComparison]::OrdinalIgnoreCase) -or -not $resolvedTarget.StartsWith($workspacePath + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw 'Project environment target is outside the verified workspace.'
}
$item = Get-Item -LiteralPath $resolvedTarget
if (-not $item.PSIsContainer -or ($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint)) {
    throw 'Project environment target is not a plain workspace directory.'
}
$active = @(Get-CimInstance Win32_Process | Where-Object {
    ($_.ExecutablePath -and $_.ExecutablePath.Replace('/', '\').IndexOf($resolvedTarget, [System.StringComparison]::OrdinalIgnoreCase) -ge 0) -or
    ($_.CommandLine -and $_.CommandLine.Replace('/', '\').IndexOf($resolvedTarget, [System.StringComparison]::OrdinalIgnoreCase) -ge 0)
})
if ($active.Count) {
    $active | Select-Object ProcessId, Name, ExecutablePath, CommandLine | Format-List
    throw 'Project environment is still referenced by active processes.'
}
Write-Output ('[DEBUG] Removing verified unused project environment: ' + $resolvedTarget)
Remove-Item -LiteralPath $resolvedTarget -Recurse -Force
if (Test-Path -LiteralPath $resolvedTarget) { throw 'Project environment cleanup did not complete.' }
Write-Output '[DEBUG] Project environment removed.'

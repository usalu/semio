param([string]$RepoRoot, [string]$Bootstrap, [string]$Pins)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$tokens = $null
$errors = $null
$ast = [Management.Automation.Language.Parser]::ParseFile($Bootstrap, [ref]$tokens, [ref]$errors)
if ($errors.Count -ne 0) { throw 'Owned bootstrap parse failed' }
foreach ($definition in $ast.FindAll({ param($node) $node -is [Management.Automation.Language.FunctionDefinitionAst] }, $false)) {
    . ([scriptblock]::Create($definition.Extent.Text))
}
$script:PinsFile = $Pins
$env:SEMIO_WORKSPACE_ROOT = $RepoRoot
Refresh-CurrentProcessPath
Ensure-Rustup
Refresh-CurrentProcessPath
Ensure-RustToolchain -repoRoot $RepoRoot
Set-UserPathPriority -PreferredEntries @((Join-HomePath @('.cargo', 'bin')))
Write-Host '[DEBUG] Pinned Rust assets installed; MSVC readiness remains a separate required check.'

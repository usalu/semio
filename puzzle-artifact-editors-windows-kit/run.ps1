[CmdletBinding()]
param(
    [string]$RepositoryPath = (Get-Location).Path,
    [ValidateRange(0, 1000000000)]
    [double]$MaxBudgetUsd = 0
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $RepositoryPath -PathType Container)) {
    throw "RepositoryPath must be an existing directory."
}

$puzzleRepository = (Resolve-Path -LiteralPath $RepositoryPath).ProviderPath
$puzzleClaude = (Get-Command claude -CommandType Application -ErrorAction Stop).Source
$puzzleVersionText = (& $puzzleClaude --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0) {
    throw "Could not read the installed Claude Code version."
}

$puzzleVersionMatch = [regex]::Match($puzzleVersionText, '\b\d+\.\d+\.\d+\b')
if (-not $puzzleVersionMatch.Success) {
    throw "Could not parse Claude Code version: $puzzleVersionText"
}
if ([version]$puzzleVersionMatch.Value -lt [version]"2.1.293") {
    throw "Use Claude Code 2.1.293 or later for this model fleet. Run claude update."
}

$puzzlePromptPath = Join-Path $PSScriptRoot "prompt.txt"
$puzzleAgentsPath = Join-Path $PSScriptRoot "agents.json"
$puzzlePrompt = Get-Content -LiteralPath $puzzlePromptPath -Raw -Encoding UTF8
$puzzleAgentsRaw = Get-Content -LiteralPath $puzzleAgentsPath -Raw -Encoding UTF8
$puzzleAgentsRaw | ConvertFrom-Json -ErrorAction Stop | Out-Null

if ([string]::IsNullOrWhiteSpace($puzzlePrompt)) {
    throw "prompt.txt is empty."
}

$puzzleEnvironment = @{
    CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS = "20"
    CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH = "1"
    CLAUDE_CODE_EFFORT_LEVEL = "medium"
    CLAUDE_CODE_SUBAGENT_MODEL_FORCE = "0"
    CLAUDE_CODE_USE_POWERSHELL_TOOL = "1"
    ANTHROPIC_DEFAULT_OPUS_MODEL = "claude-opus-5-5"
    ANTHROPIC_DEFAULT_SONNET_MODEL = "claude-sonnet-5-5"
    ANTHROPIC_DEFAULT_HAIKU_MODEL = "claude-haiku-5-5"
}
$puzzlePreviousEnvironment = @{}
foreach ($puzzleKey in $puzzleEnvironment.Keys) {
    $puzzlePreviousEnvironment[$puzzleKey] =
        [Environment]::GetEnvironmentVariable($puzzleKey, "Process")
}

$puzzlePreviousOutputEncoding = $OutputEncoding
$puzzlePreviousConsoleInputEncoding = [Console]::InputEncoding
$puzzlePreviousConsoleOutputEncoding = [Console]::OutputEncoding
$puzzlePushedLocation = $false

try {
    foreach ($puzzleKey in $puzzleEnvironment.Keys) {
        [Environment]::SetEnvironmentVariable(
            $puzzleKey, $puzzleEnvironment[$puzzleKey], "Process"
        )
    }

    $OutputEncoding = [System.Text.UTF8Encoding]::new($false)
    [Console]::InputEncoding = $OutputEncoding
    [Console]::OutputEncoding = $OutputEncoding

    $puzzleArguments = @(
        "-p"
        "--model"
        "claude-opus-5-5"
        "--effort"
        "medium"
        "--agents"
        $puzzleAgentsPath
        "--allowedTools"
        "Agent,Read,Glob,Grep,Write,Edit,Bash,PowerShell,WebSearch,WebFetch"
    )
    if ($MaxBudgetUsd -gt 0) {
        $puzzleArguments += @(
            "--max-budget-usd"
            $MaxBudgetUsd.ToString([Globalization.CultureInfo]::InvariantCulture)
        )
    }

    Push-Location -LiteralPath $puzzleRepository
    $puzzlePushedLocation = $true
    $puzzlePrompt | & $puzzleClaude @puzzleArguments
    $puzzleExitCode = $LASTEXITCODE

    if ($puzzleExitCode -ne 0) {
        throw "Claude Code exited with code $puzzleExitCode."
    }
}
finally {
    if ($puzzlePushedLocation) {
        Pop-Location
    }
    $OutputEncoding = $puzzlePreviousOutputEncoding
    [Console]::InputEncoding = $puzzlePreviousConsoleInputEncoding
    [Console]::OutputEncoding = $puzzlePreviousConsoleOutputEncoding
    foreach ($puzzleKey in $puzzleEnvironment.Keys) {
        [Environment]::SetEnvironmentVariable(
            $puzzleKey, $puzzlePreviousEnvironment[$puzzleKey], "Process"
        )
    }
}


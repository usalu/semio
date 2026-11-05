$ErrorActionPreference = 'Continue'
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = $OutputEncoding
$env:WSL_UTF8 = '1'
$launcher = Join-Path $env:SystemRoot 'System32\wsl.exe'
$queries = @(
    @{ name = 'distributions'; arguments = @('--list', '--quiet') },
    @{ name = 'status'; arguments = @('--status') }
)
$results = @()
foreach ($query in $queries) {
    $arguments = $query.arguments
    $text = (& $launcher @arguments 2>&1 | Out-String).Trim()
    $results += [ordered]@{ name = $query.name; status = $LASTEXITCODE; text = $text }
}
[ordered]@{ version = 1; platform = 'win32'; queries = $results } | ConvertTo-Json -Depth 5 -Compress

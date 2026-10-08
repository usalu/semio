param([string]$Crate, [string]$Binary, [string]$TargetDir, [string]$Target, [string[]]$TestArgs)
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$env:CARGO_TARGET_DIR = $TargetDir
$env:SEMIO_TEST_CLI = $Binary
Set-Location -LiteralPath $Crate
$exe = (cargo test --test $Target --no-run --message-format=json 2>$null | ForEach-Object { $_ | ConvertFrom-Json -ErrorAction SilentlyContinue } | Where-Object { $_.executable } | Select-Object -Last 1).executable
"exe=$exe"
& $exe @TestArgs
exit $LASTEXITCODE

param([string]$Command, [string]$Log, [string]$Exit, [string]$Directory)
Remove-Item -LiteralPath $Log, $Exit -ErrorAction SilentlyContinue
$script = "`$ErrorActionPreference = 'Continue'; Set-Location -LiteralPath '$Directory'; & { $Command } *> '$Log'; (`$LASTEXITCODE ?? 0) | Out-File -Encoding ascii -LiteralPath '$Exit'"
$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($script))
$pwsh = (Get-Command pwsh).Source
$result = Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{ CommandLine = "`"$pwsh`" -NoProfile -EncodedCommand $encoded"; CurrentDirectory = $Directory }
"started pid=$($result.ProcessId) rc=$($result.ReturnValue)"

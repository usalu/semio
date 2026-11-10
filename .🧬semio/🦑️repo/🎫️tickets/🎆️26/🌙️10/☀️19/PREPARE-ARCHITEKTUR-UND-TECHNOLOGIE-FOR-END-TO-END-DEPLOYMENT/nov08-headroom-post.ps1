$rows = @(Get-CimInstance Win32_Process)
$matches = @($rows | Where-Object { $_.CommandLine -and $_.CommandLine -match 'repo:test-compiler-headroom' -and $_.Name -notmatch 'powershell' } | Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,CommandLine)
Write-Output ('MATCHES=' + (ConvertTo-Json -InputObject $matches -Depth 4 -Compress))

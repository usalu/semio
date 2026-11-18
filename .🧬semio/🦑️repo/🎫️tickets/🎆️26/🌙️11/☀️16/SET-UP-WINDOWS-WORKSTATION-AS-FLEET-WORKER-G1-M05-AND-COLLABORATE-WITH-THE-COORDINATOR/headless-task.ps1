<#
.SYNOPSIS
🙈 Re-registers one semio fleet scheduled task so that its action runs under `conhost.exe --headless` (no console window, no terminal tab). Idempotent. Usage: headless-task.ps1 <task name>
.LINK
https://learn.microsoft.com/windows/console/creating-a-pseudoconsole-session
#>
param([Parameter(Mandatory = $true)][string]$Name)
$ErrorActionPreference = 'Stop'
[xml]$doc = Export-ScheduledTask -TaskName $Name
$exec = $doc.Task.Actions.Exec
if ($exec.Command -eq 'conhost.exe') { "$Name already headless: $($exec.Arguments)"; exit 0 }
$exec.Arguments = "--headless $($exec.Command) $($exec.Arguments)"
$exec.Command = 'conhost.exe'
Register-ScheduledTask -TaskName $Name -Xml $doc.OuterXml -Force | Out-Null
$now = (Export-ScheduledTask -TaskName $Name | Select-Xml -XPath '//*[local-name()="Exec"]').Node
"$Name -> $($now.Command) $($now.Arguments)"

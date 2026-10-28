param([Parameter(Mandatory=$true)][string]$FixtureDirectory)
$ErrorActionPreference='Stop'
$taskTicket=$PSScriptRoot
$taskManifest=Get-Content -LiteralPath (Join-Path $FixtureDirectory 'cases.json') -Raw|ConvertFrom-Json
$taskInputs=Get-Content -LiteralPath (Join-Path $taskTicket '🗑️generated/clipping-official-input-observations.json') -Raw|ConvertFrom-Json
foreach($taskRow in $taskManifest.cases){
    $taskSource=@($taskInputs|Where-Object{$_.path -eq $taskRow.path})
    if($taskSource.Count -ne 1){throw 'Original source is not uniquely identified'}
    $taskDestination=Join-Path $FixtureDirectory $taskRow.file
    if($taskDestination.Length -gt 256){throw 'Fixture exceeds the UTF16 path limit'}
    if((Get-FileHash -LiteralPath $taskSource[0].local -Algorithm SHA256).Hash.ToLowerInvariant() -ne $taskRow.sha256){throw 'Original source hash mismatch'}
    if(Test-Path -LiteralPath $taskDestination){if((Get-FileHash -LiteralPath $taskDestination -Algorithm SHA256).Hash.ToLowerInvariant() -ne $taskRow.sha256){throw 'Existing fixture changed'}}else{Copy-Item -LiteralPath $taskSource[0].local -Destination $taskDestination}
    if((Get-Item -LiteralPath $taskDestination).Length -ne $taskRow.bytes){throw 'Original source byte mismatch'}
    Write-Output "[DEBUG] Retained original $($taskRow.file)"
}
$taskLicense=Join-Path $taskTicket '🗑️generated/clipping-certification-license.txt'
$taskDestination=Join-Path $FixtureDirectory $taskManifest.license.file
if((Get-FileHash -LiteralPath $taskLicense -Algorithm SHA256).Hash.ToLowerInvariant() -ne $taskManifest.license.sha256){throw 'Original license hash mismatch'}
if(Test-Path -LiteralPath $taskDestination){if((Get-FileHash -LiteralPath $taskDestination -Algorithm SHA256).Hash.ToLowerInvariant() -ne $taskManifest.license.sha256){throw 'Existing license changed'}}else{Copy-Item -LiteralPath $taskLicense -Destination $taskDestination}
Write-Output '[DEBUG] Retained exact original license text'

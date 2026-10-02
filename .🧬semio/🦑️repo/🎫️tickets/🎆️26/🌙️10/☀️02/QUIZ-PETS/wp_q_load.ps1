# Work package Q: prints the share of the host's processors in use (the mean of one sample per second), so a timing can be
# read next to the load it was taken under. `pwsh wp_q_load.ps1 [-Seconds 2] [-Below 15 -Patience 300]`
# With -Below it waits (at most -Patience seconds) until the share is below that many percent.
param([int]$Seconds = 2, [int]$Below = 0, [int]$Patience = 300)

function Get-Load {
  $samples = for ($second = 0; $second -lt $Seconds; $second++) {
    (Get-CimInstance Win32_PerfFormattedData_PerfOS_Processor -Filter "Name='_Total'").PercentProcessorTime
    Start-Sleep -Milliseconds 900
  }
  [math]::Round(($samples | Measure-Object -Average).Average, 0)
}

$load = Get-Load
$waited = 0
while ($Below -gt 0 -and $load -ge $Below -and $waited -lt $Patience) {
  $waited += $Seconds
  $load = Get-Load
}
"load=$load% waited=${waited}s"

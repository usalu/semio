# 🕵️ Logs every taskkill/kill-like process start with its parent chain, to find who terminates native cargo runs.
param([string]$Log)
while ($true) {
  $seen = @{}
  Get-CimInstance Win32_Process -Filter "Name='taskkill.exe' OR Name='TASKKILL.EXE'" -ErrorAction SilentlyContinue | ForEach-Object {
    if (-not $seen[$_.ProcessId]) {
      $seen[$_.ProcessId] = 1; $p = $_; $chain = ""
      for ($i = 0; $i -lt 5 -and $p; $i++) { $pp = Get-CimInstance Win32_Process -Filter "ProcessId=$($p.ParentProcessId)" -ErrorAction SilentlyContinue; if ($pp) { $chain += " <= [$($pp.ProcessId)] " + $pp.CommandLine.Substring(0, [Math]::Min(220, $pp.CommandLine.Length)) }; $p = $pp }
      "$(Get-Date -Format o) taskkill [$($_.ProcessId)] $($_.CommandLine) $chain" | Add-Content -Path $Log
    }
  }
  Start-Sleep -Milliseconds 150
}

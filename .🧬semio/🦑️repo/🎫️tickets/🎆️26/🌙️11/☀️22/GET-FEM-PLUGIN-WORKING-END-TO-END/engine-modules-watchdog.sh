#!/usr/bin/env bash
# [DEBUG] kills any semio_s_fem test binary running longer than 10 minutes of wall clock.
while true; do
  powershell -NoProfile -Command "Get-Process | Where-Object { \$_.ProcessName -like 'semio_s_fem*' -and ((Get-Date) - \$_.StartTime).TotalMinutes -gt 10 } | ForEach-Object { Write-Output \"WATCHDOG killing \$(\$_.ProcessName) \$(\$_.Id)\"; Stop-Process -Id \$_.Id -Force }"
  sleep 30
done

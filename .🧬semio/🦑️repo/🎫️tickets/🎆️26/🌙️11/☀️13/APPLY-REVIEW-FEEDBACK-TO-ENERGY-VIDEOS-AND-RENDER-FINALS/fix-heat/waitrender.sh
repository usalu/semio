#!/bin/bash
while powershell -NoProfile -Command "if (Get-CimInstance Win32_Process | Where-Object { \$_.Name -eq 'python.exe' -and \$_.CommandLine -match 'render' -and \$_.CommandLine -match 'heating' }) { exit 0 } else { exit 1 }"; do sleep 30; done
echo gone

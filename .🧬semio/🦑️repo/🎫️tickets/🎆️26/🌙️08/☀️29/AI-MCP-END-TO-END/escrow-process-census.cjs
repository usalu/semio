const {execFileSync}=require('child_process');
console.log(execFileSync('powershell',['-NoProfile','-Command',"Get-CimInstance Win32_Process | Where-Object { $_.Name -in @('cargo.exe','rustc.exe') } | Select-Object ProcessId,Name,CommandLine | ConvertTo-Json -Compress"],{encoding:'utf8'}));

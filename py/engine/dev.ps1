. ..\..\powershell.ps1
StopProcessOnPort -Port 2507 # engine
StopProcessOnPort -Port 5678 # debugger
$process = Start-Process -FilePath "poetry" -ArgumentList "run dev" -NoNewWindow -PassThru
function Cleanup {
    if ($process -and !$process.HasExited) {
        Stop-Process -Id $process.Id -Force
    }
    exit
}
Register-EngineEvent PowerShell.Exiting -Action { Cleanup }
Wait-Process -Id $process.Id
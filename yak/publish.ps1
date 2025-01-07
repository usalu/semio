Set-Location .\dist
# get version from .\manifest.yml file
$version = (Get-Content .\manifest.yml | Select-String -Pattern 'version:').ToString().Split(":")[1].Trim()
$buildName = "semio-$version-rh7_30-win.yak"
$yak = "C:\Program Files\Rhino 8\System\Yak.exe"
# & $yak login
& $yak push $buildName
Set-Location ..
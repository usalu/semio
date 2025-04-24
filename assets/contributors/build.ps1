. ..\..\powershell.ps1

$resolution = 90

# read contributors from contributors.txt
$contributors = Get-Content -Path "..\lists\contributors.txt"

# create temp folder
$tempFolder = ".\temp"
if (-not (Test-Path -Path $tempFolder)) {
    New-Item -ItemType Directory -Path $tempFolder
}

# download avatars from https://github.com/ACCOUNT.png and save to .\temp
foreach ($contributor in $contributors) {
    $url = "https://github.com/$contributor.png"
    $destination = Join-Path -Path $tempFolder -ChildPath "$contributor.png"
    Invoke-WebRequest -Uri $url -OutFile $destination
}

# resize avatars to $resolution and crop to circle and save .\ACCOUNT.png
foreach ($contributor in $contributors) {
    
    
}

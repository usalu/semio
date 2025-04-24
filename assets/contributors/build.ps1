. ..\..\..\powershell.ps1

$resolutions = @(90, 400)
$contributors = Get-Content -Path "..\lists\contributors.txt"
foreach ($contributor in $contributors) {
    $url = "https://github.com/$contributor.png"
    Invoke-WebRequest -Uri $url -OutFile "$contributor.png"
    foreach ($resolution in $resolutions) {
        $image = Get-Image -Path "$contributor.png"
        $image.Resize($resolution, $resolution)
        $image.Crop(0, 0, $resolution, $resolution)
        $image.Save("$contributor_round_$resolution.png")
    }
}
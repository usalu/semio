. ..\\..\\powershell.ps1

Add-Type -AssemblyName System.Drawing

$resolutions = @(90)
$contributors = Get-Content -Path "..\\lists\\contributors.txt"
foreach ($contributor in $contributors) {
    # TODO: Download avatars from github (GitHub ignores default requests)
    # $url = "https://github.com/$contributor.png"
    # Invoke-WebRequest -Uri $url -OutFile "$contributor.png"
    # load file from current directory
    $sourcePath = Join-Path -Path $PSScriptRoot -ChildPath "$contributor.png"
    if (Test-Path $sourcePath) {
        $tempBasePath = Join-Path -Path $PSScriptRoot -ChildPath "$($contributor)_square"
        ResizeImage -sourcePath $sourcePath -targetPathBase $tempBasePath -targetResolutions $resolutions

        foreach ($resolution in $resolutions) {
            $squareImagePath = Join-Path -Path $PSScriptRoot -ChildPath "$($contributor)_square_$($resolution)x$($resolution).png"
            $circleTargetPath = Join-Path -Path $PSScriptRoot -ChildPath "$($contributor)_round_$($resolution).png"

            if (Test-Path $squareImagePath) {
                CropImageToCircle -SourcePath $squareImagePath -TargetPath $circleTargetPath -Resolution $resolution
                Remove-Item -Path $squareImagePath -Force
            }
            else {
                Write-Warning "Square image not found for resolution {$resolution}: {$squareImagePath}"
            }
        }
    }
    else {
        Write-Warning ("Image file not found for contributor {0}: {1}" -f $contributor, $sourcePath)
    }
}
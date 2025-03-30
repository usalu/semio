. ..\powershell.ps1

$resolutions = @(24, 512)
$images = @(
    @{source = "emblem_1920x1920.png"; target = "emblem" },
    @{source = "emblem_round_1920x1920.png"; target = "emblem_round" },
    @{source = "emblem_dark_1920x1920.png"; target = "emblem_dark" },
    @{source = "emblem_dark_round_1920x1920.png"; target = "emblem_dark_round" }
)

foreach ($image in $images) {
    & magick $image.source -define icon:auto-resize="256,128,96,64,48,32,16" "$($image.target).ico"
    ResizeImage -sourcePath $image.source -targetPathBase $image.target -targetResolutions $resolutions
}

Copy-Item -Path (Get-ChildItem -Path . -Filter "emblem_dark_round.svg" -Recurse).FullName -Destination "..\icons\semio.svg"
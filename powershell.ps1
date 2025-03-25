Add-Type -AssemblyName System.Drawing
function ResizeImage {
    param (
        [string]$sourcePath,
        [string]$targetPathBase,
        [int[]]$targetResolutions
    )
    
    foreach ($targetResolution in $targetResolutions) {
        $image = [System.Drawing.Image]::FromFile($sourcePath)
        $newImage = $image.GetThumbnailImage($targetResolution, $targetResolution, $null, [System.IntPtr]::Zero)
        $targetPath = $targetPathBase + "_$targetResolution" + "x" + "$targetResolution.png"
        $newImage.Save($targetPath, [System.Drawing.Imaging.ImageFormat]::Png)
        $image.Dispose()
        $newImage.Dispose()
    }
}

function Rename {
    param (
        [string]$prefix,
        [string]$newPrefix
    )
    $files = Get-ChildItem -Path . -File -Filter "$prefix*" -Recurse
    foreach ($file in $files) {
        $newFileName = $file.FullName -replace $prefix, $newPrefix
        Move-Item -Path $file.FullName -Destination $newFileName -Force
    }
}
function Kill-ProcessOnPort {
    param (
        [int]$Port
    )
    $processInfo = netstat -ano | Select-String ":$Port\s+.*LISTENING" | ForEach-Object {
        ($_ -split '\s+')[-1]
    } | Select-Object -First 1

    if ($processInfo) {
        Stop-Process -Id $processInfo -Force
    }
}
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
function RenameFilesByPattern {
    param (
        [string]$Pattern, # Regular expression pattern to match
        [string]$Replacement    # Replacement string
    )
    $files = Get-ChildItem -Path . -File -Recurse
    foreach ($file in $files) {
        $newFileName = $file.FullName -replace $Pattern, $Replacement
        if ($newFileName -ne $file.FullName) {
            Move-Item -Path $file.FullName -Destination $newFileName -Force
        }
    }
}
function DeleteFilesByPattern {
    param (
        [string]$pattern
    )
    $files = Get-ChildItem -Path . -File -Filter $pattern -Recurse
    foreach ($file in $files) {
        Remove-Item -Path $file.FullName -Force
    }
}
function StopProcessOnPort {
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

function Compress-HDR {
    param (
        [Parameter(Mandatory = $true)]
        [string]$InputFile,  # Path to the input .exr file
        [Parameter(Mandatory = $true)]
        [string]$OutputFile, # Path to the output .exr.compressed file
        [string]$Resize = "512x512" # Resize dimensions (default: 512x512)
    )
    if (-Not (Test-Path $InputFile)) {
        Write-Error "Input file '$InputFile' does not exist."
        return
    }
    $command = "magick convert `"$InputFile`" -compress DWAB -resize $Resize `"$OutputFile`""
    Write-Host "Running: $command"
    Invoke-Expression $command
    Write-Host "Compression and resizing completed: $OutputFile"
}
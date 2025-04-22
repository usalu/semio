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
        [string]$InputFile, # Path to the input .exr file
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

function CropImageToCircle {
    param(
        [Parameter(Mandatory = $true)]
        [string]$SourcePath,
        [Parameter(Mandatory = $true)]
        [string]$TargetPath,
        [Parameter(Mandatory = $true)]
        [int]$Resolution
    )

    if (-Not (Test-Path $SourcePath)) {
        Write-Error "Source image file not found: $SourcePath"
        return
    }

    $sourceImage = [System.Drawing.Image]::FromFile($SourcePath)
    
    # Ensure the source image resolution matches the target resolution
    if ($sourceImage.Width -ne $Resolution -or $sourceImage.Height -ne $Resolution) {
        Write-Warning "Source image resolution ($($sourceImage.Width)x$($sourceImage.Height)) does not match target resolution ($($Resolution)x$($Resolution)). Resizing source image before cropping."
        $resizedSource = $sourceImage.GetThumbnailImage($Resolution, $Resolution, $null, [System.IntPtr]::Zero)
        $sourceImage.Dispose()
        $sourceImage = $resizedSource # Use the resized image for cropping
    }

    $circularBitmap = New-Object System.Drawing.Bitmap $Resolution, $Resolution, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [System.Drawing.Graphics]::FromImage($circularBitmap)
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality

    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $path.AddEllipse(0, 0, $Resolution, $Resolution)
    $graphics.SetClip($path)
    $graphics.DrawImage($sourceImage, 0, 0, $Resolution, $Resolution)

    $circularBitmap.Save($TargetPath, [System.Drawing.Imaging.ImageFormat]::Png)

    # Dispose resources
    $sourceImage.Dispose()
    $circularBitmap.Dispose()
    $graphics.Dispose()
    $path.Dispose()
}
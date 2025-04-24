. ..\\..\\powershell.ps1

Add-Type -AssemblyName System.Drawing

$resolutions = @(90, 400)
$contributors = Get-Content -Path "..\\lists\\contributors.txt"
foreach ($contributor in $contributors) {
    # TODO: Download avatars from github (GitHub ignores default requests)
    # $url = "https://github.com/$contributor.png"
    # Invoke-WebRequest -Uri $url -OutFile "$contributor.png"
    # load file from current directory
    $sourcePath = Join-Path -Path $PSScriptRoot -ChildPath "$contributor.png"
    if (Test-Path $sourcePath) {
        $originalImage = [System.Drawing.Image]::FromFile($sourcePath)
        foreach ($resolution in $resolutions) {
            $resizedImage = $originalImage.GetThumbnailImage($resolution, $resolution, $null, [System.IntPtr]::Zero)
            $circularBitmap = New-Object System.Drawing.Bitmap $resolution, $resolution, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
            $graphics = [System.Drawing.Graphics]::FromImage($circularBitmap)
            $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
            $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
            $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
            $path = New-Object System.Drawing.Drawing2D.GraphicsPath
            $path.AddEllipse(0, 0, $resolution, $resolution)
            $graphics.SetClip($path)
            $graphics.DrawImage($resizedImage, 0, 0, $resolution, $resolution)
            $targetPath = Join-Path -Path $PSScriptRoot -ChildPath "$($contributor)_round_$($resolution).png"
            $circularBitmap.Save($targetPath, [System.Drawing.Imaging.ImageFormat]::Png)
            $resizedImage.Dispose()
            $circularBitmap.Dispose()
            $graphics.Dispose()
            $path.Dispose()
        }
        $originalImage.Dispose()
    }
    else {
        Write-Warning "Image file not found for contributor '$contributor': $sourcePath"
    }
}
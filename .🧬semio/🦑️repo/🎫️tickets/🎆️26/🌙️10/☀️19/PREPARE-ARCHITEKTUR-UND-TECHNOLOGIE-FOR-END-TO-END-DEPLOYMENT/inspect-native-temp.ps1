param([string]$Target)
$ErrorActionPreference = 'Stop'
$filesystem = New-Object -ComObject Scripting.FileSystemObject
try {
    $short = $filesystem.GetFolder($Target).ShortPath
    @{ path = $short; ascii = ($short -notmatch '[^\x00-\x7f]'); length = $short.Length } | ConvertTo-Json -Compress
} finally {
    [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($filesystem)
}

$win32 = New-Object ComponentModel.Win32Exception 42
@{ code = 42; win32 = $win32.Message; hresult = [Runtime.InteropServices.Marshal]::GetExceptionForHR(-2147024854).Message; missingMessage = [Runtime.InteropServices.Marshal]::GetExceptionForHR(-2147024579).Message } | ConvertTo-Json -Compress

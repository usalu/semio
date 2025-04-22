# Create value list for Grasshopper MIME types and licenses

$buildDir = ".\build"
if (-not (Test-Path -Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir
}

function Convert-CsvToValueList {
    param (
        [string]$csv,
        [string]$output,
        [string]$key,
        [string]$value
    )

    $csvData = Import-Csv -Path $csv
    $csvData | ForEach-Object {
        $line = "$($_.$key) = `"$($_.$value)`""
        Write-Output $line
    } | Out-File -FilePath $output
}

Convert-CsvToValueList -csv "..\..\meta\mimes.csv" -output (Join-Path -Path $buildDir -ChildPath "mimes.txt") -key "Extension" -value "MIME"
Convert-CsvToValueList -csv "..\..\meta\licenses.csv" -output (Join-Path -Path $buildDir -ChildPath "licenses.txt") -key "Name" -value "SPDX"
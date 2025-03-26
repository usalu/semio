$inputFilePath = "kit.json"
$outputFilePath = "kit_unescaped.json"

$jsonContent = Get-Content -Path $inputFilePath -Raw
$unescapedContent = [System.Text.RegularExpressions.Regex]::Unescape($jsonContent)
Set-Content -Path $outputFilePath -Value $unescapedContent
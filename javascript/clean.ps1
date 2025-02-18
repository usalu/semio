$folders = @("node_modules", ".turbo")
foreach ($folder in $folders) {
    Get-ChildItem -Path . -Recurse -Directory -Filter $folder | Remove-Item -Recurse -Force
}

$files = @("package-lock.json")
foreach ($file in $files) {
    Get-ChildItem -Path . -Recurse -File -Filter $file | Remove-Item -Force
}
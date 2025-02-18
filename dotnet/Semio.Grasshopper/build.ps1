.\build-value-lists.ps1

& "C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe" Semio.sln /t:Clean
& "C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe" Semio.sln /p:Configuration=Debug

$yakDistFolderPath = "..\..\yak\dist"

if (Test-Path $yakDistFolderPath) {
    Remove-Item $yakDistFolderPath -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $yakDistFolderPath
Copy-Item -Path .\bin\Debug\net48\* -Destination $yakDistFolderPath -Recurse
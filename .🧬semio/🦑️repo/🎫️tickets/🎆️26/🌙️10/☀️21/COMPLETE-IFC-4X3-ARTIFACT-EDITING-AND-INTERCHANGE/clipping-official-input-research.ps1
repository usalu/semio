param([string]$Ticket=$PSScriptRoot)
$ErrorActionPreference='Stop'
$headers=@{'User-Agent'='semio-independent-research'}
$tree=(Get-Content -LiteralPath "$Ticket/🗑️generated/clipping-official-tree-research.json" -Raw|ConvertFrom-Json)[0]
$dir="$Ticket/🗑️generated/clipping-official-inputs"
New-Item -ItemType Directory -Force -Path $dir|Out-Null
$rows=@()
foreach($file in $tree.files|Where-Object {$_.path.StartsWith('IFC 4.3.2.0')}){
 $url="https://raw.githubusercontent.com/buildingSMART/Certification-datasets/$($tree.commit)/"+(($file.path.Split('/')|ForEach-Object {[uri]::EscapeDataString($_)}) -join '/')
 $target=Join-Path $dir ([IO.Path]::GetFileName($file.path))
 Invoke-WebRequest -Headers $headers -Uri $url -OutFile $target
 $raw=Get-Content -LiteralPath $target -Raw
 $rows+=@{path=$file.path;commit=$tree.commit;url=$url;local=$target;sha256=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash;schema=[regex]::Match($raw,'FILE_SCHEMA\s*\([^;]+;').Value;clipping=[regex]::Matches($raw,'IFCBOOLEANCLIPPINGRESULT').Count;labels=[regex]::Matches($raw,"'Clipping'").Count}
}
$rows|ConvertTo-Json -Depth 5|Set-Content -LiteralPath "$Ticket/🗑️generated/clipping-official-input-observations.json"
$rows|Select-Object path,clipping,labels,schema


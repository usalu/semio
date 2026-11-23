param([int]$Lines=40,[string]$Pattern='.')
$fs=[IO.File]::Open("$env:LOCALAPPDATA\semio\fleet\entry\run.log",'Open','Read','ReadWrite,Delete')
$fs.Seek([Math]::Max(0,$fs.Length-400000),'Begin')|Out-Null
$r=New-Object IO.StreamReader($fs,[Text.Encoding]::UTF8);$t=$r.ReadToEnd();$r.Close()
$t -split "`n" | Where-Object { $_ -match $Pattern } | Select-Object -Last $Lines | ForEach-Object { if($_.Length -gt 300){$_.Substring(0,300)}else{$_} }

param([string]$Pattern,[int]$Before=25,[int]$After=3,[int]$Last=1)
$fs=[IO.File]::Open("$env:LOCALAPPDATA\semio\fleet\entry\run.log",'Open','Read','ReadWrite,Delete')
$fs.Seek([Math]::Max(0,$fs.Length-3000000),'Begin')|Out-Null
$r=New-Object IO.StreamReader($fs,[Text.Encoding]::UTF8);$lines=$r.ReadToEnd() -split "`n";$r.Close()
$hits=@(for($i=0;$i -lt $lines.Count;$i++){if($lines[$i] -match $Pattern){$i}}) | Select-Object -Last $Last
foreach($h in $hits){'=== hit '+$h; $lines[[Math]::Max(0,$h-$Before)..[Math]::Min($lines.Count-1,$h+$After)] | ForEach-Object { if($_.Length -gt 260){$_.Substring(0,260)}else{$_} }}

#!/bin/zsh
# 🧮️ Z3 one-off: runs the owner-only SDDL cases through .NET's RawSecurityDescriptor (PowerShell 7 container) and prints
# the per-case violation counts next to the fixture's expectation.
fixture="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔐️owner-only/🔣️.json"
cases=$(python3 -c "import json,sys; f=json.load(open(sys.argv[1])); print(json.dumps([r for r in f['sddl'] if 'NO_ACCESS_CONTROL' not in r['sddl']]))" "$fixture")
expected=$(python3 -c "import json,sys; f=json.load(open(sys.argv[1])); print(' '.join(str(len(r['violations'])) for r in f['sddl'] if 'NO_ACCESS_CONTROL' not in r['sddl']))" "$fixture")
script='$cases = $env:SEMIO_SDDL_CASES | ConvertFrom-Json; $trusted = @("S-1-5-18","S-1-5-32-544","S-1-3-0","S-1-3-4"); ($cases | ForEach-Object { $d = [System.Security.AccessControl.RawSecurityDescriptor]::new($_.sddl); if ($null -eq $d.DiscretionaryAcl) { 1 } else { $n = 0; if (-not ($d.ControlFlags -band [System.Security.AccessControl.ControlFlags]::DiscretionaryAclProtected)) { $n++ }; foreach ($ace in $d.DiscretionaryAcl) { if ($ace.AceQualifier -eq "AccessAllowed" -and $ace.SecurityIdentifier.Value -ne $_.sid -and $trusted -notcontains $ace.SecurityIdentifier.Value) { $n++ } }; $n } }) -join " "'
actual=$(docker run --rm -e SEMIO_SDDL_CASES="$cases" mcr.microsoft.com/powershell:latest pwsh -NoLogo -NoProfile -NonInteractive -Command "$script" 2>&1 | tr -d '\r')
echo "expected: $expected"
echo "dotnet:   $actual"
[ "$expected" = "$actual" ] && echo "AGREE" || echo "DISAGREE"

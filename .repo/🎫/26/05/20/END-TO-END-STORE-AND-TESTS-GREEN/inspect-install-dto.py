import json
import subprocess
import sys
from pathlib import Path

repo = Path(__file__).resolve().parents[4]
fixture = repo / "semio/assets/fixtures/metabolism.kit.semio.json"
out = Path(__file__).parent / "metabolism-install-dto.json"

ps = r"""
$ErrorActionPreference = 'Stop'
$code = @'
using System;
using System.IO;
using Newtonsoft.Json.Linq;
using Semio;
using Semio.Store;
var kit = Utility.DeserializeKit(File.ReadAllText(@"''' + str(fixture).replace('\\', '\\\\') + r'''"))!;
var dto = StoreKitIO.KitToJObject(kit);
File.WriteAllText(@"''' + str(out).replace('\\', '\\\\') + r'''", dto.ToString());
Console.WriteLine("wrote " + dto.ToString().Length);
'@
$tmp = Join-Path $env:TEMP ("semio-dump-" + [guid]::NewGuid().ToString("N") + ".cs")
Set-Content -Path $tmp -Value $code -Encoding UTF8
dotnet run --project ''' + str(repo / "semio/client/lib/net/Semio.Tests/Semio.Tests.csproj").replace('\\', '\\\\') + r''' -f net8.0 --no-build 2>&1 | Out-Null
"""
# simpler: use existing test project via inline csharp
print("run dotnet test filter DumpInstallDto if added")

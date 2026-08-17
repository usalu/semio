using System;
using System.IO;
using System.Linq;
using System.Reflection;

var grasshopperDll = "/home/vscode/.nuget/packages/grasshopper/8.10.24226.13001/lib/net48/Grasshopper.dll";
var rhinoDll = "/home/vscode/.nuget/packages/rhinocommon/8.10.24226.13001/lib/net48/RhinoCommon.dll";
var ghioDll = "/home/vscode/.nuget/packages/grasshopper/8.10.24226.13001/lib/net48/GH_IO.dll";
var runtimeDir = Path.GetDirectoryName(typeof(object).Assembly.Location)!;
var resolverPaths = Directory.GetFiles(runtimeDir, "*.dll")
    .Concat(new[] { grasshopperDll, rhinoDll, ghioDll })
    .Where(File.Exists)
    .Distinct(StringComparer.OrdinalIgnoreCase)
    .ToArray();

var resolver = new PathAssemblyResolver(resolverPaths);
using var metadataContext = new MetadataLoadContext(resolver);
var ghAsm = metadataContext.LoadFromAssemblyPath(grasshopperDll);
var t = ghAsm.GetType("Grasshopper.Kernel.Types.GH_GeometryGroup", true)!;
Console.WriteLine(t.FullName);
foreach (var i in t.GetInterfaces().OrderBy(i=>i.FullName))
    Console.WriteLine(i.FullName);

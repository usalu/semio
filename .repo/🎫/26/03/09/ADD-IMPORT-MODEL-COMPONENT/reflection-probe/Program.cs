using System.Reflection;
using System.Linq;
using System;
using System.IO;

var grasshopperDir = "/home/vscode/.nuget/packages/grasshopper/8.10.24226.13001/lib/net48";
var grasshopperPath = Path.Combine(grasshopperDir, "Grasshopper.dll");
var rhinoPath = "/home/vscode/.nuget/packages/rhinocommon/8.10.24226.13001/lib/net48/RhinoCommon.dll";
var runtimeDir = System.Runtime.InteropServices.RuntimeEnvironment.GetRuntimeDirectory();
var resolverPaths = Directory.GetFiles(runtimeDir, "*.dll")
    .Concat(Directory.GetFiles(grasshopperDir, "*.dll"))
    .Concat(new[] { rhinoPath, "/home/vscode/.nuget/packages/system.drawing.common/10.0.2/lib/net8.0/System.Drawing.Common.dll" })
    .Distinct(StringComparer.OrdinalIgnoreCase)
    .ToList();
using var mlc = new MetadataLoadContext(new PathAssemblyResolver(resolverPaths));
var asm = mlc.LoadFromAssemblyPath(grasshopperPath);
foreach (var t in asm.GetTypes().Where(t => t.FullName != null && t.FullName.Contains("Grasshopper.Rhinoceros.Model.Params.Param_Model")).OrderBy(t=>t.FullName))
  Console.WriteLine(t.FullName);

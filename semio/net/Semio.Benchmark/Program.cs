#region 🔖Header
// [👤semio📚net🛅semiobenchmark💻program](repo://p/u/semio/b/l/net/fd/req/Semio.Benchmark/f/Program.cs)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 🔖Header

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Semio;
using Newtonsoft.Json.Linq;

namespace Semio.Benchmark;

class Program
{
    const string AssetsPath = "../assets/semio";
    const int Iterations = 3;
    const float Tolerance = 1e-5f;

    static T LoadAsset<T>(string filename)
    {
        var path = Path.Combine(AssetsPath, filename);
        if (!System.IO.File.Exists(path)) throw new FileNotFoundException($"Asset not found at {Path.GetFullPath(path)}");
        var json = System.IO.File.ReadAllText(path);
        return JsonConvert.DeserializeObject<T>(json)!;
    }

    static void Bench(string name, Action action)
    {
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < Iterations; i++)
        {
            action();
        }
        sw.Stop();
        double duration = sw.Elapsed.TotalSeconds / Iterations;
        Console.WriteLine($"{name},{duration:F6}");
    }

    static Design FindDesign(Kit kit, string name, string? parentName = null)
    {
        string? parentGuid = null;
        if (parentName != null)
        {
            var p = kit.Designs.FirstOrDefault(d => d.Name == parentName);
            if (p == null) throw new Exception($"Parent {parentName} not found");
            parentGuid = p.Guid;
        }

        var d = kit.Designs.FirstOrDefault(d => d.Name == name && (parentGuid != null ? d.Parent?.Guid == parentGuid : d.Parent == null));
        if (d == null) throw new Exception($"Design {name} not found");
        return d;
    }

    static void Main(string[] args)
    {
        if (args.Length > 0 && args.Contains("--native-bridge"))
        {
            RunNativeBridge();
            return;
        }

        var kitMetabolism = LoadAsset<Kit>("metabolism.kit.semio.json");
        var kitInvalid = LoadAsset<Kit>("invalid.kit.semio.json");
        var diffForward = LoadAsset<KitDiff>("metabolism.kit.diff.semio.json");
        var diffInverse = LoadAsset<KitDiff>("metabolism.kit.diff.inverted.semio.json");

        Bench("Roundtrip/Metabolism", () =>
        {
            var zipPath = Path.Combine(AssetsPath, "metabolism.zip");
            var importResult = ZipRoundtrip.ImportKit(zipPath);

            var tempZipPath = "temp_benchmark_metabolism.zip";

            ZipRoundtrip.ExportKit(importResult.Kit, tempZipPath);
            if (System.IO.File.Exists(tempZipPath)) System.IO.File.Delete(tempZipPath);
        });

        Bench("Diff/Metabolism", () =>
        {
            var k2 = Kit.ApplyDiff(kitMetabolism, diffForward);
            Kit.ApplyDiff(k2, diffInverse);
        });

        var d1 = FindDesign(kitMetabolism, "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower", () =>
        {
            Design.Flatten(Entity<Design>.DeepClone(d1)!, kitMetabolism.Types);
        });

        var d2 = FindDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Slanted", () =>
        {
            Design.Flatten(Entity<Design>.DeepClone(d2)!, kitMetabolism.Types);
        });

        var d3 = FindDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Twisted", () =>
        {
            Design.Flatten(Entity<Design>.DeepClone(d3)!, kitMetabolism.Types);
        });

        var d4 = FindDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Dancing", () =>
        {
            Design.Flatten(Entity<Design>.DeepClone(d4)!, kitMetabolism.Types);
        });

        var d5 = FindDesign(kitMetabolism, "Capsule Dream");
        Bench("Flatten Design/Capsule Dream", () =>
        {
            Design.Flatten(Entity<Design>.DeepClone(d5)!, kitMetabolism.Types);
        });

        Bench("Validation/Invalid Kit", () =>
        {
            SemioValidator.ValidateKit(kitInvalid);
        });

        Bench("Validation/Metabolism", () =>
        {
            SemioValidator.ValidateKit(kitMetabolism);
        });
    }

    #region 🔖Native Bridge
    // [👤semio📚net🛅semiobenchmark💻program🔖nativebridge](repo://p/u/semio/b/l/net/fd/req/Semio.Benchmark/f/Program.cs/s/Native%20Bridge)
    // Specs: When invoked with --native-bridge, read JSON from stdin and write {ok,result,error} to stdout.
    // Summary: Provides a csharp native bridge for semio/algorithms native-algorithms REST (without semio/engine).

    class BridgeRequest
    {
        [JsonProperty("op")] public string Op { get; set; } = "";
        [JsonProperty("kit")] public JToken Kit { get; set; } = new JObject();
        [JsonProperty("design")] public JToken Design { get; set; } = new JObject();
        [JsonProperty("designGuid")] public string DesignGuid { get; set; } = "";
        [JsonProperty("pieceGuids")] public List<string> PieceGuids { get; set; } = new();
        [JsonProperty("connectionGuids")] public List<string> ConnectionGuids { get; set; } = new();
    }

    class BridgeResponse
    {
        [JsonProperty("ok")] public bool Ok { get; set; }
        [JsonProperty("result")] public JToken? Result { get; set; }
        [JsonProperty("error")] public string? Error { get; set; }
    }

    static void RunNativeBridge()
    {
        try
        {
            var input = Console.In.ReadToEnd();
            var req = JsonConvert.DeserializeObject<BridgeRequest>(input);
            if (req == null) throw new Exception("parse request: null");

            var kit = req.Kit.ToObject<Kit>();
            if (kit == null) throw new Exception("parse kit: null");

            switch (req.Op)
            {
                case "flatten":
                {
                    var diff = Kit.FlattenDesign(kit, req.DesignGuid);
                    WriteOk(JToken.FromObject(diff));
                    return;
                }
                case "delete":
                {
                    var design = req.Design.ToObject<Design>();
                    if (design == null) throw new Exception("parse design: null");
                    var diff = Design.DeletePiecesAndConnectionsInDesign(kit, design, req.PieceGuids ?? new List<string>(), req.ConnectionGuids ?? new List<string>());
                    WriteOk(JToken.FromObject(diff));
                    return;
                }
                default:
                    WriteErr("unknown op: " + req.Op);
                    return;
            }
        }
        catch (Exception e)
        {
            WriteErr(e.Message);
        }
    }

    static void WriteOk(JToken result)
    {
        var resp = new BridgeResponse { Ok = true, Result = result, Error = null };
        Console.Out.WriteLine(JsonConvert.SerializeObject(resp));
    }

    static void WriteErr(string msg)
    {
        var resp = new BridgeResponse { Ok = false, Result = null, Error = msg };
        Console.Out.WriteLine(JsonConvert.SerializeObject(resp));
    }

    #endregion 🔖Native Bridge
}

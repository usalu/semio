using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Semio;

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
        var kitMetabolism = LoadAsset<Kit>("kit_metabolism.json");
        var kitInvalid = LoadAsset<Kit>("kit_invalid.json");
        var diffForward = LoadAsset<KitDiff>("diff_kit_metabolism.json");
        var diffInverse = LoadAsset<KitDiff>("diff_kit_metabolism_inverted.json");

        // 1. Roundtrip/Metabolism
        Bench("Roundtrip/Metabolism", () =>
        {
            var zipPath = Path.Combine(AssetsPath, "metabolism.zip");
            var importResult = ZipRoundtrip.ImportKit(zipPath);

            var tempZipPath = "temp_benchmark_metabolism.zip";
            // We need schema SQL. Let's assume we can get it or pass empty if not strictly checked by implementation (it executes it).
            // The C# implementation might need the schema to create tables.
            // Let's try to read it.
            var schemaPath = Path.Combine("../../sql/sqlite/schema.sql"); // Relative to bin output?
            // AssetsPath is "../assets/semio" which is relative to execution dir?
            // "dotnet run" runs from project dir? No, usually bin/Debug/net...
            // Go benchmark used "../../sql/sqlite/schema.sql" relative to "go/semio/benchmark"
            // If running from "net" folder: "dotnet run --project Semio.Benchmark..."
            // The CWD is "net".
            // So schema is at "../sql/sqlite/semio/schema.sql".
            var schemaSql = System.IO.File.ReadAllText("../sql/sqlite/semio/schema.sql");

            ZipRoundtrip.ExportKit(importResult.Kit, importResult.Files, tempZipPath, schemaSql);
            if (System.IO.File.Exists(tempZipPath)) System.IO.File.Delete(tempZipPath);
        });

        // 2. Diff/Metabolism
        Bench("Diff/Metabolism", () =>
        {
            var k2 = kitMetabolism.ApplyDiff(diffForward);
            k2.ApplyDiff(diffInverse);
        });

        // 3. Flatten Design/Nakagin Capsule Tower
        var d1 = FindDesign(kitMetabolism, "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower", () =>
        {
            d1.Flatten(kitMetabolism.Types);
        });

        // 4. Flatten Design/Nakagin Capsule Tower/Slanted
        var d2 = FindDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Slanted", () =>
        {
            d2.Flatten(kitMetabolism.Types);
        });

        // 5. Flatten Design/Nakagin Capsule Tower/Twisted
        var d3 = FindDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Twisted", () =>
        {
            d3.Flatten(kitMetabolism.Types);
        });

        // 6. Flatten Design/Nakagin Capsule Tower/Dancing
        var d4 = FindDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Dancing", () =>
        {
            d4.Flatten(kitMetabolism.Types);
        });

        // 7. Flatten Design/Capsule Dream
        var d5 = FindDesign(kitMetabolism, "Capsule Dream");
        Bench("Flatten Design/Capsule Dream", () =>
        {
            d5.Flatten(kitMetabolism.Types);
        });

        // 8. Validation/Invalid Kit
        Bench("Validation/Invalid Kit", () =>
        {
            SemioValidator.ValidateKit(kitInvalid);
        });

        // 9. Validation/Metabolism
        Bench("Validation/Metabolism", () =>
        {
            SemioValidator.ValidateKit(kitMetabolism);
        });
    }
}

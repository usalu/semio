#region 📱️Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 📱️Header

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using Compose;

namespace Compose.Benchmark;

class Program
{
    const string AssetsPath = "../asset/compose";
    const int Iterations = 3;
    static readonly string[] BenchmarkCsvLanguages = { "go", "typescript", "python", "rust", "csharp" };

    static string ResolveAssetPath(string filename)
    {
        var candidates = new[]
        {
            Path.Combine(AssetsPath, filename),
            Path.Combine("compose", "assets", "compose", filename),
            Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "..", "assets", "compose", filename),
            Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "assets", "compose", filename),
        };
        foreach (var candidate in candidates)
        {
            var fullPath = Path.GetFullPath(candidate);
            if (System.IO.File.Exists(fullPath)) return fullPath;
        }
        return Path.GetFullPath(candidates[0]);
    }

    static T LoadAsset<T>(string filename)
    {
        var path = ResolveAssetPath(filename);
        if (!System.IO.File.Exists(path)) throw new FileNotFoundException($"Asset not found at {path}");
        var json = System.IO.File.ReadAllText(path);
        if (typeof(T) == typeof(Kit))
            return (T)(object)Utility.DeserializeKit(json)!;
        return Utility.Deserialize<T>(json)!;
    }

    static string ResolveBenchmarkCsvPath()
    {
        var candidates = new[]
        {
            Path.Combine("compose", "benchmark.csv"),
            Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "benchmark.csv"),
        };
        foreach (var candidate in candidates)
        {
            var fullPath = Path.GetFullPath(candidate);
            var directory = Path.GetDirectoryName(fullPath);
            if (directory != null && Directory.Exists(directory)) return fullPath;
        }
        return Path.GetFullPath(candidates[0]);
    }

    static string CsvValue(string value) => "\"" + value.Replace("\"", "\"\"") + "\"";

    static List<string> ParseCsvLine(string line)
    {
        var values = new List<string>();
        var current = "";
        var inQuotes = false;
        for (var i = 0; i < line.Length; i++)
        {
            var ch = line[i];
            if (ch == '"')
            {
                if (inQuotes && i + 1 < line.Length && line[i + 1] == '"')
                {
                    current += '"';
                    i++;
                }
                else
                {
                    inQuotes = !inQuotes;
                }
            }
            else if (ch == ',' && !inQuotes)
            {
                values.Add(current);
                current = "";
            }
            else
            {
                current += ch;
            }
        }
        values.Add(current);
        return values;
    }

    static void AppendBenchmarkCsv(string language, string name, double durationSeconds)
    {
        var path = ResolveBenchmarkCsvPath();
        var rows = new Dictionary<string, Dictionary<string, string>>();
        var order = new List<string>();
        if (System.IO.File.Exists(path))
        {
            var lines = System.IO.File.ReadAllLines(path).Where(l => !string.IsNullOrWhiteSpace(l)).ToList();
            if (lines.Count > 0 && lines[0].StartsWith("name,", StringComparison.Ordinal))
            {
                var headers = ParseCsvLine(lines[0]);
                foreach (var line in lines.Skip(1))
                {
                    var values = ParseCsvLine(line);
                    if (values.Count == 0 || string.IsNullOrEmpty(values[0])) continue;
                    if (!rows.ContainsKey(values[0]))
                    {
                        rows[values[0]] = new Dictionary<string, string>();
                        order.Add(values[0]);
                    }
                    for (var i = 1; i < values.Count && i < headers.Count; i++)
                    {
                        if (!string.IsNullOrEmpty(values[i])) rows[values[0]][headers[i]] = values[i];
                    }
                }
            }
        }
        if (!rows.ContainsKey(name))
        {
            rows[name] = new Dictionary<string, string>();
            order.Add(name);
        }
        rows[name][language] = (durationSeconds * 1000).ToString("F6", CultureInfo.InvariantCulture);
        var output = "name," + string.Join(",", BenchmarkCsvLanguages) + "\n";
        foreach (var rowName in order)
        {
            output += CsvValue(rowName);
            foreach (var lang in BenchmarkCsvLanguages)
            {
                output += ",";
                if (rows[rowName].TryGetValue(lang, out var value)) output += value;
            }
            output += "\n";
        }
        System.IO.File.WriteAllText(path, output);
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
        Console.WriteLine($"{name},{duration.ToString("F6", CultureInfo.InvariantCulture)}");
        AppendBenchmarkCsv("csharp", name, duration);
    }

    static Design FindDesign(Kit kit, string name, string? parentName = null)
    {
        string? parentId = null;
        if (parentName != null)
        {
            var p = kit.Designs.FirstOrDefault(d => d.Name == parentName);
            if (p == null) throw new Exception($"Parent {parentName} not found");
            parentId = p.Id;
        }

        var d = kit.Designs.FirstOrDefault(d => d.Name == name && (parentId != null ? d.Parent?.Id == parentId : d.Parent == null));
        if (d == null) throw new Exception($"Design {name} not found");
        return d;
    }

    static void Main(string[] args)
    {
        var kitMetabolism = LoadAsset<Kit>("stores/metabolism/wip/initialKit/kit.compose.json");
        var kitOriginal = LoadAsset<Kit>("stores/metabolism/wip/initialKit/kit.compose.json");
        kitOriginal.Designs = kitOriginal.Designs.Where(d => d.Parent == null).ToList();
        var kitDiffed = LoadAsset<Kit>("metabolism.kit.diffed.compose.json");
        var kitInvalid = LoadAsset<Kit>("invalid.kit.compose.json");
        var diffForward = LoadAsset<KitDiff>("metabolism.kit.diff.compose.json");
        var diffInverse = LoadAsset<KitDiff>("metabolism.kit.diff.inverted.compose.json");

        Bench("Roundtrip/Metabolism", () =>
        {
            var json = Utility.Serialize(kitMetabolism, "  ");
            var restored = Utility.DeserializeKit(json)!;
            if (!ComposeDiff.AreKitsEqual(kitMetabolism, restored)) throw new Exception("Roundtrip/Metabolism output does not match test expectation");
        });

        Bench("Diff/Metabolism", () =>
        {
            var k2 = Utility.DeserializeKit(Utility.Serialize(kitOriginal))!;
            KitInPlaceDiff.ApplyKitDiff(k2, diffForward);
            if (!ComposeDiff.AreKitsEqual(k2, kitDiffed)) throw new Exception("Diff/Metabolism forward output does not match test expectation");
            var restored = Utility.DeserializeKit(Utility.Serialize(k2))!;
            KitInPlaceDiff.ApplyKitDiff(restored, diffInverse);
            if (!ComposeDiff.AreKitsEqual(restored, kitOriginal)) throw new Exception("Diff/Metabolism inverse output does not match test expectation");
        });

        var d1 = FindDesign(kitMetabolism, "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower", () =>
        {
            var flat = Kit.FlattenDesignDiff(kitMetabolism, d1.Id);
            if (flat.Pieces?.Modified == null || flat.Pieces.Modified.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower output does not match test expectation");
        });

        var d2 = FindDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Slanted", () =>
        {
            var flat = Kit.FlattenDesignDiff(kitMetabolism, d2.Id);
            if (flat.Pieces?.Modified == null || flat.Pieces.Modified.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Slanted output does not match test expectation");
        });

        var d3 = FindDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Twisted", () =>
        {
            var flat = Kit.FlattenDesignDiff(kitMetabolism, d3.Id);
            if (flat.Pieces?.Modified == null || flat.Pieces.Modified.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Twisted output does not match test expectation");
        });

        var d4 = FindDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Dancing", () =>
        {
            var flat = Kit.FlattenDesignDiff(kitMetabolism, d4.Id);
            if (flat.Pieces?.Modified == null || flat.Pieces.Modified.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Dancing output does not match test expectation");
        });

        var d5 = FindDesign(kitMetabolism, "Capsule Dream");
        Bench("Flatten Design/Capsule Dream", () =>
        {
            var flat = Kit.FlattenDesignDiff(kitMetabolism, d5.Id);
            if (flat.Pieces?.Modified == null || flat.Pieces.Modified.Count == 0) throw new Exception("Flatten Design/Capsule Dream output does not match test expectation");
        });

        Bench("Validation/Invalid Kit", () =>
        {
            var result = ComposeValidator.ValidateKit(kitInvalid);
            if (result.Issues.Count == 0) throw new Exception("Validation/Invalid Kit output does not match test expectation");
        });

        Bench("Validation/Metabolism", () =>
        {
            var result = ComposeValidator.ValidateKit(kitMetabolism);
            if (result.Issues.Count != 0) throw new Exception("Validation/Metabolism output does not match test expectation");
        });
    }
}

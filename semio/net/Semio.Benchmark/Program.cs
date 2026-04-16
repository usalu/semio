#region 📱Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 📱Header

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using Semio;

namespace Semio.Benchmark;

class Program
{
    const string AssetsPath = "../assets/semio";
    const int Iterations = 3;

    static string ResolveAssetPath(string filename)
    {
        var candidates = new[]
        {
            Path.Combine(AssetsPath, filename),
            Path.Combine("semio", "assets", "semio", filename),
            Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "..", "assets", "semio", filename),
            Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "assets", "semio", filename),
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
        return Utility.Deserialize<T>(json)!;
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
        var kitMetabolism = LoadAsset<Kit>("metabolism.kit.semio.json");
        var kitOriginal = LoadAsset<Kit>("metabolism.kit.semio.json");
        kitOriginal.Designs = kitOriginal.Designs.Where(d => d.Parent == null).ToList();
        var kitDiffed = LoadAsset<Kit>("metabolism.kit.diffed.semio.json");
        var kitInvalid = LoadAsset<Kit>("invalid.kit.semio.json");
        var diffForward = LoadAsset<KitDiff>("metabolism.kit.diff.semio.json");
        var diffInverse = LoadAsset<KitDiff>("metabolism.kit.diff.inverted.semio.json");

        Bench("Roundtrip/Metabolism", () =>
        {
            var json = Utility.Serialize(kitMetabolism);
            var restored = Utility.Deserialize<Kit>(json)!;
            if (!SemioDiff.AreKitsEqual(kitMetabolism, restored)) throw new Exception("Roundtrip/Metabolism output does not match test expectation");
        });

        Bench("Diff/Metabolism", () =>
        {
            var change = SemioDiff.GetKitChange(kitOriginal, kitDiffed);
            if (!SemioDiff.AreKitDiffsEqual(change.Forward, diffForward)) throw new Exception("Diff/Metabolism forward diff output does not match test expectation");
            if (!SemioDiff.AreKitDiffsEqual(change.Backward, diffInverse)) throw new Exception("Diff/Metabolism inverse diff output does not match test expectation");
            var k2 = SemioDiff.ApplyKitDiff(kitOriginal, change.Forward);
            if (!SemioDiff.AreKitsEqual(k2, kitDiffed)) throw new Exception("Diff/Metabolism forward output does not match test expectation");
            var restored = SemioDiff.ApplyKitDiff(k2, change.Backward);
            if (!SemioDiff.AreKitsEqual(restored, kitOriginal)) throw new Exception("Diff/Metabolism inverse output does not match test expectation");
        });

        var d1 = FindDesign(kitMetabolism, "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower", () =>
        {
            var flat = Kit.FlattenDesign(kitMetabolism, d1.Guid);
            if (flat.Pieces?.Updated == null || flat.Pieces.Updated.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower output does not match test expectation");
        });

        var d2 = FindDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Slanted", () =>
        {
            var flat = Kit.FlattenDesign(kitMetabolism, d2.Guid);
            if (flat.Pieces?.Updated == null || flat.Pieces.Updated.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Slanted output does not match test expectation");
        });

        var d3 = FindDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Twisted", () =>
        {
            var flat = Kit.FlattenDesign(kitMetabolism, d3.Guid);
            if (flat.Pieces?.Updated == null || flat.Pieces.Updated.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Twisted output does not match test expectation");
        });

        var d4 = FindDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
        Bench("Flatten Design/Nakagin Capsule Tower/Dancing", () =>
        {
            var flat = Kit.FlattenDesign(kitMetabolism, d4.Guid);
            if (flat.Pieces?.Updated == null || flat.Pieces.Updated.Count == 0) throw new Exception("Flatten Design/Nakagin Capsule Tower/Dancing output does not match test expectation");
        });

        var d5 = FindDesign(kitMetabolism, "Capsule Dream");
        Bench("Flatten Design/Capsule Dream", () =>
        {
            var flat = Kit.FlattenDesign(kitMetabolism, d5.Guid);
            if (flat.Pieces?.Updated == null || flat.Pieces.Updated.Count == 0) throw new Exception("Flatten Design/Capsule Dream output does not match test expectation");
        });

        Bench("Validation/Invalid Kit", () =>
        {
            var result = SemioValidator.ValidateKit(kitInvalid);
            if (result.Issues.Count == 0) throw new Exception("Validation/Invalid Kit output does not match test expectation");
        });

        Bench("Validation/Metabolism", () =>
        {
            var result = SemioValidator.ValidateKit(kitMetabolism);
            if (result.Issues.Count != 0) throw new Exception("Validation/Metabolism output does not match test expectation");
        });
    }
}

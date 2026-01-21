#region Header

// net/Semio.Tests/Tests.cs

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

#endregion Header

using Newtonsoft.Json;
using Xunit;
using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

namespace Semio.Tests;

public class Tests
{
    public static readonly string AssetsPath = "../../../../../assets/semio";
    private const double Tolerance = 0.001;

    public static T LoadAsset<T>(string filename)
    {
        var path = Path.Combine(AssetsPath, filename);
        if (!System.IO.File.Exists(path)) throw new FileNotFoundException($"Asset not found at {Path.GetFullPath(path)}");
        var json = System.IO.File.ReadAllText(path);
        return JsonConvert.DeserializeObject<T>(json)!;
    }

    public static bool PlanesEqual(Plane? p1, Plane? p2)
    {
        if (p1 == null || p2 == null) return p1 == p2;
        return VectorsEqual(p1.Origin, p2.Origin) &&
               VectorsEqual(p1.XAxis, p2.XAxis) &&
               VectorsEqual(p1.YAxis, p2.YAxis);
    }

    private static bool VectorsEqual(Point? p1, Point? p2)
    {
         if (p1 == null || p2 == null) return p1 == p2;
         return Math.Abs(p1.X - p2.X) < Tolerance &&
                Math.Abs(p1.Y - p2.Y) < Tolerance &&
                Math.Abs(p1.Z - p2.Z) < Tolerance;
    }
    
    private static bool VectorsEqual(Vector? v1, Vector? v2)
    {
         if (v1 == null || v2 == null) return v1 == v2;
         return Math.Abs(v1.X - v2.X) < Tolerance &&
                Math.Abs(v1.Y - v2.Y) < Tolerance &&
                Math.Abs(v1.Z - v2.Z) < Tolerance;
    }

    public static bool CentersEqual(Coord? c1, Coord? c2)
    {
        if (c1 == null || c2 == null) return c1 == c2;
        return Math.Abs(c1.U - c2.U) < Tolerance && Math.Abs(c1.V - c2.V) < Tolerance;
    }

    // #region Roundtrip

    public class Roundtrip
    {
        public class Json
        {
            [Fact]
            public void Metabolism_Kit_Json_Kit()
            {
                var kit = Tests.LoadAsset<Kit>("kit_metabolism.json");
                var json = kit.Serialize();
                var deserializedKit = json.Deserialize<Kit>();
                Assert.Equal(kit.Serialize(), deserializedKit!.Serialize());
            }
        }

        public class Zip
        {
            [Fact]
            public void Metabolism_Zip_Kit_Zip_Kit()
            {
                var zipPath = Path.Combine(Tests.AssetsPath, "metabolism.zip");
                var (kit, files) = KitImporter.ImportFromZip(zipPath);
                
                Assert.NotNull(kit.Guid);
                Assert.Equal("Metabolism", kit.Name);
                Assert.True(kit.Types?.Count > 0);
                Assert.True(kit.Designs?.Count > 0);
                Assert.True(files.Count > 0);

                var tempPath = Path.Combine(Path.GetTempPath(), "metabolism_roundtrip.zip");
                KitExporter.ExportToZip(kit, files, tempPath);
                
                var (kit2, files2) = KitImporter.ImportFromZip(tempPath);
                Assert.Equal(kit.Guid, kit2.Guid);
                Assert.Equal(kit.Name, kit2.Name);
                Assert.Equal(kit.Types?.Count, kit2.Types?.Count);
                Assert.Equal(kit.Designs?.Count, kit2.Designs?.Count);
                Assert.Equal(files.Count, files2.Count);
                
                System.IO.File.Delete(tempPath);
            }
        }
    }

    // #endregion Roundtrip

    // #region Flatten

    public class Flatten
    {
        [Fact]
        public void Nakagin_Capsule_Tower_Kit_Flatten_Diff_Apply_Flat() => TestFlatten("Nakagin Capsule Tower");

        [Fact]
        public void Nakagin_Capsule_Tower_Slanted_Kit_Flatten_Diff_Apply_Flat() => TestFlatten("Slanted", "Nakagin Capsule Tower");
        
        [Fact]
        public void Nakagin_Capsule_Tower_Twisted_Kit_Flatten_Diff_Apply_Flat() => TestFlatten("Twisted", "Nakagin Capsule Tower");

        [Fact]
        public void Nakagin_Capsule_Tower_Dancing_Kit_Flatten_Diff_Apply_Flat() => TestFlatten("Dancing", "Nakagin Capsule Tower");

        [Fact]
        public void Capsule_Dream_Kit_Flatten_Diff_Apply_Flat() => TestFlatten("Capsule Dream");

        private void TestFlatten(string designName, string? parentName = null)
        {
            var kit = Tests.LoadAsset<Kit>("kit_metabolism.json");
            var design = FindDesign(kit, designName, parentName);
            
            var expectedDesign = kit.Designs.FirstOrDefault(d => d.Name == "Flat" && d.Parent?.Guid == design.Guid);
            Assert.NotNull(expectedDesign);

            var flatDesign = design.DeepClone()!.Flatten(kit.Types);
            
            foreach(var p in flatDesign.Pieces)
            {
                var expectedPiece = expectedDesign!.Pieces.FirstOrDefault(ep => ep.Name == p.Name);
                Assert.NotNull(expectedPiece);
                Assert.NotNull(p.Plane);
                
                if (!Tests.PlanesEqual(p.Plane, expectedPiece.Plane))
                {
                    var actual = p.Plane!;
                    var expected = expectedPiece.Plane!;
                    Console.WriteLine($"[DEBUG] Plane mismatch for piece {p.Name}");
                    Console.WriteLine($"  Expected Origin: ({expected.Origin.X:F6}, {expected.Origin.Y:F6}, {expected.Origin.Z:F6})");
                    Console.WriteLine($"  Actual   Origin: ({actual.Origin.X:F6}, {actual.Origin.Y:F6}, {actual.Origin.Z:F6})");
                    Console.WriteLine($"  Expected XAxis: ({expected.XAxis.X:F6}, {expected.XAxis.Y:F6}, {expected.XAxis.Z:F6})");
                    Console.WriteLine($"  Actual   XAxis: ({actual.XAxis.X:F6}, {actual.XAxis.Y:F6}, {actual.XAxis.Z:F6})");
                    Console.WriteLine($"  Expected YAxis: ({expected.YAxis.X:F6}, {expected.YAxis.Y:F6}, {expected.YAxis.Z:F6})");
                    Console.WriteLine($"  Actual   YAxis: ({actual.YAxis.X:F6}, {actual.YAxis.Y:F6}, {actual.YAxis.Z:F6})");
                    Assert.Fail($"Plane mismatch for piece {p.Name}");
                }
                if (p.Center != null && expectedPiece.Center != null)
                {
                     Assert.True(Tests.CentersEqual(p.Center, expectedPiece.Center), $"Center mismatch for piece {p.Name}");
                }
            }
        }

        private static Design FindDesign(Kit kit, string name, string? parentName = null)
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
    }

    // #endregion Flatten

    // #region Diff

    public class Diff
    {
        [Fact]
        public void Metabolism_Kit_Diff_DiffedKit_InverseDiff_Kit()
        {
            var kitOriginal = Tests.LoadAsset<Kit>("kit_metabolism.json");
            kitOriginal.Designs = kitOriginal.Designs?.Where(d => d.Parent == null).ToList();
            
            var kitDiff = Tests.LoadAsset<KitDiff>("diff_kit_metabolism.json");
            var kitDiffInverted = Tests.LoadAsset<KitDiff>("diff_kit_metabolism_inverted.json");
            var kitDiffed = Tests.LoadAsset<Kit>("kit_metabolism_diffed.json");

            var computedDiff = SemioDiff.GetKitDiff(kitOriginal, kitDiffed);
            Assert.True(SemioDiff.AreKitDiffsEqual(computedDiff, kitDiff));
            
            var computedInverseDiff = SemioDiff.InverseKitDiff(kitOriginal, kitDiff);
            Assert.True(SemioDiff.AreKitDiffsEqual(computedInverseDiff, kitDiffInverted));
            
            var appliedForward = SemioDiff.ApplyKitDiff(kitOriginal, kitDiff);
            Assert.True(SemioDiff.AreKitsEqual(appliedForward, kitDiffed));
            
            var appliedInverse = SemioDiff.ApplyKitDiff(kitDiffed, kitDiffInverted);
            Assert.True(SemioDiff.AreKitsEqual(appliedInverse, kitOriginal));
        }
    }

    // #endregion Diff

    // #region Validation

    public class Validation
    {
        [Fact]
        public void Metabolism_Kit_Validate_Empty_Report()
        {
            var kit = Tests.LoadAsset<Kit>("kit_metabolism.json");
            var result = SemioValidator.ValidateKit(kit);
            Assert.Empty(result.Issues);
        }

        [Fact]
        public void Invalid_Kit_Validate_Invalid_Report()
        {
            var kit = Tests.LoadAsset<Kit>("kit_invalid.json");
            var result = SemioValidator.ValidateKit(kit);
            var filePath = Path.Combine(Tests.AssetsPath, "validation.json");
            var expectedJson = System.IO.File.ReadAllText(filePath);
            var expected = ValidationResult.Parse(expectedJson);
            
            Assert.True(ValidationResult.AreEqual(expected, result), $"Expected {expected.Issues.Count} issues, got {result.Issues.Count}. Expected:\n{expected.Serialize()}\nActual:\n{result.Serialize()}");
        }
    }

    // #endregion Validation
}

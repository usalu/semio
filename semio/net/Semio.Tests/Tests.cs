#region 🔖Header
// [👤semio📚net🛅semiotests💻tests](semiorepo://p/u/semio/b/l/net/fd/req/Semio.Tests/f/Tests.cs)

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

#endregion 🔖Header

using Semio;
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

    private sealed class ModelSelectionAsset
    {
        public List<ModelSelectionCase> Cases { get; set; } = new();
    }

    private sealed class ModelSelectionCase
    {
        public string Name { get; set; } = "";
        public List<string> SelectedTagGuids { get; set; } = new();
        public string? ExpectedGuid { get; set; }
        public List<ModelSelectionModel> Models { get; set; } = new();
    }

    private sealed class ModelSelectionModel
    {
        public string Guid { get; set; } = "";
        public string FileGuid { get; set; } = "";
        public List<string> TagGuids { get; set; } = new();
    }

    public static T LoadAsset<T>(string filename)
    {
        var path = Path.Combine(AssetsPath, filename);
        if (!System.IO.File.Exists(path)) throw new FileNotFoundException($"Asset not found at {Path.GetFullPath(path)}");
        var json = System.IO.File.ReadAllText(path);
        return Utility.Deserialize<T>(json)!;
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

    public class Roundtrip
    {
        [Fact]
        public void Metabolism_Json_Memory_Json_Json_Zip_Zip_Json()
        {

            var kit = Tests.LoadAsset<Kit>("kit_metabolism.json");
            var json = Utility.Serialize(kit);
            var deserializedKit = Utility.Deserialize<Kit>(json);
            Assert.Equal(Utility.Serialize(kit), Utility.Serialize(deserializedKit!));


            var zipPath = Path.Combine(Tests.AssetsPath, "metabolism.zip");
            var result = KitImporter.ImportFromZip(zipPath);
            var zipKit = result.Kit;
            Assert.True(SemioDiff.AreKitsEqual(kit, zipKit), "Kit loaded from pre-existing zip does not semantically match kit loaded from JSON");


            var tempPath = Path.Combine(Path.GetTempPath(), "metabolism_roundtrip.zip");
            try
            {
                KitExporter.ExportToZip(kit, tempPath);
                var result2 = KitImporter.ImportFromZip(tempPath);
                Assert.True(SemioDiff.AreKitsEqual(kit, result2.Kit), "Kit exported to zip and reimported does not semantically match original kit");
            }
            finally
            {
                if (System.IO.File.Exists(tempPath))
                    System.IO.File.Delete(tempPath);
            }
        }

        public class Sqlite
        {
            [Fact]
            public void Metabolism_Kit_Sqlite_Kit()
            {
                var kit = Tests.LoadAsset<Kit>("kit_metabolism.json");

                var tempDir = Path.Combine(Path.GetTempPath(), "semio_sqlite_test_" + Guid.NewGuid().ToString("N"));
                Directory.CreateDirectory(tempDir);
                try
                {
                    KitSqlite.SaveKit(tempDir, kit);
                    var loadedKit = KitSqlite.LoadKit(tempDir);

                    Assert.Equal(kit.Guid, loadedKit.Guid);
                    Assert.Equal(kit.Name, loadedKit.Name);
                    Assert.Equal(kit.Version, loadedKit.Version);
                    Assert.Equal(kit.Description, loadedKit.Description);
                    Assert.Equal(kit.Qualities?.Count ?? 0, loadedKit.Qualities?.Count ?? 0);
                    Assert.Equal(kit.Ports?.Count ?? 0, loadedKit.Ports?.Count ?? 0);
                    Assert.Equal(kit.Tags?.Count ?? 0, loadedKit.Tags?.Count ?? 0);
                    Assert.Equal(kit.Concepts?.Count ?? 0, loadedKit.Concepts?.Count ?? 0);
                    Assert.Equal(kit.Files?.Count ?? 0, loadedKit.Files?.Count ?? 0);
                    Assert.Equal(kit.Folders?.Count ?? 0, loadedKit.Folders?.Count ?? 0);
                    Assert.Equal(kit.Authors?.Count ?? 0, loadedKit.Authors?.Count ?? 0);
                    Assert.Equal(kit.Types?.Count ?? 0, loadedKit.Types?.Count ?? 0);
                    Assert.Equal(kit.Designs?.Count ?? 0, loadedKit.Designs?.Count ?? 0);

                    foreach (var type in kit.Types ?? new List<Type>())
                    {
                        var loadedType = loadedKit.Types?.FirstOrDefault(t => t.Guid == type.Guid);
                        Assert.NotNull(loadedType);
                        Assert.Equal(type.Name, loadedType.Name);
                        Assert.Equal(type.Connectors?.Count ?? 0, loadedType.Connectors?.Count ?? 0);
                    }

                    foreach (var design in kit.Designs ?? new List<Design>())
                    {
                        var loadedDesign = loadedKit.Designs?.FirstOrDefault(d => d.Guid == design.Guid);
                        Assert.NotNull(loadedDesign);
                        Assert.Equal(design.Name, loadedDesign.Name);
                        Assert.Equal(design.Pieces?.Count ?? 0, loadedDesign.Pieces?.Count ?? 0);
                        Assert.Equal(design.Connections?.Count ?? 0, loadedDesign.Connections?.Count ?? 0);
                    }
                }
                finally
                {
                    Directory.Delete(tempDir, true);
                }
            }
        }
    }

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

            var flatDesign = Design.Flatten(Entity<Design>.DeepClone(design)!, kit.Types);

            foreach (var p in flatDesign.Pieces)
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
                    if (!Tests.CentersEqual(p.Center, expectedPiece.Center))
                    {
                        Console.WriteLine($"[DEBUG] Center mismatch for piece {p.Name}");
                        Console.WriteLine($"  Expected: ({expectedPiece.Center.U:F6}, {expectedPiece.Center.V:F6})");
                        Console.WriteLine($"  Actual:   ({p.Center.U:F6}, {p.Center.V:F6})");
                        Assert.Fail($"Center mismatch for piece {p.Name}");
                    }
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

    public class Change
    {
        [Fact]
        public void Metabolism_Kit_Change_Forward_Backward_Inverse_Behavior()
        {
            var kitOriginal = Tests.LoadAsset<Kit>("kit_metabolism.json");
            kitOriginal.Designs = kitOriginal.Designs?.Where(d => d.Parent == null).ToList();

            var kitDiff = Tests.LoadAsset<KitDiff>("diff_kit_metabolism.json");
            var kitDiffInverted = Tests.LoadAsset<KitDiff>("diff_kit_metabolism_inverted.json");
            var kitDiffed = Tests.LoadAsset<Kit>("kit_metabolism_diffed.json");

            var change = SemioDiff.GetKitChange(kitOriginal, kitDiffed);
            Assert.True(SemioDiff.AreKitDiffsEqual(change.Forward, kitDiff), "GetKitChange: forward diff doesn't match expected diff");

            Assert.True(SemioDiff.AreKitDiffsEqual(change.Backward, kitDiffInverted), "GetKitChange: backward diff doesn't match expected inverse diff");

            var appliedForward = SemioDiff.ApplyKitDiff(kitOriginal, change.Forward);
            Assert.True(SemioDiff.AreKitsEqual(appliedForward, kitDiffed), "ApplyKitDiff forward: applied kit doesn't match expected diffed kit");

            var appliedInverse = SemioDiff.ApplyKitDiff(kitDiffed, change.Backward);
            Assert.True(SemioDiff.AreKitsEqual(appliedInverse, kitOriginal), "ApplyKitDiff inverse: applied inverse kit doesn't match original kit");
        }
    }

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

    public class Drag
    {
        [Fact]
        public void Design_Pieces_Offset_DiffDesign()
        {
            var design = Tests.LoadAsset<Design>("drag/design.json");
            var pieces = Tests.LoadAsset<Design>("drag/pieces.json");
            var offset = Tests.LoadAsset<Coord>("drag/offset.json");
            var expectedDiff = Tests.LoadAsset<DesignDiff>("drag/diff_design.json");
            var computedDiff = Design.DragPiecesInDesign(design, pieces, offset);
            Assert.NotNull(computedDiff.Pieces);
            Assert.Equal(expectedDiff.Pieces!.Updated.Count, computedDiff.Pieces!.Updated.Count);
            var expectedPieceMap = expectedDiff.Pieces.Updated.ToDictionary(u => u.Piece.Guid, u => u.Diff);
            foreach (var u in computedDiff.Pieces.Updated)
            {
                Assert.True(expectedPieceMap.ContainsKey(u.Piece.Guid), $"Unexpected piece update for {u.Piece.Guid}");
                var expected = expectedPieceMap[u.Piece.Guid];
                Assert.NotNull(u.Diff!.Center);
                Assert.NotNull(expected!.Center);
                Assert.Equal(expected.Center!.U, u.Diff.Center!.U, 3);
                Assert.Equal(expected.Center.V, u.Diff.Center.V, 3);
            }
            Assert.NotNull(computedDiff.Connections);
            Assert.Equal(expectedDiff.Connections!.Updated.Count, computedDiff.Connections!.Updated.Count);
            var expectedConnMap = expectedDiff.Connections.Updated.ToDictionary(u => u.Connection.Guid, u => u.Diff);
            foreach (var u in computedDiff.Connections.Updated)
            {
                Assert.True(expectedConnMap.ContainsKey(u.Connection.Guid), $"Unexpected connection update for {u.Connection.Guid}");
                var expected = expectedConnMap[u.Connection.Guid];
                Assert.Equal(expected!.U!.Value, u.Diff!.U!.Value, 3);
                Assert.Equal(expected.V!.Value, u.Diff.V!.Value, 3);
            }
        }
    }

    public class DesignModel
    {
        private static Model? SelectBestModelLikeSemioTs(List<Model> models, List<string> selectedTagGuids)
        {
            if (models.Count == 0) return null;
            if (selectedTagGuids.Count == 0)
            {
                var defaultModel = models.FirstOrDefault(r => r.Tags == null || r.Tags.Count == 0);
                return defaultModel ?? models[0];
            }

            var filtered = models.Where(r => selectedTagGuids.All(tag => r.Tags.Any(t => t.Guid == tag))).ToList();
            if (filtered.Count == 0) return null;

            var type = new Type { Name = "selection-test", Models = filtered };
            return type.FindModel(selectedTagGuids);
        }

        [Fact]
        public void Model_Selection_From_Shared_Semio_Assets()
        {
            var payload = Tests.LoadAsset<ModelSelectionAsset>("model_selection.json");
            foreach (var testCase in payload.Cases)
            {
                var models = testCase.Models
                    .Select(model => new Model
                    {
                        Guid = model.Guid,
                        File = new FileId { Guid = model.FileGuid },
                        Tags = model.TagGuids.Select(tagGuid => new TagId { Guid = tagGuid }).ToList(),
                    })
                    .ToList();

                var selected = SelectBestModelLikeSemioTs(models, testCase.SelectedTagGuids);
                Assert.Equal(testCase.ExpectedGuid, selected?.Guid);
            }
        }
    }

}

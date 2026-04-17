#region 📱Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 📱Header

using Semio;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Xunit;
using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Globalization;
using System.Net;
using System.Net.Sockets;
using System.Text;

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

    private static Kit CreateWorkflowKit()
    {
        var blob = $"data:text/plain;base64,{Convert.ToBase64String(Encoding.UTF8.GetBytes("hello workflow"))}";
        return new Kit
        {
            Guid = "workflow-kit-guid",
            Name = "Workflow Kit",
            Version = "1.0.0",
            CreatedAt = "2026-01-01T00:00:00Z",
            UpdatedAt = "2026-01-01T00:00:00Z",
            Folders = new List<Folder>
            {
                new() { Guid = "folder-guid", Name = "docs", CreatedAt = "2026-01-01T00:00:00Z", UpdatedAt = "2026-01-01T00:00:00Z" }
            },
            Files = new List<File>
            {
                new()
                {
                    Guid = "file-guid",
                    Name = "readme.txt",
                    Folder = new FolderId { Guid = "folder-guid" },
                    Size = Encoding.UTF8.GetByteCount("hello workflow"),
                    Blob = blob,
                    CreatedAt = DateTime.Parse("2026-01-01T00:00:00Z", null, DateTimeStyles.RoundtripKind),
                    UpdatedAt = DateTime.Parse("2026-01-01T00:00:00Z", null, DateTimeStyles.RoundtripKind),
                }
            },
            Types = new List<Type>
            {
                new() { Guid = "type-guid", Name = "Wall", CreatedAt = DateTime.Parse("2026-01-01T00:00:00Z", null, DateTimeStyles.RoundtripKind), UpdatedAt = DateTime.Parse("2026-01-01T00:00:00Z", null, DateTimeStyles.RoundtripKind) }
            }
        };
    }

    public class KitKindTests
    {
        [Fact]
        public void KitKind_Has_Exactly_Five_Values()
        {
            var values = Enum.GetValues(typeof(KitKind)).Cast<KitKind>().ToList();
            Assert.Equal(5, values.Count);
            Assert.Contains(KitKind.File, values);
            Assert.Contains(KitKind.Folder, values);
            Assert.Contains(KitKind.Archive, values);
            Assert.Contains(KitKind.Remote, values);
            Assert.Contains(KitKind.Temporary, values);
        }

        [Theory]
        [InlineData(KitKind.File, "\"file\"")]
        [InlineData(KitKind.Folder, "\"folder\"")]
        [InlineData(KitKind.Archive, "\"archive\"")]
        [InlineData(KitKind.Remote, "\"remote\"")]
        [InlineData(KitKind.Temporary, "\"temporary\"")]
        public void KitKind_Serializes_To_Lowercase(KitKind kind, string expectedJson)
        {
            var json = JsonConvert.SerializeObject(kind);
            Assert.Equal(expectedJson, json);
        }

        [Theory]
        [InlineData("\"file\"", KitKind.File)]
        [InlineData("\"folder\"", KitKind.Folder)]
        [InlineData("\"archive\"", KitKind.Archive)]
        [InlineData("\"remote\"", KitKind.Remote)]
        [InlineData("\"temporary\"", KitKind.Temporary)]
        public void KitKind_Deserializes_From_Lowercase(string json, KitKind expectedKind)
        {
            var kind = JsonConvert.DeserializeObject<KitKind>(json);
            Assert.Equal(expectedKind, kind);
        }

        [Fact]
        public void KitKind_Json_Roundtrip()
        {
            foreach (var kind in Enum.GetValues(typeof(KitKind)).Cast<KitKind>())
            {
                var json = JsonConvert.SerializeObject(kind);
                var deserialized = JsonConvert.DeserializeObject<KitKind>(json);
                Assert.Equal(kind, deserialized);
            }
        }

        [Fact]
        public void AllKitKinds_Contains_All_Values()
        {
            var expected = Enum.GetValues(typeof(KitKind)).Cast<KitKind>().ToArray();
            Assert.Equal(expected, KitKinds.All);
        }
    }

    public class KitWorkflow
    {
        [Fact]
        public void File_Kit_Import_Export_Edit_Roundtrip()
        {
            var kit = CreateWorkflowKit();
            var diff = new KitDiff { Name = "Workflow Kit Edited" };
            var path = Path.Combine(Path.GetTempPath(), $"workflow-{Guid.NewGuid():N}.kit.json");
            try
            {
                FileKit.Export(kit, path);
                var imported = FileKit.Import(path);
                Assert.Equal(kit.Name, imported.Name);

                var edited = FileKit.Edit(path, diff);
                Assert.Equal("Workflow Kit Edited", edited.Name);
                Assert.Equal("Workflow Kit Edited", FileKit.Import(path).Name);
            }
            finally
            {
                if (System.IO.File.Exists(path)) System.IO.File.Delete(path);
            }
        }

        [Fact]
        public void Folder_Kit_Import_Export_Edit_Roundtrip()
        {
            var kit = CreateWorkflowKit();
            var diff = new KitDiff { Name = "Workflow Kit Edited" };
            var folderPath = Path.Combine(Path.GetTempPath(), $"workflow-folder-{Guid.NewGuid():N}");
            Directory.CreateDirectory(folderPath);
            try
            {
                FolderKit.Export(kit, folderPath);
                var imported = FolderKit.Import(folderPath);
                Assert.Equal(kit.Name, imported.Kit.Name);
                Assert.Equal("hello workflow", Encoding.UTF8.GetString(imported.Files["docs/readme.txt"]));

                var edited = FolderKit.Edit(folderPath, diff);
                Assert.Equal("Workflow Kit Edited", edited.Name);
                Assert.Equal("Workflow Kit Edited", FolderKit.Import(folderPath).Kit.Name);
            }
            finally
            {
                SqliteRuntime.EnsureInitialized();
                Microsoft.Data.Sqlite.SqliteConnection.ClearAllPools();
                if (Directory.Exists(folderPath)) Directory.Delete(folderPath, true);
            }
        }

        [Fact]
        public void Archive_Kit_Import_Export_Edit_Roundtrip()
        {
            var kit = CreateWorkflowKit();
            var diff = new KitDiff { Name = "Workflow Kit Edited" };
            var path = Path.Combine(Path.GetTempPath(), $"workflow-{Guid.NewGuid():N}.zip");
            try
            {
                ArchiveKit.Export(kit, path);
                var imported = ArchiveKit.Import(path);
                Assert.Equal(kit.Name, imported.Kit.Name);
                Assert.Equal("hello workflow", Encoding.UTF8.GetString(imported.Files["docs/readme.txt"]));

                var edited = ArchiveKit.Edit(path, diff);
                Assert.Equal("Workflow Kit Edited", edited.Name);
                Assert.Equal("Workflow Kit Edited", ArchiveKit.Import(path).Kit.Name);
            }
            finally
            {
                if (System.IO.File.Exists(path)) System.IO.File.Delete(path);
            }
        }

        [Fact]
        public void Remote_Kit_Imports_Json_And_Zip_Then_Edits()
        {
            static (string Url, Action Dispose) StartServer(byte[] body, string contentType)
            {
                var listener = new TcpListener(IPAddress.Loopback, 0);
                listener.Start();
                var port = ((IPEndPoint)listener.LocalEndpoint).Port;
                var running = true;
                var task = Task.Run(() =>
                {
                    while (running)
                    {
                        try
                        {
                            using var client = listener.AcceptTcpClient();
                            using var stream = client.GetStream();
                            using var reader = new StreamReader(stream, Encoding.ASCII, false, 1024, true);
                            string? line;
                            while (!string.IsNullOrEmpty(line = reader.ReadLine())) { }
                            var header = $"HTTP/1.1 200 OK\r\nContent-Type: {contentType}\r\nContent-Length: {body.Length}\r\nConnection: close\r\n\r\n";
                            var headerBytes = Encoding.ASCII.GetBytes(header);
                            stream.Write(headerBytes, 0, headerBytes.Length);
                            stream.Write(body, 0, body.Length);
                        }
                        catch
                        {
                            if (running) throw;
                        }
                    }
                });
                return ($"http://127.0.0.1:{port}", () => { running = false; listener.Stop(); try { task.Wait(); } catch { } });
            }

            var kit = CreateWorkflowKit();
            var diff = new KitDiff { Name = "Workflow Kit Edited" };
            var zipPath = Path.Combine(Path.GetTempPath(), $"workflow-remote-{Guid.NewGuid():N}.zip");
            ArchiveKit.Export(kit, zipPath);
            var zipBytes = System.IO.File.ReadAllBytes(zipPath);
            var jsonBytes = Encoding.UTF8.GetBytes(Utility.Serialize(kit));

            var (jsonUrl, disposeJson) = StartServer(jsonBytes, "application/json");
            var (zipUrl, disposeZip) = StartServer(zipBytes, "application/zip");
            try
            {
                var importedJson = RemoteKit.Import(jsonUrl + "/workflow.kit.json");
                Assert.Equal(kit.Name, importedJson.Kit.Name);

                var importedZip = RemoteKit.Import(zipUrl + "/workflow.zip");
                Assert.Equal(kit.Name, importedZip.Kit.Name);
                Assert.Equal("hello workflow", Encoding.UTF8.GetString(importedZip.Files["docs/readme.txt"]));

                var edited = RemoteKit.Edit(jsonUrl + "/workflow.kit.json", diff);
                Assert.Equal("Workflow Kit Edited", edited.Name);
            }
            finally
            {
                disposeJson();
                disposeZip();
                if (System.IO.File.Exists(zipPath)) System.IO.File.Delete(zipPath);
            }
        }

        [Fact]
        public void Temporary_Kit_Edit_Applies_Diff_Without_Mutating_Source()
        {
            var kit = CreateWorkflowKit();
            var edited = TemporaryKit.Edit(kit, new KitDiff { Name = "Workflow Kit Edited" });
            Assert.Equal("Workflow Kit Edited", edited.Name);
            Assert.Equal("Workflow Kit", kit.Name);
        }
    }

    public class Roundtrip
    {
        [Fact]
        public void Metabolism_Json_Memory_Json_Json_Zip_Zip_Json()
        {

            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
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
                var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");

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
                    // On Windows, SQLite connection pooling may hold file handles open.
                    // Clear the pool before trying to delete the directory.
                    SqliteRuntime.EnsureInitialized();
                    Microsoft.Data.Sqlite.SqliteConnection.ClearAllPools();
                    Directory.Delete(tempDir, true);
                }
            }
        }
    }

    public class Flatten
    {
        private static JArray LoadFlattenCases() =>
            (JArray)JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "flatten.cases.semio.json")))["cases"]!;

        [Fact]
        public void Nakagin_Capsule_Tower_Kit_Flatten_Diff_Apply_Flat()
        {
            var c = LoadFlattenCases().First(c => (string)c["name"]! == "nakagin_capsule_tower");
            var path = ((JArray)c["designPath"]!).Select(t => (string)t!).ToList();
            TestFlatten(path.Last(), path.Count > 1 ? path[path.Count - 2] : null);
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Slanted_Kit_Flatten_Diff_Apply_Flat()
        {
            var c = LoadFlattenCases().First(c => (string)c["name"]! == "nakagin_capsule_tower_slanted");
            var path = ((JArray)c["designPath"]!).Select(t => (string)t!).ToList();
            TestFlatten(path.Last(), path.Count > 1 ? path[path.Count - 2] : null);
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Twisted_Kit_Flatten_Diff_Apply_Flat()
        {
            var c = LoadFlattenCases().First(c => (string)c["name"]! == "nakagin_capsule_tower_twisted");
            var path = ((JArray)c["designPath"]!).Select(t => (string)t!).ToList();
            TestFlatten(path.Last(), path.Count > 1 ? path[path.Count - 2] : null);
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Dancing_Kit_Flatten_Diff_Apply_Flat()
        {
            var c = LoadFlattenCases().First(c => (string)c["name"]! == "nakagin_capsule_tower_dancing");
            var path = ((JArray)c["designPath"]!).Select(t => (string)t!).ToList();
            TestFlatten(path.Last(), path.Count > 1 ? path[path.Count - 2] : null);
        }

        [Fact]
        public void Capsule_Dream_Kit_Flatten_Diff_Apply_Flat()
        {
            var c = LoadFlattenCases().First(c => (string)c["name"]! == "capsule_dream");
            var path = ((JArray)c["designPath"]!).Select(t => (string)t!).ToList();
            TestFlatten(path.Last(), path.Count > 1 ? path[path.Count - 2] : null);
        }

        private void TestFlatten(string designName, string? parentName = null)
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
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

    public class FlattenMerkle
    {
        private static JObject LoadCasesDoc() =>
            JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "flatten-merkle.cases.semio.json")));

        private static JObject LoadKitJson(string kitFile) =>
            JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, kitFile)));

        private static JObject FindDesignJsonByPath(JObject kitJson, IReadOnlyList<string> designPath)
        {
            if (designPath == null || designPath.Count == 0) throw new ArgumentException("designPath must not be empty");
            JObject? current = null;
            for (int i = 0; i < designPath.Count; i++)
            {
                var name = designPath[i];
                var parentGuid = (string?)current?["guid"];
                JObject? match = null;
                foreach (var d in (JArray?)kitJson["designs"] ?? new JArray())
                {
                    if (d is not JObject dObj) continue;
                    if ((string?)dObj["name"] != name) continue;
                    var parent = dObj["parent"] as JObject;
                    if (i == 0)
                    {
                        if (parent == null || parent.Type == JTokenType.Null)
                        {
                            match = dObj;
                            break;
                        }
                    }
                    else
                    {
                        if (parent != null && (string?)parent["guid"] == parentGuid)
                        {
                            match = dObj;
                            break;
                        }
                    }
                }
                if (match == null) throw new Exception($"Design path {string.Join(" / ", designPath)} not found at segment {name}");
                current = match;
            }
            return current!;
        }

        private static void SetDottedPath(JObject target, string path, JToken value)
        {
            var keys = path.Split('.');
            JObject current = target;
            for (int i = 0; i < keys.Length - 1; i++)
            {
                var key = keys[i];
                var next = current[key] as JObject;
                if (next == null || next.Type == JTokenType.Null)
                {
                    next = new JObject();
                    current[key] = next;
                }
                current = next;
            }
            current[keys[keys.Length - 1]] = value;
        }

        private static JToken ValueFromJson(JToken valueToken) => valueToken.DeepClone();

        private static void ApplyMutations(JObject designJson, JArray mutations)
        {
            foreach (var mutToken in mutations)
            {
                if (mutToken is not JObject mutation) continue;
                var kind = (string?)mutation["kind"];
                var path = (string?)mutation["path"] ?? "";
                var value = mutation["value"] ?? JValue.CreateNull();
                if (kind == "pieceField")
                {
                    var pieceGuid = (string?)mutation["pieceGuid"];
                    var piece = ((JArray?)designJson["pieces"] ?? new JArray())
                        .OfType<JObject>()
                        .FirstOrDefault(p => (string?)p["guid"] == pieceGuid);
                    if (piece == null) throw new Exception($"Piece {pieceGuid} not found");
                    SetDottedPath(piece, path, ValueFromJson(value));
                }
                else if (kind == "connectionField")
                {
                    var connectionGuid = (string?)mutation["connectionGuid"];
                    var connection = ((JArray?)designJson["connections"] ?? new JArray())
                        .OfType<JObject>()
                        .FirstOrDefault(c => (string?)c["guid"] == connectionGuid);
                    if (connection == null) throw new Exception($"Connection {connectionGuid} not found");
                    SetDottedPath(connection, path, ValueFromJson(value));
                }
                else
                {
                    throw new Exception($"Unknown mutation kind {kind}");
                }
            }
        }

        private static (Kit kit, string designGuid) DeserializeKitAndDesignGuid(JObject kitJson, IReadOnlyList<string> designPath)
        {
            var designJson = FindDesignJsonByPath(kitJson, designPath);
            var designGuid = (string?)designJson["guid"] ?? "";
            var kit = Utility.Deserialize<Kit>(kitJson.ToString(Formatting.None))
                      ?? throw new Exception("Failed to deserialize kit");
            return (kit, designGuid);
        }

        [Fact]
        public void SharedAssetMutationCases()
        {
            var casesDoc = LoadCasesDoc();
            foreach (var caseToken in (JArray?)casesDoc["cases"] ?? new JArray())
            {
                if (caseToken is not JObject testCase) continue;
                var name = (string?)testCase["name"] ?? "<unnamed>";
                var kitFile = (string?)testCase["kit"] ?? throw new Exception($"Case {name}: missing kit");
                var designPath = ((JArray?)testCase["designPath"] ?? new JArray()).Select(t => (string)t!).ToList();
                var mutations = (JArray?)testCase["mutations"] ?? new JArray();
                var expect = (JObject?)testCase["expect"] ?? new JObject();

                var kitJsonBefore = LoadKitJson(kitFile);
                var (kitBefore, designGuidBefore) = DeserializeKitAndDesignGuid(kitJsonBefore, designPath);
                var beforeHashes = Kit.ComputeFlatHashes(kitBefore, designGuidBefore);

                var kitJsonAfter = LoadKitJson(kitFile);
                var designJsonAfter = FindDesignJsonByPath(kitJsonAfter, designPath);
                ApplyMutations(designJsonAfter, mutations);
                var (kitAfter, designGuidAfter) = DeserializeKitAndDesignGuid(kitJsonAfter, designPath);
                var afterHashes = Kit.ComputeFlatHashes(kitAfter, designGuidAfter);

                Assert.True(beforeHashes.Keys.OrderBy(g => g, StringComparer.Ordinal).SequenceEqual(afterHashes.Keys.OrderBy(g => g, StringComparer.Ordinal)),
                    $"Case {name}: piece set changed");

                var changedPlane = beforeHashes.Keys.Where(g => beforeHashes[g].PlaneHash != afterHashes[g].PlaneHash).ToHashSet();
                var changedCenter = beforeHashes.Keys.Where(g => beforeHashes[g].CenterHash != afterHashes[g].CenterHash).ToHashSet();

                bool HasBool(string key) => expect[key] != null && expect[key]!.Type == JTokenType.Boolean;
                bool GetBool(string key) => (bool)expect[key]!;

                if (HasBool("planeHashesChangedAny"))
                {
                    if (GetBool("planeHashesChangedAny")) Assert.True(changedPlane.Count > 0, $"Case {name}: expected some planeHash changes, got none");
                    else Assert.True(changedPlane.Count == 0, $"Case {name}: expected no planeHash changes, got {string.Join(",", changedPlane)}");
                }
                if (HasBool("centerHashesChangedAny"))
                {
                    if (GetBool("centerHashesChangedAny")) Assert.True(changedCenter.Count > 0, $"Case {name}: expected some centerHash changes, got none");
                    else Assert.True(changedCenter.Count == 0, $"Case {name}: expected no centerHash changes, got {string.Join(",", changedCenter)}");
                }
                if (HasBool("planeHashesChangedAll"))
                {
                    if (GetBool("planeHashesChangedAll")) Assert.Equal(beforeHashes.Count, changedPlane.Count);
                    else Assert.NotEqual(beforeHashes.Count, changedPlane.Count);
                }
                if (HasBool("centerHashesChangedAll"))
                {
                    if (GetBool("centerHashesChangedAll")) Assert.Equal(beforeHashes.Count, changedCenter.Count);
                    else Assert.NotEqual(beforeHashes.Count, changedCenter.Count);
                }
                foreach (var t in (JArray?)expect["planeHashesChangedIncludes"] ?? new JArray())
                    Assert.True(changedPlane.Contains((string)t!), $"Case {name}: expected piece {t} to have changed planeHash");
                foreach (var t in (JArray?)expect["centerHashesChangedIncludes"] ?? new JArray())
                    Assert.True(changedCenter.Contains((string)t!), $"Case {name}: expected piece {t} to have changed centerHash");
                foreach (var t in (JArray?)expect["planeHashesStableIncludes"] ?? new JArray())
                    Assert.False(changedPlane.Contains((string)t!), $"Case {name}: expected piece {t} to keep stable planeHash");
                foreach (var t in (JArray?)expect["centerHashesStableIncludes"] ?? new JArray())
                    Assert.False(changedCenter.Contains((string)t!), $"Case {name}: expected piece {t} to keep stable centerHash");
            }
        }

        [Fact]
        public void CrossLanguageParityReferenceHashes()
        {
            var casesDoc = LoadCasesDoc();
            var parity = (JObject?)casesDoc["parity"] ?? throw new Exception("parity block missing");
            var kitFile = (string?)parity["kit"] ?? throw new Exception("parity.kit missing");
            var designPath = ((JArray?)parity["designPath"] ?? new JArray()).Select(t => (string)t!).ToList();
            var kitJson = LoadKitJson(kitFile);
            var (kit, designGuid) = DeserializeKitAndDesignGuid(kitJson, designPath);
            var hashes = Kit.ComputeFlatHashes(kit, designGuid);
            foreach (var expectedToken in (JArray?)parity["expectedHashes"] ?? new JArray())
            {
                if (expectedToken is not JObject expected) continue;
                var guid = (string?)expected["pieceGuid"] ?? throw new Exception("missing pieceGuid");
                Assert.True(hashes.ContainsKey(guid), $"piece {guid} missing from computed hashes");
                Assert.Equal((string?)expected["planeHash"], hashes[guid].PlaneHash);
                Assert.Equal((string?)expected["centerHash"], hashes[guid].CenterHash);
            }
        }

        [Fact]
        public void CachedFlattenReusesValues()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var (_, firstCache) = Kit.FlattenDesignCached(kit, design.Guid);
            Assert.True(firstCache.Count > 0);
            var (_, secondCache) = Kit.FlattenDesignCached(kit, design.Guid, firstCache);
            foreach (var kvp in firstCache)
            {
                Assert.True(secondCache.ContainsKey(kvp.Key), $"piece {kvp.Key} missing from second cache");
                Assert.Equal(kvp.Value.PlaneHash, secondCache[kvp.Key].PlaneHash);
                Assert.Equal(kvp.Value.CenterHash, secondCache[kvp.Key].CenterHash);
                Assert.Equal(Utility.Serialize(kvp.Value.Plane), Utility.Serialize(secondCache[kvp.Key].Plane));
                Assert.Equal(Utility.Serialize(kvp.Value.Center), Utility.Serialize(secondCache[kvp.Key].Center));
            }
        }
    }

    public class Change
    {
        [Fact]
        public void Metabolism_Kit_Change_Forward_Backward_Inverse_Behavior()
        {
            var kitOriginal = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            kitOriginal.Designs = kitOriginal.Designs?.Where(d => d.Parent == null).ToList();

            var kitDiff = Tests.LoadAsset<KitDiff>("metabolism.kit.diff.semio.json");
            var kitDiffInverted = Tests.LoadAsset<KitDiff>("metabolism.kit.diff.inverted.semio.json");
            var kitDiffed = Tests.LoadAsset<Kit>("metabolism.kit.diffed.semio.json");

            var change = SemioDiff.GetKitChange(kitOriginal, kitDiffed);
            var forwardJson = Utility.Serialize(change.Forward);
            var expectedForwardJson = Utility.Serialize(kitDiff);
            Console.WriteLine($"[DEBUG] Forward: {forwardJson}");
            Console.WriteLine($"[DEBUG] Expected: {expectedForwardJson}");
            Assert.True(SemioDiff.AreKitDiffsEqual(change.Forward, kitDiff), "GetKitChange: forward diff doesn't match expected diff");

            var backwardJson = Utility.Serialize(change.Backward);
            var expectedBackwardJson = Utility.Serialize(kitDiffInverted);
            Console.WriteLine($"[DEBUG] Backward: {backwardJson}");
            Console.WriteLine($"[DEBUG] Expected Inverted: {expectedBackwardJson}");
            Assert.True(SemioDiff.AreKitDiffsEqual(change.Backward, kitDiffInverted), "GetKitChange: backward diff doesn't match expected inverse diff");

            var appliedForward = SemioDiff.ApplyKitDiff(kitOriginal, change.Forward);
            Assert.True(SemioDiff.AreKitsEqual(appliedForward, kitDiffed), "ApplyKitDiff forward: applied kit doesn't match expected diffed kit");

            var appliedInverse = SemioDiff.ApplyKitDiff(kitDiffed, change.Backward);
            Assert.True(SemioDiff.AreKitsEqual(appliedInverse, kitOriginal), "ApplyKitDiff inverse: applied inverse kit doesn't match original kit");
        }
    }

    public class ValidateKitDiffFacts
    {
        private sealed class ValidateKitDiffAsset
        {
            [JsonProperty("tinyKit")]
            public Kit TinyKit { get; set; } = null!;
            public List<ValidateKitDiffCase> Cases { get; set; } = new();
        }

        private sealed class ValidateKitDiffCase
        {
            public string Id { get; set; } = "";
            public KitDiff Diff { get; set; } = null!;
            [JsonProperty("expectOk")]
            public bool ExpectOk { get; set; }
            [JsonProperty("errorCodes")]
            public List<string> ErrorCodes { get; set; } = new();
            [JsonProperty("warningCodes")]
            public List<string> WarningCodes { get; set; } = new();
        }

        private static List<string> Codes(IReadOnlyList<KitDiffValidationNote> notes) =>
            notes.Where(n => !string.IsNullOrEmpty(n.Code)).Select(n => n.Code!).ToList();

        [Fact]
        public void Shared_Asset_Cases()
        {
            var asset = Tests.LoadAsset<ValidateKitDiffAsset>("validate-kit-diff.cases.semio.json");
            foreach (var c in asset.Cases)
            {
                var r = SemioDiff.ValidateKitDiff(asset.TinyKit, c.Diff, false);
                Assert.Equal(c.ExpectOk, r.Ok);
                var errCodes = Codes(r.Errors);
                foreach (var code in c.ErrorCodes)
                    Assert.Contains(code, errCodes);
                var warnCodes = Codes(r.Warnings);
                foreach (var code in c.WarningCodes)
                    Assert.Contains(code, warnCodes);
            }
        }

        [Fact]
        public void Heal_Drops_Invalid_Design_Update()
        {
            var asset = Tests.LoadAsset<ValidateKitDiffAsset>("validate-kit-diff.cases.semio.json");
            var badJson = """{"designs":{"updated":[{"design":{"guid":"99999999-9999-9999-9999-999999999999"},"diff":{"name":"X"}}]}}""";
            var bad = Utility.Deserialize<KitDiff>(badJson);
            Assert.NotNull(bad);
            var r = SemioDiff.ValidateKitDiff(asset.TinyKit, bad!, true);
            Assert.NotNull(r.Diff);
            var d = r.Diff!;
            Assert.True(d.Designs == null || d.Designs.Updated == null || d.Designs.Updated.Count == 0,
                "heal should drop invalid design update");
        }
    }

    public class Validation
    {
        [Fact]
        public void Metabolism_Kit_Validate_Empty_Report()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var result = SemioValidator.ValidateKit(kit);
            Assert.Empty(result.Issues);
        }

        [Fact]
        public void Invalid_Kit_Validate_Invalid_Report()
        {
            var kit = Tests.LoadAsset<Kit>("invalid.kit.semio.json");
            var result = SemioValidator.ValidateKit(kit);
            var filePath = Path.Combine(Tests.AssetsPath, "validation.semio.json");
            var expectedJson = System.IO.File.ReadAllText(filePath);
            var expected = ValidationResult.Parse(expectedJson);

            Assert.True(ValidationResult.AreEqual(expected, result), $"Expected {expected.Issues.Count} issues, got {result.Issues.Count}. Expected:\n{expected.Serialize()}\nActual:\n{result.Serialize()}");
        }

        [Fact]
        public void Plain_Descriptions_Do_Not_Create_Emoji_Validation_Issues()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            kit.Description = "Plain kit summary";
            for (var i = 0; i < kit.Types.Count; i++)
                kit.Types[i].Description = $"Repeated plain description {i % 2}";

            var result = SemioValidator.ValidateKit(kit);

            Assert.DoesNotContain(result.Issues, issue => issue.ConstraintId == "description-missing-emoji" || issue.ConstraintId == "description-emoji-unique");
        }
    }

    public class Drag
    {
        [Fact]
        public void Design_Pieces_Offset_DiffDesign()
        {
            var design = Tests.LoadAsset<Design>("drag/design.semio.json");
            var pieces = Tests.LoadAsset<Design>("drag/pieces.semio.json");
            var offset = Tests.LoadAsset<Coord>("drag/offset.semio.json");
            var expectedDiff = Tests.LoadAsset<DesignDiff>("drag/diff.design.semio.json");
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

    public class Move
    {
        [Fact]
        public void Design_Pieces_MoveVector_DiffDesign()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = Tests.LoadAsset<Design>("drag/design.semio.json");
            var pieces = Tests.LoadAsset<Design>("drag/pieces.semio.json");
            var vector = Tests.LoadAsset<MoveVector>("move/vector.semio.json");
            var expectedDiff = Tests.LoadAsset<DesignDiff>("move/diff.design.semio.json");
            var computedDiff = Design.MovePiecesInDesign(kit, design, pieces, vector);
            Assert.NotNull(computedDiff.Pieces);
            Assert.Equal(expectedDiff.Pieces!.Updated.Count, computedDiff.Pieces!.Updated.Count);
            var expectedPieceMap = expectedDiff.Pieces.Updated.ToDictionary(u => u.Piece.Guid, u => u.Diff);
            foreach (var u in computedDiff.Pieces.Updated)
            {
                Assert.True(expectedPieceMap.ContainsKey(u.Piece.Guid), $"Unexpected piece update for {u.Piece.Guid}");
                var expected = expectedPieceMap[u.Piece.Guid];
                Assert.NotNull(u.Diff!.Plane);
                Assert.NotNull(expected!.Plane);
                Assert.Equal(expected.Plane!.Origin.X, u.Diff.Plane!.Origin.X, 3);
                Assert.Equal(expected.Plane.Origin.Y, u.Diff.Plane!.Origin.Y, 3);
                Assert.Equal(expected.Plane.Origin.Z, u.Diff.Plane!.Origin.Z, 3);
            }
            Assert.NotNull(computedDiff.Connections);
            Assert.Equal(expectedDiff.Connections!.Updated.Count, computedDiff.Connections!.Updated.Count);
            var expectedConnMap = expectedDiff.Connections.Updated.ToDictionary(u => u.Connection.Guid, u => u.Diff);
            foreach (var u in computedDiff.Connections.Updated)
            {
                Assert.True(expectedConnMap.ContainsKey(u.Connection.Guid), $"Unexpected connection update for {u.Connection.Guid}");
                var expected = expectedConnMap[u.Connection.Guid];
                Assert.Equal(expected!.Gap ?? 0, u.Diff!.Gap ?? 0, 3);
                Assert.Equal(expected.Shift ?? 0, u.Diff.Shift ?? 0, 3);
                Assert.Equal(expected.Rise ?? 0, u.Diff.Rise ?? 0, 3);
            }
        }
    }

    public class Delete
    {
        [Fact]
        public void Nakagin_Capsule_Tower_Delete_Third_Tambour_And_First_Small_Tower_Connection()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var selection = Tests.LoadAsset<Design>("nakagin-capsule-tower.deleted.selection.semio.json");
            var expectedDiff = Tests.LoadAsset<DesignDiff>("nakagin-capsule-tower.deleted.design.diff.semio.json");

            var pieceGuids = selection.Pieces.Select(p => p.Guid).ToList();
            var connectionGuids = selection.Connections.Select(c => c.Guid).ToList();
            var computedDiff = Design.DeletePiecesAndConnectionsInDesign(kit, design, pieceGuids, connectionGuids);

            // Verify removed pieces
            Assert.NotNull(computedDiff.Pieces);
            Assert.Equal(expectedDiff.Pieces!.Removed.Count, computedDiff.Pieces!.Removed.Count);
            var expectedRemovedPieces = new HashSet<string>(expectedDiff.Pieces.Removed.Select(r => r.Guid));
            foreach (var r in computedDiff.Pieces.Removed)
                Assert.True(expectedRemovedPieces.Contains(r.Guid), $"Unexpected removed piece {r.Guid}");

            // Verify updated (fixed) pieces
            Assert.Equal(expectedDiff.Pieces.Updated.Count, computedDiff.Pieces.Updated.Count);
            var expectedUpdatedMap = expectedDiff.Pieces.Updated.ToDictionary(u => u.Piece.Guid, u => u.Diff);
            foreach (var u in computedDiff.Pieces.Updated)
            {
                Assert.True(expectedUpdatedMap.ContainsKey(u.Piece.Guid), $"Unexpected piece update for {u.Piece.Guid}");
                var expected = expectedUpdatedMap[u.Piece.Guid];
                Assert.NotNull(u.Diff!.Plane);
                Assert.NotNull(expected!.Plane);
                Assert.Equal(expected.Plane!.Origin.X, u.Diff.Plane!.Origin.X, 3);
                Assert.Equal(expected.Plane.Origin.Y, u.Diff.Plane.Origin.Y, 3);
                Assert.Equal(expected.Plane.Origin.Z, u.Diff.Plane.Origin.Z, 3);
                Assert.NotNull(u.Diff.Center);
                Assert.NotNull(expected.Center);
                Assert.Equal(expected.Center!.U, u.Diff.Center!.U, 3);
                Assert.Equal(expected.Center.V, u.Diff.Center.V, 3);
            }

            // Verify removed connections
            Assert.NotNull(computedDiff.Connections);
            Assert.Equal(expectedDiff.Connections!.Removed.Count, computedDiff.Connections!.Removed.Count);
            var expectedRemovedConns = new HashSet<string>(expectedDiff.Connections.Removed.Select(r => r.Guid));
            foreach (var r in computedDiff.Connections.Removed)
                Assert.True(expectedRemovedConns.Contains(r.Guid), $"Unexpected removed connection {r.Guid}");
        }
    }

    public class CopyAndPaste
    {
        private sealed class Selection
        {
            public List<PieceId> Pieces { get; set; } = new();
            public List<ConnectionId> Connections { get; set; } = new();
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Copy_Paste_Roundtrip()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var pasteTargetDesign = Tests.LoadAsset<Design>("nakagin-capsule-tower.paste.design.semio.json");
            var selection = Tests.LoadAsset<Selection>("nakagin-capsule-tower.copy.design.selection.semio.json");

            var pieceGuids = selection.Pieces.Select(p => p.Guid).ToList();
            var connectionGuids = selection.Connections.Select(c => c.Guid).ToList();

            // Compute CopyDesign
            var copyDesign = Design.CopyDesign(kit, design, pieceGuids, connectionGuids);

            // Compute PasteDesign without coord (paste into the second storey target design)
            var pasteDiff = Design.PasteDesign(kit, copyDesign, pasteTargetDesign, "original");

            // Compute PasteDesign with coord
            var pasteWithCoordDiff = Design.PasteDesign(kit, copyDesign, pasteTargetDesign, "original", new Coord { U = 10, V = 10 });

            // Save the generated files for cross-language comparison
            var copyJson = Utility.Serialize(copyDesign, "  ");
            var pasteJson = Utility.Serialize(pasteDiff, "  ");
            var pasteWithCoordJson = Utility.Serialize(pasteWithCoordDiff, "  ");
            System.IO.File.WriteAllText(Path.Combine(Tests.AssetsPath, "nakagin-capsule-tower.copy.design.semio.json"), copyJson);
            System.IO.File.WriteAllText(Path.Combine(Tests.AssetsPath, "nakagin-capsule-tower.paste.design.diff.semio.json"), pasteJson);
            System.IO.File.WriteAllText(Path.Combine(Tests.AssetsPath, "nakagin-capsule-tower.paste.with-coord.design.diff.semio.json"), pasteWithCoordJson);

            // Verify copy piece and connection counts
            Assert.Equal(11, copyDesign.Pieces.Count);
            Assert.Equal(9, copyDesign.Connections.Count);

            // Verify each piece guid exists
            var copyPieceGuids = new HashSet<string>(copyDesign.Pieces.Select(p => p.Guid));
            foreach (var g in pieceGuids)
                Assert.True(copyPieceGuids.Contains(g), $"Selected piece {g} not found in copy output");

            // Verify external pieces have semio.piece.origin and semio.center attributes
            var externalPieces = copyDesign.Pieces.Where(p => p.Attributes.Any(a => a.Key == "semio.piece.origin" && a.Value == "external")).ToList();
            Assert.Single(externalPieces);
            foreach (var ext in externalPieces)
            {
                Assert.True(ext.Attributes.Any(a => a.Key == "semio.center"), $"External piece {ext.Guid} missing semio.center");
            }

            // Verify pp_excl_pc_incl pieces have semio.center and semio.plane attributes
            var ppExclPcInclPieces = copyDesign.Pieces.Where(p =>
                p.Attributes.Any(a => a.Key == "semio.center") && !p.Attributes.Any(a => a.Key == "semio.piece.origin")).ToList();
            Assert.Single(ppExclPcInclPieces);
            foreach (var pp in ppExclPcInclPieces)
            {
                Assert.True(pp.Attributes.Any(a => a.Key == "semio.plane"), $"Pp-excl-pc-incl piece {pp.Guid} missing semio.plane");
            }

            // Verify paste without coord
            Assert.NotNull(pasteDiff.Pieces);
            Assert.NotNull(pasteDiff.Pieces!.Added);
            foreach (var addedPiece in pasteDiff.Pieces.Added)
            {
                Assert.False(addedPiece.Attributes.Any(a => a.Key == "semio.piece.origin" && a.Value == "external"),
                    $"External-origin piece {addedPiece.Guid} should not be in paste output");
            }
            Assert.NotNull(pasteDiff.Connections);
            Assert.NotNull(pasteDiff.Connections!.Added);

            // Verify paste with coord
            Assert.NotNull(pasteWithCoordDiff.Pieces);
            Assert.NotNull(pasteWithCoordDiff.Pieces!.Added);
            foreach (var addedPiece in pasteWithCoordDiff.Pieces.Added)
            {
                Assert.False(addedPiece.Attributes.Any(a => a.Key == "semio.piece.origin" && a.Value == "external"),
                    $"External-origin piece {addedPiece.Guid} should not be in paste with-coord output");
            }
            Assert.NotNull(pasteWithCoordDiff.Connections);
            Assert.NotNull(pasteWithCoordDiff.Connections!.Added);
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
            var payload = Tests.LoadAsset<ModelSelectionAsset>("model.selection.semio.json");
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

    public class KitFilterDesign
    {
        private static JObject LoadFilterKitAsset() =>
            JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "filter-kit.cases.semio.json")));

        [Fact]
        public void Nakagin_Capsule_Tower_Filter_Produces_Expected_Subset()
        {
            var asset = LoadFilterKitAsset();
            var fc = ((JArray)asset["cases"]!).First(c => (string)c["name"]! == "nakagin_capsule_tower");
            var kitFile = (string)fc["kit"]!;
            var designName = (string)fc["designName"]!;
            var expectedKitFile = (string)fc["expectedKit"]!;
            var kit = Tests.LoadAsset<Kit>(kitFile);
            var expected = Tests.LoadAsset<Kit>(expectedKitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);

            var filtered = Kit.FilterKit(kit, new Kit.KitFilter { DesignGuid = design.Guid });

            Assert.Equal(expected.Designs?.Count ?? 0, filtered.Designs?.Count ?? 0);
            Assert.Equal(expected.Types?.Count ?? 0, filtered.Types?.Count ?? 0);
            Assert.Equal(expected.Files?.Count ?? 0, filtered.Files?.Count ?? 0);
            Assert.Equal(expected.Ports?.Count ?? 0, filtered.Ports?.Count ?? 0);
            Assert.Equal(expected.Qualities?.Count ?? 0, filtered.Qualities?.Count ?? 0);
            Assert.Equal(expected.Authors?.Count ?? 0, filtered.Authors?.Count ?? 0);

            var filteredDesign = filtered.Designs!.FirstOrDefault(d => d.Guid == design.Guid);
            Assert.NotNull(filteredDesign);
            Assert.Equal(design.Pieces?.Count ?? 0, filteredDesign!.Pieces?.Count ?? 0);

            foreach (var expectedType in expected.Types ?? new List<Type>())
            {
                var filteredType = filtered.Types!.FirstOrDefault(t => t.Guid == expectedType.Guid);
                Assert.NotNull(filteredType);
                Assert.Equal(expectedType.Models?.Count ?? 0, filteredType!.Models?.Count ?? 0);
            }

            foreach (var piece in filteredDesign.Pieces ?? new List<Piece>())
            {
                if (piece.Type?.Guid == null) continue;
                Assert.Contains(filtered.Types!, t => t.Guid == piece.Type.Guid);
            }

            foreach (var kind in filtered.Types ?? new List<Type>())
            {
                Assert.True((kind.Models?.Count ?? 0) <= 1, $"Type {kind.Guid} has more than one model");
                foreach (var model in kind.Models ?? new List<Model>())
                    Assert.Contains(filtered.Files ?? new List<File>(), file => file.Guid == model.File.Guid);
                foreach (var connector in kind.Connectors ?? new List<Connector>())
                {
                    if (connector.Port?.Guid == null) continue;
                    Assert.Contains(filtered.Ports ?? new List<Port>(), port => port.Guid == connector.Port.Guid);
                }
            }
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Filter_Preserves_Metadata()
        {
            var asset = LoadFilterKitAsset();
            var fc = ((JArray)asset["cases"]!).First(c => (string)c["name"]! == "nakagin_capsule_tower");
            var kit = Tests.LoadAsset<Kit>((string)fc["kit"]!);
            var design = kit.Designs!.First(d => d.Name == (string)fc["designName"]! && d.Parent == null);

            var filtered = Kit.FilterKit(kit, new Kit.KitFilter { DesignGuid = design.Guid });

            Assert.Equal(kit.Guid, filtered.Guid);
            Assert.Equal(kit.Name, filtered.Name);
            Assert.Equal(kit.Version, filtered.Version);
        }

        [Fact]
        public void Glob_Filters_Types_By_Name_Include()
        {
            var asset = LoadFilterKitAsset();
            var gc = ((JArray)asset["globCases"]!).First(c => (string)c["name"]! == "type_include_capsule");
            var kit = Tests.LoadAsset<Kit>((string)gc["kit"]!);
            var patterns = ((JArray)gc["typeInclude"]!).Select(t => (string)t!).ToList();
            var filtered = Kit.FilterKit(kit, new Kit.KitFilter { Types = new Kit.GlobFilter { Include = patterns } });
            Assert.NotEmpty(filtered.Types!);
            foreach (var t in filtered.Types!)
                Assert.True(patterns.Any(p => Kit.GlobMatch(t.Name, p)), $"Type {t.Name} should match one of {string.Join(", ", patterns)}");
        }

        [Fact]
        public void Glob_Filters_Types_By_Name_Exclude()
        {
            var asset = LoadFilterKitAsset();
            var gc = ((JArray)asset["globCases"]!).First(c => (string)c["name"]! == "type_exclude_capsule");
            var kit = Tests.LoadAsset<Kit>((string)gc["kit"]!);
            var patterns = ((JArray)gc["typeExclude"]!).Select(t => (string)t!).ToList();
            var totalTypes = kit.Types!.Count;
            var filtered = Kit.FilterKit(kit, new Kit.KitFilter { Types = new Kit.GlobFilter { Exclude = patterns } });
            Assert.True(filtered.Types!.Count < totalTypes);
            foreach (var t in filtered.Types!)
                Assert.False(patterns.Any(p => Kit.GlobMatch(t.Name, p)), $"Type {t.Name} should have been excluded");
        }

        [Fact]
        public void Glob_Filters_Designs_By_Name_Include()
        {
            var asset = LoadFilterKitAsset();
            var gc = ((JArray)asset["globCases"]!).First(c => (string)c["name"]! == "design_include_nakagin");
            var kit = Tests.LoadAsset<Kit>((string)gc["kit"]!);
            var patterns = ((JArray)gc["designInclude"]!).Select(t => (string)t!).ToList();
            var filtered = Kit.FilterKit(kit, new Kit.KitFilter { Designs = new Kit.GlobFilter { Include = patterns } });
            Assert.NotEmpty(filtered.Designs!);
            foreach (var d in filtered.Designs!)
                Assert.True(patterns.Any(p => Kit.GlobMatch(d.Name, p)), $"Design {d.Name} should match one of {string.Join(", ", patterns)}");
        }

        [Fact]
        public void Empty_Filter_Returns_Kit_Unchanged()
        {
            var asset = LoadFilterKitAsset();
            var gc = ((JArray)asset["globCases"]!).First(c => (string)c["name"]! == "empty_filter");
            var kit = Tests.LoadAsset<Kit>((string)gc["kit"]!);
            var filtered = Kit.FilterKit(kit, new Kit.KitFilter());
            Assert.Equal(kit.Types!.Count, filtered.Types!.Count);
            Assert.Equal(kit.Designs!.Count, filtered.Designs!.Count);
        }

        [Fact]
        public void Combines_DesignGuid_With_Glob_Filters()
        {
            var asset = LoadFilterKitAsset();
            var gc = ((JArray)asset["globCases"]!).First(c => (string)c["name"]! == "combined_design_and_type_exclude");
            var kit = Tests.LoadAsset<Kit>((string)gc["kit"]!);
            var designName = (string)gc["designName"]!;
            var typeExclude = ((JArray)gc["typeExclude"]!).Select(t => (string)t!).ToList();
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);
            var designFiltered = Kit.FilterKit(kit, new Kit.KitFilter { DesignGuid = design.Guid });
            var combinedFiltered = Kit.FilterKit(kit, new Kit.KitFilter { DesignGuid = design.Guid, Types = new Kit.GlobFilter { Exclude = typeExclude } });
            Assert.True(combinedFiltered.Types!.Count < designFiltered.Types!.Count);
            foreach (var t in combinedFiltered.Types!)
                Assert.False(typeExclude.Any(p => Kit.GlobMatch(t.Name, p)), $"Type {t.Name} should have been excluded");
        }
    }

    public class DesignQualitySum
    {
        [Fact]
        public void Nakagin_Capsule_Tower_Sum_Effective_Floor_Area()
        {
            var asset = JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "quality-sum.cases.semio.json")));
            var c = ((JArray)asset["cases"]!).First(c => (string)c["name"]! == "sum_effective_floor_area");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var qualityName = (string)c["qualityName"]!;
            var expectedValue = (double)c["expected"]!;
            var tolerance = (double)c["tolerance"]!;
            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs.First(d => d.Name == designName && d.Parent == null);
            var quality = kit.Qualities.First(q => q.Name == qualityName);
            var result = Kit.SumQualityInDesign(kit, design.Guid, quality.Guid);
            Assert.True(Math.Abs(result - expectedValue) < tolerance, $"Expected ~{expectedValue}, got {result}");
        }
    }

    public class GetGeometricInsightsForModel
    {
        static double Round6(double x) => Math.Round(x, 6);
        static object Pt(Point p) => p == null ? null : new { x = Round6(p.X), y = Round6(p.Y), z = Round6(p.Z) };

        [Fact]
        public void Nakagin_Capsule_Tower_Gltf_Returns_Insights()
        {
            var path = Path.Combine(AssetsPath, "nakagin-capsule-tower.gltf");
            if (!System.IO.File.Exists(path))
                return;
            var insights = Kit.GetGeometricInsightsForModel(path);

            var reportsDir = Path.Combine("..", "..", "reports", "model-kpi");
            Directory.CreateDirectory(reportsDir);
            var report = new JObject
            {
                ["aspect_ratio_xy"] = Round6(insights.AspectRatioXy),
                ["aspect_ratio_xz"] = Round6(insights.AspectRatioXz),
                ["aspect_ratio_yz"] = Round6(insights.AspectRatioYz),
                ["bounding_box_max"] = JObject.FromObject(Pt(insights.BoundingBoxMax)),
                ["bounding_box_min"] = JObject.FromObject(Pt(insights.BoundingBoxMin)),
                ["centroid"] = JObject.FromObject(Pt(insights.Centroid)),
                ["characteristic_length"] = Round6(insights.CharacteristicLength),
                ["dimension_x"] = Round6(insights.DimensionX),
                ["dimension_y"] = Round6(insights.DimensionY),
                ["dimension_z"] = Round6(insights.DimensionZ),
                ["face_count"] = insights.FaceCount,
                ["footprint_area"] = Round6(insights.FootprintArea),
                ["is_watertight"] = insights.IsWatertight,
                ["slenderness"] = Round6(insights.Slenderness),
                ["total_surface_area"] = Round6(insights.TotalSurfaceArea),
                ["vertex_count"] = insights.VertexCount,
            };
            System.IO.File.WriteAllText(Path.Combine(reportsDir, "net.json"), report.ToString());

            var canonicalPath = Path.Combine(AssetsPath, "nakagin.kpi.model.semio.json");
            var canonical = JObject.Parse(System.IO.File.ReadAllText(canonicalPath));
            var skip = new HashSet<string> { "centroid", "total_surface_area" };
            foreach (var kv in canonical)
            {
                if (skip.Contains(kv.Key)) continue;
                Assert.True(JToken.DeepEquals(kv.Value, report[kv.Key]), $"Mismatch for {kv.Key}");
            }
        }
    }

    public class ExportDesignModel
    {
        [Fact]
        public void Nakagin_Capsule_Tower_Export_Glb_Valid_Header()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var result = Kit.ExportDesignModel(kit, design.Guid, ".glb");
            Assert.NotNull(result);
            Assert.True(result.Length > 0, "Result must not be empty");
            Assert.True(result.Length >= 12, "GLB header requires at least 12 bytes");
            Assert.Equal((byte)'g', result[0]);
            Assert.Equal((byte)'l', result[1]);
            Assert.Equal((byte)'T', result[2]);
            Assert.Equal((byte)'F', result[3]);
            var version = BitConverter.ToUInt32(result, 4);
            Assert.Equal(2u, version);
            var totalLength = BitConverter.ToUInt32(result, 8);
            Assert.Equal((uint)result.Length, totalLength);
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Export_Gltf_Valid_Json()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var result = Kit.ExportDesignModel(kit, design.Guid, ".gltf");
            Assert.NotNull(result);
            Assert.True(result.Length > 0, "Result must not be empty");
            var json = System.Text.Encoding.UTF8.GetString(result);
            var parsed = JsonConvert.DeserializeObject(json);
            Assert.NotNull(parsed);
        }

        [Fact]
        public void Invalid_Format_Throws()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            Assert.Throws<ArgumentException>(() => Kit.ExportDesignModel(kit, design.Guid, ".invalid"));
        }

        [Fact]
        public void Nakagin_Capsule_Tower_Export_Scene_Graph_Report()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Name == "Nakagin Capsule Tower" && d.Parent == null);
            var result = Kit.ExportDesignModel(kit, design.Guid, ".gltf");
            Assert.NotNull(result);
            Assert.True(result.Length > 0, "Result must not be empty");
            var json = System.Text.Encoding.UTF8.GetString(result);
            var parsed = JsonConvert.DeserializeObject(json);
            Assert.NotNull(parsed);
            var reportsDir = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "../../../../../../reports/export-design-model"));
            Directory.CreateDirectory(reportsDir);
            System.IO.File.WriteAllBytes(Path.Combine(reportsDir, "net.gltf"), result);
        }
    }

    public class MetaShallow
    {
        [Fact]
        public void Type_Meta_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var type = kit.Types.First();
            var meta = type.ToMeta();

            Assert.Equal(type.Guid, meta.Guid);
            Assert.Equal(type.Name, meta.Name);
            Assert.Equal(type.Parent?.Guid, meta.Parent?.Guid);
            Assert.Equal(type.IsAbstract, meta.IsAbstract);
            Assert.Equal(type.Folder, meta.Folder);
            Assert.Equal(type.Description, meta.Description);
            Assert.Equal(type.Icon, meta.Icon);
            Assert.Equal(type.Image, meta.Image);
            Assert.Equal(type.Stock, meta.Stock);
            Assert.Equal(type.Virtual, meta.Virtual);
            Assert.Equal(type.Uri, meta.Uri);
            Assert.Equal(type.Unit, meta.Unit);
            Assert.Equal(type.CreatedAt, meta.CreatedAt);
            Assert.Equal(type.UpdatedAt, meta.UpdatedAt);
        }

        [Fact]
        public void Type_Shallow_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var type = kit.Types.First();
            var shallow = type.ToShallow();

            Assert.Equal(type.Guid, shallow.Guid);
            Assert.Equal(type.Name, shallow.Name);
            Assert.Equal(type.Models.Count, shallow.Models.Count);
            Assert.Equal(type.Connectors.Count, shallow.Connectors.Count);
            Assert.Equal(type.Props.Count, shallow.Props.Count);
            Assert.Equal(type.Authors.Count, shallow.Authors.Count);
            Assert.Equal(type.Concepts.Count, shallow.Concepts.Count);
            Assert.Equal(type.Attributes.Count, shallow.Attributes.Count);

            for (int i = 0; i < type.Models.Count; i++)
            {
                Assert.Equal(type.Models[i].Guid, shallow.Models[i].Guid);
                Assert.Equal(type.Models[i].Name, shallow.Models[i].Name);
            }
            for (int i = 0; i < type.Connectors.Count; i++)
            {
                Assert.Equal(type.Connectors[i].Guid, shallow.Connectors[i].Guid);
                Assert.Equal(type.Connectors[i].Name, shallow.Connectors[i].Name);
            }
        }

        [Fact]
        public void Design_Meta_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Parent == null);
            var meta = design.ToMeta();

            Assert.Equal(design.Guid, meta.Guid);
            Assert.Equal(design.Name, meta.Name);
            Assert.Equal(design.Parent?.Guid, meta.Parent?.Guid);
            Assert.Equal(design.IsAbstract, meta.IsAbstract);
            Assert.Equal(design.Folder, meta.Folder);
            Assert.Equal(design.Description, meta.Description);
            Assert.Equal(design.Icon, meta.Icon);
            Assert.Equal(design.Image, meta.Image);
            Assert.Equal(design.Unit, meta.Unit);
            Assert.Equal(design.CanScale, meta.CanScale);
            Assert.Equal(design.CanMirror, meta.CanMirror);
            Assert.Equal(design.CreatedAt, meta.CreatedAt);
            Assert.Equal(design.UpdatedAt, meta.UpdatedAt);
        }

        [Fact]
        public void Design_Shallow_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var design = kit.Designs.First(d => d.Parent == null);
            var shallow = design.ToShallow();

            Assert.Equal(design.Guid, shallow.Guid);
            Assert.Equal(design.Name, shallow.Name);
            Assert.Equal(design.Pieces.Count, shallow.Pieces.Count);
            Assert.Equal(design.Connections.Count, shallow.Connections.Count);
            Assert.Equal(design.Stats.Count, shallow.Stats.Count);
            Assert.Equal(design.Props.Count, shallow.Props.Count);
            Assert.Equal(design.Layers.Count, shallow.Layers.Count);
            Assert.Equal(design.Groups.Count, shallow.Groups.Count);
            Assert.Equal(design.Attributes.Count, shallow.Attributes.Count);
            Assert.Equal(design.Authors.Count, shallow.Authors.Count);
            Assert.Equal(design.Concepts.Count, shallow.Concepts.Count);

            for (int i = 0; i < design.Pieces.Count; i++)
            {
                Assert.Equal(design.Pieces[i].Guid, shallow.Pieces[i].Guid);
                Assert.Equal(design.Pieces[i].Name, shallow.Pieces[i].Name);
            }
        }

        [Fact]
        public void Kit_Meta_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var meta = kit.ToMeta();

            Assert.Equal(kit.Guid, meta.Guid);
            Assert.Equal(kit.Name, meta.Name);
            Assert.Equal(kit.Version, meta.Version);
            Assert.Equal(kit.Description, meta.Description);
            Assert.Equal(kit.Icon, meta.Icon);
            Assert.Equal(kit.Image, meta.Image);
            Assert.Equal(kit.Remote, meta.Remote);
            Assert.Equal(kit.Homepage, meta.Homepage);
            Assert.Equal(kit.License, meta.License);
            Assert.Equal(kit.Preview, meta.Preview);
            Assert.Equal(kit.CreatedAt, meta.CreatedAt);
            Assert.Equal(kit.UpdatedAt, meta.UpdatedAt);
        }

        [Fact]
        public void Kit_Shallow_From_Asset()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var shallow = kit.ToShallow();

            Assert.Equal(kit.Guid, shallow.Guid);
            Assert.Equal(kit.Name, shallow.Name);
            Assert.Equal(kit.Types.Count, shallow.Types.Count);
            Assert.Equal(kit.Designs.Count, shallow.Designs.Count);
            Assert.Equal(kit.Tags.Count, shallow.Tags.Count);
            Assert.Equal(kit.Concepts.Count, shallow.Concepts.Count);
            Assert.Equal(kit.Ports.Count, shallow.Ports.Count);
            Assert.Equal(kit.Qualities.Count, shallow.Qualities.Count);
            Assert.Equal(kit.Files.Count, shallow.Files.Count);
            Assert.Equal(kit.Folders.Count, shallow.Folders.Count);
            Assert.Equal(kit.Authors.Count, shallow.Authors.Count);
            Assert.Equal(kit.Attributes.Count, shallow.Attributes.Count);

            for (int i = 0; i < kit.Types.Count; i++)
            {
                Assert.Equal(kit.Types[i].Guid, shallow.Types[i].Guid);
                Assert.Equal(kit.Types[i].Name, shallow.Types[i].Name);
            }
        }

        [Fact]
        public void Kit_To_Meta_To_Shallow()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");

            var meta = kit.ToMeta();
            Assert.NotNull(meta);
            Assert.Equal(kit.Name, meta.Name);

            var shallow = kit.ToShallow();
            Assert.NotNull(shallow);
            Assert.Equal(kit.Name, shallow.Name);
            Assert.Equal(kit.Types.Count, shallow.Types.Count);
            Assert.Equal(kit.Designs.Count, shallow.Designs.Count);

            var metaJson = Utility.Serialize(meta);
            var shallowJson = Utility.Serialize(shallow);
            Assert.NotNull(metaJson);
            Assert.NotNull(shallowJson);

            var metaDeserialized = Utility.Deserialize<KitMeta>(metaJson);
            Assert.NotNull(metaDeserialized);
            Assert.Equal(meta.Name, metaDeserialized!.Name);
            Assert.Equal(meta.Version, metaDeserialized.Version);

            var shallowDeserialized = Utility.Deserialize<KitShallow>(shallowJson);
            Assert.NotNull(shallowDeserialized);
            Assert.Equal(shallow.Name, shallowDeserialized!.Name);
            Assert.Equal(shallow.Types.Count, shallowDeserialized.Types.Count);

            foreach (var type in kit.Types)
            {
                var typeMeta = type.ToMeta();
                var typeShallow = type.ToShallow();
                Assert.Equal(type.Guid, typeMeta.Guid);
                Assert.Equal(type.Guid, typeShallow.Guid);
                Assert.Equal(type.Models.Count, typeShallow.Models.Count);
                Assert.Equal(type.Connectors.Count, typeShallow.Connectors.Count);
            }

            foreach (var design in kit.Designs)
            {
                var designMeta = design.ToMeta();
                var designShallow = design.ToShallow();
                Assert.Equal(design.Guid, designMeta.Guid);
                Assert.Equal(design.Guid, designShallow.Guid);
                Assert.Equal(design.Pieces.Count, designShallow.Pieces.Count);
                Assert.Equal(design.Connections.Count, designShallow.Connections.Count);
            }
        }
    }

    #region 🎬Hash
    // Hash tests for Merkle hash functions.

    public class HashTests
    {
        private static JObject LoadHashAsset() =>
            JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "hash.cases.semio.json")));

        [Fact]
        public void HashKit_Metabolism_Matches_Expected()
        {
            var asset = LoadHashAsset();
            var kitFile = (string)asset["kitHash"]!["kit"]!;
            var kit = Tests.LoadAsset<Kit>(kitFile);

            var hash = Hashing.HashKit(kit);
#if NET48
            var expected = (string)asset["kitHash"]!["expectedNet48"]!;
#else
            var expected = (string)asset["kitHash"]!["expected"]!;
#endif
            Assert.Equal(expected, hash);
        }

        [Fact]
        public void HashKit_Deterministic()
        {
            var kit = Tests.LoadAsset<Kit>("metabolism.kit.semio.json");
            var hash1 = Hashing.HashKit(kit);
            var hash2 = Hashing.HashKit(kit);
            System.IO.File.WriteAllText(Path.Combine(Tests.AssetsPath, "_debug_hash.txt"), hash1);
            Assert.Equal(hash1, hash2);
        }

        [Fact]
        public void HashPoint_Deterministic()
        {
            var p = new Point { X = 1.5, Y = -2.3, Z = 0 };
            var h1 = Hashing.HashPoint(p);
            var h2 = Hashing.HashPoint(p);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void HashVector_Deterministic()
        {
            var v = new Vector { X = 0, Y = 0, Z = 1 };
            var h1 = Hashing.HashVector(v);
            var h2 = Hashing.HashVector(v);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void HashConnector_Deterministic()
        {
            var c = new Connector
            {
                Guid = "test-guid",
                T = 0.5,
                Mandatory = true,
                Point = new Point { X = 0, Y = 0, Z = 0 },
                Direction = new Vector { X = 0, Y = 0, Z = 1 }
            };
            var h1 = Hashing.HashConnector(c);
            var h2 = Hashing.HashConnector(c);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void HashDesign_Deterministic()
        {
            var d = new Design { Guid = "test-design", Name = "TestDesign" };
            var h1 = Hashing.HashDesign(d);
            var h2 = Hashing.HashDesign(d);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void HashConnection_Deterministic()
        {
            var c = new Connection
            {
                Guid = "test-conn",
                Gap = 0,
                Shift = 0,
                Rise = 0,
                Rotation = 90,
                Turn = 0,
                Tilt = 0,
                Connected = new Side { Piece = new PieceId { Guid = "p1" } },
                Connecting = new Side { Piece = new PieceId { Guid = "p2" } }
            };
            var h1 = Hashing.HashConnection(c);
            var h2 = Hashing.HashConnection(c);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void FormatNumberForHash_Integers()
        {
            Assert.Equal("0", Hashing.FormatNumberForHash(0));
            Assert.Equal("1", Hashing.FormatNumberForHash(1));
            Assert.Equal("-1", Hashing.FormatNumberForHash(-1));
            Assert.Equal("42", Hashing.FormatNumberForHash(42));
        }

        [Fact]
        public void FormatNumberForHash_Decimals()
        {
            Assert.Equal("0.5", Hashing.FormatNumberForHash(0.5));
            Assert.Equal("1.5", Hashing.FormatNumberForHash(1.5));
        }

        [Fact]
        public void FormatNumberForHash_JavaScriptThresholds()
        {
            Assert.Equal("-2.7755576e-17", Hashing.FormatNumberForHash(-2.7755576e-17));
            Assert.Equal("0.000001", Hashing.FormatNumberForHash(0.000001));
            Assert.Equal("1e-7", Hashing.FormatNumberForHash(1e-7));
            Assert.Equal("100000000000000000000", Hashing.FormatNumberForHash(1e20));
            Assert.Equal("1e+21", Hashing.FormatNumberForHash(1e21));
        }

        [Fact]
        public void HashKitDiff_Canonical()
        {
            var asset = LoadHashAsset();
            var expectedHash = (string)asset["kitDiffHash"]!["expected"]!;
            var d = new KitDiff { Name = "updated", Description = null };
            d.GetType().GetMethod("ShouldSerializeDescription")!.Invoke(d, null);
            var dManual = new KitDiff();
            dManual.Name = "updated";
            dManual.Description = null;
            var hash = Hashing.HashKitDiff(dManual);
            Assert.Equal(expectedHash, hash);
        }

        [Fact]
        public void HashKitDiff_NameOnly()
        {
            var asset = LoadHashAsset();
            var canonicalHash = (string)asset["kitDiffHash"]!["expected"]!;
            var d = new KitDiff();
            d.Name = "updated";
            var hash = Hashing.HashKitDiff(d);
            Assert.Equal(64, hash.Length);
            Assert.NotEqual(canonicalHash, hash);
        }

        [Fact]
        public void HashKitDiff_Empty()
        {
            var d = new KitDiff();
            var hash = Hashing.HashKitDiff(d);
            Assert.Equal(64, hash.Length);
        }

        [Fact]
        public void HashKitDiff_Deterministic()
        {
            var d = new KitDiff();
            d.Name = "test";
            d.Description = "desc";
            var h1 = Hashing.HashKitDiff(d);
            var h2 = Hashing.HashKitDiff(d);
            Assert.Equal(h1, h2);
        }

        [Fact]
        public void HashAttributeDiff_Deterministic()
        {
            var d = new AttributeDiff();
            d.Key = "k1";
            d.Value = "v1";
            var h1 = Hashing.HashAttributeDiff(d);
            var h2 = Hashing.HashAttributeDiff(d);
            Assert.Equal(h1, h2);
            Assert.Equal(64, h1.Length);
        }

        [Fact]
        public void HashKitDiff_FieldOrderMatters()
        {
            var d1 = new KitDiff();
            d1.Name = "a";
            d1.Description = "b";
            var d2 = new KitDiff();
            d2.Name = "b";
            d2.Description = "a";
            Assert.NotEqual(Hashing.HashKitDiff(d1), Hashing.HashKitDiff(d2));
        }
    }

    #endregion 🎬Hash

    #region 🐙MaxChildren

    public class MaxChildrenTests
    {
        [Fact]
        public void Port_MaxChildren_Serialization_Roundtrip()
        {
            var port = new Port { Guid = "p1", Name = "TestPort", MaxChildren = 3 };
            var json = Utility.Serialize(port);
            var restored = Utility.Deserialize<Port>(json)!;
            Assert.Equal(3, restored.MaxChildren);
        }

        [Fact]
        public void PortDiff_MaxChildren_Null_Omitted()
        {
            var diff = new PortDiff { Guid = "p1", Name = "TestPort" };
            var json = Utility.Serialize(diff);
            Assert.DoesNotContain("maxChildren", json);
        }

        [Fact]
        public void Connector_MaxChildren_Serialization_Roundtrip()
        {
            var connector = new Connector
            {
                Guid = "c1",
                T = 0,
                Point = new Point { X = 0, Y = 0, Z = 0 },
                Direction = new Vector { X = 0, Y = 0, Z = 1 },
                MaxChildren = 5,
            };
            var json = Utility.Serialize(connector);
            var restored = Utility.Deserialize<Connector>(json)!;
            Assert.Equal(5, restored.MaxChildren);
        }

        [Fact]
        public void Kit_MaxChildren_Roundtrip()
        {
            var kit = new Kit
            {
                Guid = "kit-1",
                Name = "TestKit",
                Ports = new List<Port>
                {
                    new Port { Guid = "p1", Name = "Port1", MaxChildren = 3 },
                },
                Types = new List<Type>
                {
                    new Type
                    {
                        Guid = "t1",
                        Name = "Type1",
                        Connectors = new List<Connector>
                        {
                            new Connector
                            {
                                Guid = "c1",
                                T = 0,
                                Point = new Point { X = 0, Y = 0, Z = 0 },
                                Direction = new Vector { X = 0, Y = 0, Z = 1 },
                                MaxChildren = 5,
                            },
                        },
                    },
                },
            };
            var json = Utility.Serialize(kit);
            var restored = Utility.Deserialize<Kit>(json)!;
            Assert.Equal(3, restored.Ports![0].MaxChildren);
            Assert.Equal(5, restored.Types![0].Connectors![0].MaxChildren);
        }
    }

    #endregion 🐙MaxChildren

    #region 🔍Find Replaceable Types In Designs Tests

    public class FindReplaceableTypesInDesignsTests
    {
        private sealed class Selection
        {
            public List<PieceId> Pieces { get; set; } = new();
            public List<ConnectionId> Connections { get; set; } = new();
        }

        private static JObject LoadFindReplaceableAsset() =>
            JObject.Parse(System.IO.File.ReadAllText(Path.Combine(Tests.AssetsPath, "find-replaceable-types.cases.semio.json")));

        private static JToken GetCase(JObject asset, string name) =>
            ((JArray)asset["cases"]!).First(c => (string)c["name"]! == name);

        private static Design FindDesign(Kit kit, string designName, string? parentName)
        {
            if (parentName != null)
            {
                var parent = kit.Designs!.First(d => d.Name == parentName && d.Parent == null);
                return kit.Designs!.First(d => d.Name == designName && d.Parent?.Guid == parent.Guid);
            }
            return kit.Designs!.First(d => d.Name == designName && d.Parent == null);
        }

        [Fact]
        public void Selection_Asset_Returns_Compatible_Type_And_Design_Guids()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "selection_asset_returns_compatible_guids");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var selectionAssetFile = (string)c["selectionAsset"]!;
            var expectedPieceCount = (int)c["expectedSelectionPieceCount"]!;
            var expectedConnectionCount = (int)c["expectedSelectionConnectionCount"]!;

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);
            var selection = Tests.LoadAsset<Selection>(selectionAssetFile);

            Assert.Equal(expectedPieceCount, selection.Pieces.Count);
            Assert.Equal(expectedConnectionCount, selection.Connections.Count);

            var pieceGuids = selection.Pieces.Select(p => p.Guid).ToList();
            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(kit, design.Guid, pieceGuids);

            Assert.NotEmpty(result.TypeGuids);
            Assert.NotEmpty(result.DesignGuids);
        }

        [Fact]
        public void Connected_Piece()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "connected_piece_yields_only_exact_design_matches");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var pieceNames = ((JArray)c["pieceNames"]!).Select(t => (string)t!).ToList();

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);
            var piece = design.Pieces.First(p => p.Name == pieceNames[0]);

            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(
                kit, design.Guid, new List<string> { piece.Guid });
            var typeGuids = result.TypeGuids;

            Assert.NotEmpty(typeGuids);
        }

        [Fact]
        public void Isolated_Piece()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "isolated_piece");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var designParentName = (string?)c["designParentName"];
            var usePieceIndex = (int)c["usePieceIndex"]!;

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var flatDesign = FindDesign(kit, designName, designParentName);
            var piece = flatDesign.Pieces[usePieceIndex];

            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(
                kit, flatDesign.Guid, new List<string> { piece.Guid });
            var typeGuids = result.TypeGuids;

            Assert.NotEmpty(typeGuids);

            var pieceTypeGuid = piece.Type!.Guid;
            Assert.Contains(pieceTypeGuid, typeGuids);
        }

        [Fact]
        public void Capital_Piece()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "capital_piece");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var lookupTypeName = (string)c["lookupTypeName"]!;
            var forbiddenTypeNames = ((JArray)c["forbiddenTypeNames"]!).Select(t => (string)t!).ToList();

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);
            var lookupType = kit.Types!.First(t => t.Name == lookupTypeName);
            var piece = design.Pieces.First(p => p.Type?.Guid == lookupType.Guid);

            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(
                kit, design.Guid, new List<string> { piece.Guid });
            var typeGuids = result.TypeGuids;

            Assert.NotEmpty(typeGuids);

            foreach (var forbiddenName in forbiddenTypeNames)
            {
                var forbiddenType = kit.Types!.First(t => t.Name == forbiddenName);
                Assert.DoesNotContain(forbiddenType.Guid, typeGuids);
            }
        }

        [Fact]
        public void Multiple_Selected_Pieces()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "multiple_selected_pieces");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var pieceNames = ((JArray)c["pieceNames"]!).Select(t => (string)t!).ToList();

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);
            var pieceGuids = pieceNames.Select(name => design.Pieces.First(p => p.Name == name).Guid).ToList();

            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(
                kit, design.Guid, pieceGuids);
            var typeGuids = result.TypeGuids;

            Assert.NotEmpty(typeGuids);
        }

        [Fact]
        public void Empty_Selection()
        {
            var asset = LoadFindReplaceableAsset();
            var c = GetCase(asset, "empty_selection");
            var kitFile = (string)c["kit"]!;
            var designName = (string)c["designName"]!;
            var pieceNames = ((JArray)c["pieceNames"]!).Select(t => (string)t!).ToList();

            var kit = Tests.LoadAsset<Kit>(kitFile);
            var design = kit.Designs!.First(d => d.Name == designName && d.Parent == null);

            var result = Kit.FindReplaceableTypesInDesignsForPiecesInDesign(
                kit, design.Guid, new List<string>());
            var typeGuids = result.TypeGuids;

            var connectorlessCount = kit.Types!.Count(t => t.Connectors.Count == 0);
            Assert.Equal(connectorlessCount, typeGuids.Count);
        }
    }

    #endregion 🔍Find Replaceable Types In Designs Tests

}

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
    private static readonly string AssetsPath = "../../../../../assets/semio";
    private const double Tolerance = 0.001;

    // Helper: Load asset
    private static T LoadAsset<T>(string filename)
    {
        var path = Path.Combine(AssetsPath, filename);
        if (!File.Exists(path)) throw new FileNotFoundException($"Asset not found at {Path.GetFullPath(path)}");
        var json = File.ReadAllText(path);
        return JsonConvert.DeserializeObject<T>(json)!;
    }

    // Helper: Geometry Equality
    private static bool PlanesEqual(Plane? p1, Plane? p2)
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

    private static bool CentersEqual(Coord? c1, Coord? c2)
    {
        if (c1 == null || c2 == null) return c1 == c2;
        return Math.Abs(c1.U - c2.U) < Tolerance && Math.Abs(c1.V - c2.V) < Tolerance;
    }

    // Helper: Find Design
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

    // Tests

    [Fact]
    public void Kit_Serialization_Roundtrip()
    {
        var kit = LoadAsset<Kit>("kit_metabolism.json");
        var json = kit.Serialize();
        var deserializedKit = json.Deserialize<Kit>();
        Assert.Equal(kit.Serialize(), deserializedKit!.Serialize());
    }

    [Fact]
    public void Flatten_Nakagin_Capsule_Tower() => TestFlatten("Nakagin Capsule Tower");

    [Fact]
    public void Flatten_Nakagin_Capsule_Tower_Slanted() => TestFlatten("Slanted", "Nakagin Capsule Tower");
    
    [Fact]
    public void Flatten_Nakagin_Capsule_Tower_Twisted() => TestFlatten("Twisted", "Nakagin Capsule Tower");

    [Fact]
    public void Flatten_Nakagin_Capsule_Tower_Dancing() => TestFlatten("Dancing", "Nakagin Capsule Tower");

    [Fact]
    public void Flatten_Capsule_Dream() => TestFlatten("Capsule Dream");

    private void TestFlatten(string designName, string? parentName = null)
    {
        var kit = LoadAsset<Kit>("kit_metabolism.json");
        var design = FindDesign(kit, designName, parentName);
        
        var expectedDesign = kit.Designs.FirstOrDefault(d => d.Name == "Flat" && d.Parent?.Guid == design.Guid);
        Assert.NotNull(expectedDesign);

        // Perform Flatten
        var flatDesign = design.Flatten(kit.Types);
        
        // Assertions
        foreach(var p in flatDesign.Pieces)
        {
            var expectedPiece = expectedDesign!.Pieces.FirstOrDefault(ep => ep.Name == p.Name);
            Assert.NotNull(expectedPiece);
            Assert.NotNull(p.Plane);
            
            Assert.True(PlanesEqual(p.Plane, expectedPiece.Plane), $"Plane mismatch for piece {p.Name}");
            // Note: Piece.Center is Coord in C# (u,v). TS checks centersEqual (u,v).
            if (p.Center != null && expectedPiece.Center != null)
            {
                 Assert.True(CentersEqual(p.Center, expectedPiece.Center), $"Center mismatch for piece {p.Name}");
            }
        }
    }

    [Fact]
    public void Validation_Metabolism()
    {
        var kit = LoadAsset<Kit>("kit_metabolism.json");
        var result = SemioValidator.ValidateKit(kit);
        Assert.Empty(result.Problems);
    }

    [Fact]
    public void Validation_Invalid_Kit()
    {
        var kit = LoadAsset<Kit>("kit_invalid.json");
        var result = SemioValidator.ValidateKit(kit);
        var expected = LoadAsset<ValidationResult>("validation.json");
        
        // Check structural equality by serialization
        Assert.Equal(expected.Serialize(), result.Serialize());
    }
}

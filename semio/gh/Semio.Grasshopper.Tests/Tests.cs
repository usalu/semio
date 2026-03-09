#region 🔖Header
// [👤semio📚gh🛅semiograsshoppertests💻tests](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper.Tests/f/Tests.cs)

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

using System.Drawing;
using Rhino.FileIO;
using Rhino.Geometry;
using Semio.Grasshopper;

namespace Semio.Grasshopper.Tests;

#region 🔖IconResourceTests
// Tests MUST verify icon resolution supports renamed keys and placeholder fallback.
public class IconResourceTests
{
    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveExistingIcon()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("semio_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }

    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveLegacyAliasWithoutUnderscore()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("attributeid_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }

    [Fact]
    public void ResolveOrPlaceholder_ShouldReturnPlaceholderWhenResourceIsMissing()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("missing_resource_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }
}
#endregion 🔖IconResourceTests

#region 🔖ImportModelUtilityTests
// Tests MUST verify model import utility supports base64 and data URI file blobs.
public class ImportModelUtilityTests
{
    [Fact]
    public void DecodeFileBlobString_ShouldDecodePlainBase64()
    {
        var payload = new byte[] { 1, 2, 3, 4 };
        var blob = Convert.ToBase64String(payload);

        var decoded = Utility.DecodeFileBlobString(blob);

        Assert.Equal(payload, decoded);
    }

    [Fact]
    public void DecodeFileBlobString_ShouldDecodeDataUriBlob()
    {
        var payload = new byte[] { 10, 20, 30 };
        var blob = $"data:application/octet-stream;base64,{Convert.ToBase64String(payload)}";

        var decoded = Utility.DecodeFileBlobString(blob);

        Assert.Equal(payload, decoded);
    }

    [Fact]
    public void ImportRhinoModelObjectFromBlob_ShouldReturnFirstModelObject()
    {
        var model = new File3dm();
        model.Objects.AddPoint(Point3d.Origin);
        model.Objects.AddPoint(new Point3d(1, 1, 1));
        var blob = Convert.ToBase64String(model.ToByteArray());

        var modelObject = Utility.ImportRhinoModelObjectFromBlob(blob);

        Assert.NotNull(modelObject);
        Assert.Equal(Rhino.DocObjects.ObjectType.Point, modelObject.Geometry.ObjectType);
        Assert.Equal(2, model.Objects.Count);
    }

    [Fact]
    public void TranslateRhinoModelObjectToSingleGroup_ShouldCreateRecursiveNamedLayerGroups()
    {
        var model = new File3dm();
        var parentLayer = new Rhino.DocObjects.Layer { Name = "Parent", Id = Guid.NewGuid(), Color = Color.Red };
        var childLayer = new Rhino.DocObjects.Layer { Name = "Child", Id = Guid.NewGuid(), ParentLayerId = parentLayer.Id, Color = Color.Blue };
        model.Layers.Add(parentLayer);
        model.Layers.Add(childLayer);
        var parentLayerIndex = 0;
        var childLayerIndex = 1;

        var parentAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = parentLayerIndex };
        var childAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = childLayerIndex };
        model.Objects.AddPoint(new Point3d(0, 0, 0), parentAttributes);
        model.Objects.AddPoint(new Point3d(1, 0, 0), childAttributes);

        var blob = Convert.ToBase64String(model.ToByteArray());
        var imported = Utility.ImportRhinoModelContextFromBlob(blob);
        var group = Utility.TranslateRhinoModelObjectToSingleGroup(imported);

        Assert.NotNull(group);
        Assert.Equal("Imported Rhino Layer Group", group.Name);
        Assert.Equal(2, group.Pieces.Count);
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Parent");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Parent/Child");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup");
    }
}
#endregion 🔖ImportModelUtilityTests

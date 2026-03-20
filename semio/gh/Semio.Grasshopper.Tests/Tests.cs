#region 🔖Header
// [👤semio📚gh🛅semiograsshoppertests💻tests](repo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper.Tests/f/Tests.cs)

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
using System.Reflection;
using System.Text.RegularExpressions;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Grasshopper.Rhinoceros.Model;
using Grasshopper.Rhinoceros.Model.Params;
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
    public void ImportRhinoModelContextFromSemioFile_ShouldImportFromFileBlob()
    {
        var model = new File3dm();
        model.Objects.AddPoint(Point3d.Origin);
        var blob = Convert.ToBase64String(model.ToByteArray());
        var file = new Semio.File { Guid = "file-1", Blob = blob };

        var context = Utility.ImportRhinoModelContextFromSemioFile(file);

        Assert.NotNull(context);
        Assert.NotNull(context.Model);
        Assert.NotNull(context.ModelObject);
        Assert.Equal(Rhino.DocObjects.ObjectType.Point, context.ModelObject.Geometry.ObjectType);
    }

    [Fact]
    public void ImportRhinoModelContextFromSemioFile_ShouldAllowModelWithoutObjects()
    {
        var model = new File3dm();
        var blob = Convert.ToBase64String(model.ToByteArray());
        var file = new Semio.File { Guid = "file-empty", Blob = blob, Name = "empty.3dm" };

        var context = Utility.ImportRhinoModelContextFromSemioFile(file);

        Assert.NotNull(context);
        Assert.NotNull(context.Model);
        Assert.Null(context.ModelObject);
    }

    [Fact]
    public void ImportRhinoModelObjectDataFromSemioFile_ShouldReturnGrasshopperModelObjectWithImportMetadata()
    {
        var model = new File3dm();
        model.Objects.AddPoint(Point3d.Origin);
        var blob = Convert.ToBase64String(model.ToByteArray());
        var file = new Semio.File { Guid = "file-object-data", Blob = blob, Name = "sample.3dm" };

        var importedModelObjectData = Utility.ImportRhinoModelObjectDataFromSemioFile(file);

        Assert.NotNull(importedModelObjectData);
        Assert.IsType<ModelObject>(importedModelObjectData);
        Assert.True(importedModelObjectData.IsValid);
        Assert.True(importedModelObjectData.UserText.TryGetValue("semio.import-model.blob", out var resolvedBlob));
        Assert.Equal(blob, resolvedBlob);
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

    [Fact]
    public void TranslateRhinoModelObjectsToSingleGroup_ShouldMergeListIntoSingleRecursiveGroup()
    {
        var firstModel = new File3dm();
        var firstLayer = new Rhino.DocObjects.Layer { Name = "First", Id = Guid.NewGuid(), Color = Color.Red };
        var firstChildLayer = new Rhino.DocObjects.Layer { Name = "Nested", Id = Guid.NewGuid(), ParentLayerId = firstLayer.Id, Color = Color.Orange };
        firstModel.Layers.Add(firstLayer);
        firstModel.Layers.Add(firstChildLayer);
        firstModel.Objects.AddPoint(new Point3d(0, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 0 });
        firstModel.Objects.AddPoint(new Point3d(1, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 1 });

        var secondModel = new File3dm();
        var secondLayer = new Rhino.DocObjects.Layer { Name = "Second", Id = Guid.NewGuid(), Color = Color.Blue };
        secondModel.Layers.Add(secondLayer);
        secondModel.Objects.AddPoint(new Point3d(2, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 0 });

        var importedModelObjects = new List<Utility.RhinoModelObject>
        {
            Utility.ImportRhinoModelContextFromBlob(Convert.ToBase64String(firstModel.ToByteArray())),
            Utility.ImportRhinoModelContextFromBlob(Convert.ToBase64String(secondModel.ToByteArray()))
        };

        var group = Utility.TranslateRhinoModelObjectsToSingleGroup(importedModelObjects);

        Assert.NotNull(group);
        Assert.Equal("Imported Rhino Layer Group", group.Name);
        Assert.Equal(3, group.Pieces.Count);
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/First");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/First/Nested");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Second");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup");
    }

    [Fact]
    public void TranslateRhinoModelObjectsToSingleGroup_ShouldThrowWhenListIsEmpty()
    {
        var exception = Assert.Throws<InvalidOperationException>(() => Utility.TranslateRhinoModelObjectsToSingleGroup(new List<Utility.RhinoModelObject>()));
        Assert.Contains("at least one Rhino ModelObject", exception.Message, StringComparison.OrdinalIgnoreCase);
    }
}
#endregion 🔖ImportModelUtilityTests

#region 🔖ModelObjectToGroupComponentTests
// Tests MUST verify ModelObject To Group consumes list input from Import Model output.
public class ModelObjectToGroupComponentTests
{
    [Fact]
    public void RegisterParams_ShouldUseNativeRhinoKindsForInputAndOutput()
    {
        var component = new ModelObjectToGroupComponent();

        Assert.Single(component.Params.Input);
        Assert.Equal(GH_ParamAccess.list, component.Params.Input[0].Access);
        Assert.Equal("Mo*", component.Params.Input[0].NickName);
        Assert.IsType<Param_ModelObject>(component.Params.Input[0]);
        Assert.Single(component.Params.Output);
        Assert.Equal("Gr", component.Params.Output[0].NickName);
        Assert.IsType<Param_Group>(component.Params.Output[0]);
    }

    [Fact]
    public void BuildNativeRhinoGeometryGroup_ShouldKeepUnlayeredImportedObjectsAsFlatGeometryItems()
    {
        var model = new File3dm();
        const int expectedObjectCount = 459;
        for (var index = 0; index < expectedObjectCount; index++)
        {
            var unlayeredAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = -1 };
            model.Objects.AddPoint(new Point3d(index, 0, 0), unlayeredAttributes);
        }

        var file = new Semio.File
        {
            Guid = "unlayered-import",
            Name = "unlayered.3dm",
            Blob = Convert.ToBase64String(model.ToByteArray())
        };

        var importedRhinoObjects = Utility.ImportRhinoDocumentObjectsFromSemioFile(file);
        Assert.Equal(expectedObjectCount, importedRhinoObjects.Count);

        var resolvedContexts = new List<Utility.RhinoModelObject>();
        foreach (var importedRhinoObject in importedRhinoObjects)
        {
            var didResolve = Utility.TryResolveRhinoModelContext(new ModelObject(importedRhinoObject), out var resolvedContext);
            Assert.True(didResolve);
            Assert.NotNull(resolvedContext.ModelObject);
            resolvedContexts.Add(resolvedContext);
        }

        var buildNativeGroupMethod = typeof(ModelObjectToGroupComponent)
            .GetMethod("BuildNativeRhinoGeometryGroup", BindingFlags.NonPublic | BindingFlags.Static);
        Assert.NotNull(buildNativeGroupMethod);

        var nativeGroup = Assert.IsType<GH_GeometryGroup>(buildNativeGroupMethod!.Invoke(null, new object[] { resolvedContexts }));
        Assert.Equal(0, nativeGroup.Objects.Count(geometryItem => geometryItem is GH_GeometryGroup));
        Assert.Equal(expectedObjectCount, nativeGroup.Objects.Count(geometryItem => geometryItem is not GH_GeometryGroup));
    }
}
#endregion 🔖ModelObjectToGroupComponentTests

#region 🔖GroupToModelObjectComponentTests
// Tests MUST verify Group To Model Object consumes a single Group input and produces list ModelObject output.
public class GroupToModelObjectComponentTests
{
    [Fact]
    public void RegisterParams_ShouldUseNativeRhinoKindsForInputAndOutput()
    {
        var component = new GroupToModelObjectComponent();

        Assert.Single(component.Params.Input);
        Assert.Equal(GH_ParamAccess.item, component.Params.Input[0].Access);
        Assert.Equal("Gr", component.Params.Input[0].NickName);
        Assert.IsType<Param_Group>(component.Params.Input[0]);
        Assert.Single(component.Params.Output);
        Assert.Equal("Mo*", component.Params.Output[0].NickName);
        Assert.Equal(GH_ParamAccess.list, component.Params.Output[0].Access);
        Assert.IsType<Param_ModelObject>(component.Params.Output[0]);
    }
}
#endregion 🔖GroupToModelObjectComponentTests

#region 🔖NamingConventionTests
// Tests MUST verify Grasshopper components and parameters follow repo naming and description rules.
public class NamingConventionTests
{
    [Fact]
    public void AllComponents_ShouldUseConsistentNicknamesAndDescriptions()
    {
        var componentKind = typeof(Component);
        var scriptingComponentKind = typeof(ScriptingComponent);
        var componentKinds = componentKind.Assembly.GetTypes()
            .Where(kind => kind.IsClass && !kind.IsAbstract && componentKind.IsAssignableFrom(kind) && !scriptingComponentKind.IsAssignableFrom(kind))
            .OrderBy(kind => kind.FullName)
            .ToList();

        Assert.NotEmpty(componentKinds);

        foreach (var kind in componentKinds)
        {
            var component = Activator.CreateInstance(kind) as Component;
            Assert.NotNull(component);

            Assert.Matches("^[A-Z0-9]{3}$", component!.NickName);
            Assert.False(string.IsNullOrWhiteSpace(component.Description));
            Assert.Contains(component.Name, component.Description, StringComparison.OrdinalIgnoreCase);

            foreach (var parameter in component.Params.Input)
                AssertParamMatchesNamingRules(component, parameter, false);
            foreach (var parameter in component.Params.Output)
                AssertParamMatchesNamingRules(component, parameter, true);
        }
    }

    private static void AssertParamMatchesNamingRules(Component component, IGH_Param parameter, bool isOutput)
    {
        var cardinalitySuffix = parameter.Access is GH_ParamAccess.list or GH_ParamAccess.tree
            ? "*"
            : parameter.Optional ? "?" : "";
        var expectedPattern = cardinalitySuffix.Length == 0
            ? "^[A-Za-z0-9]{2}$"
            : $"^[A-Za-z0-9]{{2}}{Regex.Escape(cardinalitySuffix)}$";
        Assert.Matches(expectedPattern, parameter.NickName);

        Assert.False(string.IsNullOrWhiteSpace(parameter.Description));
        Assert.Contains(component.Name, parameter.Description, StringComparison.OrdinalIgnoreCase);
        Assert.Contains(parameter.Name, parameter.Description, StringComparison.OrdinalIgnoreCase);

        if (isOutput)
            Assert.Contains("produced by", parameter.Description, StringComparison.OrdinalIgnoreCase);
        else
            Assert.Contains("consumed by", parameter.Description, StringComparison.OrdinalIgnoreCase);
    }
}
#endregion 🔖NamingConventionTests

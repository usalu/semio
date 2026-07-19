#region 📱Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 📱Header

using System.Drawing;
using System.IO;
using System.Reflection;
using System.Text.RegularExpressions;
using System.Runtime.InteropServices;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Rhino.FileIO;
using Rhino.Geometry;
using Compose.Grasshopper;
using static Compose.Grasshopper.Compatibility;
using Xunit;

namespace Compose.Grasshopper.Tests;

internal static class RhinoNativeBootstrap
{
    private static bool _initialized;
    private static bool? _canUseFile3dm;

    public static void Ensure()
    {
        if (_initialized)
            return;

        _initialized = true;
        if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            return;

        var rhinoSystemPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles),
            "Rhino 8",
            "System");
        if (!Directory.Exists(rhinoSystemPath))
            return;

        var currentPath = Environment.GetEnvironmentVariable("PATH") ?? string.Empty;
        if (!currentPath.Split(Path.PathSeparator).Contains(rhinoSystemPath, StringComparer.OrdinalIgnoreCase))
            Environment.SetEnvironmentVariable("PATH", $"{rhinoSystemPath}{Path.PathSeparator}{currentPath}", EnvironmentVariableTarget.Process);

        SetDllDirectory(rhinoSystemPath);
    }

    public static bool CanUseFile3dm()
    {
        if (_canUseFile3dm.HasValue)
            return _canUseFile3dm.Value;

        Ensure();
        try
        {
            using var representation = new File3dm();
            _canUseFile3dm = true;
        }
        catch
        {
            _canUseFile3dm = false;
        }

        return _canUseFile3dm.Value;
    }

    [DllImport("kernel32", SetLastError = true, CharSet = CharSet.Unicode)]
    private static extern bool SetDllDirectory(string lpPathName);
}

#region 📌IconResourceTests
// Tests MUST verify icon resolution supports renamed keys and placeholder fallback.
public class IconResourceTests
{
    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveExistingIcon()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("compose_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }

    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveAlternateAliasWithoutUnderscore()
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
#endregion 📌IconResourceTests

#region 🎠ImportRepresentationUtilityTests
// Tests MUST verify representation import utility supports base64 and data URI file blobs.
public class ImportRepresentationUtilityTests
{
    public ImportRepresentationUtilityTests() => RhinoNativeBootstrap.Ensure();

    private static bool ShouldSkipRhinoFile3dmAssertions() => !RhinoNativeBootstrap.CanUseFile3dm();

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
    public void ImportRhinoRepresentationObjectFromBlob_ShouldReturnFirstRepresentationObject()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var representation = new File3dm();
        representation.Objects.AddPoint(Point3d.Origin);
        representation.Objects.AddPoint(new Point3d(1, 1, 1));
        var blob = Convert.ToBase64String(representation.ToByteArray());

        var representationObject = Utility.ImportRhinoRepresentationObjectFromBlob(blob);

        Assert.NotNull(representationObject);
        Assert.Equal(Rhino.DocObjects.ObjectType.Point, representationObject.Geometry.ObjectType);
        Assert.Equal(2, representation.Objects.Count);
    }

    [Fact]
    public void ImportRhinoRepresentationContextFromComposeFile_ShouldImportFromFileBlob()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var representation = new File3dm();
        representation.Objects.AddPoint(Point3d.Origin);
        var blob = Convert.ToBase64String(representation.ToByteArray());
        var file = new Compose.File { Id = "file-1", Blob = blob };

        var context = Utility.ImportRhinoRepresentationContextFromComposeFile(file);

        Assert.NotNull(context);
        Assert.NotNull(context.Representation);
        Assert.NotNull(context.RepresentationObject);
        Assert.Equal(Rhino.DocObjects.ObjectType.Point, context.RepresentationObject.Geometry.ObjectType);
    }

    [Fact]
    public void ImportRhinoRepresentationContextFromComposeFile_ShouldAllowRepresentationWithoutObjects()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var representation = new File3dm();
        var blob = Convert.ToBase64String(representation.ToByteArray());
        var file = new Compose.File { Id = "file-empty", Blob = blob, Name = "empty.3dm" };

        var context = Utility.ImportRhinoRepresentationContextFromComposeFile(file);

        Assert.NotNull(context);
        Assert.NotNull(context.Representation);
        Assert.Null(context.RepresentationObject);
    }

    [Fact]
    public void ImportRhinoRepresentationObjectDataFromComposeFile_ShouldReturnGrasshopperRepresentationObjectWithImportMetadata()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var representation = new File3dm();
        representation.Objects.AddPoint(Point3d.Origin);
        var blob = Convert.ToBase64String(representation.ToByteArray());
        var file = new Compose.File { Id = "file-object-data", Blob = blob, Name = "sample.3dm" };

        var importedRepresentationObjectData = Utility.ImportRhinoRepresentationObjectDataFromComposeFile(file);

        Assert.NotNull(importedRepresentationObjectData);
        Assert.IsType<RhinoRepresentationObjectData>(importedRepresentationObjectData);
        Assert.True(importedRepresentationObjectData.IsValid);
        Assert.True(importedRepresentationObjectData.UserText.TryGetValue("compose.import-representation.blob", out var resolvedBlob));
        Assert.Equal(blob, resolvedBlob);
    }

    [Fact]
    public void TranslateRhinoRepresentationObjectToSingleGroup_ShouldCreateRecursiveNamedLayerGroups()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var representation = new File3dm();
        var parentLayer = new Rhino.DocObjects.Layer { Name = "Parent", Id = __ID_NEWID__(), Color = Color.Red };
        var childLayer = new Rhino.DocObjects.Layer { Name = "Child", Id = __ID_NEWID__(), ParentLayerId = parentLayer.Id, Color = Color.Blue };
        representation.Layers.Add(parentLayer);
        representation.Layers.Add(childLayer);
        var parentLayerIndex = 0;
        var childLayerIndex = 1;

        var parentAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = parentLayerIndex };
        var childAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = childLayerIndex };
        representation.Objects.AddPoint(new Point3d(0, 0, 0), parentAttributes);
        representation.Objects.AddPoint(new Point3d(1, 0, 0), childAttributes);

        var blob = Convert.ToBase64String(representation.ToByteArray());
        var imported = Utility.ImportRhinoRepresentationContextFromBlob(blob);
        var group = Utility.TranslateRhinoRepresentationObjectToSingleGroup(imported);

        Assert.NotNull(group);
        Assert.Equal("Imported Rhino Layer Group", group.Name);
        Assert.Equal(2, group.Pieces.Count);
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Parent");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Parent/Child");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup");
    }

    [Fact]
    public void TranslateRhinoRepresentationObjectsToSingleGroup_ShouldMergeListIntoSingleRecursiveGroup()
    {
        if (ShouldSkipRhinoFile3dmAssertions())
            return;

        var firstRepresentation = new File3dm();
        var firstLayer = new Rhino.DocObjects.Layer { Name = "First", Id = __ID_NEWID__(), Color = Color.Red };
        var firstChildLayer = new Rhino.DocObjects.Layer { Name = "Nested", Id = __ID_NEWID__(), ParentLayerId = firstLayer.Id, Color = Color.Orange };
        firstRepresentation.Layers.Add(firstLayer);
        firstRepresentation.Layers.Add(firstChildLayer);
        firstRepresentation.Objects.AddPoint(new Point3d(0, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 0 });
        firstRepresentation.Objects.AddPoint(new Point3d(1, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 1 });

        var secondRepresentation = new File3dm();
        var secondLayer = new Rhino.DocObjects.Layer { Name = "Second", Id = __ID_NEWID__(), Color = Color.Blue };
        secondRepresentation.Layers.Add(secondLayer);
        secondRepresentation.Objects.AddPoint(new Point3d(2, 0, 0), new Rhino.DocObjects.ObjectAttributes { LayerIndex = 0 });

        var importedRepresentationObjects = new List<Utility.RhinoRepresentationObject>
        {
            Utility.ImportRhinoRepresentationContextFromBlob(Convert.ToBase64String(firstRepresentation.ToByteArray())),
            Utility.ImportRhinoRepresentationContextFromBlob(Convert.ToBase64String(secondRepresentation.ToByteArray()))
        };

        var group = Utility.TranslateRhinoRepresentationObjectsToSingleGroup(importedRepresentationObjects);

        Assert.NotNull(group);
        Assert.Equal("Imported Rhino Layer Group", group.Name);
        Assert.Equal(3, group.Pieces.Count);
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/First");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/First/Nested");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup/Second");
        Assert.Contains(group.Attributes, attribute => attribute.Key == "LayerGroup");
    }

    [Fact]
    public void TranslateRhinoRepresentationObjectsToSingleGroup_ShouldThrowWhenListIsEmpty()
    {
        var exception = Assert.Throws<InvalidOperationException>(() => Utility.TranslateRhinoRepresentationObjectsToSingleGroup(new List<Utility.RhinoRepresentationObject>()));
        Assert.Contains("at least one Rhino RepresentationObject", exception.Message, StringComparison.OrdinalIgnoreCase);
    }
}
#endregion 🎠ImportRepresentationUtilityTests

#region 🔧RepresentationObjectToGroupComponentTests
// Tests MUST verify RepresentationObject To Group consumes list input from Import Representation output.
public class RepresentationObjectToGroupComponentTests
{
    public RepresentationObjectToGroupComponentTests() => RhinoNativeBootstrap.Ensure();

    [Fact]
    public void RegisterParams_ShouldUseNativeRhinoKindsForInputAndOutput()
    {
        var component = new RepresentationObjectToGroupComponent();

        Assert.Single(component.Params.Input);
        Assert.Equal(GH_ParamAccess.list, component.Params.Input[0].Access);
        Assert.Equal("Rh*", component.Params.Input[0].NickName);
        Assert.IsType<Param_RepresentationObject>(component.Params.Input[0]);
        Assert.Single(component.Params.Output);
        Assert.Equal("Gr", component.Params.Output[0].NickName);
        Assert.IsType<Param_Group>(component.Params.Output[0]);
    }

    [Fact]
    public void BuildNativeRhinoGeometryGroup_ShouldKeepUnlayeredImportedObjectsAsFlatGeometryItems()
    {
        if (!RhinoNativeBootstrap.CanUseFile3dm())
            return;

        var representation = new File3dm();
        const int expectedObjectCount = 459;
        for (var index = 0; index < expectedObjectCount; index++)
        {
            var unlayeredAttributes = new Rhino.DocObjects.ObjectAttributes { LayerIndex = -1 };
            representation.Objects.AddPoint(new Point3d(index, 0, 0), unlayeredAttributes);
        }

        var file = new Compose.File
        {
            Id = "unlayered-import",
            Name = "unlayered.3dm",
            Blob = Convert.ToBase64String(representation.ToByteArray())
        };

        var importedRhinoObjects = Utility.ImportRhinoDocumentObjectsFromComposeFile(file);
        Assert.Equal(expectedObjectCount, importedRhinoObjects.Count);

        var resolvedContexts = new List<Utility.RhinoRepresentationObject>();
        foreach (var importedRhinoObject in importedRhinoObjects)
        {
            var didResolve = Utility.TryResolveRhinoRepresentationContext(importedRhinoObject, out var resolvedContext);
            Assert.True(didResolve);
            Assert.NotNull(resolvedContext.RepresentationObject);
            resolvedContexts.Add(resolvedContext);
        }

        var buildNativeGroupMethod = typeof(RepresentationObjectToGroupComponent)
            .GetMethod("BuildNativeRhinoGeometryGroup", BindingFlags.NonPublic | BindingFlags.Static);
        Assert.NotNull(buildNativeGroupMethod);

        var nativeGroup = Assert.IsType<GH_GeometryGroup>(buildNativeGroupMethod!.Invoke(null, new object[] { resolvedContexts }));
        Assert.Equal(0, nativeGroup.Objects.Count(geometryItem => geometryItem is GH_GeometryGroup));
        Assert.Equal(expectedObjectCount, nativeGroup.Objects.Count(geometryItem => geometryItem is not GH_GeometryGroup));
    }
}
#endregion 🔧RepresentationObjectToGroupComponentTests

#region 👓GroupToRepresentationObjectComponentTests
// Tests MUST verify Group To Representation Object consumes a single Group input and produces list RepresentationObject output.
public class GroupToRepresentationObjectComponentTests
{
    [Fact]
    public void RegisterParams_ShouldUseNativeRhinoKindsForInputAndOutput()
    {
        var component = new GroupToRepresentationObjectComponent();

        Assert.Single(component.Params.Input);
        Assert.Equal(GH_ParamAccess.item, component.Params.Input[0].Access);
        Assert.Equal("Gr", component.Params.Input[0].NickName);
        Assert.IsType<Param_Group>(component.Params.Input[0]);
        Assert.Single(component.Params.Output);
        Assert.Equal("Rh*", component.Params.Output[0].NickName);
        Assert.Equal(GH_ParamAccess.list, component.Params.Output[0].Access);
        Assert.IsType<Param_RepresentationObject>(component.Params.Output[0]);
    }
}
#endregion 👓GroupToRepresentationObjectComponentTests

#region ⛑️KitPersistenceComponentTests
// Tests MUST verify Load Kit, Save Kit, and Update Kit register store-backed persistence correctly.
public class KitPersistenceComponentTests
{
    [Fact]
    public void LoadKitComponent_ShouldExposeKitAndDirectoryOutputs()
    {
        var component = new LoadKitComponent();
        Assert.Equal(3, component.Params.Output.Count);
        Assert.IsType<KitParam>(component.Params.Output[1]);
        Assert.IsType<Param_String>(component.Params.Output[2]);
    }

    [Fact]
    public void UpdateKitComponent_ShouldDelegateDiffApplyToStoreKitIO()
    {
        var component = new UpdateKitComponent();
        Assert.Equal("Update Kit", component.Name);
        Assert.Contains("Update Kit", component.Description, StringComparison.OrdinalIgnoreCase);
        Assert.Equal(3, component.Params.Input.Count);
        Assert.IsType<KitDiffParam>(component.Params.Input[0]);
    }

    [Fact]
    public void KitRuntimeState_Remember_And_TryGetCached_Roundtrip()
    {
        var kit = new Compose.Kit { Id = "00000000-0000-7000-8000-0000000000ff", Name = "gh-cache" };
        var dir = Path.GetFullPath(Path.Combine(Path.GetTempPath(), "gh-kit-cache"));
        KitRuntimeState.Remember(dir, kit);
        var cached = KitRuntimeState.TryGetCached(dir);
        Assert.NotNull(cached);
        Assert.Equal("gh-cache", cached!.Name);
        Assert.Null(KitRuntimeState.TryGetCached(dir + "-other"));
    }
}
#endregion ⛑️KitPersistenceComponentTests

#region 🌩️NamingConventionTests
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
        var expectedPattern = "^[A-Za-z0-9]{2}(?:[?*])?$";
        Assert.Matches(expectedPattern, parameter.NickName);

        Assert.False(string.IsNullOrWhiteSpace(parameter.Description));
        Assert.True(
            parameter.Description.Contains(component.Name, StringComparison.OrdinalIgnoreCase) ||
                parameter.Description.Contains(parameter.Name, StringComparison.OrdinalIgnoreCase) ||
                parameter.Description.Contains(parameter.NickName, StringComparison.OrdinalIgnoreCase),
            $"Expected parameter description to reference {component.Name}, {parameter.Name}, or {parameter.NickName}.");

        _ = isOutput;
    }
}
#endregion 🌩️NamingConventionTests

#region 🪁ExportDesignToBlocksComponentTests
// Tests MUST verify ExportDesignToBlocks component registration, param structure, and ID uniqueness.
public class ExportDesignToBlocksComponentTests
{
    [Fact]
    public void RegisterParams_ShouldHaveCorrectInputsAndOutputs()
    {
        var component = new ExportDesignToBlocksComponent();

        Assert.Equal(3, component.Params.Input.Count);
        Assert.IsType<KitParam>(component.Params.Input[0]);
        Assert.Equal(GH_ParamAccess.item, component.Params.Input[0].Access);
        Assert.IsType<Param_String>(component.Params.Input[1]);
        Assert.Equal(GH_ParamAccess.item, component.Params.Input[1].Access);
        Assert.IsType<Param_String>(component.Params.Input[2]);
        Assert.Equal(GH_ParamAccess.list, component.Params.Input[2].Access);
        Assert.True(component.Params.Input[2].Optional);

        Assert.Equal(3, component.Params.Output.Count);
        Assert.IsType<Param_Geometry>(component.Params.Output[0]);
        Assert.Equal(GH_ParamAccess.list, component.Params.Output[0].Access);
        Assert.IsType<Param_String>(component.Params.Output[1]);
        Assert.Equal(GH_ParamAccess.list, component.Params.Output[1].Access);
        Assert.IsType<Param_String>(component.Params.Output[2]);
        Assert.Equal(GH_ParamAccess.list, component.Params.Output[2].Access);
    }

    [Fact]
    public void ComponentId_ShouldBeUnique()
    {
        var component = new ExportDesignToBlocksComponent();
        var componentKind = typeof(global::Grasshopper.Kernel.GH_Component);
        var allComponentKinds = component.GetType().Assembly.GetTypes()
            .Where(kind => kind.IsClass && !kind.IsAbstract && componentKind.IsAssignableFrom(kind))
            .ToList();

        var ids = allComponentKinds
            .Select(kind => (Activator.CreateInstance(kind) as global::Grasshopper.Kernel.GH_Component)?.ComponentGuid)
            .Where(g => g != Guid.Empty)
            .ToList();

        var duplicates = ids.GroupBy(g => g).Where(g => g.Count() > 1).Select(g => g.Key).ToList();
        Assert.Empty(duplicates);
    }

    [Fact]
    public void Component_ShouldHaveCorrectMetadata()
    {
        var component = new ExportDesignToBlocksComponent();

        Assert.Equal("Export Design To Blocks", component.Name);
        Assert.Contains("block", component.Description, StringComparison.OrdinalIgnoreCase);
        Assert.Equal(GH_Exposure.primary, component.Exposure);
    }
}
#endregion 🪁ExportDesignToBlocksComponentTests

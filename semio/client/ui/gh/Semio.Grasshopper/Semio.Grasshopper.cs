#region 📱Header

// 2023-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Main Grasshopper plugin providing domain components for Rhino.

#endregion 📱Header

#region ⌛Imports
// Callers MUST import all required namespaces listed here.
using System.Drawing;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Reflection;
using System.Text;
using GH_IO.Serialization;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Grasshopper.Rhinoceros;
using Humanizer;
using Rhino;
using Rhino.Geometry;
using Semio;
using System.Text.RegularExpressions;
using Point = Semio.Point;
using Vector = Semio.Vector;
using Plane = Semio.Plane;
using SemioGroup = Semio.Group;
using Attribute = Semio.Attribute;
using Type = Semio.Type;
using File = Semio.File;
using SemioRepresentation = Semio.Representation;
using Semio.Store;
using AttributeDiffUpdate = Semio.AttributeModification;
using AuthorDiffUpdate = Semio.AuthorModification;
using ConceptDiffUpdate = Semio.ConceptModification;
using ConnectionDiffUpdate = Semio.ConnectionModification;
using ConnectorDiffUpdate = Semio.ConnectorModification;
using DesignDiffUpdate = Semio.DesignModification;
using FileDiffUpdate = Semio.FileModification;
using FolderDiffUpdate = Semio.FolderModification;
using KitDiffUpdate = Semio.KitModification;
using PieceDiffUpdate = Semio.PieceModification;
using PortDiffUpdate = Semio.PortModification;
using RepresentationDiffUpdate = Semio.RepresentationModification;
using TagDiffUpdate = Semio.TagModification;
using TypeDiffUpdate = Semio.TypeModification;
using static Semio.Grasshopper.Compatibility;

#endregion ⌛Imports

#region ✨Namespace
// Implementations MUST reside in this namespace.
namespace Semio.Grasshopper;
#endregion ✨Namespace

#region 🧩Compatibility
// Callers MUST use these shims to bridge stale Grasshopper source assumptions to the current Semio and Rhino APIs.

public static class Compatibility
{
    public static readonly Guid __ID_EMPTY__ = Guid.Empty;

    public static Guid __ID_NEWID__()
    {
        return Guid.NewGuid();
    }
}

public sealed class Param_RepresentationObject : Param_GenericObject
{
    public Param_RepresentationObject()
    {
        Name = "Rhino RepresentationObject";
        NickName = "Mo";
        Description = "Rhino representation object metadata wrapper.";
        Category = Constants.Category;
        SubCategory = "Utility";
    }

    public override Guid ComponentGuid => new("8CF6C13F-C86A-4EB2-9B48-5E7B16A750C5");
}

#endregion 🧩Compatibility

#region 🎠Constants
// Consumers MUST use these shared constants for configuration.

public static class Constants
{
    public const string Category = Semio.Constants.Name;
    public const string Version = "6.0.0";
}

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => Semio.Constants.Name;
    public override Bitmap Icon => IconResources.ResolveOrPlaceholder("semio_24x24");
    public override Bitmap AssemblyIcon => IconResources.ResolveOrPlaceholder("semio_24x24");
    public override string Description => "semio within 🦗.";
    public override Guid Id => new("FE587CBF-5F7D-4091-AA6D-D9D30CF80B64");
    public override string Version => Constants.Version;
    public override string AuthorName => "Ueli Saluz";
    public override string AuthorContact => "ueli@semio-tech.com";
}

public class SemioCategoryIcon : GH_AssemblyPriority
{
    public override GH_LoadingInstruction PriorityLoad()
    {
        Instances.ComponentServer.AddCategoryIcon("semio", IconResources.ResolveOrPlaceholder("semio_24x24"));
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion 🎠Constants

#region 🌡️IconResources
// Callers MUST resolve icon resources through this helper to support renamed keys and placeholders.
public static class IconResources
{
    //#region 📸Private
    // Private MUST provide the private functionality.
    private static readonly Lazy<Dictionary<string, string>> canonicalResourceNames = new(BuildCanonicalResourceNames, true);
    //#endregion 📸Private

    //#region ⚙️Public
    // Public MUST provide the public functionality.
    public static Bitmap ResolveOrPlaceholder(params string[] resourceNames)
    {
        foreach (var resourceName in resourceNames.Where(name => !string.IsNullOrWhiteSpace(name)))
        {
            var candidate = ResolveResource(resourceName!);
            if (candidate is not null)
                return candidate;
        }
        return BuildPlaceholder();
    }
    //#endregion ⚙️Public

    //#region 🪨PrivateHelpers
    // PrivateHelpers MUST provide the privatehelpers functionality.
    private static Bitmap? ResolveResource(string resourceName)
    {
        var direct = Resources.ResourceManager.GetObject(resourceName, CultureInfo.InvariantCulture) as Bitmap;
        if (direct is not null)
            return direct;

        if (!canonicalResourceNames.Value.TryGetValue(Canonicalize(resourceName), out var mappedName))
            return null;

        return Resources.ResourceManager.GetObject(mappedName, CultureInfo.InvariantCulture) as Bitmap;
    }

    private static Dictionary<string, string> BuildCanonicalResourceNames()
    {
        var dictionary = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        var resourceSet = Resources.ResourceManager.GetResourceSet(CultureInfo.InvariantCulture, true, true);
        if (resourceSet is null)
            return dictionary;

        foreach (DictionaryEntry entry in resourceSet)
        {
            if (entry.Key is string name)
                dictionary[Canonicalize(name)] = name;
        }
        return dictionary;
    }

    private static string Canonicalize(string value)
    {
        var chars = value.ToLowerInvariant().Where(char.IsLetterOrDigit).ToArray();
        return new string(chars);
    }

    private static Bitmap BuildPlaceholder()
    {
        var fallback = Resources.ResourceManager.GetObject("semio_24x24", CultureInfo.InvariantCulture) as Bitmap;
        if (fallback is not null)
            return fallback;

        var bitmap = new Bitmap(24, 24);
        using var graphics = Graphics.FromImage(bitmap);
        graphics.Clear(Color.FromArgb(52, 58, 64));
        return bitmap;
    }
    //#endregion 🪨PrivateHelpers
}
#endregion 🌡️IconResources

#region 🎇Utility
// Callers MUST use these utility functions for encoding and serialization.

public static class Utility
{
    internal const string SemioImportRepresentationBlobKey = "semio.import-representation.blob";
    internal const string SemioImportRepresentationObjectIdKey = "semio.import-representation.object-id";
    private static readonly object semioImportDocumentsLock = new();
    private static readonly List<RhinoDoc> semioImportDocuments = new();

    private static RhinoDoc CreateTrackedHeadlessImportDocument()
    {
        var doc = RhinoDoc.CreateHeadless(null);
        lock (semioImportDocumentsLock)
        {
            semioImportDocuments.Add(doc);
            return doc;
        }
    }

    public sealed class RhinoRepresentationObject
    {
        public RhinoRepresentationObject(Rhino.FileIO.File3dm representation, Rhino.FileIO.File3dmObject? representationObject)
        {
            Representation = representation;
            RepresentationObject = representationObject;
        }

        public Rhino.FileIO.File3dm Representation { get; }
        public Rhino.FileIO.File3dmObject? RepresentationObject { get; }
    }

    public static string Serialize<TEntity>(this TEntity value, string indent = "") => Semio.Utility.Serialize(value, indent);
    public static TEntity? Deserialize<TEntity>(this string value) => Semio.Utility.Deserialize<TEntity>(value);
    public static TEntity? DeepClone<TEntity>(this TEntity value) where TEntity : Semio.Entity<TEntity>
        => Semio.Entity<TEntity>.DeepClone(value);

    public static bool IsValidLengthUnitSystem(string unit) => new[] { "nm", "mm", "cm", "dm", "m", "km", "µin", "in", "ft", "yd" }.Contains(unit);
    public static string LengthUnitSystemToAbbreviation(UnitSystem unitSystem)
    {
        var unit = unitSystem switch
        {
            UnitSystem.Nanometers => "nm",
            UnitSystem.Millimeters => "mm",
            UnitSystem.Centimeters => "cm",
            UnitSystem.Decimeters => "dm",
            UnitSystem.Meters => "m",
            UnitSystem.Kilometers => "km",
            UnitSystem.Microinches => "µin",
            UnitSystem.Inches => "in",
            UnitSystem.Feet => "ft",
            UnitSystem.Yards => "yd",
            _ => "unsupported length unit system"
        };
        if (IsValidLengthUnitSystem(unit) == false)
            throw new ArgumentException("Invalid length unit system", nameof(unitSystem));
        return unit;
    }

    public static UnitSystem LengthUnitAbbreviationToUnitSystem(string unit)
    {
        if (string.IsNullOrWhiteSpace(unit))
            throw new ArgumentException("Length unit abbreviation cannot be empty.", nameof(unit));

        var normalizedUnit = unit.Trim().ToLowerInvariant();
        return normalizedUnit switch
        {
            "nm" => UnitSystem.Nanometers,
            "mm" => UnitSystem.Millimeters,
            "cm" => UnitSystem.Centimeters,
            "dm" => UnitSystem.Decimeters,
            "m" => UnitSystem.Meters,
            "km" => UnitSystem.Kilometers,
            "µin" => UnitSystem.Microinches,
            "uin" => UnitSystem.Microinches,
            "in" => UnitSystem.Inches,
            "ft" => UnitSystem.Feet,
            "yd" => UnitSystem.Yards,
            _ => throw new ArgumentException($"Unsupported length unit abbreviation '{unit}'.", nameof(unit))
        };
    }

    public static Rhino.Geometry.Plane GetPlaneFromYAxis(Vector3d yAxis, float theta, Point3d origin)
    {
        var thetaRad = RhinoMath.ToRadians(theta);
        var orientation = Transform.Rotation(Vector3d.YAxis, yAxis, Point3d.Origin);
        var rotation = Transform.Rotation(thetaRad, yAxis, Point3d.Origin);
        var xAxis = Vector3d.XAxis;
        xAxis.Transform(rotation * orientation);
        return new Rhino.Geometry.Plane(origin, xAxis, yAxis);
    }

    public static Plane ComputeChildPlane(Plane parentPlane, Point parentPoint, Vector parentDirection,
        Point childPoint, Vector childDirection, double gap, double shift, double raise, double rotation, double turn,
        double tilt)
    {
        var parentPointR = new Vector3d(parentPoint.Convert());
        var parentDirectionR = parentDirection.Convert();
        var revertedChildPointR = new Vector3d(childPoint.Convert());
        revertedChildPointR.Reverse();

        var reverseChildDirectionR = childDirection.Convert();
        reverseChildDirectionR.Reverse();
        var rotationRad = RhinoMath.ToRadians(rotation);
        var turnRad = RhinoMath.ToRadians(turn);
        var tiltRad = RhinoMath.ToRadians(tilt);

        var areDirectionsSame = parentDirectionR.IsParallelTo(childDirection.Convert(), Semio.Constants.Tolerance) == 1;

        Transform directionT;
        if (areDirectionsSame)
        {

            if (Math.Abs(parentDirectionR.Z) < Semio.Constants.Tolerance)
                directionT = Transform.Rotation(RhinoMath.ToRadians(180), Vector3d.ZAxis, new Point3d());
            else
                directionT = Transform.Rotation(RhinoMath.ToRadians(180),
                    Vector3d.CrossProduct(Vector3d.ZAxis, parentDirectionR), new Point3d());
        }
        else
        {
            directionT = Transform.Rotation(reverseChildDirectionR, parentDirectionR, new Point3d());
        }

        var rotationAxis = Vector3d.YAxis;
        var turnAxis = Vector3d.ZAxis;
        var tiltAxis = Vector3d.XAxis;
        var gapDirection = Vector3d.YAxis;
        var shiftDirection = Vector3d.XAxis;
        var raiseDirection = Vector3d.ZAxis;

        var parentRotation = Transform.Rotation(Vector3d.YAxis, parentDirectionR, new Point3d());

        gapDirection.Transform(parentRotation);
        shiftDirection.Transform(parentRotation);
        raiseDirection.Transform(parentRotation);
        turnAxis.Transform(parentRotation);
        tiltAxis.Transform(parentRotation);

        var orientationT = directionT;

        var rotateT = Transform.Rotation(-rotationRad, parentDirectionR, new Point3d());
        orientationT = rotateT * orientationT;
        turnAxis.Transform(rotateT);
        tiltAxis.Transform(rotateT);

        var turnT = Transform.Rotation(turnRad, turnAxis, new Point3d());
        orientationT = turnT * orientationT;

        var tiltT = Transform.Rotation(tiltRad, tiltAxis, new Point3d());
        orientationT = tiltT * orientationT;

        var centerChild = Transform.Translation(revertedChildPointR);
        var moveToParent = Transform.Translation(parentPointR);
        var transform = orientationT * centerChild;

        var gapTransform = Transform.Translation(gapDirection * gap);
        var shiftTransform = Transform.Translation(shiftDirection * shift);
        var raiseTransform = Transform.Translation(raiseDirection * raise);
        var translation = gapTransform * shiftTransform;
        translation = raiseTransform * translation;

        transform = translation * transform;

        transform = moveToParent * transform;
        var childPlaneR = Rhino.Geometry.Plane.WorldXY;
        childPlaneR.Transform(transform);

        var parentPlaneR = parentPlane.Convert();
        var parentPlaneT = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, parentPlaneR);
        childPlaneR.Transform(parentPlaneT);

        return childPlaneR.Convert();
    }

    public static byte[] DecodeFileBlobString(string fileBlob)
    {
        if (string.IsNullOrWhiteSpace(fileBlob))
            throw new ArgumentException("File blob string cannot be empty.", nameof(fileBlob));

        var encodedData = fileBlob.Trim();
        if (encodedData.StartsWith("data:", StringComparison.OrdinalIgnoreCase))
        {
            var payloadStart = encodedData.IndexOf(',');
            if (payloadStart < 0 || payloadStart == encodedData.Length - 1)
                throw new ArgumentException("Invalid data URI file blob string.", nameof(fileBlob));
            encodedData = encodedData.Substring(payloadStart + 1);
        }

        try
        {
            return Convert.FromBase64String(encodedData);
        }
        catch (FormatException formatException)
        {
            throw new ArgumentException("Invalid file blob string. Expected base64 payload or data URI.", nameof(fileBlob), formatException);
        }
    }

    public static Rhino.FileIO.File3dmObject ImportRhinoRepresentationObjectFromBlob(string fileBlob)
    {
        var context = ImportRhinoRepresentationContextFromBlob(fileBlob);
        return context.RepresentationObject ?? throw new InvalidOperationException("Imported Rhino representation has no representation objects.");
    }

    public static RhinoRepresentationObjectData ImportRhinoRepresentationObjectDataFromSemioFile(File file)
    {
        if (file is null)
            throw new ArgumentNullException(nameof(file));
        if (string.IsNullOrWhiteSpace(file.Blob))
            throw new ArgumentException("Semio file blob cannot be empty.", nameof(file));

        var importedContext = ImportRhinoRepresentationContextFromSemioFile(file);
        return ConvertRhinoRepresentationContextToRhinoRepresentationObjectData(importedContext, file.Blob);
    }

    public static Rhino.DocObjects.RhinoObject ImportRhinoDocumentObjectFromSemioFile(File file, SemioRepresentation? representation = null)
    {
        return ImportRhinoDocumentObjectsFromSemioFile(file, representation).First();
    }

    public static List<Rhino.DocObjects.RhinoObject> ImportRhinoDocumentObjectsFromSemioFile(File file, SemioRepresentation? representation = null)
    {
        if (file is null)
            throw new ArgumentNullException(nameof(file));
        if (string.IsNullOrWhiteSpace(file.Blob))
            throw new ArgumentException("Semio file blob cannot be empty.", nameof(file));

        var sourceName = !string.IsNullOrWhiteSpace(file.Name) ? file.Name : file.Remote;
        var targetUnitSystem = ResolveRepresentationUnitSystem(representation);
        var importedContext = ImportRhinoRepresentationContextFromSemioFile(file);
        var sourceRepresentationObjects = importedContext.Representation.Objects
            .Where(sourceRepresentationObject => sourceRepresentationObject?.Geometry is not null)
            .ToList();
        if (sourceRepresentationObjects.Count > 0)
        {
            var sourceUnitSystem = importedContext.Representation.Settings.ModelUnitSystem;
            return AddFile3dmObjectsToIsolatedDocument(sourceRepresentationObjects!, file.Blob, sourceUnitSystem, targetUnitSystem);
        }

        return ImportRhinoDocumentObjectsFromBlobFallback(file.Blob, sourceName, targetUnitSystem);
    }

    public static RhinoRepresentationObject ImportRhinoRepresentationContextFromBlob(string fileBlob, string? sourceName = null)
    {
        var fileBytes = DecodeFileBlobString(fileBlob);
        var file3dm = Rhino.FileIO.File3dm.FromByteArray(fileBytes);
        if (file3dm is null)
        {
            var extension = ResolveImportExtension(sourceName, fileBlob, fileBytes);
            file3dm = ImportRhinoRepresentationViaHeadlessDocument(fileBytes, extension);
        }

        var representationObject = file3dm.Objects.FirstOrDefault();
        AttachSemioImportMetadata(representationObject, fileBlob);
        return new RhinoRepresentationObject(file3dm, representationObject);
    }

    public static RhinoRepresentationObject ImportRhinoRepresentationContextFromSemioFile(File file)
    {
        if (file is null)
            throw new ArgumentNullException(nameof(file));
        if (string.IsNullOrWhiteSpace(file.Blob))
            throw new ArgumentException("Semio file blob cannot be empty.", nameof(file));
        var sourceName = !string.IsNullOrWhiteSpace(file.Name) ? file.Name : file.Remote;
        return ImportRhinoRepresentationContextFromBlob(file.Blob, sourceName);
    }

    public static void AttachSemioImportMetadata(Rhino.FileIO.File3dmObject? representationObject, string fileBlob)
    {
        if (representationObject?.Attributes is null || string.IsNullOrWhiteSpace(fileBlob))
            return;

        representationObject.Attributes.SetUserString(SemioImportRepresentationBlobKey, fileBlob);
        representationObject.Attributes.SetUserString(SemioImportRepresentationObjectIdKey, ResolveRhinoObjectId(representationObject, -1));
    }

    public static RhinoRepresentationObjectData ConvertRhinoRepresentationContextToRhinoRepresentationObjectData(RhinoRepresentationObject rhinoRepresentationObject, string fileBlob)
    {
        if (rhinoRepresentationObject is null)
            throw new ArgumentNullException(nameof(rhinoRepresentationObject));
        if (string.IsNullOrWhiteSpace(fileBlob))
            throw new ArgumentException("Semio file blob cannot be empty.", nameof(fileBlob));

        var sourceRepresentationObject = rhinoRepresentationObject.RepresentationObject ?? rhinoRepresentationObject.Representation.Objects.FirstOrDefault();
        var attributes = new RhinoRepresentationObjectData.Attributes();
        var metadata = new[]
        {
            new KeyValuePair<string, string>(SemioImportRepresentationBlobKey, fileBlob),
            new KeyValuePair<string, string>(
                SemioImportRepresentationObjectIdKey,
                sourceRepresentationObject is null
                    ? string.Empty
                    : ResolveRhinoObjectId(sourceRepresentationObject, -1))
        };
        attributes.UserText = new RepresentationUserText(metadata);

        if (sourceRepresentationObject?.Geometry is null)
            return new RhinoRepresentationObjectData(attributes);

        var objectAttributes = sourceRepresentationObject.Attributes?.Duplicate() ?? new Rhino.DocObjects.ObjectAttributes();
        objectAttributes.SetUserString(SemioImportRepresentationBlobKey, fileBlob);
        objectAttributes.SetUserString(
            SemioImportRepresentationObjectIdKey,
            ResolveRhinoObjectId(sourceRepresentationObject, -1));

        var objectGeometry = sourceRepresentationObject.Geometry.Duplicate();
        var addedRhinoObject = AddRhinoObjectToTargetDocument(objectGeometry, objectAttributes);
        if (addedRhinoObject is not null)
            return new RhinoRepresentationObjectData(addedRhinoObject);

        var directRepresentationObject = new RhinoRepresentationObjectData(new RhinoRepresentationObjectData.Attributes { UserText = new RepresentationUserText(metadata) });
        if (directRepresentationObject.IsValid)
            return directRepresentationObject;

        return new RhinoRepresentationObjectData(attributes);
    }

    private static string ResolveImportExtension(string? sourceName, string fileBlob, byte[] fileBytes)
    {
        if (!string.IsNullOrWhiteSpace(sourceName))
        {
            var pathExtension = Path.GetExtension(sourceName);
            if (!string.IsNullOrWhiteSpace(pathExtension))
                return pathExtension;
        }

        if (fileBlob.StartsWith("data:", StringComparison.OrdinalIgnoreCase))
        {
            var mimeStart = "data:".Length;
            var mimeEnd = fileBlob.IndexOf(';');
            if (mimeEnd > mimeStart)
            {
                var mime = fileBlob.Substring(mimeStart, mimeEnd - mimeStart).Trim().ToLowerInvariant();
                if (mime == "representation/gltf-binary")
                    return ".glb";
                if (mime == "representation/gltf+json")
                    return ".gltf";
                if (mime == "representation/3dm" || mime == "application/vnd.rhino")
                    return ".3dm";
            }
        }

        if (fileBytes.Length >= 4 &&
            fileBytes[0] == (byte)'g' &&
            fileBytes[1] == (byte)'l' &&
            fileBytes[2] == (byte)'T' &&
            fileBytes[3] == (byte)'F')
            return ".glb";

        return ".3dm";
    }

    private static Rhino.FileIO.File3dm ImportRhinoRepresentationViaHeadlessDocument(byte[] fileBytes, string extension)
    {
        var normalizedExtension = extension.StartsWith(".") ? extension : "." + extension;
        var tempDirectory = Path.Combine(Path.GetTempPath(), "semio-gh-import");
        Directory.CreateDirectory(tempDirectory);

        var importPath = Path.Combine(tempDirectory, $"{__ID_NEWID__():N}{normalizedExtension}");
        var exportPath = Path.Combine(tempDirectory, $"{__ID_NEWID__():N}.3dm");
        try
        {
            System.IO.File.WriteAllBytes(importPath, fileBytes);

            using var document = RhinoDoc.CreateHeadless(null);
            if (!document.Import(importPath))
                throw new InvalidOperationException($"Could not import Rhino representation using RhinoDoc.Import for extension {normalizedExtension}.");
            if (!document.SaveAs(exportPath))
                throw new InvalidOperationException("Could not export imported Rhino representation to temporary 3dm.");

            return Rhino.FileIO.File3dm.Read(exportPath)
                ?? throw new InvalidOperationException("Could not read temporary 3dm after Rhino import.");
        }
        finally
        {
            TryDeletePath(importPath);
            TryDeletePath(exportPath);
        }
    }

    private static void TryDeletePath(string path)
    {
        try
        {
            if (System.IO.File.Exists(path))
                System.IO.File.Delete(path);
        }
        catch
        {
            // Best-effort cleanup for temp import artifacts.
        }
    }

    private static List<Rhino.DocObjects.RhinoObject> ImportRhinoDocumentObjectsFromBlobFallback(
        string fileBlob,
        string? sourceName,
        UnitSystem? targetUnitSystem)
    {
        var fileBytes = DecodeFileBlobString(fileBlob);
        var extension = ResolveImportExtension(sourceName, fileBlob, fileBytes);
        var normalizedExtension = extension.StartsWith(".") ? extension : "." + extension;
        var tempDirectory = Path.Combine(Path.GetTempPath(), "semio-gh-import");
        Directory.CreateDirectory(tempDirectory);

        var importPath = Path.Combine(tempDirectory, $"{__ID_NEWID__():N}{normalizedExtension}");
        var targetDocument = CreateTrackedHeadlessImportDocument();
        if (targetUnitSystem.HasValue)
            targetDocument.ModelUnitSystem = targetUnitSystem.Value;
        var existingObjectIds = targetDocument.Objects
            .Select(rhinoObject => rhinoObject.Id)
            .ToHashSet();

        try
        {
            System.IO.File.WriteAllBytes(importPath, fileBytes);
            if (!targetDocument.Import(importPath))
                throw new InvalidOperationException($"Could not import Rhino representation using RhinoDoc.Import for extension {normalizedExtension}.");

            var importedRhinoObjects = targetDocument.Objects
                .Where(rhinoObject => !existingObjectIds.Contains(rhinoObject.Id) && rhinoObject.Geometry is not null)
                .ToList();
            if (importedRhinoObjects.Count == 0)
                throw new InvalidOperationException("Imported Rhino representation has no representation objects.");

            foreach (var importedRhinoObject in importedRhinoObjects)
                AttachSemioImportMetadata(importedRhinoObject, fileBlob, importedRhinoObject.Id.ToString());
            return importedRhinoObjects;
        }
        finally
        {
            TryDeletePath(importPath);
        }
    }

    private static List<Rhino.DocObjects.RhinoObject> AddFile3dmObjectsToIsolatedDocument(
        IReadOnlyList<Rhino.FileIO.File3dmObject> sourceRepresentationObjects,
        string fileBlob,
        UnitSystem sourceUnitSystem,
        UnitSystem? targetUnitSystem)
    {
        var targetDocument = CreateTrackedHeadlessImportDocument();
        if (targetUnitSystem.HasValue)
            targetDocument.ModelUnitSystem = targetUnitSystem.Value;

        var requiresScaling = targetUnitSystem.HasValue && targetUnitSystem.Value != sourceUnitSystem;
        var unitScale = requiresScaling
            ? RhinoMath.UnitScale(sourceUnitSystem, targetUnitSystem!.Value)
            : 1.0;
        var importedRhinoObjects = new List<Rhino.DocObjects.RhinoObject>();
        for (var sourceIndex = 0; sourceIndex < sourceRepresentationObjects.Count; sourceIndex++)
        {
            var sourceRepresentationObject = sourceRepresentationObjects[sourceIndex];
            if (sourceRepresentationObject?.Geometry is null)
                continue;

            var objectAttributes = sourceRepresentationObject.Attributes?.Duplicate() ?? new Rhino.DocObjects.ObjectAttributes();
            var objectGeometry = sourceRepresentationObject.Geometry.Duplicate();
            if (requiresScaling && !RhinoMath.EpsilonEquals(unitScale, 1.0, RhinoMath.ZeroTolerance))
                objectGeometry.Transform(Transform.Scale(Point3d.Origin, unitScale));
            var addedObjectId = targetDocument.Objects.Add(objectGeometry, objectAttributes);
            if (addedObjectId == __ID_EMPTY__)
                continue;

            var importedRhinoObject = targetDocument.Objects.FindId(addedObjectId);
            if (importedRhinoObject is null)
                continue;

            AttachSemioImportMetadata(importedRhinoObject, fileBlob, ResolveRhinoObjectId(sourceRepresentationObject, sourceIndex));
            importedRhinoObjects.Add(importedRhinoObject);
        }

        if (importedRhinoObjects.Count == 0)
            throw new InvalidOperationException("Imported Rhino representation has no representation objects.");
        return importedRhinoObjects;
    }

    private static UnitSystem? ResolveRepresentationUnitSystem(SemioRepresentation? representation)
    {
        if (representation is null)
            return null;

        var representationUnitAttributeValue = representation.Attributes?
            .FirstOrDefault(attribute => string.Equals(attribute?.Key, "Unit", StringComparison.OrdinalIgnoreCase))
            ?.Value;
        if (string.IsNullOrWhiteSpace(representationUnitAttributeValue))
            return null;
        if (!IsValidLengthUnitSystem(representationUnitAttributeValue))
            return null;

        return LengthUnitAbbreviationToUnitSystem(representationUnitAttributeValue);
    }

    private static Rhino.DocObjects.RhinoObject AddRhinoObjectToTargetDocument(
        Rhino.Geometry.GeometryBase objectGeometry,
        Rhino.DocObjects.ObjectAttributes objectAttributes)
    {
        var targetDocument = CreateTrackedHeadlessImportDocument();
        var addedObjectId = targetDocument.Objects.Add(objectGeometry, objectAttributes);
        if (addedObjectId == __ID_EMPTY__)
            throw new InvalidOperationException("Could not add imported Rhino representation object to target document.");

        return targetDocument.Objects.FindId(addedObjectId)
            ?? throw new InvalidOperationException("Could not resolve imported Rhino representation object in target document.");
    }

    private static void AttachSemioImportMetadata(
        Rhino.DocObjects.RhinoObject rhinoObject,
        string fileBlob,
        string importedObjectId)
    {
        if (rhinoObject is null || string.IsNullOrWhiteSpace(fileBlob))
            return;

        var objectAttributes = rhinoObject.Attributes.Duplicate();
        objectAttributes.SetUserString(SemioImportRepresentationBlobKey, fileBlob);
        objectAttributes.SetUserString(SemioImportRepresentationObjectIdKey, importedObjectId);
        var targetDocument = rhinoObject.Document;
        if (targetDocument is not null)
            targetDocument.Objects.ModifyAttributes(rhinoObject, objectAttributes, true);
    }

    public static bool TryResolveRhinoRepresentationContext(object representationObjectInput, out RhinoRepresentationObject rhinoRepresentationObject)
    {
        rhinoRepresentationObject = null!;
        if (representationObjectInput is RhinoRepresentationObject existingContext)
        {
            rhinoRepresentationObject = existingContext;
            return true;
        }

        if (representationObjectInput is RhinoRepresentationObjectData representationObjectData &&
            TryResolveSemioImportMetadata(representationObjectData, out var representationBlob, out var representationObjectId))
        {
            return TryResolveRhinoRepresentationContextFromMetadata(representationBlob, representationObjectId, out rhinoRepresentationObject);
        }

        if (representationObjectInput is IGH_Goo goo)
            representationObjectInput = goo.ScriptVariable();

        if (representationObjectInput is RhinoRepresentationObjectData scriptedRepresentationObjectData &&
            TryResolveSemioImportMetadata(scriptedRepresentationObjectData, out var scriptedBlob, out var scriptedRepresentationObjectId))
        {
            return TryResolveRhinoRepresentationContextFromMetadata(scriptedBlob, scriptedRepresentationObjectId, out rhinoRepresentationObject);
        }

        if (representationObjectInput is not Rhino.FileIO.File3dmObject importedRepresentationObject)
        {
            if (representationObjectInput is Rhino.DocObjects.RhinoObject rhinoObject)
            {
                var rhinoFileBlob = rhinoObject.Attributes?.GetUserString(SemioImportRepresentationBlobKey);
                if (string.IsNullOrWhiteSpace(rhinoFileBlob))
                    return false;

                var rhinoObjectId = rhinoObject.Attributes?.GetUserString(SemioImportRepresentationObjectIdKey);
                return TryResolveRhinoRepresentationContextFromMetadata(rhinoFileBlob, rhinoObjectId, out rhinoRepresentationObject);
            }
            return false;
        }

        var fileBlob = importedRepresentationObject.Attributes?.GetUserString(SemioImportRepresentationBlobKey);
        if (string.IsNullOrWhiteSpace(fileBlob))
            return false;

        var importedContext = ImportRhinoRepresentationContextFromBlob(fileBlob);
        var importedObjectId = importedRepresentationObject.Attributes?.GetUserString(SemioImportRepresentationObjectIdKey);
        var matchingRepresentationObject = ResolveRepresentationObjectByImportedId(importedContext.Representation, importedObjectId);
        rhinoRepresentationObject = matchingRepresentationObject is null
            ? importedContext
            : new RhinoRepresentationObject(importedContext.Representation, matchingRepresentationObject);
        return true;
    }

    private static bool TryResolveSemioImportMetadata(
        RhinoRepresentationObjectData representationObjectData,
        out string fileBlob,
        out string? importedObjectId)
    {
        fileBlob = string.Empty;
        importedObjectId = null;

        var userText = representationObjectData.UserText;
        if (!userText.TryGetValue(SemioImportRepresentationBlobKey, out fileBlob) || string.IsNullOrWhiteSpace(fileBlob))
            return false;
        userText.TryGetValue(SemioImportRepresentationObjectIdKey, out importedObjectId);
        return true;
    }

    private static bool TryResolveRhinoRepresentationContextFromMetadata(
        string fileBlob,
        string? importedObjectId,
        out RhinoRepresentationObject rhinoRepresentationObject)
    {
        rhinoRepresentationObject = null!;
        if (string.IsNullOrWhiteSpace(fileBlob))
            return false;

        var importedContext = ImportRhinoRepresentationContextFromBlob(fileBlob);
        var matchingRepresentationObject = ResolveRepresentationObjectByImportedId(importedContext.Representation, importedObjectId);
        rhinoRepresentationObject = matchingRepresentationObject is null
            ? importedContext
            : new RhinoRepresentationObject(importedContext.Representation, matchingRepresentationObject);
        return true;
    }

    //#region 🎢ImportedRhinoObjectResolution
    /// <summary>
    /// Resolves a single imported Rhino representation object by metadata identifier.
    ///
    /// Specs:
    /// Tries native object IDs first, then deterministic fallback IDs ("rhino-object-{index}") used by import metadata.
    /// Returns null when no matching source representation object can be found.
    /// </summary>
    private static Rhino.FileIO.File3dmObject? ResolveRepresentationObjectByImportedId(Rhino.FileIO.File3dm representation, string? importedObjectId)
    {
        var nonNullRepresentationObjects = representation.Objects.Where(representationObject => representationObject is not null).ToList();
        if (nonNullRepresentationObjects.Count == 0)
            return null;

        if (string.IsNullOrWhiteSpace(importedObjectId))
            return nonNullRepresentationObjects.FirstOrDefault();

        var objectByNativeId = nonNullRepresentationObjects
            .FirstOrDefault(representationObject => ResolveRhinoObjectId(representationObject, -1) == importedObjectId);
        if (objectByNativeId is not null)
            return objectByNativeId;

        const string fallbackObjectIdPrefix = "rhino-object-";
        if (importedObjectId.StartsWith(fallbackObjectIdPrefix, StringComparison.OrdinalIgnoreCase) &&
            int.TryParse(importedObjectId.Substring(fallbackObjectIdPrefix.Length), out var objectIndex) &&
            objectIndex >= 0 &&
            objectIndex < nonNullRepresentationObjects.Count)
        {
            return nonNullRepresentationObjects[objectIndex];
        }

        return null;
    }
    //#endregion 🎢ImportedRhinoObjectResolution

    public static List<Attribute> ToAttributesList(AttributesDiff? attributesDiff)
    {
        if (attributesDiff is null)
            return new List<Attribute>();

        var attributes = new List<Attribute>();
        if (attributesDiff.Added is not null)
            attributes.AddRange(attributesDiff.Added.Where(attribute => attribute is not null).Select(attribute => attribute.DeepClone()));
        if (attributesDiff.Modified is not null)
        {
            foreach (var update in attributesDiff.Modified.Where(update => update is not null))
            {
                var sourceAttribute = update.Attribute is null ? new Attribute() : ((Attribute)update.Attribute).DeepClone();
                attributes.Add(update.Diff is null ? sourceAttribute : Attribute.ApplyDiff(sourceAttribute, update.Diff.DeepClone()));
            }
        }
        return attributes;
    }

    public static SemioGroup TranslateRhinoRepresentationObjectToSingleGroup(RhinoRepresentationObject rhinoRepresentationObject)
    {
        if (rhinoRepresentationObject is null)
            throw new ArgumentNullException(nameof(rhinoRepresentationObject));

        var representation = rhinoRepresentationObject.Representation;
        var importedObject = rhinoRepresentationObject.RepresentationObject;

        var layersById = new Dictionary<Guid, Rhino.DocObjects.Layer>();
        foreach (var layer in representation.Layers)
        {
            if (layer is not null && !layer.IsDeleted)
                layersById[layer.Id] = layer;
        }

        var layerChildren = new Dictionary<Guid, List<Rhino.DocObjects.Layer>>();
        foreach (var layer in layersById.Values)
        {
            var parentLayerId = layer.ParentLayerId;
            if (parentLayerId != __ID_EMPTY__ && !layersById.ContainsKey(parentLayerId))
                parentLayerId = __ID_EMPTY__;

            if (!layerChildren.TryGetValue(parentLayerId, out var children))
            {
                children = new List<Rhino.DocObjects.Layer>();
                layerChildren[parentLayerId] = children;
            }
            children.Add(layer);
        }

        foreach (var children in layerChildren.Values)
            children.Sort((left, right) => string.Compare(left.Name, right.Name, StringComparison.OrdinalIgnoreCase));

        var objectIdsByLayerId = new Dictionary<Guid, List<string>>();
        var allPieceIds = new List<string>();
        var objectCounter = 0;
        foreach (var fileObject in representation.Objects)
        {
            var objectId = ResolveRhinoObjectId(fileObject, objectCounter++);
            allPieceIds.Add(objectId);
            var layerId = ResolveRhinoLayerId(representation, fileObject);
            if (!objectIdsByLayerId.TryGetValue(layerId, out var objectIds))
            {
                objectIds = new List<string>();
                objectIdsByLayerId[layerId] = objectIds;
            }
            objectIds.Add(objectId);
        }

        var attributes = new List<Attribute>();

        List<string> BuildLayerGroupAttributes(Guid parentLayerId, string parentPath)
        {
            var recursiveIds = new List<string>();
            if (!layerChildren.TryGetValue(parentLayerId, out var children))
                return recursiveIds;

            foreach (var layer in children)
            {
                var layerPath = string.IsNullOrWhiteSpace(parentPath) ? layer.Name : $"{parentPath}/{layer.Name}";
                var layerIds = new List<string>();
                if (objectIdsByLayerId.TryGetValue(layer.Id, out var directIds))
                    layerIds.AddRange(directIds);

                var nestedIds = BuildLayerGroupAttributes(layer.Id, layerPath);
                layerIds.AddRange(nestedIds);
                recursiveIds.AddRange(layerIds);

                attributes.Add(new Attribute
                {
                    Id = __ID_NEWID__().ToString(),
                    Key = $"LayerGroup/{layerPath}",
                    Value = string.Join(",", layerIds.Distinct()),
                    Definition = "Recursive named layer group from imported Rhino representation."
                });
            }

            return recursiveIds;
        }

        var rootLayerIds = BuildLayerGroupAttributes(__ID_EMPTY__, "");
        attributes.Add(new Attribute
        {
            Id = __ID_NEWID__().ToString(),
            Key = "LayerGroup",
            Value = string.Join(",", rootLayerIds.Distinct()),
            Definition = "Root named layer group containing all recursive layers."
        });

        return new SemioGroup
        {
            Id = importedObject is null ? __ID_NEWID__().ToString() : ResolveRhinoObjectId(importedObject, -1),
            Name = "Imported Rhino Layer Group",
            Description = "Single semio group translated from Rhino representation object with recursive named layer groups.",
            Pieces = allPieceIds.Distinct().Select(pieceId => new PieceId { Id = pieceId }).ToList(),
            Attributes = attributes
        };
    }

    public static SemioGroup TranslateRhinoRepresentationObjectsToSingleGroup(IEnumerable<RhinoRepresentationObject> rhinoRepresentationObjects)
    {
        if (rhinoRepresentationObjects is null)
            throw new ArgumentNullException(nameof(rhinoRepresentationObjects));

        var translatedGroups = rhinoRepresentationObjects
            .Where(representationObject => representationObject is not null)
            .Select(TranslateRhinoRepresentationObjectToSingleGroup)
            .ToList();
        if (translatedGroups.Count == 0)
            throw new InvalidOperationException("Input must contain at least one Rhino RepresentationObject output of Import Representation.");

        var pieceIds = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        var attributeValuesByKey = new Dictionary<string, HashSet<string>>(StringComparer.OrdinalIgnoreCase);
        foreach (var group in translatedGroups)
        {
            foreach (var piece in group.Pieces ?? new List<PieceId>())
            {
                if (!string.IsNullOrWhiteSpace(piece?.Id))
                    pieceIds.Add(piece.Id);
            }

            foreach (var attribute in group.Attributes ?? new List<Attribute>())
            {
                if (attribute is null || string.IsNullOrWhiteSpace(attribute.Key))
                    continue;

                if (!attributeValuesByKey.TryGetValue(attribute.Key, out var attributeValues))
                {
                    attributeValues = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
                    attributeValuesByKey[attribute.Key] = attributeValues;
                }

                foreach (var valuePart in (attribute.Value ?? string.Empty).Split(','))
                {
                    var trimmedValuePart = valuePart.Trim();
                    if (!string.IsNullOrWhiteSpace(trimmedValuePart))
                        attributeValues.Add(trimmedValuePart);
                }
            }
        }

        var mergedAttributes = attributeValuesByKey
            .OrderBy(entry => entry.Key, StringComparer.OrdinalIgnoreCase)
            .Select(entry => new Attribute
            {
                Id = __ID_NEWID__().ToString(),
                Key = entry.Key,
                Value = string.Join(",", entry.Value.OrderBy(value => value, StringComparer.OrdinalIgnoreCase)),
                Definition = entry.Key == "LayerGroup"
                    ? "Root named layer group containing all recursive layers."
                    : "Recursive named layer group from imported Rhino representation."
            })
            .ToList();

        return new SemioGroup
        {
            Id = translatedGroups.First().Id,
            Name = "Imported Rhino Layer Group",
            Description = "Single semio group translated from Rhino representation object with recursive named layer groups.",
            Pieces = pieceIds.OrderBy(pieceId => pieceId, StringComparer.OrdinalIgnoreCase)
                .Select(pieceId => new PieceId { Id = pieceId })
                .ToList(),
            Attributes = mergedAttributes
        };
    }

    private static Guid ResolveRhinoLayerId(Rhino.FileIO.File3dm representation, Rhino.FileIO.File3dmObject fileObject)
    {
        var layerIndex = fileObject.Attributes?.LayerIndex ?? -1;
        if (layerIndex < 0 || layerIndex >= representation.Layers.Count)
            return __ID_EMPTY__;
        var layer = representation.Layers[layerIndex];
        return layer?.Id ?? __ID_EMPTY__;
    }

    private static string ResolveRhinoObjectId(Rhino.FileIO.File3dmObject fileObject, int fallbackIndex)
    {
        var objectId = fileObject.Attributes?.ObjectId ?? __ID_EMPTY__;
        if (objectId != __ID_EMPTY__)
            return objectId.ToString();

        return fallbackIndex >= 0
            ? $"rhino-object-{fallbackIndex}"
            : __ID_NEWID__().ToString();
    }

    public static IEnumerable<RhinoRepresentationObjectData> ExtractRhinoRepresentationObjectDataFromGeometryGroup(GH_GeometryGroup group)
    {
        if (group is null)
            throw new ArgumentNullException(nameof(group));

        return ExtractRhinoRepresentationObjectDataFromGeometricGoo(group);
    }

    private static IEnumerable<RhinoRepresentationObjectData> ExtractRhinoRepresentationObjectDataFromGeometricGoo(IGH_GeometricGoo goo)
    {
        if (goo is GH_GeometryGroup subGroup)
        {
            foreach (var obj in subGroup.Objects)
                foreach (var data in ExtractRhinoRepresentationObjectDataFromGeometricGoo(obj))
                    yield return data;
        }
        else if (goo is not null)
        {
            var scriptVariable = goo.ScriptVariable();
            var geometry = scriptVariable as Rhino.Geometry.GeometryBase;
            if (geometry is null && scriptVariable is Rhino.Geometry.Point3d point3d)
                geometry = new Rhino.Geometry.Point(point3d);
            if (geometry is null)
                yield break;
            var rhinoObject = AddRhinoObjectToTargetDocument(geometry.Duplicate(), new Rhino.DocObjects.ObjectAttributes());
            yield return new RhinoRepresentationObjectData(rhinoObject);
        }
    }
}

#endregion 🎇Utility

#region 🧰RhinoRepresentationData
// Callers MUST use this wrapper to preserve imported Rhino representation metadata in Grasshopper.

public sealed class RepresentationUserText : Dictionary<string, string>
{
    public RepresentationUserText()
        : base(StringComparer.Ordinal)
    {
    }

    public RepresentationUserText(IEnumerable<KeyValuePair<string, string>> entries)
        : this()
    {
        foreach (var entry in entries)
        {
            if (!string.IsNullOrWhiteSpace(entry.Key) && entry.Value is not null)
                this[entry.Key] = entry.Value;
        }
    }
}

public sealed class RhinoRepresentationObjectData
{
    public sealed class Attributes
    {
        public RepresentationUserText UserText { get; set; } = new();
    }

    public RhinoRepresentationObjectData(Attributes attributes)
    {
        Metadata = attributes ?? new Attributes();
    }

    public RhinoRepresentationObjectData(Rhino.FileIO.File3dmObject? rhinoObject)
    {
        RhinoObject = rhinoObject;
        Metadata = new Attributes();

        var representationBlob = rhinoObject?.Attributes?.GetUserString(Utility.SemioImportRepresentationBlobKey);
        if (!string.IsNullOrWhiteSpace(representationBlob))
            Metadata.UserText[Utility.SemioImportRepresentationBlobKey] = representationBlob;

        var representationObjectId = rhinoObject?.Attributes?.GetUserString(Utility.SemioImportRepresentationObjectIdKey);
        if (!string.IsNullOrWhiteSpace(representationObjectId))
            Metadata.UserText[Utility.SemioImportRepresentationObjectIdKey] = representationObjectId;
    }

    public RhinoRepresentationObjectData(Rhino.DocObjects.RhinoObject? rhinoObject)
    {
        SourceRhinoObject = rhinoObject;
        Metadata = new Attributes();

        var representationBlob = rhinoObject?.Attributes?.GetUserString(Utility.SemioImportRepresentationBlobKey);
        if (!string.IsNullOrWhiteSpace(representationBlob))
            Metadata.UserText[Utility.SemioImportRepresentationBlobKey] = representationBlob;

        var representationObjectId = rhinoObject?.Attributes?.GetUserString(Utility.SemioImportRepresentationObjectIdKey);
        if (!string.IsNullOrWhiteSpace(representationObjectId))
            Metadata.UserText[Utility.SemioImportRepresentationObjectIdKey] = representationObjectId;
    }

    public Attributes Metadata { get; }
    public Rhino.FileIO.File3dmObject? RhinoObject { get; }
    public Rhino.DocObjects.RhinoObject? SourceRhinoObject { get; }
    public RepresentationUserText UserText => Metadata.UserText;
    public bool IsValid => RhinoObject is not null || SourceRhinoObject is not null || UserText.Count > 0;
}

#endregion 🧰RhinoRepresentationData

#region 💧Converters
// Implementations MUST convert between semio and Grasshopper data types.

public static class RhinoConverter
{
    public static object Convert(this object value) => value;
    public static string Convert(this string value) => value;
    public static int Convert(this int value) => value;
    public static float Convert(this double value) => (float)value;
    public static Point3d Convert(this Point point) => new Point3d(point.X, point.Y, point.Z);
    public static Point Convert(this Point3d point) => new Point { X = (float)point.X, Y = (float)point.Y, Z = (float)point.Z };
    public static Vector3d Convert(this Vector vector) => new Vector3d(vector.X, vector.Y, vector.Z);
    public static Vector Convert(this Vector3d vector) => new Vector { X = (float)vector.X, Y = (float)vector.Y, Z = (float)vector.Z };
    public static Rhino.Geometry.Plane Convert(this Plane plane) => new(
        new Point3d(plane.Origin.X, plane.Origin.Y, plane.Origin.Z),
        new Vector3d(plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z),
        new Vector3d(plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z));
    public static Plane Convert(this Rhino.Geometry.Plane plane) => new()
    {
        Origin = new Point { X = (float)plane.OriginX, Y = (float)plane.OriginY, Z = (float)plane.OriginZ },
        XAxis = new Vector { X = (float)plane.XAxis.X, Y = (float)plane.XAxis.Y, Z = (float)plane.XAxis.Z },
        YAxis = new Vector { X = (float)plane.YAxis.X, Y = (float)plane.YAxis.Y, Z = (float)plane.YAxis.Z }
    };
    public static Color HexToColor(string hex)
    {
        if (string.IsNullOrEmpty(hex)) return Color.Transparent;
        hex = hex.TrimStart('#');
        if (hex.Length == 6)
            return Color.FromArgb(
                System.Convert.ToInt32(hex.Substring(0, 2), 16),
                System.Convert.ToInt32(hex.Substring(2, 2), 16),
                System.Convert.ToInt32(hex.Substring(4, 2), 16));
        if (hex.Length == 8)
            return Color.FromArgb(
                System.Convert.ToInt32(hex.Substring(0, 2), 16),
                System.Convert.ToInt32(hex.Substring(2, 2), 16),
                System.Convert.ToInt32(hex.Substring(4, 2), 16),
                System.Convert.ToInt32(hex.Substring(6, 2), 16));
        return Color.Transparent;
    }
    public static string ColorToHex(Color color)
    {
        if (color.A == 255)
            return $"#{color.R:X2}{color.G:X2}{color.B:X2}";
        return $"#{color.A:X2}{color.R:X2}{color.G:X2}{color.B:X2}";
    }
}

#endregion 💧Converters

#region 🔓Bases
// Implementations MUST extend these abstract base classes for Goo, Param, and Component.

/// Generic Grasshopper data wrapper for semio entity types.
/// Implementations MUST override CastFrom and CastTo for type conversion.
public abstract class Goo<TEntity> : GH_Goo<TEntity> where TEntity : Entity<TEntity>, new()
{
    public Goo() { Value = new TEntity(); }
    public Goo(TEntity value) { Value = value; }
    public override bool IsValid => true;
    public override string TypeName => typeof(TEntity).Name;
    public override string TypeDescription => typeof(TEntity).Name;
    public override IGH_Goo Duplicate()
    {
        var duplicate = (Goo<TEntity>)(Activator.CreateInstance(GetType()) ?? throw new InvalidOperationException($"Could not create instance of {GetType()}"));
        duplicate.Value = Value.DeepClone() ?? throw new InvalidOperationException($"Could not clone {typeof(TEntity).Name}");
        return duplicate;
    }
    public override string ToString() => Value.ToString();
    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(TEntity).Name, Value.Serialize());
        return base.Write(writer);
    }
    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString(typeof(TEntity).Name).Deserialize<TEntity>() ?? throw new InvalidOperationException($"Could not deserialize {typeof(TEntity).Name}");
        return base.Read(reader);
    }
    internal virtual bool CustomCastTo<Q>(ref Q target) => false;
    internal virtual bool CustomCastFrom(object source) => false;
    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TEntity)))
        {
            target = (Q)(object)this;
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source is null) return false;
        if (source is TEntity representation)
        {
            Value = representation;
            return true;
        }
        return CustomCastFrom(source);
    }
}

internal static class GhNaming
{
    private static readonly Dictionary<string, string> dictionaryCodes = new(StringComparer.OrdinalIgnoreCase)
    {
        ["attribute"] = "At",
        ["attributes"] = "At",
        ["author"] = "Au",
        ["authors"] = "Au",
        ["benchmark"] = "Jc",
        ["benchmarks"] = "Jc",
        ["blob"] = "Bl",
        ["cache"] = "Ca",
        ["center"] = "Ce",
        ["child"] = "CNa",
        ["children"] = "CNa",
        ["collider"] = "Cl",
        ["color"] = "Cl",
        ["colour"] = "Cl",
        ["concept"] = "Ce",
        ["connection"] = "Co",
        ["connections"] = "Co",
        ["connector"] = "Po",
        ["connectors"] = "Po",
        ["createdat"] = "CA",
        ["createdby"] = "Au",
        ["default"] = "De",
        ["definition"] = "Df",
        ["definitions"] = "Df",
        ["description"] = "Dc",
        ["design"] = "Dn",
        ["designs"] = "Dn",
        ["diagram"] = "Dg",
        ["directory"] = "Di",
        ["direction"] = "Dr",
        ["email"] = "Em",
        ["file"] = "Ca",
        ["files"] = "Ca",
        ["folder"] = "Di",
        ["folders"] = "Di",
        ["formula"] = "Fx",
        ["geometry"] = "Ge",
        ["id"] = "Id",
        ["image"] = "Im",
        ["imperial"] = "Ip",
        ["indent"] = "In",
        ["input"] = "In",
        ["key"] = "FK",
        ["kit"] = "Kt",
        ["latitude"] = "Y",
        ["layer"] = "LN",
        ["layers"] = "LN",
        ["length"] = "Ln",
        ["location"] = "Og",
        ["longitude"] = "X",
        ["mandatory"] = "CD",
        ["max"] = "Ma",
        ["min"] = "Mi",
        ["representation"] = "Rp",
        ["representations"] = "Rp",
        ["name"] = "Na",
        ["object"] = "Ob",
        ["objects"] = "Ob",
        ["output"] = "Ou",
        ["parent"] = "Pa",
        ["path"] = "Ph",
        ["piece"] = "Pc",
        ["pieces"] = "Pc",
        ["plane"] = "Pn",
        ["point"] = "Pt",
        ["port"] = "Po",
        ["ports"] = "Po",
        ["preview"] = "Pv",
        ["prop"] = "Pp",
        ["props"] = "Pp",
        ["quality"] = "Jc",
        ["remote"] = "Rm",
        ["replace"] = "Re",
        ["rotation"] = "Rt",
        ["run"] = "Ru",
        ["scale"] = "Sc",
        ["shift"] = "Sf",
        ["side"] = "Sd",
        ["size"] = "Pl",
        ["slot"] = "Sl",
        ["slots"] = "Sl",
        ["source"] = "SD",
        ["success"] = "Su",
        ["tag"] = "Tg",
        ["tags"] = "Tg",
        ["target"] = "TD",
        ["text"] = "Tx",
        ["tilt"] = "Tl",
        ["transform"] = "Tr",
        ["type"] = "Ty",
        ["types"] = "Ty",
        ["unit"] = "Ut",
        ["updatedat"] = "Up",
        ["updatedby"] = "Up",
        ["uri"] = "Ui",
        ["url"] = "Ur",
        ["validate"] = "Vd",
        ["value"] = "Vl",
        ["variant"] = "Vn",
        ["vector"] = "Vc",
        ["version"] = "Ve",
        ["view"] = "Vw",
        ["x"] = "X",
        ["y"] = "Y",
        ["z"] = "Z",
    };

    private static readonly HashSet<string> stopWords = new(StringComparer.OrdinalIgnoreCase)
    {
        "a", "an", "and", "for", "from", "in", "of", "on", "the", "to", "with"
    };

    public static string NormalizeComponentNickname(string componentName, string fallbackNickname)
    {
        var source = string.IsNullOrWhiteSpace(componentName) ? fallbackNickname : componentName;
        var letters = string.Concat(ExtractWords(source)
            .Where(word => !stopWords.Contains(word))
            .Select(word => char.ToUpperInvariant(word[0])));
        if (letters.Length >= 3)
            return letters.Substring(0, 3);
        if (letters.Length == 2)
            return $"{letters}X";
        if (letters.Length == 1)
            return $"{letters}XX";
        return "Cmp";
    }

    public static void NormalizeComponentParameters(GH_Component component)
    {
        var componentName = string.IsNullOrWhiteSpace(component.Name) ? component.GetType().Name : component.Name;
        NormalizeParameterCollection(component.Params.Input, componentName, false);
        NormalizeParameterCollection(component.Params.Output, componentName, true);
    }

    private static void NormalizeParameterCollection(IReadOnlyList<IGH_Param> parameters, string componentName, bool isOutput)
    {
        foreach (var parameter in parameters)
        {
            parameter.NickName = NormalizeParameterNickname(parameter.Name, parameter.Access, parameter.Optional, parameter.NickName);
            parameter.Description = BuildParameterDescription(componentName, parameter.Name, parameter.Access, parameter.Optional, isOutput);
        }
    }

    private static string BuildParameterDescription(string componentName, string parameterName, GH_ParamAccess access, bool optional, bool isOutput)
    {
        var cardinality = access is GH_ParamAccess.list or GH_ParamAccess.tree ? "zero or more" : optional ? "zero or one" : "exactly one";
        var direction = isOutput ? "produced by" : "consumed by";
        return $"{cardinality} `{parameterName}` value {direction} `{componentName}`.";
    }

    public static string NormalizeParameterNickname(string parameterName, GH_ParamAccess access, bool optional, string fallbackNickname = "")
    {
        var key = string.Concat(ExtractWords(parameterName)).ToLowerInvariant();
        var code = dictionaryCodes.TryGetValue(key, out var mappedCode)
            ? mappedCode
            : ResolveFallbackCode(parameterName, fallbackNickname);
        code = string.Concat(code.Where(char.IsLetterOrDigit));
        if (code.Length >= 2)
            code = code.Substring(0, 2);
        else if (code.Length == 1)
            code = $"{char.ToUpperInvariant(code[0])}x";
        else
            code = "Px";

        var suffix = access is GH_ParamAccess.list or GH_ParamAccess.tree ? "*" : optional ? "?" : string.Empty;
        return $"{code}{suffix}";
    }

    private static string ResolveFallbackCode(string parameterName, string fallbackNickname)
    {
        var words = ExtractWords(parameterName).Where(word => !stopWords.Contains(word)).ToList();
        if (words.Count > 0)
        {
            var first = words[0];
            return first.Length >= 2
                ? $"{char.ToUpperInvariant(first[0])}{char.ToLowerInvariant(first[1])}"
                : $"{char.ToUpperInvariant(first[0])}x";
        }

        var fallback = string.Concat((fallbackNickname ?? string.Empty).Where(char.IsLetterOrDigit));
        return fallback.Length >= 2 ? fallback.Substring(0, 2) : "Px";
    }

    private static IEnumerable<string> ExtractWords(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
            return Array.Empty<string>();

        var splitOnWhitespace = Regex.Split(value, @"[\s_\-/→⇒…+Δ]+")
            .Where(part => !string.IsNullOrWhiteSpace(part));

        var words = new List<string>();
        foreach (var part in splitOnWhitespace)
            words.AddRange(Regex.Matches(part, @"[A-Z]?[a-z0-9]+|[A-Z]+(?![a-z])").Cast<Match>().Select(match => match.Value));
        return words;
    }
}

/// Generic Grasshopper parameter for semio entity types.
/// Implementations MUST provide component exposure and icon metadata.
public abstract class Param<TGoo, TRepresentation> : GH_PersistentParam<TGoo> where TGoo : Goo<TRepresentation> where TRepresentation : Entity<TRepresentation>, new()
{
    protected abstract string RepresentationName { get; }
    protected abstract string RepresentationNickname { get; }
    protected abstract string RepresentationDescription { get; }
    protected abstract string IconResourceName { get; }
    protected Param() : base("", "", "", Constants.Category, "Params") { }
    public override string Name => RepresentationName;
    public override string NickName => GhNaming.NormalizeParameterNickname(RepresentationName, GH_ParamAccess.item, false, RepresentationNickname);
    public override string Description => $"exactly one `{RepresentationName}` value persisted in `{GetType().Name}`.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(IconResourceName);

    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => throw new NotImplementedException();
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => throw new NotImplementedException();
}

/// Generic Grasshopper data wrapper for enum values.
/// Implementations MUST convert between string names and enum values.
public abstract class EnumGoo<TEnum> : GH_Goo<TEnum> where TEnum : struct, Enum
{
    public EnumGoo() { }
    public EnumGoo(TEnum value) => Value = value;
    public override bool IsValid => true;
    public override IGH_Goo Duplicate() => (IGH_Goo)(Activator.CreateInstance(GetType(), Value) ?? throw new InvalidOperationException($"Could not create instance of {GetType()}"));
    public override bool CastFrom(object source)
    {
        if (source is TEnum enumValue) { Value = enumValue; return true; }
        if (source is string str && Enum.TryParse<TEnum>(str, true, out var parsed)) { Value = parsed; return true; }
        if (source is int intVal && Enum.IsDefined(typeof(TEnum), intVal)) { Value = (TEnum)Enum.ToObject(typeof(TEnum), intVal); return true; }
        return false;
    }
    public override bool CastTo<U>(ref U target)
    {
        if (typeof(U) == typeof(TEnum)) { target = (U)(object)Value; return true; }
        if (typeof(U) == typeof(string)) { target = (U)(object)Value.ToString(); return true; }
        if (typeof(U) == typeof(int)) { target = (U)(object)Convert.ToInt32(Value); return true; }
        return false;
    }
    public override string ToString() => Value.ToString();
    public override string TypeName => typeof(TEnum).Name;
    public override string TypeDescription => typeof(TEnum).Name;
}

/// Generic Grasshopper parameter for enum values.
/// Implementations MUST restrict input to valid enum members.
public abstract class EnumParam<TEnumGoo, TEnum> : GH_Param<TEnumGoo>
    where TEnumGoo : EnumGoo<TEnum>, new()
    where TEnum : struct, Enum
{
    protected EnumParam(Guid id) : base(typeof(TEnum).Name, typeof(TEnum).Name, typeof(TEnum).Name, "Semio", "Param", GH_ParamAccess.item)
    {
        ComponentGuid = id;
    }
    public override Guid ComponentGuid { get; }
}
public abstract class Component : GH_Component
{
    public Component(string name, string nickname, string description, string subcategory) : base(
        name, nickname, description, Constants.Category, subcategory)
    {
        ApplyNamingPolicy();
    }

    public override void AddedToDocument(GH_Document document)
    {
        base.AddedToDocument(document);
        ApplyNamingPolicy();
    }

    public override void CreateAttributes()
    {
        base.CreateAttributes();
        ApplyNamingPolicy();
    }

    private void ApplyNamingPolicy()
    {
        NickName = GhNaming.NormalizeComponentNickname(Name, NickName);
        Description = $"{Name} operation in semio Grasshopper.";
        GhNaming.NormalizeComponentParameters(this);
    }
}

/// Abstract Grasshopper component that passes input through transformation.
/// Implementations MUST transform input data and output the result.
public abstract class PassthroughComponent<TParam, TGoo, TRepresentation> : Component
    where TParam : Param<TGoo, TRepresentation>, new() where TGoo : Goo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected abstract string RepresentationName { get; }
    protected abstract string RepresentationNickname { get; }
    protected abstract string RepresentationDescription { get; }
    protected abstract string IconResourceName { get; }

    protected PassthroughComponent() : base("", "", "", "Data") { }

    public override string Name => $"Passthrough {RepresentationName}";
    public override string NickName => GhNaming.NormalizeComponentNickname(Name, RepresentationNickname);
    public override string Description => $"{Name} operation in semio Grasshopper.";

    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(
        $"{IconResourceName.Replace("_24x24", "")}_modify_24x24",
        IconResourceName);

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected virtual void RegisterRepresentationInputParams(GH_InputParamManager pManager) { }
    protected virtual void RegisterRepresentationOutputParams(GH_OutputParamManager pManager) { }
    protected virtual void GetRepresentationData(IGH_DataAccess DA, TRepresentation representation) { }
    protected virtual void SetRepresentationData(IGH_DataAccess DA, TRepresentation representation) { }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), RepresentationName, RepresentationNickname + "?",
            $"The optional {RepresentationName.ToLower()} to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?",
            $"Whether the {RepresentationName.ToLower()} should be validated.", GH_ParamAccess.item);
        RegisterRepresentationInputParams(pManager);
        for (var i = 0; i < pManager.ParamCount; i++)
            pManager[i].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), RepresentationName, RepresentationNickname,
            $"The constructed or modified {RepresentationName.ToLower()}.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?",
            $"True if the {RepresentationName.ToLower()} is valid. Null if no validation was performed.", GH_ParamAccess.item);
        RegisterRepresentationOutputParams(pManager);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var representationGoo = new TGoo();
        var validate = false;
        if (DA.GetData(0, ref representationGoo))
            representationGoo = (TGoo)representationGoo.Duplicate();
        DA.GetData(1, ref validate);

        GetRepresentationData(DA, representationGoo.Value);
        representationGoo.Value = ProcessRepresentation(representationGoo.Value);

        if (validate)
        {
            var (isValid, errors) = representationGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, representationGoo.Duplicate());
        SetRepresentationData(DA, representationGoo.Value);
    }

    protected virtual TRepresentation ProcessRepresentation(TRepresentation representation) => representation;
}

/// Generic Grasshopper data wrapper for entity ID types.
/// Implementations MUST wrap entity ID types for Grasshopper data flow.
public abstract class IdGoo<TRepresentation> : Goo<TRepresentation> where TRepresentation : Entity<TRepresentation>, new()
{
    public IdGoo() : base() { }
    public IdGoo(TRepresentation value) : base(value) { }
}

/// Generic Grasshopper parameter for entity ID types.
/// Implementations MUST provide type-safe parameter access for IDs.
public abstract class IdParam<TGoo, TRepresentation> : Param<TGoo, TRepresentation> where TGoo : IdGoo<TRepresentation> where TRepresentation : Entity<TRepresentation>, new()
{
    protected IdParam() : base() { }
    protected abstract string IdIconResourceName { get; }
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(IdIconResourceName, IconResourceName);
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

/// Abstract Grasshopper component for constructing entity IDs.
/// Implementations MUST register input parameters matching ID fields.
public abstract class IdComponent<TParam, TGoo, TRepresentation> : PassthroughComponent<TParam, TGoo, TRepresentation>
    where TParam : IdParam<TGoo, TRepresentation>, new() where TGoo : IdGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected IdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

/// Generic Grasshopper data wrapper for entity diff types.
/// Implementations MUST wrap entity diff types for Grasshopper data flow.
public abstract class DiffGoo<TRepresentation> : Goo<TRepresentation> where TRepresentation : Entity<TRepresentation>, new()
{
    public DiffGoo() : base() { }
    public DiffGoo(TRepresentation value) : base(value) { }
}

/// Generic Grasshopper parameter for entity diff types.
/// Implementations MUST provide type-safe parameter access for diffs.
public abstract class DiffParam<TGoo, TRepresentation> : Param<TGoo, TRepresentation> where TGoo : DiffGoo<TRepresentation> where TRepresentation : Entity<TRepresentation>, new()
{
    protected DiffParam() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

/// Abstract Grasshopper component for constructing entity diffs.
/// Implementations MUST register input parameters matching diff fields.
public abstract class DiffComponent<TParam, TGoo, TRepresentation> : PassthroughComponent<TParam, TGoo, TRepresentation>
    where TParam : DiffParam<TGoo, TRepresentation>, new() where TGoo : DiffGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected DiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

/// Generic Grasshopper data wrapper for semio change types.
/// Implementations MUST convert between JSON and typed change values.
public abstract class ChangeGoo<TChange> : GH_Goo<TChange> where TChange : class, new()
{
    public ChangeGoo() { Value = new TChange(); }
    public ChangeGoo(TChange value) { Value = value; }
    public override bool IsValid => Value is not null;
    public override string TypeName => typeof(TChange).Name;
    public override string TypeDescription => typeof(TChange).Name;
    public override IGH_Goo Duplicate()
    {
        var duplicate = (ChangeGoo<TChange>)(Activator.CreateInstance(GetType())
            ?? throw new InvalidOperationException($"Could not create instance of {GetType()}"));
        duplicate.Value = Value.Serialize().Deserialize<TChange>() ?? new TChange();
        return duplicate;
    }
    public override string ToString() => Value?.ToString() ?? typeof(TChange).Name;
    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(TChange).Name, Value.Serialize());
        return base.Write(writer);
    }
    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString(typeof(TChange).Name).Deserialize<TChange>() ?? new TChange();
        return base.Read(reader);
    }
    internal virtual bool CustomCastTo<Q>(ref Q target) => false;
    internal virtual bool CustomCastFrom(object source) => false;
    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TChange)))
        {
            target = (Q)(object)Value;
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Serialize());
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source is null) return false;
        if (source is TChange representation)
        {
            Value = representation;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            var deserialized = str.Deserialize<TChange>();
            if (deserialized is null)
                return false;
            Value = deserialized;
            return true;
        }
        return CustomCastFrom(source);
    }
}

/// Generic Grasshopper parameter for semio change types.
/// Implementations MUST provide component exposure and icon metadata.
public abstract class ChangeParam<TGoo, TChange> : GH_PersistentParam<TGoo>
    where TGoo : ChangeGoo<TChange>
    where TChange : class, new()
{
    protected abstract string RepresentationName { get; }
    protected abstract string RepresentationNickname { get; }
    protected abstract string RepresentationDescription { get; }
    protected abstract string IconResourceName { get; }
    protected ChangeParam() : base("", "", "", Constants.Category, "Params") { }
    public override string Name => RepresentationName;
    public override string NickName => GhNaming.NormalizeParameterNickname(RepresentationName, GH_ParamAccess.item, false, RepresentationNickname);
    public override string Description => $"exactly one `{RepresentationName}` value persisted in `{GetType().Name}`.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(IconResourceName);
    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => throw new NotImplementedException();
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => throw new NotImplementedException();
}

/// Abstract Grasshopper component for constructing entity changes.
/// Implementations MUST register input parameters matching change fields.
public abstract class ChangeComponent<TEntityParam, TEntityGoo, TEntity, TDiffParam, TDiffGoo, TDiff, TChangeParam, TChangeGoo, TChange> : Component
    where TEntityParam : Param<TEntityGoo, TEntity>, new()
    where TEntityGoo : Goo<TEntity>, new()
    where TEntity : Entity<TEntity>, new()
    where TDiffParam : DiffParam<TDiffGoo, TDiff>, new()
    where TDiffGoo : DiffGoo<TDiff>, new()
    where TDiff : Entity<TDiff>, new()
    where TChangeParam : ChangeParam<TChangeGoo, TChange>, new()
    where TChangeGoo : ChangeGoo<TChange>, new()
    where TChange : Change<TEntity, TDiff>, new()
{
    protected abstract string EntityName { get; }
    protected abstract string EntityNickname { get; }
    protected abstract string IconResourceName { get; }

    protected ChangeComponent() : base("", "", "", "Data") { }

    public override string Name => $"Passthrough {EntityName} Change";
    public override string NickName => GhNaming.NormalizeComponentNickname(Name, EntityNickname);
    public override string Description => $"{Name} operation in semio Grasshopper.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(IconResourceName);
    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TChangeParam(), $"{EntityName} Change", $"{EntityNickname}Ch?", $"The optional {EntityName.ToLower()} change.", GH_ParamAccess.item);
        pManager.AddParameter(new TDiffParam(), $"Forward {EntityName} Diff", $"{EntityNickname}Fw?", $"The optional forward {EntityName.ToLower()} diff.", GH_ParamAccess.item);
        pManager.AddParameter(new TDiffParam(), $"Backward {EntityName} Diff", $"{EntityNickname}Bw?", $"The optional backward {EntityName.ToLower()} diff.", GH_ParamAccess.item);
        pManager.AddTextParameter("Author", "Au?", "The optional author.", GH_ParamAccess.item);
        pManager.AddTimeParameter("Time", "Tm?", "The optional change timestamp.", GH_ParamAccess.item);
        pManager.AddParameter(new TEntityParam(), $"Before {EntityName}", "Bf?", $"The optional {EntityName.ToLower()} before change.", GH_ParamAccess.item);
        pManager.AddParameter(new TEntityParam(), $"After {EntityName}", "Af?", $"The optional {EntityName.ToLower()} after change.", GH_ParamAccess.item);
        for (var i = 0; i < pManager.ParamCount; i++)
            pManager[i].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TChangeParam(), $"{EntityName} Change", $"{EntityNickname}Ch", $"The constructed or modified {EntityName.ToLower()} change.", GH_ParamAccess.item);
        pManager.AddParameter(new TDiffParam(), $"Forward {EntityName} Diff", $"{EntityNickname}Fw", $"The optional forward {EntityName.ToLower()} diff.", GH_ParamAccess.item);
        pManager.AddParameter(new TDiffParam(), $"Backward {EntityName} Diff", $"{EntityNickname}Bw", $"The optional backward {EntityName.ToLower()} diff.", GH_ParamAccess.item);
        pManager.AddTextParameter("Author", "Au?", "The optional author.", GH_ParamAccess.item);
        pManager.AddTimeParameter("Time", "Tm?", "The optional change timestamp.", GH_ParamAccess.item);
        pManager.AddParameter(new TEntityParam(), $"Before {EntityName}", "Bf", $"The optional {EntityName.ToLower()} before change.", GH_ParamAccess.item);
        pManager.AddParameter(new TEntityParam(), $"After {EntityName}", "Af", $"The optional {EntityName.ToLower()} after change.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var changeGoo = new TChangeGoo();
        if (DA.GetData(0, ref changeGoo))
            changeGoo = (TChangeGoo)changeGoo.Duplicate();

        var forward = new TDiffGoo();
        var backward = new TDiffGoo();
        var author = "";
        var time = default(DateTime);
        var before = new TEntityGoo();
        var after = new TEntityGoo();

        if (DA.GetData(1, ref forward))
            changeGoo.Value.Forward = forward.Value.DeepClone();
        if (DA.GetData(2, ref backward))
            changeGoo.Value.Backward = backward.Value.DeepClone();
        if (DA.GetData(3, ref author))
            changeGoo.Value.Author = author;
        if (DA.GetData(4, ref time))
            changeGoo.Value.Time = time;
        if (DA.GetData(5, ref before))
            changeGoo.Value.Before = before.Value.DeepClone();
        if (DA.GetData(6, ref after))
            changeGoo.Value.After = after.Value.DeepClone();

        DA.SetData(0, changeGoo.Duplicate());
        if (changeGoo.Value.Forward is not null)
        {
            var forwardGoo = new TDiffGoo { Value = changeGoo.Value.Forward.DeepClone() };
            DA.SetData(1, forwardGoo);
        }
        if (changeGoo.Value.Backward is not null)
        {
            var backwardGoo = new TDiffGoo { Value = changeGoo.Value.Backward.DeepClone() };
            DA.SetData(2, backwardGoo);
        }
        DA.SetData(3, changeGoo.Value.Author);
        DA.SetData(4, changeGoo.Value.Time);
        if (changeGoo.Value.Before is not null)
        {
            var beforeGoo = new TEntityGoo { Value = changeGoo.Value.Before.DeepClone() };
            DA.SetData(5, beforeGoo);
        }
        if (changeGoo.Value.After is not null)
        {
            var afterGoo = new TEntityGoo { Value = changeGoo.Value.After.DeepClone() };
            DA.SetData(6, afterGoo);
        }
    }
}

/// Abstract Grasshopper component for applying an entity diff to an entity.
/// Implementations MUST apply diffs without performing persistence operations.
public abstract class ApplyDiffComponent<TEntityParam, TEntityGoo, TEntity, TDiffParam, TDiffGoo, TDiff> : ScriptingComponent
    where TEntityParam : Param<TEntityGoo, TEntity>, new()
    where TEntityGoo : Goo<TEntity>, new()
    where TEntity : Entity<TEntity>, new()
    where TDiffParam : DiffParam<TDiffGoo, TDiff>, new()
    where TDiffGoo : DiffGoo<TDiff>, new()
    where TDiff : Entity<TDiff>, new()
{
    protected ApplyDiffComponent() : base("", "", "") { }

    protected abstract string EntityName { get; }
    protected abstract string EntityNickname { get; }
    protected abstract string DiffNickname { get; }
    protected abstract string IconResourceName { get; }
    protected abstract TEntity Apply(TEntity entity, TDiff diff);

    public override string Name => $"Apply {EntityName} Diff";
    public override string NickName => GhNaming.NormalizeComponentNickname(Name, EntityNickname);
    public override string Description => $"{Name} operation in semio Grasshopper.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder(IconResourceName);
    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TEntityParam(), EntityName, EntityNickname, $"The {EntityName.ToLower()} to update.", GH_ParamAccess.item);
        pManager.AddParameter(new TDiffParam(), $"{EntityName} Diff", DiffNickname, $"The diff to apply.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TEntityParam(), EntityName, EntityNickname, $"Updated {EntityName.ToLower()}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var entityGoo = new TEntityGoo();
        var diffGoo = new TDiffGoo();
        if (!DA.GetData(0, ref entityGoo)) return;
        if (!DA.GetData(1, ref diffGoo)) return;

        var updatedEntity = Apply(entityGoo.Value, diffGoo.Value);
        var updatedGoo = new TEntityGoo { Value = updatedEntity };
        DA.SetData(0, updatedGoo);
    }
}

public class ApplyAttributeDiffComponent : ApplyDiffComponent<AttributeParam, AttributeGoo, Attribute, AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public override Guid ComponentGuid => new("4A901F8D-53C5-4F36-B32E-5FE7D038B3C1");
    protected override string EntityName => "Attribute";
    protected override string EntityNickname => "At";
    protected override string DiffNickname => "AtΔ";
    protected override string IconResourceName => "attribute_modify_24x24";
    protected override Attribute Apply(Attribute entity, AttributeDiff diff) => Attribute.ApplyDiff(entity, diff);
}

public class ApplyFolderDiffComponent : ApplyDiffComponent<FolderParam, FolderGoo, Folder, FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public override Guid ComponentGuid => new("B9D5A6E1-C51E-4B7A-B433-E83A42D5A861");
    protected override string EntityName => "Folder";
    protected override string EntityNickname => "Fd";
    protected override string DiffNickname => "FdΔ";
    protected override string IconResourceName => "folder_modify_24x24";
    protected override Folder Apply(Folder entity, FolderDiff diff) => Folder.ApplyDiff(entity, diff);
}

public class ApplyRepresentationDiffComponent : ApplyDiffComponent<RepresentationParam, RepresentationGoo, Representation, RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public override Guid ComponentGuid => new("06D898D4-3CA6-4742-9C9F-E9BA1C5D7541");
    protected override string EntityName => "Representation";
    protected override string EntityNickname => "Mo";
    protected override string DiffNickname => "MoΔ";
    protected override string IconResourceName => "representation_modify_24x24";
    protected override Representation Apply(Representation entity, RepresentationDiff diff) => Representation.ApplyDiff(entity, diff);
}

public class ApplyConnectorDiffComponent : ApplyDiffComponent<ConnectorParam, ConnectorGoo, Connector, ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public override Guid ComponentGuid => new("E450E49A-5ECA-4A6C-95EE-9FCA3D2C6B4A");
    protected override string EntityName => "Connector";
    protected override string EntityNickname => "Cn";
    protected override string DiffNickname => "CnΔ";
    protected override string IconResourceName => "connector_modify_24x24";
    protected override Connector Apply(Connector entity, ConnectorDiff diff) => Connector.ApplyDiff(entity, diff);
}

public class ApplyTypeDiffComponent : ApplyDiffComponent<TypeParam, TypeGoo, Type, TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("A45F0A6C-1EB0-44A4-A5A7-81C4F248A873");
    protected override string EntityName => "Type";
    protected override string EntityNickname => "Tp";
    protected override string DiffNickname => "TpΔ";
    protected override string IconResourceName => "type_modify_24x24";
    protected override Type Apply(Type entity, TypeDiff diff) => Type.ApplyDiff(entity, diff);
}

public class ApplyPieceDiffComponent : ApplyDiffComponent<PieceParam, PieceGoo, Piece, PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("1C29E52E-B2A7-4600-BAC2-996FF80A5015");
    protected override string EntityName => "Piece";
    protected override string EntityNickname => "Pc";
    protected override string DiffNickname => "PcΔ";
    protected override string IconResourceName => "piece_modify_24x24";
    protected override Piece Apply(Piece entity, PieceDiff diff) => Piece.ApplyDiff(entity, diff);
}

public class ApplySideDiffComponent : ApplyDiffComponent<SideParam, SideGoo, Side, SideDiffParam, SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("4F2B5F8A-4AB4-4B58-B6CB-58231F8FF7BF");
    protected override string EntityName => "Side";
    protected override string EntityNickname => "Sd";
    protected override string DiffNickname => "SdΔ";
    protected override string IconResourceName => "side_modify_24x24";
    protected override Side Apply(Side entity, SideDiff diff) => Side.ApplyDiff(entity, diff);
}

public class ApplyConnectionDiffComponent : ApplyDiffComponent<ConnectionParam, ConnectionGoo, Connection, ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("9F954A31-E53D-4C9A-BB2E-4A17EB8C333D");
    protected override string EntityName => "Connection";
    protected override string EntityNickname => "Cnx";
    protected override string DiffNickname => "CnxΔ";
    protected override string IconResourceName => "connection_modify_24x24";
    protected override Connection Apply(Connection entity, ConnectionDiff diff) => Connection.ApplyDiff(entity, diff);
}

public class ApplyDesignDiffComponent : ApplyDiffComponent<DesignParam, DesignGoo, Design, DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("31BE87E8-045F-4F12-9651-C4C1A130D7A7");
    protected override string EntityName => "Design";
    protected override string EntityNickname => "De";
    protected override string DiffNickname => "DeΔ";
    protected override string IconResourceName => "design_modify_24x24";
    protected override Design Apply(Design entity, DesignDiff diff) => Design.ApplyDiff(entity, diff);
}

/// Abstract Grasshopper component for serializing entities to JSON.
/// Implementations MUST convert entities to valid JSON strings.
public abstract class SerializeComponent<TParam, TGoo, TRepresentation> : ScriptingComponent
    where TParam : Param<TGoo, TRepresentation>, new() where TGoo : Goo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected virtual string RepresentationName => typeof(TRepresentation).Name;
    protected virtual string RepresentationNickname => typeof(TRepresentation).Name.Substring(0, 3);

    protected SerializeComponent() : base("", "", "") { }

    public override string Name => $"Serialize {RepresentationName}";
    public override string NickName => GhNaming.NormalizeComponentNickname(Name, RepresentationNickname);
    public override string Description => $"{Name} operation in semio Grasshopper.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{typeof(TRepresentation).Name.ToLower()}_serialize_24x24");
    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), RepresentationName, RepresentationNickname, $"The {RepresentationName.ToLower()} to serialize.", GH_ParamAccess.item);
        pManager.AddTextParameter("Indent", "In?", $"The optional indent unit for the serialized {RepresentationName.ToLower()}. Empty text for no indent or spaces or tabs", GH_ParamAccess.item, "");

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", $"Text of serialized {RepresentationName}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var goo = new TGoo();
        var indent = "";
        DA.GetData(0, ref goo);
        DA.GetData(1, ref indent);
        var text = goo.Value.Serialize(indent);
        DA.SetData(0, text);
    }
}

/// Abstract Grasshopper component for deserializing entities from JSON.
/// Implementations MUST parse JSON strings into entity instances.
public abstract class DeserializeComponent<TParam, TGoo, TRepresentation> : ScriptingComponent
    where TParam : Param<TGoo, TRepresentation>, new() where TGoo : Goo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected virtual string RepresentationName => typeof(TRepresentation).Name;
    protected virtual string RepresentationNickname => typeof(TRepresentation).Name.Substring(0, 3);

    protected DeserializeComponent() : base("", "", "") { }

    public override string Name => $"Deserialize {RepresentationName}";
    public override string NickName => GhNaming.NormalizeComponentNickname(Name, RepresentationNickname);
    public override string Description => $"{Name} operation in semio Grasshopper.";
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{typeof(TRepresentation).Name.ToLower()}_deserialize_24x24");
    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", $"Text of serialized {RepresentationName}.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), RepresentationName, RepresentationNickname, $"Deserialized {RepresentationName}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        var value = text.Deserialize<TRepresentation>() ?? throw new InvalidOperationException($"Could not deserialize {typeof(TRepresentation).Name}");
        var goo = new TGoo();
        goo.Value = value;
        DA.SetData(0, goo);
    }
}

/// Abstract Grasshopper component for serializing diffs to JSON.
/// Implementations MUST convert diffs to valid JSON strings.
public abstract class SerializeDiffComponent<TParam, TGoo, TRepresentation> : SerializeComponent<TParam, TGoo, TRepresentation>
    where TParam : DiffParam<TGoo, TRepresentation>, new() where TGoo : DiffGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected SerializeDiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{GetEntityName()}_diff_serialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TRepresentation).Name.ToLower();
        return typeName.EndsWith("diff") ? typeName.Substring(0, typeName.Length - 4) :
               typeName.EndsWith("sdiff") ? typeName.Substring(0, typeName.Length - 5) : typeName;
    }
}

/// Abstract Grasshopper component for deserializing diffs from JSON.
/// Implementations MUST parse JSON strings into diff instances.
public abstract class DeserializeDiffComponent<TParam, TGoo, TRepresentation> : DeserializeComponent<TParam, TGoo, TRepresentation>
    where TParam : DiffParam<TGoo, TRepresentation>, new() where TGoo : DiffGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected DeserializeDiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{GetEntityName()}_diff_deserialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TRepresentation).Name.ToLower();
        return typeName.EndsWith("diff") ? typeName.Substring(0, typeName.Length - 4) :
               typeName.EndsWith("sdiff") ? typeName.Substring(0, typeName.Length - 5) : typeName;
    }
}

/// Abstract Grasshopper component for serializing entity IDs to JSON.
/// Implementations MUST convert entity IDs to valid JSON strings.
public abstract class SerializeIdComponent<TParam, TGoo, TRepresentation> : SerializeComponent<TParam, TGoo, TRepresentation>
    where TParam : IdParam<TGoo, TRepresentation>, new() where TGoo : IdGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected SerializeIdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{GetEntityName()}_id_serialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TRepresentation).Name.ToLower();
        return typeName.EndsWith("id") ? typeName.Substring(0, typeName.Length - 2) : typeName;
    }
}

/// Abstract Grasshopper component for deserializing entity IDs from JSON.
/// Implementations MUST parse JSON strings into entity ID instances.
public abstract class DeserializeIdComponent<TParam, TGoo, TRepresentation> : DeserializeComponent<TParam, TGoo, TRepresentation>
    where TParam : IdParam<TGoo, TRepresentation>, new() where TGoo : IdGoo<TRepresentation>, new() where TRepresentation : Entity<TRepresentation>, new()
{
    protected DeserializeIdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => IconResources.ResolveOrPlaceholder($"{GetEntityName()}_id_deserialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TRepresentation).Name.ToLower();
        return typeName.EndsWith("id") ? typeName.Substring(0, typeName.Length - 2) : typeName;
    }
}

/// Generic Grasshopper data wrapper with built-in entity validation.
/// Implementations MUST validate entities before exposing them downstream.
public abstract class EntityGoo<TEntity, TEntityDiff, TEntityId> : Goo<TEntity>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityGoo() : base() { }
    public EntityGoo(TEntity value) : base(value) { }
}

/// Generic Grasshopper parameter with entity validation support.
/// Implementations MUST enforce entity validation on parameter access.
public abstract class EntityParam<TGoo, TEntity, TEntityDiff, TEntityId> : Param<TGoo, TEntity>
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entities.
/// Implementations MUST validate constructed entities before output.
public abstract class EntityComponent<TParam, TGoo, TEntity, TEntityDiff, TEntityId> : PassthroughComponent<TParam, TGoo, TEntity>
    where TParam : EntityParam<TGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityComponent() : base() { }
}

/// Generic Grasshopper data wrapper for validated entity ID types.
/// Implementations MUST validate entity IDs before exposing them downstream.
public abstract class EntityIdGoo<TEntity, TEntityDiff, TEntityId> : IdGoo<TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityIdGoo() : base() { }
    public EntityIdGoo(TEntityId value) : base(value) { }
}

/// Generic Grasshopper parameter for validated entity ID types.
/// Implementations MUST enforce entity ID validation on parameter access.
public abstract class EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId> : IdParam<TIdGoo, TEntityId>
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityIdParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entity IDs.
/// Implementations MUST validate constructed entity IDs before output.
public abstract class EntityIdComponent<TIdParam, TIdGoo, TEntity, TEntityDiff, TEntityId> : IdComponent<TIdParam, TIdGoo, TEntityId>
    where TIdParam : EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityIdComponent() : base() { }
}

/// Generic Grasshopper data wrapper for validated entity diff types.
/// Implementations MUST validate entity diffs before exposing them downstream.
public abstract class EntityDiffGoo<TEntity, TEntityDiff, TEntityId> : DiffGoo<TEntityDiff>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityDiffGoo() : base() { }
    public EntityDiffGoo(TEntityDiff value) : base(value) { }
}

/// Generic Grasshopper parameter for validated entity diff types.
/// Implementations MUST enforce entity diff validation on parameter access.
public abstract class EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffParam<TDiffGoo, TEntityDiff>
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityDiffParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entity diffs.
/// Implementations MUST validate constructed entity diffs before output.
public abstract class EntityDiffComponent<TDiffParam, TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffComponent<TDiffParam, TDiffGoo, TEntityDiff>
    where TDiffParam : EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityDiffComponent() : base() { }
}

#endregion 🔓Bases

#region 🧲Attribute
// Implementations MUST provide key-value metadata for annotating entities.

public class AttributeGoo : Goo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeIdGoo)))
        {
            target = (Q)(object)new AttributeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeDiffGoo)))
        {
            target = (Q)(object)new AttributeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is AttributeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Attribute { Key = str };
            return true;
        }
        return false;
    }
}

public class AttributeParam : Param<AttributeGoo, Attribute>
{
    protected override string RepresentationName => "Attribute";
    protected override string RepresentationNickname => "Atr";
    protected override string RepresentationDescription => "Key-value metadata";
    protected override string IconResourceName => "attribute_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
}

public class AttributeComponent : PassthroughComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
    protected override string RepresentationName => "Attribute";
    protected override string RepresentationNickname => "Atr";
    protected override string RepresentationDescription => "Construct, deconstruct or modify an attribute.";
    protected override string IconResourceName => "attribute_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Attribute representation)
    {
        var id = ""; var key = ""; var value = ""; var definition = "";
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref key)) representation.Key = key;
        if (DA.GetData(4, ref value)) representation.Value = value;
        if (DA.GetData(5, ref definition)) representation.Definition = definition;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Attribute representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Key);
        DA.SetData(4, representation.Value);
        DA.SetData(5, representation.Definition);
    }
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
    protected override string RepresentationName => "Attribute";
    protected override string RepresentationNickname => "Atr";
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8975-8588BCA75250");
    protected override string RepresentationName => "Attribute";
    protected override string RepresentationNickname => "Atr";
}

public class AttributeIdGoo : IdGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeGoo)))
        {
            target = (Q)(object)new AttributeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeDiffGoo)))
        {
            target = (Q)(object)new AttributeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is AttributeGoo attrGoo)
        {
            Value = attrGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AttributeId { Id = str };
            return true;
        }
        return false;
    }
}

public class AttributeIdParam : IdParam<AttributeIdGoo, AttributeId>
{
    protected override string RepresentationName => "AttributeId";
    protected override string RepresentationNickname => "AId";
    protected override string RepresentationDescription => "Attribute identifier";
    protected override string IconResourceName => "attribute_24x24";
    protected override string IdIconResourceName => "attributeid_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B93");
}

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeIdGoo)))
        {
            target = (Q)(object)new AttributeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeGoo)))
        {
            target = (Q)(object)new AttributeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is AttributeGoo attrGoo)
        {
            Value = attrGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<AttributeDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    protected override string RepresentationName => "AttributeDiff";
    protected override string RepresentationNickname => "ADf";
    protected override string RepresentationDescription => "Attribute differences";
    protected override string IconResourceName => "attribute_diff_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B96");
    protected override string RepresentationName => "AttributeDiff";
    protected override string RepresentationNickname => "ADf";
    protected override string RepresentationDescription => "Construct, deconstruct or modify an attribute diff.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke?", "The optional key.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va?", "The optional value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke?", "The optional key.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va?", "The optional value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, AttributeDiff representation)
    {
        string id = null, key = null, value = null, definition = null;
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref key)) representation.Key = key;
        if (DA.GetData(4, ref value)) representation.Value = value;
        if (DA.GetData(5, ref definition)) representation.Definition = definition;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, AttributeDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeKey()) DA.SetData(3, representation.Key);
        if (representation.ShouldSerializeValue()) DA.SetData(4, representation.Value);
        if (representation.ShouldSerializeDefinition()) DA.SetData(5, representation.Definition);
    }
}

public class SerializeAttributeDiffComponent : SerializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public SerializeAttributeDiffComponent() { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B97");
}

public class DeserializeAttributeDiffComponent : DeserializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public DeserializeAttributeDiffComponent() { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B98");
}

#endregion 🧲Attribute

#region 🌥️Coordinate
// Implementations MUST share X, Y, Z coordinate fields for spatial types.

public class CoordinateGoo : Goo<Coordinate>
{
    public CoordinateGoo() { }
    public CoordinateGoo(Coordinate value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.U, Value.V, 0));
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        Point3d point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Coordinate { U = (float)point.X, V = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class CoordinateParam : Param<CoordinateGoo, Coordinate>
{
    protected override string RepresentationName => "Coordinate";
    protected override string RepresentationNickname => "DPt";
    protected override string RepresentationDescription => "2D coordinate";
    protected override string IconResourceName => "coordinate_24x24";
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");
}

public class CoordinateComponent : PassthroughComponent<CoordinateParam, CoordinateGoo, Coordinate>
{
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");
    protected override string RepresentationName => "Coordinate";
    protected override string RepresentationNickname => "DPt";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a 2d coordinate.";
    protected override string IconResourceName => "coordinate_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("U", "U", "The u-coordinate.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V", "The v-coordinate.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("U", "U", "The u-coordinate.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V", "The v-coordinate.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Coordinate representation)
    {
        double u = 0, v = 0;
        if (DA.GetData(2, ref u)) representation.U = (float)u;
        if (DA.GetData(3, ref v)) representation.V = (float)v;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Coordinate representation)
    {
        DA.SetData(2, representation.U);
        DA.SetData(3, representation.V);
    }
}

public class SerializeCoordinateComponent : SerializeComponent<CoordinateParam, CoordinateGoo, Coordinate>
{
    public SerializeCoordinateComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");
}

public class DeserializeCoordinateComponent : DeserializeComponent<CoordinateParam, CoordinateGoo, Coordinate>
{
    public DeserializeCoordinateComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A9A");
}

#endregion 🌥️Coordinate

#region ⏲️Location
// Implementations MUST combine a plane with rotation and elevation for placement.

public class LocationGoo : Goo<Location>
{
    public LocationGoo() { }
    public LocationGoo(Location value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.Longitude, Value.Latitude, 0));
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        var point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Location { Longitude = (float)point.X, Latitude = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class LocationParam : Param<LocationGoo, Location>
{
    protected override string RepresentationName => "Location";
    protected override string RepresentationNickname => "Loc";
    protected override string RepresentationDescription => "Geographic location";
    protected override string IconResourceName => "location_24x24";
    public override Guid ComponentGuid => new("CA9DA889-398E-469B-BF1B-AD2BDFCA7957");
}

public class LocationComponent : PassthroughComponent<LocationParam, LocationGoo, Location>
{
    public override Guid ComponentGuid => new("6F2EDF42-6E10-4944-8B05-4D41F4876ED0");
    protected override string RepresentationName => "Location";
    protected override string RepresentationNickname => "Loc";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a location.";
    protected override string IconResourceName => "location_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the location.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Longitude", "Lo", "The longitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "The latitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Altitude", "Al?", "The optional altitude.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the location.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Longitude", "Lo", "The longitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "The latitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Altitude", "Al?", "The optional altitude.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Location representation)
    {
        string id = "";
        double lon = 0, lat = 0, altitude = 0;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref lon)) representation.Longitude = (float)lon;
        if (DA.GetData(4, ref lat)) representation.Latitude = (float)lat;
        if (DA.GetData(5, ref altitude)) representation.Altitude = (float)altitude;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Location representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Longitude);
        DA.SetData(4, representation.Latitude);
        DA.SetData(5, representation.Altitude);
        DA.SetDataList(6, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeLocationComponent : SerializeComponent<LocationParam, LocationGoo, Location>
{
    public SerializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D466");
}

public class DeserializeLocationComponent : DeserializeComponent<LocationParam, LocationGoo, Location>
{
    public DeserializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D467");
}

#endregion ⏲️Location

#region 🤸Author
// Implementations MUST provide author identity with name and contact.

public class AuthorGoo : Goo<Author>
{
    public AuthorGoo() { }
    public AuthorGoo(Author value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Author { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorParam : Param<AuthorGoo, Author>
{
    protected override string RepresentationName => "Author";
    protected override string RepresentationNickname => "Aut";
    protected override string RepresentationDescription => "Author information";
    protected override string IconResourceName => "author_24x24";
    public override Guid ComponentGuid => new("9F52380B-1812-42F7-9DAD-952C2F7A635A");
}

public class AuthorComponent : PassthroughComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
    protected override string RepresentationName => "Author";
    protected override string RepresentationNickname => "Aut";
    protected override string RepresentationDescription => "Construct, deconstruct or modify an author.";
    protected override string IconResourceName => "author_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "Em", "The email of the author.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "Em", "The email of the author.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Author representation)
    {
        string id = "", name = "", email = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref email)) representation.Email = email;
        if (DA.GetDataList(5, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Author representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Email);
        DA.SetDataList(5, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public SerializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public DeserializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634879");
}

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AuthorId { Id = str };
            return true;
        }
        return false;
    }
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    protected override string RepresentationName => "AuthorId";
    protected override string RepresentationNickname => "AuI";
    protected override string RepresentationDescription => "Author identifier";
    protected override string IconResourceName => "author_24x24";
    protected override string IdIconResourceName => "authorid_24x24";
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1C");
}

#endregion 🤸Author

#region 🪨File
// Implementations MUST reference a file with URI and optional content.

public class FileGoo : Goo<File>
{
    public FileGoo() { }
    public FileGoo(File value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(FileIdGoo)))
        {
            target = (Q)(object)new FileIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new File { Id = str, Name = str, CreatedAt = DateTime.UtcNow, ModificationdAt = DateTime.UtcNow };
            return true;
        }
        return false;
    }
}

public class FileParam : Param<FileGoo, File>
{
    protected override string RepresentationName => "File";
    protected override string RepresentationNickname => "Fil";
    protected override string RepresentationDescription => "File reference";
    protected override string IconResourceName => "file_24x24";
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class FileComponent : PassthroughComponent<FileParam, FileGoo, File>
{
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");
    protected override string RepresentationName => "File";
    protected override string RepresentationNickname => "Fil";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a file.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Size", "Sz?", "The optional file size in bytes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Hash", "Hs?", "The optional file hash.", GH_ParamAccess.item);
        pManager.AddTextParameter("Blob", "Bl?", "The optional file blob.", GH_ParamAccess.item);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional creator.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updater.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Size", "Sz?", "The optional file size in bytes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Hash", "Hs?", "The optional file hash.", GH_ParamAccess.item);
        pManager.AddTextParameter("Blob", "Bl?", "The optional file blob.", GH_ParamAccess.item);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional creator.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updater.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, File representation)
    {
        string id = "", name = "", remote = "", hash = "", blob = "", createdBy = "", updatedBy = "";
        var folderIdGoo = new FolderIdGoo();
        int size = 0;
        DateTime createdAt = default, updatedAt = default;
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref remote)) representation.Remote = remote;
        if (DA.GetData(5, ref folderIdGoo)) representation.Folder = folderIdGoo.Value;
        if (DA.GetData(6, ref size)) representation.Size = size;
        if (DA.GetData(7, ref hash)) representation.Hash = hash;
        if (DA.GetData(8, ref blob)) representation.Blob = blob;
        if (DA.GetData(9, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(10, ref createdBy)) representation.CreatedBy = createdBy;
        if (DA.GetData(11, ref updatedAt)) representation.ModificationdAt = updatedAt;
        if (DA.GetData(12, ref updatedBy)) representation.ModificationdBy = updatedBy;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, File representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Remote);
        DA.SetData(5, representation.Folder is not null ? new FolderIdGoo(representation.Folder) : null);
        DA.SetData(6, representation.Size);
        DA.SetData(7, representation.Hash);
        DA.SetData(8, representation.Blob);
        DA.SetData(9, representation.CreatedAt);
        DA.SetData(10, representation.CreatedBy);
        DA.SetData(11, representation.ModificationdAt);
        DA.SetData(12, representation.ModificationdBy);
    }
}

public class SerializeFileComponent : SerializeComponent<FileParam, FileGoo, File>
{
    public SerializeFileComponent() { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FA");
}

public class DeserializeFileComponent : DeserializeComponent<FileParam, FileGoo, File>
{
    public DeserializeFileComponent() { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FB");
}

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is FileGoo fileGoo)
        {
            Value = fileGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FileId { Id = str };
            return true;
        }
        return false;
    }
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    protected override string RepresentationName => "FileId";
    protected override string RepresentationNickname => "FId";
    protected override string RepresentationDescription => "File identifier";
    protected override string IconResourceName => "file_24x24";
    protected override string IdIconResourceName => "fileid_24x24";
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id ?? "");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FileDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    protected override string RepresentationName => "FileDiff";
    protected override string RepresentationNickname => "FD";
    protected override string RepresentationDescription => "File diff";
    protected override string IconResourceName => "filediff_24x24";
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");
    protected override string RepresentationName => "FileDiff";
    protected override string RepresentationNickname => "FD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a file diff.";
    protected override string IconResourceName => "filediff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Folder", "Fo?", "The optional folder id.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Size", "Sz?", "The optional size.", GH_ParamAccess.item);
        pManager.AddTextParameter("Hash", "Hs?", "The optional hash.", GH_ParamAccess.item);
        pManager.AddTextParameter("Blob", "Bl?", "The optional blob.", GH_ParamAccess.item);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional created-by.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updated-by.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Folder", "Fo?", "The optional folder id.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Size", "Sz?", "The optional size.", GH_ParamAccess.item);
        pManager.AddTextParameter("Hash", "Hs?", "The optional hash.", GH_ParamAccess.item);
        pManager.AddTextParameter("Blob", "Bl?", "The optional blob.", GH_ParamAccess.item);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional created-by.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updated-by.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, FileDiff representation)
    {
        string id = null, name = null, remote = null, hash = null, blob = null, createdBy = null, updatedBy = null;
        int size = 0;
        DateTime createdAt = default, updatedAt = default;
        var folder = new FolderIdGoo();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref remote)) representation.Remote = remote;
        if (DA.GetData(5, ref folder)) representation.Folder = folder.Value.DeepClone();
        if (DA.GetData(6, ref size)) representation.Size = size;
        if (DA.GetData(7, ref hash)) representation.Hash = hash;
        if (DA.GetData(8, ref blob)) representation.Blob = blob;
        if (DA.GetData(9, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(10, ref createdBy)) representation.CreatedBy = createdBy;
        if (DA.GetData(11, ref updatedAt)) representation.ModificationdAt = updatedAt;
        if (DA.GetData(12, ref updatedBy)) representation.ModificationdBy = updatedBy;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, FileDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeRemote()) DA.SetData(4, representation.Remote);
        if (representation.ShouldSerializeFolder()) DA.SetData(5, representation.Folder is not null ? new FolderIdGoo(representation.Folder.DeepClone()) : null);
        if (representation.ShouldSerializeSize()) DA.SetData(6, representation.Size);
        if (representation.ShouldSerializeHash()) DA.SetData(7, representation.Hash);
        if (representation.ShouldSerializeBlob()) DA.SetData(8, representation.Blob);
        if (representation.ShouldSerializeCreatedAt()) DA.SetData(9, representation.CreatedAt);
        if (representation.ShouldSerializeCreatedBy()) DA.SetData(10, representation.CreatedBy);
        if (representation.ShouldSerializeModificationdAt()) DA.SetData(11, representation.ModificationdAt);
        if (representation.ShouldSerializeModificationdBy()) DA.SetData(12, representation.ModificationdBy);
    }
}

public class SerializeFileDiffComponent : SerializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public SerializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F2");
}

public class DeserializeFileDiffComponent : DeserializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public DeserializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F3");
}

public class FilesDiffGoo : DiffGoo<FilesDiff>
{
    public FilesDiffGoo() { }
    public FilesDiffGoo(FilesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FilesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FilesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FilesDiffParam : DiffParam<FilesDiffGoo, FilesDiff>
{
    protected override string RepresentationName => "FilesDiff";
    protected override string RepresentationNickname => "FDs";
    protected override string RepresentationDescription => "File collection diff";
    protected override string IconResourceName => "filesdiff_24x24";
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A1");
}

public class FilesDiffComponent : DiffComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A2");
    protected override string RepresentationName => "FilesDiff";
    protected override string RepresentationNickname => "FDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of file diffs.";
    protected override string IconResourceName => "filesdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FileIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed file ids.", GH_ParamAccess.list);
        pManager.AddParameter(new FileDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated file diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new FileParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added files.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FileIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed file ids.", GH_ParamAccess.list);
        pManager.AddParameter(new FileDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated file diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new FileParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added files.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, FilesDiff representation)
    {
        var removed = new List<FileIdGoo>();
        var updated = new List<FileDiffGoo>();
        var added = new List<FileGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new FileDiffUpdate { File = new FileId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, FilesDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new FileIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new FileDiffGoo((u.Diff ?? new FileDiff { Id = u.File.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new FileGoo(a.DeepClone())).ToList());
    }

}

public class SerializeFilesDiffComponent : SerializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public SerializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A3");
}

public class DeserializeFilesDiffComponent : DeserializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public DeserializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A4");
}

#endregion 🪨File

#region 🪩Folder
// Implementations MUST reference a folder with name and optional parent.

public class FolderGoo : Goo<Folder>
{
    public FolderGoo() { }
    public FolderGoo(Folder value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(FolderIdGoo)))
        {
            target = (Q)(object)new FolderIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(FolderDiffGoo)))
        {
            target = (Q)(object)new FolderDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is FolderIdGoo folderIdGoo)
        {
            Value = folderIdGoo.Value;
            return true;
        }
        if (source is FolderDiffGoo folderDiffGoo)
        {
            Value = folderDiffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Folder { Id = str };
            return true;
        }
        return false;
    }
}

public class FolderParam : Param<FolderGoo, Folder>
{
    protected override string RepresentationName => "Folder";
    protected override string RepresentationNickname => "Fld";
    protected override string RepresentationDescription => "Folder container";
    protected override string IconResourceName => "folder_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A0");
}

public class FolderComponent : PassthroughComponent<FolderParam, FolderGoo, Folder>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A1");
    protected override string RepresentationName => "Folder";
    protected override string RepresentationNickname => "Fol";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a folder.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the folder.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Parent", "Pa?", "The optional parent folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional creator.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updater.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the folder.", GH_ParamAccess.item);
        pManager.AddParameter(new FolderIdParam(), "Parent", "Pa?", "The optional parent folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional creator.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updater.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Folder representation)
    {
        string id = "", name = "", description = "", createdBy = "", updatedBy = "";
        var parentIdGoo = new FolderIdGoo();
        DateTime createdAt = default, updatedAt = default;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parentIdGoo)) representation.Parent = parentIdGoo.Value;
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value).ToList();
        if (DA.GetData(7, ref createdAt)) representation.CreatedAt = createdAt.ToString("o");
        if (DA.GetData(8, ref createdBy)) representation.CreatedBy = createdBy;
        if (DA.GetData(9, ref updatedAt)) representation.ModificationdAt = updatedAt.ToString("o");
        if (DA.GetData(10, ref updatedBy)) representation.ModificationdBy = updatedBy;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Folder representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Parent != null ? new FolderIdGoo(representation.Parent) : null);
        DA.SetData(5, representation.Description);
        DA.SetDataList(6, representation.Attributes.Select(a => new AttributeGoo(a)).ToList());
        DA.SetData(7, !string.IsNullOrEmpty(representation.CreatedAt) && DateTime.TryParse(representation.CreatedAt, null, System.Globalization.DateTimeStyles.RoundtripKind, out var ca) ? ca : (DateTime?)null);
        DA.SetData(8, representation.CreatedBy);
        DA.SetData(9, !string.IsNullOrEmpty(representation.ModificationdAt) && DateTime.TryParse(representation.ModificationdAt, null, System.Globalization.DateTimeStyles.RoundtripKind, out var ua) ? ua : (DateTime?)null);
        DA.SetData(10, representation.ModificationdBy);
    }
}

public class SerializeFolderComponent : SerializeComponent<FolderParam, FolderGoo, Folder>
{
    public SerializeFolderComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A2");
}

public class DeserializeFolderComponent : DeserializeComponent<FolderParam, FolderGoo, Folder>
{
    public DeserializeFolderComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A3");
}

public class FolderIdGoo : IdGoo<FolderId>
{
    public FolderIdGoo() { }
    public FolderIdGoo(FolderId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is FolderGoo folderGoo)
        {
            Value = folderGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FolderId { Id = str };
            return true;
        }
        return false;
    }
}

public class FolderIdParam : IdParam<FolderIdGoo, FolderId>
{
    protected override string RepresentationName => "FolderId";
    protected override string RepresentationNickname => "FlI";
    protected override string RepresentationDescription => "Folder identifier";
    protected override string IconResourceName => "folder_24x24";
    protected override string IdIconResourceName => "folderid_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A4");
}

public class FolderDiffGoo : DiffGoo<FolderDiff>
{
    public FolderDiffGoo() { }
    public FolderDiffGoo(FolderDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id ?? "");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FolderDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FolderDiffParam : DiffParam<FolderDiffGoo, FolderDiff>
{
    protected override string RepresentationName => "FolderDiff";
    protected override string RepresentationNickname => "FD";
    protected override string RepresentationDescription => "Folder diff";
    protected override string IconResourceName => "folderdiff_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A5");
}

public class FolderDiffComponent : DiffComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A6");
    protected override string RepresentationName => "FolderDiff";
    protected override string RepresentationNickname => "FD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a folder diff.";
    protected override string IconResourceName => "folderdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Parent", "Pa?", "The optional parent.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTextParameter("CreatedAt", "CA?", "The optional created-at.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional created-by.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedAt", "UA?", "The optional updated-at.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updated-by.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Parent", "Pa?", "The optional parent.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTextParameter("CreatedAt", "CA?", "The optional created-at.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedBy", "CB?", "The optional created-by.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedAt", "UA?", "The optional updated-at.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedBy", "UB?", "The optional updated-by.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, FolderDiff representation)
    {
        string id = null, name = null, parent = null, description = null, createdAt = null, createdBy = null, updatedAt = null, updatedBy = null;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parent)) representation.Parent = string.IsNullOrEmpty(parent) ? null : new FolderId { Id = parent };
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(7, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(8, ref createdBy)) representation.CreatedBy = createdBy;
        if (DA.GetData(9, ref updatedAt)) representation.ModificationdAt = updatedAt;
        if (DA.GetData(10, ref updatedBy)) representation.ModificationdBy = updatedBy;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, FolderDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeParent()) DA.SetData(4, representation.Parent);
        if (representation.ShouldSerializeDescription()) DA.SetData(5, representation.Description);
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(6, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        if (representation.ShouldSerializeCreatedAt()) DA.SetData(7, representation.CreatedAt);
        if (representation.ShouldSerializeCreatedBy()) DA.SetData(8, representation.CreatedBy);
        if (representation.ShouldSerializeModificationdAt()) DA.SetData(9, representation.ModificationdAt);
        if (representation.ShouldSerializeModificationdBy()) DA.SetData(10, representation.ModificationdBy);
    }
}

public class SerializeFolderDiffComponent : SerializeComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public SerializeFolderDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A7");
}

public class DeserializeFolderDiffComponent : DeserializeComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public DeserializeFolderDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A8");
}

public class FoldersDiffGoo : DiffGoo<FoldersDiff>
{
    public FoldersDiffGoo() { }
    public FoldersDiffGoo(FoldersDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FoldersDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FoldersDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FoldersDiffParam : DiffParam<FoldersDiffGoo, FoldersDiff>
{
    protected override string RepresentationName => "FoldersDiff";
    protected override string RepresentationNickname => "FDs";
    protected override string RepresentationDescription => "Folder collection diff";
    protected override string IconResourceName => "foldersdiff_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
}

public class FoldersDiffComponent : DiffComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AA");
    protected override string RepresentationName => "FoldersDiff";
    protected override string RepresentationNickname => "FDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of folder diffs.";
    protected override string IconResourceName => "foldersdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FolderIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed folder ids.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated folder diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added folders.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FolderIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed folder ids.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated folder diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added folders.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, FoldersDiff representation)
    {
        var removed = new List<FolderIdGoo>();
        var updated = new List<FolderDiffGoo>();
        var added = new List<FolderGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new FolderDiffUpdate { Folder = new FolderId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, FoldersDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new FolderIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new FolderDiffGoo((u.Diff ?? new FolderDiff { Id = u.Folder.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new FolderGoo(a.DeepClone())).ToList());
    }

}

public class SerializeFoldersDiffComponent : SerializeComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public SerializeFoldersDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
}

public class DeserializeFoldersDiffComponent : DeserializeComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public DeserializeFoldersDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AC");
}

#endregion 🪩Folder

#region 💾Benchmark
// Implementations MUST capture benchmark metadata for performance measurement.

public class BenchmarkGoo : Goo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Benchmark { Name = str };
            return true;
        }
        return false;
    }
}

public class BenchmarkParam : Param<BenchmarkGoo, Benchmark>
{
    protected override string RepresentationName => "Benchmark";
    protected override string RepresentationNickname => "Bmk";
    protected override string RepresentationDescription => "Performance benchmark";
    protected override string IconResourceName => "benchmark_24x24";
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class BenchmarkComponent : PassthroughComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string RepresentationName => "Benchmark";
    protected override string RepresentationNickname => "Bmk";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a benchmark.";
    protected override string IconResourceName => "benchmark_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the benchmark.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the benchmark.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the benchmark.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the benchmark.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Benchmark representation)
    {
        string id = "", name = "", icon = "";
        double min = 0, max = 0;
        bool minExcluded = false, maxExcluded = false;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref icon)) representation.Icon = icon;
        if (DA.GetData(5, ref min)) representation.Min = (float)min;
        if (DA.GetData(6, ref minExcluded)) representation.MinExcluded = minExcluded;
        if (DA.GetData(7, ref max)) representation.Max = (float)max;
        if (DA.GetData(8, ref maxExcluded)) representation.MaxExcluded = maxExcluded;
        if (DA.GetDataList(9, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Benchmark representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Icon);
        DA.SetData(5, representation.Min);
        DA.SetData(6, representation.MinExcluded);
        DA.SetData(7, representation.Max);
        DA.SetData(8, representation.MaxExcluded);
        DA.SetDataList(9, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeBenchmarkComponent : SerializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public SerializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeBenchmarkComponent : DeserializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public DeserializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 💾Benchmark

#region 🖨️QualityKind
// Implementations MUST categorize quality metrics by kind.

public class QualityKindGoo : EnumGoo<QualityKind>
{
    public QualityKindGoo() { }
    public QualityKindGoo(QualityKind value) : base(value) { }
}

public class QualityKindParam : EnumParam<QualityKindGoo, QualityKind>
{
    public QualityKindParam() : base(new("A1B2C3D4-E5F6-4A5B-9C8D-7E6F5A4B3C2D")) { }
}

#endregion 🖨️QualityKind

#region 🎊Quality
// Implementations MUST combine kind, name, value, and unit for quality metrics.

public class QualityGoo : Goo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityDiffGoo)))
        {
            target = (Q)(object)new QualityDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityIdGoo)))
        {
            target = (Q)(object)new QualityIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is QualityIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Quality>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new((Quality)idGoo.Value);
}

public class QualityParam : Param<QualityGoo, Quality>
{
    protected override string RepresentationName => "Quality";
    protected override string RepresentationNickname => "Qal";
    protected override string RepresentationDescription => "Quality measurement";
    protected override string IconResourceName => "quality_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class QualityComponent : PassthroughComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string RepresentationName => "Quality";
    protected override string RepresentationNickname => "Qal";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a quality.";
    protected override string IconResourceName => "quality_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ky", "The key of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur", "The URI of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Sc?", "Whether the quality is scalable.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kd?", "The quality kind (0=General, 1=Design, 2=Type, 4=Piece, 8=Connection, 16=Connector).", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "The SI unit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Imp", "The imperial unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi", "The minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx", "The maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Df", "The default value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Fm", "The formula.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam() { Access = GH_ParamAccess.list }, "Benchmarks", "Bm*", "The optional benchmarks.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ky", "The key of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur", "The URI of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Sc?", "Whether the quality is scalable.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kd?", "The quality kind (0=General, 1=Design, 2=Type, 4=Piece, 8=Connection, 16=Connector).", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "The SI unit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Imp", "The imperial unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi", "The minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx", "The maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Df", "The default value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Fm", "The formula.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam() { Access = GH_ParamAccess.list }, "Benchmarks", "Bm*", "The optional benchmarks.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Quality representation)
    {
        string id = "", key = "", name = "", description = "", uri = "", folder = "", si = "", imperial = "", formula = "", icon = "", image = "", unit = "";
        bool scalable = false, minExcluded = true, maxExcluded = true;
        int kind = 0;
        double min = 0, max = 0, defaultValue = 0;
        var benchmarks = new List<BenchmarkGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref key)) representation.Key = key;
        if (DA.GetData(4, ref name)) representation.Name = name;
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetData(6, ref uri)) representation.Uri = uri;
        if (DA.GetData(7, ref folder)) representation.Folder = folder;
        if (DA.GetData(8, ref scalable)) representation.Scalable = scalable;
        if (DA.GetData(9, ref kind)) representation.Kind = (QualityKind)kind;
        if (DA.GetData(10, ref si)) representation.SI = si;
        if (DA.GetData(11, ref imperial)) representation.Imperial = imperial;
        if (DA.GetData(12, ref min)) representation.Min = (float)min;
        if (DA.GetData(13, ref minExcluded)) representation.MinExcluded = minExcluded;
        if (DA.GetData(14, ref max)) representation.Max = (float)max;
        if (DA.GetData(15, ref maxExcluded)) representation.MaxExcluded = maxExcluded;
        if (DA.GetData(16, ref defaultValue)) representation.Default = (float)defaultValue;
        if (DA.GetData(17, ref formula)) representation.Formula = formula;
        if (DA.GetData(18, ref icon)) representation.Icon = icon;
        if (DA.GetData(19, ref image)) representation.Image = image;
        if (DA.GetData(20, ref unit)) representation.Unit = unit;
        if (DA.GetDataList(21, benchmarks)) representation.Benchmarks = benchmarks.Select(b => b.Value.DeepClone()).ToList();
        if (DA.GetDataList(22, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Quality representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Key);
        DA.SetData(4, representation.Name);
        DA.SetData(5, representation.Description);
        DA.SetData(6, representation.Uri);
        DA.SetData(7, representation.Folder);
        DA.SetData(8, representation.Scalable);
        DA.SetData(9, (int)representation.Kind);
        DA.SetData(10, representation.SI);
        DA.SetData(11, representation.Imperial);
        DA.SetData(12, representation.Min);
        DA.SetData(13, representation.MinExcluded);
        DA.SetData(14, representation.Max);
        DA.SetData(15, representation.MaxExcluded);
        DA.SetData(16, representation.Default);
        DA.SetData(17, representation.Formula);
        DA.SetData(18, representation.Icon);
        DA.SetData(19, representation.Image);
        DA.SetData(20, representation.Unit);
        DA.SetDataList(21, representation.Benchmarks?.Select(b => new BenchmarkGoo(b.DeepClone())).ToList());
        DA.SetDataList(22, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public SerializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
}

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityGoo)))
        {
            target = (Q)(object)new QualityGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityDiffGoo)))
        {
            target = (Q)(object)new QualityDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityGoo qualityGoo)
        {
            Value = qualityGoo.Value;
            return true;
        }
        if (source is QualityDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<QualityId>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    protected override string RepresentationName => "QualityId";
    protected override string RepresentationNickname => "QId";
    protected override string RepresentationDescription => "Quality identifier";
    protected override string IconResourceName => "quality_24x24";
    protected override string IdIconResourceName => "qualityid_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class SerializeQualityIdComponent : SerializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public SerializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CA");
}

public class DeserializeQualityIdComponent : DeserializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public DeserializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CB");
}

public class QualityDiffGoo : DiffGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityGoo)))
        {
            target = (Q)(object)new QualityGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityIdGoo)))
        {
            target = (Q)(object)new QualityIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityGoo qualityGoo)
        {
            Value = qualityGoo.Value;
            return true;
        }
        if (source is QualityIdGoo qualityIdGoo)
        {
            Value = qualityIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<QualityDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityDiffGoo diffGoo) => new((QualityId)diffGoo.Value);
    public static implicit operator QualityGoo(QualityDiffGoo diffGoo) => new((Quality)diffGoo.Value);
    public static implicit operator QualityDiffGoo(QualityIdGoo idGoo) => new((QualityDiff)idGoo.Value);
    public static implicit operator QualityDiffGoo(QualityGoo goo) => new((QualityDiff)goo.Value);
}

public class QualityDiffParam : DiffParam<QualityDiffGoo, QualityDiff>
{
    protected override string RepresentationName => "QualityDiff";
    protected override string RepresentationNickname => "QD";
    protected override string RepresentationDescription => "Quality diff";
    protected override string IconResourceName => "qualitydiff_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DA");
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DB");
    protected override string RepresentationName => "QualityDiff";
    protected override string RepresentationNickname => "QD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a quality diff.";
    protected override string IconResourceName => "qualitydiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur", "The uri.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Sc", "Whether scalable.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kd", "The quality kind enum value.", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "The SI unit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Im", "The imperial unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi", "The minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx", "The maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Df", "The default value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Fm", "The formula.", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam() { Access = GH_ParamAccess.list }, "Benchmarks", "Bm*", "The optional benchmarks.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur", "The uri.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Sc", "Whether scalable.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kd", "The quality kind enum value.", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "The SI unit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Im", "The imperial unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi", "The minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx", "The maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE", "Whether max is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Df", "The default value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Fm", "The formula.", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam() { Access = GH_ParamAccess.list }, "Benchmarks", "Bm*", "The optional benchmarks.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, QualityDiff representation)
    {
        string id = null, key = null, name = null, description = null, uri = null, si = null, imperial = null, formula = null;
        bool scalable = false, minExcluded = false, maxExcluded = false;
        int kind = 0;
        double min = 0, max = 0, @default = 0;
        var benchmarks = new List<BenchmarkGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref key)) representation.Key = key;
        if (DA.GetData(4, ref name)) representation.Name = name;
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetData(6, ref uri)) representation.Uri = uri;
        if (DA.GetData(7, ref scalable)) representation.Scalable = scalable;
        if (DA.GetData(8, ref kind)) representation.Kind = (QualityKind)kind;
        if (DA.GetData(9, ref si)) representation.SI = si;
        if (DA.GetData(10, ref imperial)) representation.Imperial = imperial;
        if (DA.GetData(11, ref min)) representation.Min = (float)min;
        if (DA.GetData(12, ref minExcluded)) representation.MinExcluded = minExcluded;
        if (DA.GetData(13, ref max)) representation.Max = (float)max;
        if (DA.GetData(14, ref maxExcluded)) representation.MaxExcluded = maxExcluded;
        if (DA.GetData(15, ref @default)) representation.Default = (float)@default;
        if (DA.GetData(16, ref formula)) representation.Formula = formula;
        if (DA.GetDataList(17, benchmarks)) representation.Benchmarks = benchmarks.Select(b => b.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, QualityDiff representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Key);
        DA.SetData(4, representation.Name);
        DA.SetData(5, representation.Description);
        DA.SetData(6, representation.Uri);
        DA.SetData(7, representation.Scalable);
        DA.SetData(8, (int)representation.Kind);
        DA.SetData(9, representation.SI);
        DA.SetData(10, representation.Imperial);
        DA.SetData(11, representation.Min);
        DA.SetData(12, representation.MinExcluded);
        DA.SetData(13, representation.Max);
        DA.SetData(14, representation.MaxExcluded);
        DA.SetData(15, representation.Default);
        DA.SetData(16, representation.Formula);
        DA.SetDataList(17, representation.Benchmarks?.Select(b => new BenchmarkGoo(b.DeepClone())).ToList());
        DA.SetDataList(18, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

}

public class SerializeQualityDiffComponent : SerializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public SerializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DC");
}

public class DeserializeQualityDiffComponent : DeserializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public DeserializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DD");
}

#endregion 🎊Quality

#region 🪄Tag
// Implementations MUST provide lightweight labels for categorizing entities.

public class TagGoo : Goo<Tag>
{
    public TagGoo() { }
    public TagGoo(Tag value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TagIdGoo)))
        {
            target = (Q)(object)new TagIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TagIdGoo tagIdGoo)
        {
            Value = tagIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Tag { Id = str };
            return true;
        }
        return false;
    }
}

public class TagParam : Param<TagGoo, Tag>
{
    protected override string RepresentationName => "Tag";
    protected override string RepresentationNickname => "Tag";
    protected override string RepresentationDescription => "Representation tag";
    protected override string IconResourceName => "tag_24x24";
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class TagComponent : PassthroughComponent<TagParam, TagGoo, Tag>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");
    protected override string RepresentationName => "Tag";
    protected override string RepresentationNickname => "Tag";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a tag.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Tag representation)
    {
        string id = "", name = "", description = "", icon = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref icon)) representation.Icon = icon;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Tag representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetData(5, representation.Icon);
        DA.SetDataList(6, representation.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeTagComponent : SerializeComponent<TagParam, TagGoo, Tag>
{
    public SerializeTagComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");
}

public class DeserializeTagComponent : DeserializeComponent<TagParam, TagGoo, Tag>
{
    public DeserializeTagComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");
}

public class TagIdGoo : IdGoo<TagId>
{
    public TagIdGoo() { }
    public TagIdGoo(TagId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TagGoo tagGoo)
        {
            Value = tagGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new TagId { Id = str };
            return true;
        }
        return false;
    }
}

public class TagIdParam : IdParam<TagIdGoo, TagId>
{
    protected override string RepresentationName => "TagId";
    protected override string RepresentationNickname => "TId";
    protected override string RepresentationDescription => "Tag identifier";
    protected override string IconResourceName => "tag_24x24";
    protected override string IdIconResourceName => "tagid_24x24";
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B4");
}

#endregion 🪄Tag

#region 🎆Prop
// Implementations MUST bind a property name to an expression value.

public class PropGoo : Goo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Quality.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Prop>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PropParam : Param<PropGoo, Prop>
{
    protected override string RepresentationName => "Prop";
    protected override string RepresentationNickname => "Prp";
    protected override string RepresentationDescription => "Connector property";
    protected override string IconResourceName => "prop_24x24";
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class PropComponent : PassthroughComponent<PropParam, PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string RepresentationName => "Prop";
    protected override string RepresentationNickname => "Prp";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a prop.";
    protected override string IconResourceName => "prop_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the prop.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl", "The value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The unit.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the prop.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl", "The value.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The unit.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Prop representation)
    {
        string id = "", value = "", unit = "";
        var quality = new QualityIdGoo();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref quality)) representation.Quality = quality.Value.DeepClone();
        if (DA.GetData(4, ref value)) representation.Value = value;
        if (DA.GetData(5, ref unit)) representation.Unit = unit;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Prop representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, new QualityIdGoo(representation.Quality.DeepClone()));
        DA.SetData(4, representation.Value);
        DA.SetData(5, representation.Unit);
        DA.SetDataList(6, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePropComponent : SerializeComponent<PropParam, PropGoo, Prop>
{
    public SerializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializePropComponent : DeserializeComponent<PropParam, PropGoo, Prop>
{
    public DeserializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🎆Prop

#region 🧊Representation
// Implementations MUST reference a 3D representation with URI, MIME type, and local plane.

public class RepresentationGoo : Goo<Representation>
{
    public RepresentationGoo() { }
    public RepresentationGoo(Representation value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Representation { Id = str };
            return true;
        }
        return false;
    }
}

public class RepresentationParam : Param<RepresentationGoo, Representation>
{
    protected override string RepresentationName => "Representation";
    protected override string RepresentationNickname => "Mdl";
    protected override string RepresentationDescription => "3D representation";
    protected override string IconResourceName => "representation_24x24";
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
}

public class RepresentationComponent : PassthroughComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");
    protected override string RepresentationName => "Representation";
    protected override string RepresentationNickname => "Rep";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a representation.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the representation.", GH_ParamAccess.item);
        pManager.AddParameter(new FileIdParam(), "File", "Fl", "The file of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TagIdParam(), "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the representation.", GH_ParamAccess.item);
        pManager.AddParameter(new FileIdParam(), "File", "Fl", "The file of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TagIdParam(), "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Representation representation)
    {
        string id = "", name = "", description = "";
        var fileIdGoo = new FileIdGoo();
        var tagIdGoos = new List<TagIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref fileIdGoo)) representation.File = fileIdGoo.Value;
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetDataList(6, tagIdGoos)) representation.Tags = tagIdGoos.Select(t => t.Value).ToList();
        if (DA.GetDataList(7, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Representation representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.File is not null ? new FileIdGoo(representation.File) : null);
        DA.SetData(5, representation.Description);
        DA.SetDataList(6, representation.Tags?.Select(t => new TagIdGoo(t)).ToList());
        DA.SetDataList(7, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

    protected override Representation ProcessRepresentation(Representation representation)
    {
        return representation;
    }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public SerializeRepresentationComponent() { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public DeserializeRepresentationComponent() { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");
}

public class RepresentationIdGoo : IdGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(RepresentationDiffGoo)))
        {
            target = (Q)(object)new RepresentationDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(RepresentationGoo)))
        {
            target = (Q)(object)new RepresentationGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is RepresentationDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is RepresentationGoo representationGoo)
        {
            Value = representationGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new RepresentationId { Id = str };
            return true;
        }
        return false;
    }
}

public class RepresentationIdParam : IdParam<RepresentationIdGoo, RepresentationId>
{
    protected override string RepresentationName => "RepresentationId";
    protected override string RepresentationNickname => "MId";
    protected override string RepresentationDescription => "Representation identifier";
    protected override string IconResourceName => "representation_24x24";
    protected override string IdIconResourceName => "representationid_24x24";
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class RepresentationDiffGoo : DiffGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(string.Join(",", Value.Tags));
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<RepresentationDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationDiffParam : DiffParam<RepresentationDiffGoo, RepresentationDiff>
{
    protected override string RepresentationName => "RepresentationDiff";
    protected override string RepresentationNickname => "MD";
    protected override string RepresentationDescription => "Representation diff";
    protected override string IconResourceName => "representationdiff_24x24";
    public override Guid ComponentGuid => new("7C8E9FA0-B1C2-D3E4-F5A6-B7C8D9E0F1A2");
}

public class RepresentationDiffComponent : DiffComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public override Guid ComponentGuid => new("8D9FA0B1-C2D3-E4F5-A6B7-C8D9E0F1A2B3");
    protected override string RepresentationName => "RepresentationDiff";
    protected override string RepresentationNickname => "MD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a representation diff.";
    protected override string IconResourceName => "representationdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new FileIdParam(), "File", "Fi?", "The optional file id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TagIdParam() { Access = GH_ParamAccess.list }, "Tags", "Tg*", "The optional tag ids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new FileIdParam(), "File", "Fi?", "The optional file id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TagIdParam() { Access = GH_ParamAccess.list }, "Tags", "Tg*", "The optional tag ids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, RepresentationDiff representation)
    {
        string id = null, name = null, description = null;
        var file = new FileIdGoo();
        var tags = new List<TagIdGoo>();
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref file)) representation.File = file.Value.DeepClone();
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetDataList(6, tags)) representation.Tags = tags.Select(t => t.Value.DeepClone()).ToList();
        if (DA.GetDataList(7, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, RepresentationDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeFile()) DA.SetData(4, representation.File is not null ? new FileIdGoo(representation.File.DeepClone()) : null);
        if (representation.ShouldSerializeDescription()) DA.SetData(5, representation.Description);
        if (representation.ShouldSerializeTags()) DA.SetDataList(6, representation.Tags?.Select(t => new TagIdGoo(t.DeepClone())).ToList());
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(7, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeRepresentationDiffComponent : SerializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public SerializeRepresentationDiffComponent() { }
    public override Guid ComponentGuid => new("71E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
}

public class DeserializeRepresentationDiffComponent : DeserializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public DeserializeRepresentationDiffComponent() { }
    public override Guid ComponentGuid => new("AFB1C2D3-E4F5-A6B7-C8D9-E0F1A2B3C4D5");
}

public class RepresentationsDiffGoo : DiffGoo<RepresentationsDiff>
{
    public RepresentationsDiffGoo() { }
    public RepresentationsDiffGoo(RepresentationsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("RepresentationsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<RepresentationsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationsDiffParam : DiffParam<RepresentationsDiffGoo, RepresentationsDiff>
{
    protected override string RepresentationName => "RepresentationsDiff";
    protected override string RepresentationNickname => "MDs";
    protected override string RepresentationDescription => "Representation collection diff";
    protected override string IconResourceName => "representationsdiff_24x24";
    public override Guid ComponentGuid => new("9EA0B1C2-D3E4-F5A6-B7C8-D9E0F1A2B3C4");
}

public class RepresentationsDiffComponent : DiffComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AD");
    protected override string RepresentationName => "RepresentationsDiff";
    protected override string RepresentationNickname => "MDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of representation diffs.";
    protected override string IconResourceName => "representationsdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed representation ids.", GH_ParamAccess.list);
        pManager.AddParameter(new RepresentationDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated representation diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new RepresentationParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added representations.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed representation ids.", GH_ParamAccess.list);
        pManager.AddParameter(new RepresentationDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated representation diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new RepresentationParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added representations.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, RepresentationsDiff representation)
    {
        var removed = new List<RepresentationIdGoo>();
        var updated = new List<RepresentationDiffGoo>();
        var added = new List<RepresentationGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new RepresentationDiffUpdate { Representation = new RepresentationId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, RepresentationsDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new RepresentationIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new RepresentationDiffGoo((u.Diff ?? new RepresentationDiff { Id = u.Representation.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new RepresentationGoo(a.DeepClone())).ToList());
    }

}

public class SerializeRepresentationsDiffComponent : SerializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public SerializeRepresentationsDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AE");
}

public class DeserializeRepresentationsDiffComponent : DeserializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public DeserializeRepresentationsDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AF");
}

#endregion 🧊Representation

#region 🦀Connector
// Implementations MUST define located interface points on a type.

public class ConnectorGoo : Goo<Connector>
{
    public ConnectorGoo() { }
    public ConnectorGoo(Connector value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            if (Value.Direction is null || Value.Point is null) return false;
            target = (Q)(object)new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorIdGoo)))
        {
            target = (Q)(object)new ConnectorIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorDiffGoo)))
        {
            target = (Q)(object)new ConnectorDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is ConnectorDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        var plane = new Rhino.Geometry.Plane();
        if (GH_Convert.ToPlane(source, ref plane, GH_Conversion.Both))
        {
            Value.Point = plane.Origin.Convert();
            Value.Direction = plane.YAxis.Convert();
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            var deserialized = str.Deserialize<Connector>();
            if (deserialized is null) return false;
            Value = deserialized;
            return true;
        }
        return false;
    }
}

public class ConnectorParam : Param<ConnectorGoo, Connector>
{
    protected override string RepresentationName => "Connector";
    protected override string RepresentationNickname => "Con";
    protected override string RepresentationDescription => "Connection point";
    protected override string IconResourceName => "connector_24x24";
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
}

public class ConnectorComponent : PassthroughComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
    protected override string RepresentationName => "Connector";
    protected override string RepresentationNickname => "Por";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a connector.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether the connector is mandatory.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Port", "Pt?", "The optional port.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pn", "The connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Dr", "The direction of the connector.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The t parameter [0,1[.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether the connector is mandatory.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Port", "Pt?", "The optional port.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pn", "The connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Dr", "The direction of the connector.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The t parameter [0,1[.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Connector representation)
    {
        string id = "", name = "", description = "";
        bool mandatory = false;
        var port = new PortIdGoo();
        Point3d point = Point3d.Origin;
        Vector3d direction = Vector3d.YAxis;
        double t = 0;
        var props = new List<PropGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref mandatory)) representation.Mandatory = mandatory;
        if (DA.GetData(6, ref port)) representation.Port = port.Value.DeepClone();
        if (DA.GetData(7, ref point)) representation.Point = RhinoConverter.Convert(point);
        if (DA.GetData(8, ref direction)) representation.Direction = RhinoConverter.Convert(direction);
        if (DA.GetData(9, ref t)) representation.T = (float)t;
        if (DA.GetDataList(10, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(11, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Connector representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetData(5, representation.Mandatory);
        DA.SetData(6, representation.Port is not null ? new PortIdGoo(representation.Port.DeepClone()) : null);
        DA.SetData(7, representation.Point is not null ? RhinoConverter.Convert(representation.Point) : Point3d.Origin);
        DA.SetData(8, representation.Direction is not null ? RhinoConverter.Convert(representation.Direction) : Vector3d.YAxis);
        DA.SetData(9, representation.T);
        DA.SetDataList(10, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        DA.SetDataList(11, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeConnectorComponent : SerializeComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public SerializeConnectorComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
}

public class DeserializeConnectorComponent : DeserializeComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public DeserializeConnectorComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
}

public class ConnectorIdGoo : IdGoo<ConnectorId>
{
    public ConnectorIdGoo() { }
    public ConnectorIdGoo(ConnectorId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorDiffGoo)))
        {
            target = (Q)(object)new ConnectorDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorGoo)))
        {
            target = (Q)(object)new ConnectorGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorGoo connectorGoo)
        {
            Value = connectorGoo.Value;
            return true;
        }
        if (source is ConnectorDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new ConnectorId { Id = str };
            return true;
        }
        return false;
    }
}

public class ConnectorIdParam : IdParam<ConnectorIdGoo, ConnectorId>
{
    protected override string RepresentationName => "ConnectorId";
    protected override string RepresentationNickname => "CId";
    protected override string RepresentationDescription => "Connector identifier";
    protected override string IconResourceName => "connector_24x24";
    protected override string IdIconResourceName => "connectorid_24x24";
    public override Guid ComponentGuid => new("C1D2E3F4-A5B6-C7D8-E9F0-A1B2C3D4E5F6");
}

public class ConnectorDiffGoo : DiffGoo<ConnectorDiff>
{
    public ConnectorDiffGoo() { }
    public ConnectorDiffGoo(ConnectorDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorIdGoo)))
        {
            target = (Q)(object)new ConnectorIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorGoo)))
        {
            target = (Q)(object)new ConnectorGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is ConnectorGoo connectorGoo)
        {
            Value = connectorGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectorDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectorDiffParam : DiffParam<ConnectorDiffGoo, ConnectorDiff>
{
    protected override string RepresentationName => "ConnectorDiff";
    protected override string RepresentationNickname => "CD";
    protected override string RepresentationDescription => "Connector diff";
    protected override string IconResourceName => "connectordiff_24x24";
    public override Guid ComponentGuid => new("B0C1D2E3-F4A5-B6C7-D8E9-F0A1B2C3D4E5");
}

public class ConnectorDiffComponent : DiffComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public override Guid ComponentGuid => new("E3F4A5B6-C7D8-E9F0-A1B2-C3D4E5F6A7B8");
    protected override string RepresentationName => "ConnectorDiff";
    protected override string RepresentationNickname => "CD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a connector diff.";
    protected override string IconResourceName => "connectordiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Port", "Po?", "The optional port id.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether mandatory.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T?", "The optional t value.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pt?", "The optional point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Di?", "The optional direction.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Port", "Po?", "The optional port id.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether mandatory.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T?", "The optional t value.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pt?", "The optional point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Di?", "The optional direction.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, ConnectorDiff representation)
    {
        string id = null, name = null, description = null;
        bool mandatory = false;
        double t = 0;
        Point3d point = Point3d.Origin;
        Vector3d direction = Vector3d.YAxis;
        var port = new PortIdGoo();
        var props = new List<PropGoo>();
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref port)) representation.Port = port.Value.DeepClone();
        if (DA.GetData(6, ref mandatory)) representation.Mandatory = mandatory;
        if (DA.GetData(7, ref t)) representation.T = (float)t;
        if (DA.GetData(8, ref point)) representation.Point = point.Convert();
        if (DA.GetData(9, ref direction)) representation.Direction = direction.Convert();
        if (DA.GetDataList(10, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(11, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, ConnectorDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeDescription()) DA.SetData(4, representation.Description);
        if (representation.ShouldSerializePort()) DA.SetData(5, representation.Port is not null ? new PortIdGoo(representation.Port.DeepClone()) : null);
        if (representation.ShouldSerializeMandatory()) DA.SetData(6, representation.Mandatory);
        if (representation.ShouldSerializeT()) DA.SetData(7, representation.T);
        if (representation.ShouldSerializePoint()) DA.SetData(8, representation.Point is not null ? representation.Point.Convert() : Point3d.Origin);
        if (representation.ShouldSerializeDirection()) DA.SetData(9, representation.Direction is not null ? representation.Direction.Convert() : Vector3d.YAxis);
        if (representation.ShouldSerializeProps()) DA.SetDataList(10, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(11, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePortDiffComponent : SerializeComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public SerializePortDiffComponent() { }
    public override Guid ComponentGuid => new("F4A5B6C7-D8E9-F0A1-B2C3-D4E5F6A7B8C9");
}

public class DeserializePortDiffComponent : DeserializeComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public DeserializePortDiffComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B5");
}

public class ConnectorsDiffGoo : DiffGoo<ConnectorsDiff>
{
    public ConnectorsDiffGoo() { }
    public ConnectorsDiffGoo(ConnectorsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectorsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectorsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectorsDiffParam : DiffParam<ConnectorsDiffGoo, ConnectorsDiff>
{
    protected override string RepresentationName => "ConnectorsDiff";
    protected override string RepresentationNickname => "CDs";
    protected override string RepresentationDescription => "Connector collection diff";
    protected override string IconResourceName => "connectorsdiff_24x24";
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C0");
}

public class ConnectorsDiffComponent : DiffComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C1");
    protected override string RepresentationName => "ConnectorsDiff";
    protected override string RepresentationNickname => "CDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of connector diffs.";
    protected override string IconResourceName => "connectorsdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed connector ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated connector diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added connectors.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed connector ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated connector diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added connectors.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, ConnectorsDiff representation)
    {
        var removed = new List<ConnectorIdGoo>();
        var updated = new List<ConnectorDiffGoo>();
        var added = new List<ConnectorGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new ConnectorDiffUpdate { Connector = new ConnectorId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, ConnectorsDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new ConnectorIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new ConnectorDiffGoo((u.Diff ?? new ConnectorDiff { Id = u.Connector.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new ConnectorGoo(a.DeepClone())).ToList());
    }

}

public class SerializePortsDiffComponent : SerializeComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public SerializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C2");
}

public class DeserializePortsDiffComponent : DeserializeComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public DeserializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C3");
}

#endregion 🦀Connector

#region 🎑Concept
// Implementations MUST link a semantic concept name to description and icon.

public class ConceptGoo : Goo<Concept>
{
    public ConceptGoo() { }
    public ConceptGoo(Concept value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConceptIdGoo)))
        {
            target = (Q)(object)new ConceptIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConceptIdGoo conceptIdGoo)
        {
            Value = conceptIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Concept { Id = str };
            return true;
        }
        return false;
    }
}

public class ConceptParam : Param<ConceptGoo, Concept>
{
    protected override string RepresentationName => "Concept";
    protected override string RepresentationNickname => "Cpt";
    protected override string RepresentationDescription => "Semantic concept";
    protected override string IconResourceName => "concept_24x24";
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class ConceptComponent : PassthroughComponent<ConceptParam, ConceptGoo, Concept>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
    protected override string RepresentationName => "Concept";
    protected override string RepresentationNickname => "Con";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a concept.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Concept representation)
    {
        string id = "", name = "", description = "", icon = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref icon)) representation.Icon = icon;
        if (DA.GetDataList(6, attributes)) representation.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Concept representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetData(5, representation.Icon);
        DA.SetDataList(6, representation.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeConceptComponent : SerializeComponent<ConceptParam, ConceptGoo, Concept>
{
    public SerializeConceptComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");
}

public class DeserializeConceptComponent : DeserializeComponent<ConceptParam, ConceptGoo, Concept>
{
    public DeserializeConceptComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");
}

public class ConceptIdGoo : IdGoo<ConceptId>
{
    public ConceptIdGoo() { }
    public ConceptIdGoo(ConceptId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new ConceptId { Id = str };
            return true;
        }
        return false;
    }
}

public class ConceptIdParam : IdParam<ConceptIdGoo, ConceptId>
{
    protected override string RepresentationName => "ConceptId";
    protected override string RepresentationNickname => "CId";
    protected override string RepresentationDescription => "Concept identifier";
    protected override string IconResourceName => "concept_24x24";
    protected override string IdIconResourceName => "conceptid_24x24";
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C5");
}

#endregion 🎑Concept

#region 🎀Port
// Implementations MUST define connection ports as typed interfaces on a type.

public class PortGoo : Goo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PortIdGoo portIdGoo)
        {
            Value = portIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Port { Id = str };
            return true;
        }
        return false;
    }
}

public class PortParam : Param<PortGoo, Port>
{
    protected override string RepresentationName => "Port";
    protected override string RepresentationNickname => "Ifc";
    protected override string RepresentationDescription => "Connector compatibility";
    protected override string IconResourceName => "interface_24x24";
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D2");
}

public class PortComponent : PassthroughComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D3");
    protected override string RepresentationName => "Port";
    protected override string RepresentationNickname => "Ifc";
    protected override string RepresentationDescription => "Construct, deconstruct or modify an port.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam() { Access = GH_ParamAccess.list }, "CompatiblePorts", "CF*", "The optional compatible ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam() { Access = GH_ParamAccess.list }, "CompatiblePorts", "CF*", "The optional compatible ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Port representation)
    {
        string id = "", name = "", description = "", icon = "";
        var compatiblePorts = new List<PortIdGoo>();
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref icon)) representation.Icon = icon;
        if (DA.GetDataList(6, compatiblePorts)) representation.CompatiblePorts = compatiblePorts.Select(i => i.Value).ToList();
        if (DA.GetDataList(7, attributes)) representation.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Port representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetData(5, representation.Icon);
        DA.SetDataList(6, representation.CompatiblePorts.Select(i => new PortIdGoo(i)).ToList());
        DA.SetDataList(7, representation.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public SerializePortComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public DeserializePortComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");
}

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new PortId { Id = str };
            return true;
        }
        return false;
    }
}

public class PortIdParam : IdParam<PortIdGoo, PortId>
{
    protected override string RepresentationName => "PortId";
    protected override string RepresentationNickname => "IId";
    protected override string RepresentationDescription => "Port identifier";
    protected override string IconResourceName => "interface_24x24";
    protected override string IdIconResourceName => "interfaceid_24x24";
    public override Guid ComponentGuid => new("78187B1A-F476-44D9-A382-DE2C47019DB8");
}

#endregion 🎀Port

#region 🤖Type
// Implementations MUST compose ports, connectors, and representations into a parametric type.

public class TypeGoo : Goo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeDiffGoo)))
        {
            target = (Q)(object)new TypeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeIdGoo)))
        {
            target = (Q)(object)new TypeIdGoo(Value);
            return true;
        }
        if (target is PieceGoo piece)
        {
            piece.Value = new Piece
            {
                Id = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Id = Value.Id }
            };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is TypeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PieceGoo piece)
        {
            if (piece.Value.Type is null) return false;
            Value = new Type { Id = piece.Value.Type.Id, Name = piece.Value.Type.Id };
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Type { Name = str };
            return true;
        }
        return false;
    }
}

public class TypeParam : Param<TypeGoo, Type>
{
    protected override string RepresentationName => "Type";
    protected override string RepresentationNickname => "Typ";
    protected override string RepresentationDescription => "Reusable component";
    protected override string IconResourceName => "type_24x24";
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
}

public class TypeComponent : PassthroughComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");
    protected override string RepresentationName => "Type";
    protected override string RepresentationNickname => "Typ";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a type.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Parent", "Pr?", "The optional parent type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the type is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "St?", "The stock quantity.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vi?", "Whether the type is virtual.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur?", "The optional URI.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Representations", "Md*", "The optional representations.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam(), "Connectors", "Co*", "The optional connectors.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam(), "Concepts", "Cp*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Parent", "Pr?", "The optional parent type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the type is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "St?", "The stock quantity.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vi?", "Whether the type is virtual.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur?", "The optional URI.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Representations", "Md*", "The optional representations.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam(), "Connectors", "Co*", "The optional connectors.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam(), "Concepts", "Cp*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional creation timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional update timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Type representation)
    {
        string id = "", name = "", folder = "", description = "", icon = "", image = "", uri = "", unit = "";
        DateTime createdAt = default, updatedAt = default;
        var parent = new TypeIdGoo();
        bool isAbstract = false, virtual_ = false;
        int stock = 0;
        var location = new LocationGoo();
        var representations = new List<RepresentationGoo>();
        var connectors = new List<ConnectorGoo>();
        var authors = new List<AuthorIdGoo>();
        var attributes = new List<AttributeGoo>();
        var concepts = new List<ConceptIdGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parent)) representation.Parent = parent.Value.DeepClone();
        if (DA.GetData(5, ref isAbstract)) representation.IsAbstract = isAbstract;
        if (DA.GetData(6, ref folder)) representation.Folder = folder;
        if (DA.GetData(7, ref description)) representation.Description = description;
        if (DA.GetData(8, ref icon)) representation.Icon = icon;
        if (DA.GetData(9, ref image)) representation.Image = image;
        if (DA.GetData(10, ref stock)) representation.Stock = stock;
        if (DA.GetData(11, ref virtual_)) representation.Virtual = virtual_;
        if (DA.GetData(12, ref uri)) representation.Uri = uri;
        if (DA.GetData(13, ref location)) representation.Location = location.Value.DeepClone();
        if (DA.GetData(14, ref unit)) representation.Unit = unit;
        if (DA.GetDataList(15, representations)) representation.Representations = representations.Select(m => m.Value.DeepClone()).ToList();
        if (DA.GetDataList(16, connectors)) representation.Connectors = connectors.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(17, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(19, concepts)) representation.Concepts = concepts.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetData(20, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(21, ref updatedAt)) representation.ModificationdAt = updatedAt;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Type representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Parent is not null ? new TypeIdGoo(representation.Parent.DeepClone()) : null);
        DA.SetData(5, representation.IsAbstract);
        DA.SetData(6, representation.Folder);
        DA.SetData(7, representation.Description);
        DA.SetData(8, representation.Icon);
        DA.SetData(9, representation.Image);
        DA.SetData(10, representation.Stock);
        DA.SetData(11, representation.Virtual);
        DA.SetData(12, representation.Uri);
        DA.SetData(13, representation.Location is not null ? new LocationGoo(representation.Location.DeepClone()) : null);
        DA.SetData(14, representation.Unit);
        DA.SetDataList(15, representation.Representations?.Select(m => new RepresentationGoo(m.DeepClone())).ToList());
        DA.SetDataList(16, representation.Connectors?.Select(p => new ConnectorGoo(p.DeepClone())).ToList());
        DA.SetDataList(17, representation.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        DA.SetDataList(18, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        DA.SetDataList(19, representation.Concepts?.Select(c => new ConceptIdGoo(c.DeepClone())).ToList());
        DA.SetData(20, representation.CreatedAt);
        DA.SetData(21, representation.ModificationdAt);
    }

    protected override Type ProcessRepresentation(Type type)
    {
        if (type.Unit == "")
            try { type.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { type.Unit = "m"; }

        type.Icon = type.Icon?.Replace('\\', '/');
        type.Image = type.Image?.Replace('\\', '/');
        return type;
    }
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public SerializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public DeserializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673B");
}

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeGoo)))
        {
            target = (Q)(object)new TypeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeDiffGoo)))
        {
            target = (Q)(object)new TypeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeGoo typeGoo)
        {
            Value = typeGoo.Value;
            return true;
        }
        if (source is TypeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new TypeId { Id = str };
            return true;
        }
        return false;
    }
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    protected override string RepresentationName => "TypeId";
    protected override string RepresentationNickname => "TId";
    protected override string RepresentationDescription => "Type identifier";
    protected override string IconResourceName => "type_24x24";
    protected override string IdIconResourceName => "typeid_24x24";
    public override Guid ComponentGuid => new("A1B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeGoo)))
        {
            target = (Q)(object)new TypeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeIdGoo)))
        {
            target = (Q)(object)new TypeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeGoo typeGoo)
        {
            Value = typeGoo.Value;
            return true;
        }
        if (source is TypeIdGoo typeIdGoo)
        {
            Value = typeIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<TypeDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypeDiffParam : DiffParam<TypeDiffGoo, TypeDiff>
{
    protected override string RepresentationName => "TypeDiff";
    protected override string RepresentationNickname => "TD";
    protected override string RepresentationDescription => "Type diff";
    protected override string IconResourceName => "typediff_24x24";
    public override Guid ComponentGuid => new("C3D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("D4E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
    protected override string RepresentationName => "TypeDiff";
    protected override string RepresentationNickname => "TD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a type diff.";
    protected override string IconResourceName => "typediff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Parent", "Pa?", "The optional parent type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "Sk?", "The optional stock.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vr?", "Whether virtual.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur?", "The optional uri.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationsDiffParam(), "Representations", "Md?", "The optional representations diff.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorsDiffParam(), "Connectors", "Cn?", "The optional connectors diff.", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorIdParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam() { Access = GH_ParamAccess.list }, "Concepts", "Cp*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Parent", "Pa?", "The optional parent type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "Sk?", "The optional stock.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vr?", "Whether virtual.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Ur?", "The optional uri.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationsDiffParam(), "Representations", "Md?", "The optional representations diff.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorsDiffParam(), "Connectors", "Cn?", "The optional connectors diff.", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorIdParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam() { Access = GH_ParamAccess.list }, "Concepts", "Cp*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, TypeDiff representation)
    {
        string id = null, name = null, folder = null, description = null, icon = null, image = null, uri = null, unit = null;
        int stock = 0;
        bool isAbstract = false, virtualValue = false;
        DateTime createdAt = default, updatedAt = default;
        var parent = new TypeIdGoo();
        var location = new LocationGoo();
        var representations = new RepresentationsDiffGoo();
        var connectors = new ConnectorsDiffGoo();
        var authors = new List<AuthorIdGoo>();
        var attributes = new List<AttributeGoo>();
        var concepts = new List<ConceptIdGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parent)) representation.Parent = parent.Value.DeepClone();
        if (DA.GetData(5, ref isAbstract)) representation.IsAbstract = isAbstract;
        if (DA.GetData(6, ref folder)) representation.Folder = folder;
        if (DA.GetData(7, ref description)) representation.Description = description;
        if (DA.GetData(8, ref icon)) representation.Icon = icon;
        if (DA.GetData(9, ref image)) representation.Image = image;
        if (DA.GetData(10, ref stock)) representation.Stock = stock;
        if (DA.GetData(11, ref virtualValue)) representation.Virtual = virtualValue;
        if (DA.GetData(12, ref uri)) representation.Uri = uri;
        if (DA.GetData(13, ref unit)) representation.Unit = unit;
        if (DA.GetData(14, ref location)) representation.Location = location.Value.DeepClone();
        if (DA.GetData(15, ref representations)) representation.Representations = representations.Value.DeepClone();
        if (DA.GetData(16, ref connectors)) representation.Connectors = connectors.Value.DeepClone();
        if (DA.GetDataList(17, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(19, concepts)) representation.Concepts = concepts.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetData(20, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(21, ref updatedAt)) representation.ModificationdAt = updatedAt;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, TypeDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeParent()) DA.SetData(4, representation.Parent is not null ? new TypeIdGoo(representation.Parent.DeepClone()) : null);
        if (representation.ShouldSerializeIsAbstract()) DA.SetData(5, representation.IsAbstract);
        if (representation.ShouldSerializeFolder()) DA.SetData(6, representation.Folder);
        if (representation.ShouldSerializeDescription()) DA.SetData(7, representation.Description);
        if (representation.ShouldSerializeIcon()) DA.SetData(8, representation.Icon);
        if (representation.ShouldSerializeImage()) DA.SetData(9, representation.Image);
        if (representation.ShouldSerializeStock()) DA.SetData(10, representation.Stock);
        if (representation.ShouldSerializeVirtual()) DA.SetData(11, representation.Virtual);
        if (representation.ShouldSerializeUri()) DA.SetData(12, representation.Uri);
        if (representation.ShouldSerializeUnit()) DA.SetData(13, representation.Unit);
        if (representation.ShouldSerializeLocation()) DA.SetData(14, representation.Location is not null ? new LocationGoo(representation.Location.DeepClone()) : null);
        if (representation.ShouldSerializeRepresentations()) DA.SetData(15, representation.Representations is not null ? new RepresentationsDiffGoo(representation.Representations.DeepClone()) : null);
        if (representation.ShouldSerializeConnectors()) DA.SetData(16, representation.Connectors is not null ? new ConnectorsDiffGoo(representation.Connectors.DeepClone()) : null);
        if (representation.ShouldSerializeAuthors()) DA.SetDataList(17, representation.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(18, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        if (representation.ShouldSerializeConcepts()) DA.SetDataList(19, representation.Concepts?.Select(c => new ConceptIdGoo(c.DeepClone())).ToList());
        if (representation.ShouldSerializeCreatedAt()) DA.SetData(20, representation.CreatedAt);
        if (representation.ShouldSerializeModificationdAt()) DA.SetData(21, representation.ModificationdAt);
    }
}

public class SerializeTypeDiffComponent : SerializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public SerializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("E5F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class DeserializeTypeDiffComponent : DeserializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public DeserializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C6");
}

public class TypesDiffGoo : DiffGoo<TypesDiff>
{
    public TypesDiffGoo() { }
    public TypesDiffGoo(TypesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("TypesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<TypesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypesDiffParam : DiffParam<TypesDiffGoo, TypesDiff>
{
    protected override string RepresentationName => "TypesDiff";
    protected override string RepresentationNickname => "TDs";
    protected override string RepresentationDescription => "Type collection diff";
    protected override string IconResourceName => "typesdiff_24x24";
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B6");
}

public class TypesDiffComponent : DiffComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B7");
    protected override string RepresentationName => "TypesDiff";
    protected override string RepresentationNickname => "TDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of type diffs.";
    protected override string IconResourceName => "typesdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed type ids.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated type diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added types.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed type ids.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated type diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added types.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, TypesDiff representation)
    {
        var removed = new List<TypeIdGoo>();
        var updated = new List<TypeDiffGoo>();
        var added = new List<TypeGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new TypeDiffUpdate { Type = new TypeId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, TypesDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new TypeIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new TypeDiffGoo((u.Diff ?? new TypeDiff { Id = u.Type.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new TypeGoo(a.DeepClone())).ToList());
    }

}

public class SerializeTypesDiffComponent : SerializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public SerializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B8");
}

public class DeserializeTypesDiffComponent : DeserializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public DeserializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B9");
}

#endregion 🤖Type

#region ⏳Layer
// Implementations MUST organize pieces into named layers within a design.

public class LayerGoo : Goo<Layer>
{
    public LayerGoo() { }
    public LayerGoo(Layer value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Layer>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class LayerParam : Param<LayerGoo, Layer>
{
    protected override string RepresentationName => "Layer";
    protected override string RepresentationNickname => "Lyr";
    protected override string RepresentationDescription => "Design layer";
    protected override string IconResourceName => "layer_24x24";
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class LayerComponent : PassthroughComponent<LayerParam, LayerGoo, Layer>
{
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string RepresentationName => "Layer";
    protected override string RepresentationNickname => "Lyr";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a layer.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the layer.", GH_ParamAccess.item);
        pManager.AddTextParameter("Path", "Pa", "The path of the layer.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the layer is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the layer is locked.", GH_ParamAccess.item);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the layer.", GH_ParamAccess.item);
        pManager.AddTextParameter("Path", "Pa", "The path of the layer.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the layer is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the layer is locked.", GH_ParamAccess.item);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Layer representation)
    {
        string id = "", path = "", description = "";
        Color color = Color.Transparent;
        bool isHidden = false, isLocked = false;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref path)) representation.Path = path;
        if (DA.GetData(4, ref isHidden)) representation.IsHidden = isHidden;
        if (DA.GetData(5, ref isLocked)) representation.IsLocked = isLocked;
        if (DA.GetData(6, ref color)) representation.Color = RhinoConverter.ColorToHex(color);
        if (DA.GetData(7, ref description)) representation.Description = description;
        if (DA.GetDataList(8, attributes)) representation.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Layer representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Path);
        DA.SetData(4, representation.IsHidden);
        DA.SetData(5, representation.IsLocked);
        DA.SetData(6, RhinoConverter.HexToColor(representation.Color));
        DA.SetData(7, representation.Description);
        DA.SetDataList(8, representation.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeLayerComponent : SerializeComponent<LayerParam, LayerGoo, Layer>
{
    public SerializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeLayerComponent : DeserializeComponent<LayerParam, LayerGoo, Layer>
{
    public DeserializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion ⏳Layer

#region 🔍Group
// Implementations MUST group pieces by name within a design.

public class GroupGoo : Goo<SemioGroup>
{
    public GroupGoo() { }
    public GroupGoo(SemioGroup value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Group>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class GroupParam : Param<GroupGoo, SemioGroup>
{
    protected override string RepresentationName => "Group";
    protected override string RepresentationNickname => "Grp";
    protected override string RepresentationDescription => "Piece grouping";
    protected override string IconResourceName => "group_24x24";
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class GroupComponent : PassthroughComponent<GroupParam, GroupGoo, SemioGroup>
{
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string RepresentationName => "Group";
    protected override string RepresentationNickname => "Grp";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a group.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pc*", "The pieces in the group.", GH_ParamAccess.list);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pc*", "The pieces in the group.", GH_ParamAccess.list);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Group representation)
    {
        string id = "", name = "", description = "";
        Color color = Color.Transparent;
        var pieces = new List<PieceIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetDataList(5, pieces)) representation.Pieces = pieces.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetData(6, ref color)) representation.Color = RhinoConverter.ColorToHex(color);
        if (DA.GetDataList(7, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Group representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetDataList(5, representation.Pieces?.Select(p => new PieceIdGoo(p.DeepClone())).ToList());
        DA.SetData(6, RhinoConverter.HexToColor(representation.Color));
        DA.SetDataList(7, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeGroupComponent : SerializeComponent<GroupParam, GroupGoo, Group>
{
    public SerializeGroupComponent() { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeGroupComponent : DeserializeComponent<GroupParam, GroupGoo, Group>
{
    public DeserializeGroupComponent() { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔍Group

#region 🎈Piece
// Implementations MUST place an instantiated type within a design hierarchy.

public class PieceGoo : Goo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceDiffGoo)))
        {
            target = (Q)(object)new PieceDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceIdGoo)))
        {
            target = (Q)(object)new PieceIdGoo(Value);
            return true;
        }
        if (target is TypeGoo type)
        {
            if (Value.Type is null) return false;
            type.Value = new Type { Id = Value.Type.Id, Name = Value.Type.Id };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is PieceIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is TypeGoo type)
        {
            Value = new Piece
            {
                Id = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Id = type.Value.Id }
            };
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Piece { Id = str };
            return true;
        }
        return false;
    }
}

public class PieceParam : Param<PieceGoo, Piece>
{
    protected override string RepresentationName => "Piece";
    protected override string RepresentationNickname => "Pce";
    protected override string RepresentationDescription => "Design instance";
    protected override string IconResourceName => "piece_24x24";
    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");
}

public class PieceComponent : PassthroughComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");
    protected override string RepresentationName => "Piece";
    protected override string RepresentationNickname => "Pce";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a piece.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Design", "Dn?", "The optional design of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordinateParam(), "Center", "Ce?", "The optional center in the diagram.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scale", "Sc?", "The optional scale factor.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("MirrorPlane", "MP?", "The optional mirror plane.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the piece is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the piece is locked.", GH_ParamAccess.item);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pp*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Design", "Dn?", "The optional design of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordinateParam(), "Center", "Ce?", "The optional center in the diagram.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scale", "Sc?", "The optional scale factor.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("MirrorPlane", "MP?", "The optional mirror plane.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the piece is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the piece is locked.", GH_ParamAccess.item);
        pManager.AddColourParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pp*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Piece representation)
    {
        string id = "", name = "", description = "";
        Color color = Color.Transparent;
        var type = new TypeIdGoo();
        var design = new DesignIdGoo();
        Rhino.Geometry.Plane plane = Rhino.Geometry.Plane.WorldXY;
        var center = new CoordinateGoo();
        double scale = 0;
        Rhino.Geometry.Plane mirrorPlane = Rhino.Geometry.Plane.WorldXY;
        bool isHidden = false, isLocked = false;
        var props = new List<PropGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref type)) representation.Type = type.Value.DeepClone();
        if (DA.GetData(6, ref design)) representation.Design = design.Value.DeepClone();
        if (DA.GetData(7, ref plane)) representation.Plane = RhinoConverter.Convert(plane);
        if (DA.GetData(8, ref center)) representation.Center = center.Value.DeepClone();
        if (DA.GetData(9, ref scale)) representation.Scale = (float)scale;
        if (DA.GetData(10, ref mirrorPlane)) representation.MirrorPlane = RhinoConverter.Convert(mirrorPlane);
        if (DA.GetData(11, ref isHidden)) representation.IsHidden = isHidden;
        if (DA.GetData(12, ref isLocked)) representation.IsLocked = isLocked;
        if (DA.GetData(13, ref color)) representation.Color = RhinoConverter.ColorToHex(color);
        if (DA.GetDataList(14, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(15, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Piece representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Description);
        DA.SetData(5, representation.Type is not null ? new TypeIdGoo(representation.Type.DeepClone()) : null);
        DA.SetData(6, representation.Design is not null ? new DesignIdGoo(representation.Design.DeepClone()) : null);
        DA.SetData(7, representation.Plane is not null ? RhinoConverter.Convert(representation.Plane) : Rhino.Geometry.Plane.WorldXY);
        DA.SetData(8, representation.Center is not null ? new CoordinateGoo(representation.Center.DeepClone()) : null);
        DA.SetData(9, representation.Scale);
        DA.SetData(10, representation.MirrorPlane is not null ? RhinoConverter.Convert(representation.MirrorPlane) : Rhino.Geometry.Plane.Unset);
        DA.SetData(11, representation.IsHidden);
        DA.SetData(12, representation.IsLocked);
        DA.SetData(13, RhinoConverter.HexToColor(representation.Color));
        DA.SetDataList(14, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        DA.SetDataList(15, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public SerializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public DeserializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
}

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceGoo)))
        {
            target = (Q)(object)new PieceGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceDiffGoo)))
        {
            target = (Q)(object)new PieceDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceGoo pieceGoo)
        {
            Value = pieceGoo.Value;
            return true;
        }
        if (source is PieceDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new PieceId { Id = str };
            return true;
        }
        return false;
    }
}

public class PieceIdParam : IdParam<PieceIdGoo, PieceId>
{
    protected override string RepresentationName => "PieceId";
    protected override string RepresentationNickname => "PId";
    protected override string RepresentationDescription => "Piece identifier";
    protected override string IconResourceName => "piece_24x24";
    protected override string IdIconResourceName => "pieceid_24x24";
    public override Guid ComponentGuid => new("F6A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceGoo)))
        {
            target = (Q)(object)new PieceGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceIdGoo)))
        {
            target = (Q)(object)new PieceIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceGoo pieceGoo)
        {
            Value = pieceGoo.Value;
            return true;
        }
        if (source is PieceIdGoo pieceIdGoo)
        {
            Value = pieceIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<PieceDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PieceDiffParam : DiffParam<PieceDiffGoo, PieceDiff>
{
    protected override string RepresentationName => "PieceDiff";
    protected override string RepresentationNickname => "PD";
    protected override string RepresentationDescription => "Piece diff";
    protected override string IconResourceName => "piecediff_24x24";
    public override Guid ComponentGuid => new("B8C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("C9D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
    protected override string RepresentationName => "PieceDiff";
    protected override string RepresentationNickname => "PD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a piece diff.";
    protected override string IconResourceName => "piecediff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Design", "Dn?", "The optional design.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordinateParam(), "Center", "Ce?", "The optional center.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scale", "Sc?", "The optional scale.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("MirrorPlane", "MP?", "The optional mirror plane.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether locked.", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Design", "Dn?", "The optional design.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordinateParam(), "Center", "Ce?", "The optional center.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scale", "Sc?", "The optional scale.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("MirrorPlane", "MP?", "The optional mirror plane.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether locked.", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Cl?", "The optional color.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, PieceDiff representation)
    {
        string id = null, name = null, description = null, color = null;
        var type = new TypeIdGoo();
        var design = new DesignIdGoo();
        Rhino.Geometry.Plane plane = Rhino.Geometry.Plane.WorldXY, mirrorPlane = Rhino.Geometry.Plane.WorldXY;
        var center = new CoordinateGoo();
        double scale = 0;
        bool isHidden = false, isLocked = false;
        var props = new List<PropGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref type)) representation.Type = type.Value.DeepClone();
        if (DA.GetData(6, ref design)) representation.Design = design.Value.DeepClone();
        if (DA.GetData(7, ref plane)) representation.Plane = RhinoConverter.Convert(plane);
        if (DA.GetData(8, ref center)) representation.Center = center.Value.DeepClone();
        if (DA.GetData(9, ref scale)) representation.Scale = (float)scale;
        if (DA.GetData(10, ref mirrorPlane)) representation.MirrorPlane = RhinoConverter.Convert(mirrorPlane);
        if (DA.GetData(11, ref isHidden)) representation.IsHidden = isHidden;
        if (DA.GetData(12, ref isLocked)) representation.IsLocked = isLocked;
        if (DA.GetData(13, ref color)) representation.Color = color;
        if (DA.GetDataList(14, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(15, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, PieceDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeDescription()) DA.SetData(4, representation.Description);
        if (representation.ShouldSerializeType()) DA.SetData(5, representation.Type is not null ? new TypeIdGoo(representation.Type.DeepClone()) : null);
        if (representation.ShouldSerializeDesign()) DA.SetData(6, representation.Design is not null ? new DesignIdGoo(representation.Design.DeepClone()) : null);
        if (representation.ShouldSerializePlane()) DA.SetData(7, representation.Plane is not null ? RhinoConverter.Convert(representation.Plane) : Rhino.Geometry.Plane.Unset);
        if (representation.ShouldSerializeCenter()) DA.SetData(8, representation.Center is not null ? new CoordinateGoo(representation.Center.DeepClone()) : null);
        if (representation.ShouldSerializeScale()) DA.SetData(9, representation.Scale);
        if (representation.ShouldSerializeMirrorPlane()) DA.SetData(10, representation.MirrorPlane is not null ? RhinoConverter.Convert(representation.MirrorPlane) : Rhino.Geometry.Plane.Unset);
        if (representation.ShouldSerializeIsHidden()) DA.SetData(11, representation.IsHidden);
        if (representation.ShouldSerializeIsLocked()) DA.SetData(12, representation.IsLocked);
        if (representation.ShouldSerializeColor()) DA.SetData(13, representation.Color);
        if (representation.ShouldSerializeProps()) DA.SetDataList(14, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(15, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePieceDiffComponent : SerializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public SerializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D6");
}

public class DeserializePieceDiffComponent : DeserializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public DeserializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D7");
}

public class PiecesDiffGoo : DiffGoo<PiecesDiff>
{
    public PiecesDiffGoo() { }
    public PiecesDiffGoo(PiecesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PiecesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<PiecesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PiecesDiffParam : DiffParam<PiecesDiffGoo, PiecesDiff>
{
    protected override string RepresentationName => "PiecesDiff";
    protected override string RepresentationNickname => "PDs";
    protected override string RepresentationDescription => "Piece collection diff";
    protected override string IconResourceName => "piecesdiff_24x24";
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C7");
}

public class PiecesDiffComponent : DiffComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C8");
    protected override string RepresentationName => "PiecesDiff";
    protected override string RepresentationNickname => "PDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of piece diffs.";
    protected override string IconResourceName => "piecesdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed piece ids.", GH_ParamAccess.list);
        pManager.AddParameter(new PieceDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated piece diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added pieces.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed piece ids.", GH_ParamAccess.list);
        pManager.AddParameter(new PieceDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated piece diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added pieces.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, PiecesDiff representation)
    {
        var removed = new List<PieceIdGoo>();
        var updated = new List<PieceDiffGoo>();
        var added = new List<PieceGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new PieceDiffUpdate { Piece = new PieceId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, PiecesDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new PieceIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new PieceDiffGoo((u.Diff ?? new PieceDiff { Id = u.Piece.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new PieceGoo(a.DeepClone())).ToList());
    }

}

public class SerializePiecesDiffComponent : SerializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public SerializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C9");
}

public class DeserializePiecesDiffComponent : DeserializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public DeserializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6CA");
}

#endregion 🎈Piece

#region 🎺Side
// Implementations MUST reference a piece and connector as a connection endpoint.

public class SideGoo : Goo<Side>
{
    public SideGoo() { }
    public SideGoo(Side value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Piece.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Side { Piece = new PieceId { Id = str } };
            return true;
        }
        return false;
    }
}

public class SideParam : Param<SideGoo, Side>
{
    protected override string RepresentationName => "Side";
    protected override string RepresentationNickname => "Sid";
    protected override string RepresentationDescription => "Connection side";
    protected override string IconResourceName => "side_24x24";
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
}

public class SideComponent : PassthroughComponent<SideParam, SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E7");
    protected override string RepresentationName => "Side";
    protected override string RepresentationNickname => "Sde";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a side.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pc", "The piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Po", "The connector of the side.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pc", "The piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Po", "The connector of the side.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Side representation)
    {
        var piece = new PieceIdGoo();
        var designPiece = new PieceIdGoo();
        var connector = new ConnectorIdGoo();

        if (DA.GetData(2, ref piece)) representation.Piece = piece.Value.DeepClone();
        if (DA.GetData(3, ref designPiece)) representation.DesignPiece = designPiece.Value.DeepClone();
        if (DA.GetData(4, ref connector)) representation.Connector = connector.Value.DeepClone();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Side representation)
    {
        DA.SetData(2, new PieceIdGoo(representation.Piece.DeepClone()));
        DA.SetData(3, representation.DesignPiece is not null ? new PieceIdGoo(representation.DesignPiece.DeepClone()) : null);
        DA.SetData(4, new ConnectorIdGoo(representation.Connector.DeepClone()));
    }
}

public class SerializeSideComponent : SerializeComponent<SideParam, SideGoo, Side>
{
    public SerializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E8");
}

public class DeserializeSideComponent : DeserializeComponent<SideParam, SideGoo, Side>
{
    public DeserializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E9");
}

public class SideDiffGoo : DiffGoo<SideDiff>
{
    public SideDiffGoo() { }
    public SideDiffGoo(SideDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("SideDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<SideDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class SideDiffParam : DiffParam<SideDiffGoo, SideDiff>
{
    protected override string RepresentationName => "SideDiff";
    protected override string RepresentationNickname => "SD";
    protected override string RepresentationDescription => "Side diff";
    protected override string IconResourceName => "sidediff_24x24";
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
}

public class SideDiffComponent : DiffComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E4");
    protected override string RepresentationName => "SideDiff";
    protected override string RepresentationNickname => "SD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a side diff.";
    protected override string IconResourceName => "sidediff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pi?", "The optional piece.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Co?", "The optional connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pi?", "The optional piece.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Co?", "The optional connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, SideDiff representation)
    {
        var piece = new PieceIdGoo();
        var designPiece = new PieceIdGoo();
        var connector = new ConnectorIdGoo();
        string description = null;
        if (DA.GetData(2, ref piece)) representation.Piece = piece.Value?.DeepClone();
        if (DA.GetData(3, ref designPiece)) representation.DesignPiece = designPiece.Value?.DeepClone();
        if (DA.GetData(4, ref connector)) representation.Connector = connector.Value?.DeepClone();
        if (DA.GetData(5, ref description)) representation.Description = description;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, SideDiff representation)
    {
        if (representation.ShouldSerializePiece() && representation.Piece is not null) DA.SetData(2, new PieceIdGoo(representation.Piece.DeepClone()));
        if (representation.ShouldSerializeDesignPiece()) DA.SetData(3, representation.DesignPiece is not null ? new PieceIdGoo(representation.DesignPiece.DeepClone()) : null);
        if (representation.ShouldSerializeConnector()) DA.SetData(4, representation.Connector is not null ? new ConnectorIdGoo(representation.Connector.DeepClone()) : null);
        if (representation.ShouldSerializeDescription()) DA.SetData(5, representation.Description);
    }
}

public class SerializeSideDiffComponent : SerializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public SerializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B1C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
}

public class DeserializeSideDiffComponent : DeserializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public DeserializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B2C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
}

#endregion 🎺Side

#region 💡Connection
// Implementations MUST link two sides to connect pieces in a design.

public class ConnectionGoo : Goo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionDiffGoo)))
        {
            target = (Q)(object)new ConnectionDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionIdGoo)))
        {
            target = (Q)(object)new ConnectionIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is ConnectionIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Connection>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator ConnectionIdGoo(ConnectionGoo goo) => new((ConnectionId)goo.Value);
    public static implicit operator ConnectionDiffGoo(ConnectionGoo goo) => new((ConnectionDiff)goo.Value);
    public static implicit operator ConnectionGoo(ConnectionIdGoo idGoo) => new((Connection)idGoo.Value);
    public static implicit operator ConnectionGoo(ConnectionDiffGoo diffGoo) => new((Connection)diffGoo.Value);
}

public class ConnectionParam : Param<ConnectionGoo, Connection>
{
    protected override string RepresentationName => "Connection";
    protected override string RepresentationNickname => "Cnx";
    protected override string RepresentationDescription => "Piece connection";
    protected override string IconResourceName => "connection_24x24";
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
}

public class ConnectionComponent : PassthroughComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");
    protected override string RepresentationName => "Connection";
    protected override string RepresentationNickname => "Con";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a connection.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the connection.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Cd", "The connected side.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cg", "The connecting side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp", "The longitudinal gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf", "The lateral shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs", "The vertical rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt", "The rotation around y-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn", "The turn around z-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl", "The tilt around x-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("U", "U?", "The optional u parameter.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V?", "The optional v parameter.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the connection.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Cd", "The connected side.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cg", "The connecting side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp", "The longitudinal gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf", "The lateral shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs", "The vertical rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt", "The rotation around y-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn", "The turn around z-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl", "The tilt around x-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("U", "U?", "The optional u parameter.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V?", "The optional v parameter.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Connection representation)
    {
        string id = "", description = "";
        var connected = new SideGoo();
        var connecting = new SideGoo();
        double gap = 0, shift = 0, rise = 0, rotation = 0, turn = 0, tilt = 0, u = 0, v = 0;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref connected)) representation.Parent = connected.Value.DeepClone();
        if (DA.GetData(4, ref connecting)) representation.Child = connecting.Value.DeepClone();
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetData(6, ref gap)) representation.Gap = (float)gap;
        if (DA.GetData(7, ref shift)) representation.Shift = (float)shift;
        if (DA.GetData(8, ref rise)) representation.Rise = (float)rise;
        if (DA.GetData(9, ref rotation)) representation.Rotation = (float)rotation;
        if (DA.GetData(10, ref turn)) representation.Turn = (float)turn;
        if (DA.GetData(11, ref tilt)) representation.Tilt = (float)tilt;
        if (DA.GetData(12, ref u)) representation.U = (float)u;
        if (DA.GetData(13, ref v)) representation.V = (float)v;
        if (DA.GetDataList(14, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Connection representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, new SideGoo(representation.Parent.DeepClone()));
        DA.SetData(4, new SideGoo(representation.Child.DeepClone()));
        DA.SetData(5, representation.Description);
        DA.SetData(6, representation.Gap);
        DA.SetData(7, representation.Shift);
        DA.SetData(8, representation.Rise);
        DA.SetData(9, representation.Rotation);
        DA.SetData(10, representation.Turn);
        DA.SetData(11, representation.Tilt);
        DA.SetData(12, representation.U);
        DA.SetData(13, representation.V);
        DA.SetDataList(14, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public SerializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public DeserializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
}

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionGoo)))
        {
            target = (Q)(object)new ConnectionGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionDiffGoo)))
        {
            target = (Q)(object)new ConnectionDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionGoo connectionGoo)
        {
            Value = connectionGoo.Value;
            return true;
        }
        if (source is ConnectionDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionId>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator ConnectionGoo(ConnectionIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator ConnectionDiffGoo(ConnectionIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator ConnectionIdGoo(ConnectionGoo goo) => new((ConnectionId)goo.Value);
    public static implicit operator ConnectionIdGoo(ConnectionDiffGoo diffGoo) => new((ConnectionId)diffGoo.Value);
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    protected override string RepresentationName => "ConnectionId";
    protected override string RepresentationNickname => "CId";
    protected override string RepresentationDescription => "Connection identifier";
    protected override string IconResourceName => "connection_24x24";
    protected override string IdIconResourceName => "connectionid_24x24";
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionGoo)))
        {
            target = (Q)(object)new ConnectionGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionIdGoo)))
        {
            target = (Q)(object)new ConnectionIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionGoo connectionGoo)
        {
            Value = connectionGoo.Value;
            return true;
        }
        if (source is ConnectionIdGoo connectionIdGoo)
        {
            Value = connectionIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectionDiffParam : DiffParam<ConnectionDiffGoo, ConnectionDiff>
{
    protected override string RepresentationName => "ConnectionDiff";
    protected override string RepresentationNickname => "CD";
    protected override string RepresentationDescription => "Connection diff";
    protected override string IconResourceName => "connectiondiff_24x24";
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");
    protected override string RepresentationName => "ConnectionDiff";
    protected override string RepresentationNickname => "CD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a connection diff.";
    protected override string IconResourceName => "connectiondiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SideDiffParam(), "Connected", "Co?", "The optional connected.", GH_ParamAccess.item);
        pManager.AddParameter(new SideDiffParam(), "Connecting", "Cg?", "The optional connecting.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp?", "The optional gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf?", "The optional shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs?", "The optional rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?", "The optional rotation.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn?", "The optional turn.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl?", "The optional tilt.", GH_ParamAccess.item);
        pManager.AddNumberParameter("U", "U?", "The optional u parameter.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V?", "The optional v parameter.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SideDiffParam(), "Connected", "Co?", "The optional connected.", GH_ParamAccess.item);
        pManager.AddParameter(new SideDiffParam(), "Connecting", "Cg?", "The optional connecting.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp?", "The optional gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf?", "The optional shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs?", "The optional rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?", "The optional rotation.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn?", "The optional turn.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl?", "The optional tilt.", GH_ParamAccess.item);
        pManager.AddNumberParameter("U", "U?", "The optional u parameter.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V?", "The optional v parameter.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, ConnectionDiff representation)
    {
        var connected = new SideDiffGoo();
        var connecting = new SideDiffGoo();
        string description = null;
        double gap = 0, shift = 0, rise = 0, rotation = 0, turn = 0, tilt = 0, u = 0, v = 0;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref connected)) representation.Parent = connected.Value?.DeepClone();
        if (DA.GetData(3, ref connecting)) representation.Child = connecting.Value?.DeepClone();
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref gap)) representation.Gap = (float)gap;
        if (DA.GetData(6, ref shift)) representation.Shift = (float)shift;
        if (DA.GetData(7, ref rise)) representation.Rise = (float)rise;
        if (DA.GetData(8, ref rotation)) representation.Rotation = (float)rotation;
        if (DA.GetData(9, ref turn)) representation.Turn = (float)turn;
        if (DA.GetData(10, ref tilt)) representation.Tilt = (float)tilt;
        if (DA.GetData(11, ref u)) representation.U = (float)u;
        if (DA.GetData(12, ref v)) representation.V = (float)v;
        if (DA.GetDataList(13, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, ConnectionDiff representation)
    {
        if (representation.ShouldSerializeParent() && representation.Parent is not null) DA.SetData(2, new SideDiffGoo(representation.Parent.DeepClone()));
        if (representation.ShouldSerializeChild() && representation.Child is not null) DA.SetData(3, new SideDiffGoo(representation.Child.DeepClone()));
        if (representation.ShouldSerializeDescription()) DA.SetData(4, representation.Description);
        if (representation.ShouldSerializeGap()) DA.SetData(5, representation.Gap);
        if (representation.ShouldSerializeShift()) DA.SetData(6, representation.Shift);
        if (representation.ShouldSerializeRise()) DA.SetData(7, representation.Rise);
        if (representation.ShouldSerializeRotation()) DA.SetData(8, representation.Rotation);
        if (representation.ShouldSerializeTurn()) DA.SetData(9, representation.Turn);
        if (representation.ShouldSerializeTilt()) DA.SetData(10, representation.Tilt);
        if (representation.ShouldSerializeU()) DA.SetData(11, representation.U);
        if (representation.ShouldSerializeV()) DA.SetData(12, representation.V);
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(13, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeConnectionDiffComponent : SerializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public SerializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F6");
}

public class DeserializeConnectionDiffComponent : DeserializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public DeserializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F7");
}

public class ConnectionsDiffGoo : DiffGoo<ConnectionsDiff>
{
    public ConnectionsDiffGoo() { }
    public ConnectionsDiffGoo(ConnectionsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectionsDiffParam : DiffParam<ConnectionsDiffGoo, ConnectionsDiff>
{
    protected override string RepresentationName => "ConnectionsDiff";
    protected override string RepresentationNickname => "CDs";
    protected override string RepresentationDescription => "Connection collection diff";
    protected override string IconResourceName => "connectionsdiff_24x24";
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D8");
}

public class ConnectionsDiffComponent : DiffComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D9");
    protected override string RepresentationName => "ConnectionsDiff";
    protected override string RepresentationNickname => "CDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of connection diffs.";
    protected override string IconResourceName => "connectionsdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed connection ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated connection diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added connections.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed connection ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated connection diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added connections.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, ConnectionsDiff representation)
    {
        var removed = new List<ConnectionIdGoo>();
        var updated = new List<ConnectionDiffGoo>();
        var added = new List<ConnectionGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated))
        {
            representation.Modified = updated.Select(u =>
            {
                var connection = new ConnectionId();
                if (u.Value.Parent is not null)
                {
                    connection.Parent = new Side { Piece = u.Value.Parent.Piece, Connector = u.Value.Parent.Connector };
                }
                if (u.Value.Child is not null)
                {
                    connection.Child = new Side { Piece = u.Value.Child.Piece, Connector = u.Value.Child.Connector };
                }

                return new ConnectionDiffUpdate { Connection = connection, Diff = u.Value.DeepClone() };
            }).ToList();
        }
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, ConnectionsDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new ConnectionIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u =>
        {
            if (u.Diff is not null)
            {
                return new ConnectionDiffGoo(u.Diff.DeepClone());
            }

            return new ConnectionDiffGoo(new ConnectionDiff
            {
                Parent = new SideDiff
                {
                    Piece = u.Connection.Parent.Piece,
                    Connector = u.Connection.Parent.Connector,
                },
                Child = new SideDiff
                {
                    Piece = u.Connection.Child.Piece,
                    Connector = u.Connection.Child.Connector,
                },
            });
        }).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new ConnectionGoo(a.DeepClone())).ToList());
    }

}

public class SerializeConnectionsDiffComponent : SerializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public SerializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DA");
}

public class DeserializeConnectionsDiffComponent : DeserializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public DeserializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DB");
}

#endregion 💡Connection

#region 🪵Stat
// Implementations MUST associate statistical metrics with a design.

public class StatGoo : Goo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToHumanIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Stat>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class StatParam : Param<StatGoo, Stat>
{
    protected override string RepresentationName => "Stat";
    protected override string RepresentationNickname => "Sta";
    protected override string RepresentationDescription => "Design statistic";
    protected override string IconResourceName => "stat_24x24";
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class StatComponent : PassthroughComponent<StatParam, StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string RepresentationName => "Stat";
    protected override string RepresentationNickname => "Stt";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a stat.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the stat.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql?", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the stat.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql?", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Stat representation)
    {
        string id = "";
        QualityId quality = new();
        string unit = "";
        double min = 0, max = 0;
        bool minExcluded = false, maxExcluded = false;

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref quality)) representation.Quality = quality;
        if (DA.GetData(4, ref unit)) representation.Unit = unit;
        if (DA.GetData(5, ref min)) representation.Min = (float)min;
        if (DA.GetData(6, ref minExcluded)) representation.MinExcluded = minExcluded;
        if (DA.GetData(7, ref max)) representation.Max = (float)max;
        if (DA.GetData(8, ref maxExcluded)) representation.MaxExcluded = maxExcluded;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Stat representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, new QualityIdGoo(representation.Quality));
        DA.SetData(4, representation.Unit);
        DA.SetData(5, representation.Min);
        DA.SetData(6, representation.MinExcluded);
        DA.SetData(7, representation.Max);
        DA.SetData(8, representation.MaxExcluded);
    }
}

public class SerializeStatComponent : SerializeComponent<StatParam, StatGoo, Stat>
{
    public SerializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeStatComponent : DeserializeComponent<StatParam, StatGoo, Stat>
{
    public DeserializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🪵Stat

#region 🧬Design
// Implementations MUST compose pieces, connections, and metadata into a layout.

public class DesignGoo : Goo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignDiffGoo)))
        {
            target = (Q)(object)new DesignDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignIdGoo)))
        {
            target = (Q)(object)new DesignIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is DesignIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Design { Name = str };
            return true;
        }
        return false;
    }
}

public class DesignParam : Param<DesignGoo, Design>
{
    protected override string RepresentationName => "Design";
    protected override string RepresentationNickname => "Des";
    protected override string RepresentationDescription => "Assembly design";
    protected override string IconResourceName => "design_24x24";
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
}

public class DesignComponent : PassthroughComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");
    protected override string RepresentationName => "Design";
    protected override string RepresentationNickname => "Dsn";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a design.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the design.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Parent", "Pa?", "The optional parent design.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the design is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fd?", "The optional folder path.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new ConceptIdParam(), "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanScale", "CS?", "Whether pieces can be scaled.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanMirror", "CM?", "Whether pieces can be mirrored.", GH_ParamAccess.item);
        pManager.AddParameter(new LayerParam(), "Layers", "Ly*", "The optional layers.", GH_ParamAccess.list);
        pManager.AddTextParameter("ActiveLayer", "AL?", "The optional active layer name.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc*", "The optional pieces.", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Gr*", "The optional groups.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "The optional connections.", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pp*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St*", "The optional stats.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated at timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the design.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Parent", "Pa?", "The optional parent design.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the design is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fd?", "The optional folder path.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new ConceptIdParam(), "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanScale", "CS?", "Whether pieces can be scaled.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanMirror", "CM?", "Whether pieces can be mirrored.", GH_ParamAccess.item);
        pManager.AddParameter(new LayerParam(), "Layers", "Ly*", "The optional layers.", GH_ParamAccess.list);
        pManager.AddTextParameter("ActiveLayer", "AL?", "The optional active layer name.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc*", "The optional pieces.", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Gr*", "The optional groups.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "The optional connections.", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pp*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St*", "The optional stats.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The created at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The updated at timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Design representation)
    {
        string id = "", name = "", folder = "", description = "", icon = "", image = "", unit = "", activeLayer = "";
        DateTime createdAt = default, updatedAt = default;
        var parent = new DesignIdGoo();
        bool isAbstract = false, canScale = false, canMirror = false;
        var concepts = new List<ConceptIdGoo>();
        var authors = new List<AuthorIdGoo>();
        var location = new LocationGoo();
        var layers = new List<LayerGoo>();
        var pieces = new List<PieceGoo>();
        var groups = new List<GroupGoo>();
        var connections = new List<ConnectionGoo>();
        var props = new List<PropGoo>();
        var stats = new List<StatGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parent)) representation.Parent = parent.Value.DeepClone();
        if (DA.GetData(5, ref isAbstract)) representation.IsAbstract = isAbstract;
        if (DA.GetData(6, ref folder)) representation.Folder = folder;
        if (DA.GetData(7, ref description)) representation.Description = description;
        if (DA.GetData(8, ref icon)) representation.Icon = icon;
        if (DA.GetData(9, ref image)) representation.Image = image;
        if (DA.GetDataList(10, concepts)) representation.Concepts = concepts.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetDataList(11, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(12, ref location)) representation.Location = location.Value.DeepClone();
        if (DA.GetData(13, ref unit)) representation.Unit = unit;
        if (DA.GetData(14, ref canScale)) representation.CanScale = canScale;
        if (DA.GetData(15, ref canMirror)) representation.CanMirror = canMirror;
        if (DA.GetDataList(16, layers)) representation.Layers = layers.Select(l => l.Value.DeepClone()).ToList();
        if (DA.GetData(17, ref activeLayer)) representation.ActiveLayer = string.IsNullOrEmpty(activeLayer) ? null : new LayerId { Id = activeLayer };
        if (DA.GetDataList(18, pieces)) representation.Pieces = pieces.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(19, groups)) representation.Groups = groups.Select(g => g.Value.DeepClone()).ToList();
        if (DA.GetDataList(20, connections)) representation.Connections = connections.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetDataList(21, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(22, stats)) representation.Stats = stats.Select(s => s.Value.DeepClone()).ToList();
        if (DA.GetDataList(23, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(24, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(25, ref updatedAt)) representation.ModificationdAt = updatedAt;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Design representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Parent is not null ? new DesignIdGoo(representation.Parent.DeepClone()) : null);
        DA.SetData(5, representation.IsAbstract);
        DA.SetData(6, representation.Folder);
        DA.SetData(7, representation.Description);
        DA.SetData(8, representation.Icon);
        DA.SetData(9, representation.Image);
        DA.SetDataList(10, representation.Concepts?.Select(c => new ConceptIdGoo(c.DeepClone())).ToList());
        DA.SetDataList(11, representation.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        DA.SetData(12, representation.Location is not null ? new LocationGoo(representation.Location.DeepClone()) : null);
        DA.SetData(13, representation.Unit);
        DA.SetData(14, representation.CanScale);
        DA.SetData(15, representation.CanMirror);
        DA.SetDataList(16, representation.Layers?.Select(l => new LayerGoo(l.DeepClone())).ToList());
        DA.SetData(17, representation.ActiveLayer);
        DA.SetDataList(18, representation.Pieces?.Select(p => new PieceGoo(p.DeepClone())).ToList());
        DA.SetDataList(19, representation.Groups?.Select(g => new GroupGoo(g.DeepClone())).ToList());
        DA.SetDataList(20, representation.Connections?.Select(c => new ConnectionGoo(c.DeepClone())).ToList());
        DA.SetDataList(21, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        DA.SetDataList(22, representation.Stats?.Select(s => new StatGoo(s.DeepClone())).ToList());
        DA.SetDataList(23, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        DA.SetData(24, representation.CreatedAt);
        DA.SetData(25, representation.ModificationdAt);
    }

    protected override Design ProcessRepresentation(Design design)
    {
        if (design.Unit == "")
            try { design.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { design.Unit = "m"; }
        design.Icon = design.Icon?.Replace('\\', '/');
        design.Image = design.Image?.Replace('\\', '/');
        return design;
    }
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public SerializeDesignComponent() { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public DeserializeDesignComponent() { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB59");
}

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignGoo)))
        {
            target = (Q)(object)new DesignGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignDiffGoo)))
        {
            target = (Q)(object)new DesignDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToHumanIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignGoo designGoo)
        {
            Value = designGoo.Value;
            return true;
        }
        if (source is DesignDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new DesignId { Id = str };
            return true;
        }
        return false;
    }
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    protected override string RepresentationName => "DesignId";
    protected override string RepresentationNickname => "DId";
    protected override string RepresentationDescription => "Design identifier";
    protected override string IconResourceName => "design_24x24";
    protected override string IdIconResourceName => "designid_24x24";
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A6");
}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignGoo)))
        {
            target = (Q)(object)new DesignGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignIdGoo)))
        {
            target = (Q)(object)new DesignIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignGoo designGoo)
        {
            Value = designGoo.Value;
            return true;
        }
        if (source is DesignIdGoo designIdGoo)
        {
            Value = designIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<DesignDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class DesignDiffParam : DiffParam<DesignDiffGoo, DesignDiff>
{
    protected override string RepresentationName => "DesignDiff";
    protected override string RepresentationNickname => "DD";
    protected override string RepresentationDescription => "Design diff";
    protected override string IconResourceName => "designdiff_24x24";
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A5");
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A8");
    protected override string RepresentationName => "DesignDiff";
    protected override string RepresentationNickname => "DD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a design diff.";
    protected override string IconResourceName => "designdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Parent", "Pa?", "The optional parent design.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the design is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanScale", "Sc?", "Whether scaling is enabled.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanMirror", "Mi?", "Whether mirroring is enabled.", GH_ParamAccess.item);
        pManager.AddTextParameter("ActiveLayer", "Ly?", "The optional active layer.", GH_ParamAccess.item);
        pManager.AddParameter(new PiecesDiffParam(), "Pieces", "Pc?", "The optional piece diff.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectionsDiffParam(), "Connections", "Cn?", "The optional connection diff.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam() { Access = GH_ParamAccess.list }, "Stats", "St*", "The optional stats.", GH_ParamAccess.list);
        pManager.AddParameter(new LayerParam() { Access = GH_ParamAccess.list }, "Layers", "Ly*", "The optional layers.", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam() { Access = GH_ParamAccess.list }, "Groups", "Gp*", "The optional groups.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional author ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam() { Access = GH_ParamAccess.list }, "Concepts", "Cp*", "The optional concept ids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Parent", "Pa?", "The optional parent design.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsAbstract", "Ab?", "Whether the design is abstract.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "Lo?", "The optional location.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanScale", "Sc?", "Whether scaling is enabled.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("CanMirror", "Mi?", "Whether mirroring is enabled.", GH_ParamAccess.item);
        pManager.AddTextParameter("ActiveLayer", "Ly?", "The optional active layer.", GH_ParamAccess.item);
        pManager.AddParameter(new PiecesDiffParam(), "Pieces", "Pc?", "The optional piece diff.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectionsDiffParam(), "Connections", "Cn?", "The optional connection diff.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam() { Access = GH_ParamAccess.list }, "Props", "Pr*", "The optional props.", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam() { Access = GH_ParamAccess.list }, "Stats", "St*", "The optional stats.", GH_ParamAccess.list);
        pManager.AddParameter(new LayerParam() { Access = GH_ParamAccess.list }, "Layers", "Ly*", "The optional layers.", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam() { Access = GH_ParamAccess.list }, "Groups", "Gp*", "The optional groups.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional author ids.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptIdParam() { Access = GH_ParamAccess.list }, "Concepts", "Cp*", "The optional concept ids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, DesignDiff representation)
    {
        string id = null, name = null, folder = null, description = null, icon = null, image = null, unit = null, activeLayer = null;
        bool isAbstract = false, canScale = false, canMirror = false;
        DateTime createdAt = default, updatedAt = default;
        var parent = new DesignIdGoo();
        var location = new LocationGoo();
        var pieces = new PiecesDiffGoo();
        var connections = new ConnectionsDiffGoo();
        var props = new List<PropGoo>();
        var stats = new List<StatGoo>();
        var layers = new List<LayerGoo>();
        var groups = new List<GroupGoo>();
        var authors = new List<AuthorIdGoo>();
        var concepts = new List<ConceptIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref parent)) representation.Parent = parent.Value.DeepClone();
        if (DA.GetData(5, ref isAbstract)) representation.IsAbstract = isAbstract;
        if (DA.GetData(6, ref folder)) representation.Folder = folder;
        if (DA.GetData(7, ref description)) representation.Description = description;
        if (DA.GetData(8, ref icon)) representation.Icon = icon;
        if (DA.GetData(9, ref image)) representation.Image = image;
        if (DA.GetData(10, ref location)) representation.Location = location.Value.DeepClone();
        if (DA.GetData(11, ref unit)) representation.Unit = unit;
        if (DA.GetData(12, ref canScale)) representation.CanScale = canScale;
        if (DA.GetData(13, ref canMirror)) representation.CanMirror = canMirror;
        if (DA.GetData(14, ref activeLayer)) representation.ActiveLayer = string.IsNullOrEmpty(activeLayer) ? null : new LayerId { Id = activeLayer };
        if (DA.GetData(15, ref pieces)) representation.Pieces = pieces.Value.DeepClone();
        if (DA.GetData(16, ref connections)) representation.Connections = connections.Value.DeepClone();
        if (DA.GetDataList(17, props)) representation.Props = props.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, stats)) representation.Stats = stats.Select(s => s.Value.DeepClone()).ToList();
        if (DA.GetDataList(19, layers)) representation.Layers = layers.Select(l => l.Value.DeepClone()).ToList();
        if (DA.GetDataList(20, groups)) representation.Groups = groups.Select(g => g.Value.DeepClone()).ToList();
        if (DA.GetDataList(21, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(22, concepts)) representation.Concepts = concepts.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetDataList(23, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(24, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(25, ref updatedAt)) representation.ModificationdAt = updatedAt;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, DesignDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeParent()) DA.SetData(4, representation.Parent is not null ? new DesignIdGoo(representation.Parent.DeepClone()) : null);
        if (representation.ShouldSerializeIsAbstract()) DA.SetData(5, representation.IsAbstract);
        if (representation.ShouldSerializeFolder()) DA.SetData(6, representation.Folder);
        if (representation.ShouldSerializeDescription()) DA.SetData(7, representation.Description);
        if (representation.ShouldSerializeIcon()) DA.SetData(8, representation.Icon);
        if (representation.ShouldSerializeImage()) DA.SetData(9, representation.Image);
        if (representation.ShouldSerializeLocation()) DA.SetData(10, representation.Location is not null ? new LocationGoo(representation.Location.DeepClone()) : null);
        if (representation.ShouldSerializeUnit()) DA.SetData(11, representation.Unit);
        if (representation.ShouldSerializeCanScale()) DA.SetData(12, representation.CanScale);
        if (representation.ShouldSerializeCanMirror()) DA.SetData(13, representation.CanMirror);
        if (representation.ShouldSerializeActiveLayer()) DA.SetData(14, representation.ActiveLayer);
        if (representation.ShouldSerializePieces()) DA.SetData(15, representation.Pieces is not null ? new PiecesDiffGoo(representation.Pieces.DeepClone()) : null);
        if (representation.ShouldSerializeConnections()) DA.SetData(16, representation.Connections is not null ? new ConnectionsDiffGoo(representation.Connections.DeepClone()) : null);
        if (representation.ShouldSerializeProps()) DA.SetDataList(17, representation.Props?.Select(p => new PropGoo(p.DeepClone())).ToList());
        if (representation.ShouldSerializeStats()) DA.SetDataList(18, representation.Stats?.Select(s => new StatGoo(s.DeepClone())).ToList());
        if (representation.ShouldSerializeLayers()) DA.SetDataList(19, representation.Layers?.Select(l => new LayerGoo(l.DeepClone())).ToList());
        if (representation.ShouldSerializeGroups()) DA.SetDataList(20, representation.Groups?.Select(g => new GroupGoo(g.DeepClone())).ToList());
        if (representation.ShouldSerializeAuthors()) DA.SetDataList(21, representation.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        if (representation.ShouldSerializeConcepts()) DA.SetDataList(22, representation.Concepts?.Select(c => new ConceptIdGoo(c.DeepClone())).ToList());
        if (representation.ShouldSerializeAttributes()) DA.SetDataList(23, representation.Attributes?.Added?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        if (representation.ShouldSerializeCreatedAt()) DA.SetData(24, representation.CreatedAt);
        if (representation.ShouldSerializeModificationdAt()) DA.SetData(25, representation.ModificationdAt);
    }
}

public class SerializeDesignDiffComponent : SerializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public SerializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A9");
}

public class DeserializeDesignDiffComponent : DeserializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DeserializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4AA");
}

public class DesignsDiffGoo : DiffGoo<DesignsDiff>
{
    public DesignsDiffGoo() { }
    public DesignsDiffGoo(DesignsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("DesignsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<DesignsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class DesignsDiffParam : DiffParam<DesignsDiffGoo, DesignsDiff>
{
    protected override string RepresentationName => "DesignsDiff";
    protected override string RepresentationNickname => "DDs";
    protected override string RepresentationDescription => "Design collection diff";
    protected override string IconResourceName => "designsdiff_24x24";
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8E9");
}

public class DesignsDiffComponent : DiffComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EA");
    protected override string RepresentationName => "DesignsDiff";
    protected override string RepresentationNickname => "DDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of design diffs.";
    protected override string IconResourceName => "designsdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed design ids.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated design diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added designs.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed design ids.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated design diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added designs.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, DesignsDiff representation)
    {
        var removed = new List<DesignIdGoo>();
        var updated = new List<DesignDiffGoo>();
        var added = new List<DesignGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new DesignDiffUpdate { Design = new DesignId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, DesignsDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new DesignIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new DesignDiffGoo((u.Diff ?? new DesignDiff { Id = u.Design.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new DesignGoo(a.DeepClone())).ToList());
    }

}

public class SerializeDesignsDiffComponent : SerializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public SerializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EB");
}

public class DeserializeDesignsDiffComponent : DeserializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public DeserializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EC");
}

#endregion 🧬Design

#region 🧳Kit
// Implementations MUST collect types and designs into a reusable library.

public class KitGoo : Goo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(KitDiffGoo)))
        {
            target = (Q)(object)new KitDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Kit { Name = str };
            return true;
        }
        return false;
    }
}

public class KitParam : Param<KitGoo, Kit>
{
    protected override string RepresentationName => "Kit";
    protected override string RepresentationNickname => "Kit";
    protected override string RepresentationDescription => "Component library";
    protected override string IconResourceName => "kit_24x24";
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
}

public class KitComponent : PassthroughComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
    protected override string RepresentationName => "Kit";
    protected override string RepresentationNickname => "Kit";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a kit.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new ConceptParam(), "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new TagParam(), "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Pv?", "The optional preview.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*", "The optional qualities.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Ports", "Pt*", "The optional ports.", GH_ParamAccess.list);
        pManager.AddParameter(new FileParam(), "Files", "Fl*", "The optional files.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderParam(), "Folders", "Fo*", "The optional folders.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "The optional types.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "The optional designs.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The optional created at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The optional updated at timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gd", "The id of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddParameter(new ConceptParam(), "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new TagParam(), "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Pv?", "The optional preview.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*", "The optional qualities.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Ports", "Pt*", "The optional ports.", GH_ParamAccess.list);
        pManager.AddParameter(new FileParam(), "Files", "Fl*", "The optional files.", GH_ParamAccess.list);
        pManager.AddParameter(new FolderParam(), "Folders", "Fo*", "The optional folders.", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "The optional types.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "The optional designs.", GH_ParamAccess.list);
        pManager.AddTimeParameter("CreatedAt", "CA?", "The created at timestamp.", GH_ParamAccess.item);
        pManager.AddTimeParameter("UpdatedAt", "UA?", "The updated at timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, Kit representation)
    {
        string id = "", name = "", version = "", description = "", icon = "", image = "", remote = "", homepage = "", license = "", preview = "";
        DateTime createdAt = default, updatedAt = default;
        var concepts = new List<ConceptGoo>();
        var tags = new List<TagGoo>();
        var authors = new List<AuthorGoo>();
        var attributes = new List<AttributeGoo>();
        var qualities = new List<QualityGoo>();
        var ports = new List<PortGoo>();
        var files = new List<FileGoo>();
        var folders = new List<FolderGoo>();
        var types = new List<TypeGoo>();
        var designs = new List<DesignGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref version)) representation.Version = version;
        if (DA.GetData(5, ref description)) representation.Description = description;
        if (DA.GetData(6, ref icon)) representation.Icon = icon;
        if (DA.GetData(7, ref image)) representation.Image = image;
        if (DA.GetDataList(8, concepts)) representation.Concepts = concepts.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetDataList(9, tags)) representation.Tags = tags.Select(t => t.Value.DeepClone()).ToList();
        if (DA.GetData(10, ref remote)) representation.Remote = remote;
        if (DA.GetData(11, ref homepage)) representation.Homepage = homepage;
        if (DA.GetData(12, ref license)) representation.License = license;
        if (DA.GetDataList(13, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(14, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(15, ref preview)) representation.Preview = preview;
        if (DA.GetDataList(16, qualities)) representation.Qualities = qualities.Select(q => q.Value.DeepClone()).ToList();
        if (DA.GetDataList(17, ports)) representation.Ports = ports.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, files)) representation.Files = files.Select(f => f.Value.DeepClone()).ToList();
        if (DA.GetDataList(19, folders)) representation.Folders = folders.Select(f => f.Value.DeepClone()).ToList();
        if (DA.GetDataList(20, types)) representation.Types = types.Select(t => t.Value.DeepClone()).ToList();
        if (DA.GetDataList(21, designs)) representation.Designs = designs.Select(d => d.Value.DeepClone()).ToList();
        if (DA.GetData(22, ref createdAt)) representation.CreatedAt = createdAt.ToString("o");
        if (DA.GetData(23, ref updatedAt)) representation.ModificationdAt = updatedAt.ToString("o");
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, Kit representation)
    {
        DA.SetData(2, representation.Id);
        DA.SetData(3, representation.Name);
        DA.SetData(4, representation.Version);
        DA.SetData(5, representation.Description);
        DA.SetData(6, representation.Icon);
        DA.SetData(7, representation.Image);
        DA.SetDataList(8, representation.Concepts?.Select(c => new ConceptGoo(c.DeepClone())).ToList());
        DA.SetDataList(9, representation.Tags?.Select(t => new TagGoo(t.DeepClone())).ToList());
        DA.SetData(10, representation.Remote);
        DA.SetData(11, representation.Homepage);
        DA.SetData(12, representation.License);
        DA.SetDataList(13, representation.Authors?.Select(a => new AuthorGoo(a.DeepClone())).ToList());
        DA.SetDataList(14, representation.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
        DA.SetData(15, representation.Preview);
        DA.SetDataList(16, representation.Qualities?.Select(q => new QualityGoo(q.DeepClone())).ToList());
        DA.SetDataList(17, representation.Ports?.Select(p => new PortGoo(p.DeepClone())).ToList());
        DA.SetDataList(18, representation.Files?.Select(f => new FileGoo(f.DeepClone())).ToList());
        DA.SetDataList(19, representation.Folders?.Select(f => new FolderGoo(f.DeepClone())).ToList());
        DA.SetDataList(20, representation.Types?.Select(t => new TypeGoo(t.DeepClone())).ToList());
        DA.SetDataList(21, representation.Designs?.Select(d => new DesignGoo(d.DeepClone())).ToList());
        DA.SetData(22, !string.IsNullOrEmpty(representation.CreatedAt) && DateTime.TryParse(representation.CreatedAt, null, System.Globalization.DateTimeStyles.RoundtripKind, out var kitCa) ? kitCa : (DateTime?)null);
        DA.SetData(23, !string.IsNullOrEmpty(representation.ModificationdAt) && DateTime.TryParse(representation.ModificationdAt, null, System.Globalization.DateTimeStyles.RoundtripKind, out var kitUa) ? kitUa : (DateTime?)null);
    }

    protected override Kit ProcessRepresentation(Kit kit)
    {
        kit.Icon = kit.Icon?.Replace('\\', '/');
        kit.Image = kit.Image?.Replace('\\', '/');
        kit.Preview = kit.Preview?.Replace('\\', '/');
        return kit;
    }
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public SerializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public DeserializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4166");
}

public class KitIdGoo : IdGoo<KitId>
{
    public KitIdGoo() { }
    public KitIdGoo(KitId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new KitId { Id = str };
            return true;
        }
        return false;
    }
}

public class KitIdParam : IdParam<KitIdGoo, KitId>
{
    protected override string RepresentationName => "KitId";
    protected override string RepresentationNickname => "KId";
    protected override string RepresentationDescription => "Kit identifier";
    protected override string IconResourceName => "kit_24x24";
    protected override string IdIconResourceName => "kitid_24x24";
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B0");
}

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(KitGoo)))
        {
            target = (Q)(object)new KitGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<KitDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class KitDiffParam : DiffParam<KitDiffGoo, KitDiff>
{
    protected override string RepresentationName => "KitDiff";
    protected override string RepresentationNickname => "KD";
    protected override string RepresentationDescription => "Kit diff";
    protected override string IconResourceName => "kitdiff_24x24";
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B2");
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B3");
    protected override string RepresentationName => "KitDiff";
    protected override string RepresentationNickname => "KD";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a kit diff.";
    protected override string IconResourceName => "kitdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddTextParameter("Preview", "Pv?", "The optional preview.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new TypesDiffParam(), "Types", "Ty?", "The optional types diff.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignsDiffParam(), "Designs", "Dn?", "The optional designs diff.", GH_ParamAccess.item);
        pManager.AddParameter(new TagParam() { Access = GH_ParamAccess.list }, "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptParam() { Access = GH_ParamAccess.list }, "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam() { Access = GH_ParamAccess.list }, "Ports", "Pt*", "The optional ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new FilesDiffParam(), "Files", "Fl?", "The optional files diff.", GH_ParamAccess.item);
        pManager.AddParameter(new FoldersDiffParam(), "Folders", "Fo?", "The optional folders diff.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "Gu?", "The optional id.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na?", "The optional name.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image.", GH_ParamAccess.item);
        pManager.AddTextParameter("Preview", "Pv?", "The optional preview.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new TypesDiffParam(), "Types", "Ty?", "The optional types diff.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignsDiffParam(), "Designs", "Dn?", "The optional designs diff.", GH_ParamAccess.item);
        pManager.AddParameter(new TagParam() { Access = GH_ParamAccess.list }, "Tags", "Tg*", "The optional tags.", GH_ParamAccess.list);
        pManager.AddParameter(new ConceptParam() { Access = GH_ParamAccess.list }, "Concepts", "Cn*", "The optional concepts.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam() { Access = GH_ParamAccess.list }, "Ports", "Pt*", "The optional ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam() { Access = GH_ParamAccess.list }, "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
        pManager.AddParameter(new FilesDiffParam(), "Files", "Fl?", "The optional files diff.", GH_ParamAccess.item);
        pManager.AddParameter(new FoldersDiffParam(), "Folders", "Fo?", "The optional folders diff.", GH_ParamAccess.item);
        pManager.AddTextParameter("CreatedAt", "CA?", "The optional created-at timestamp.", GH_ParamAccess.item);
        pManager.AddTextParameter("UpdatedAt", "UA?", "The optional updated-at timestamp.", GH_ParamAccess.item);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, KitDiff representation)
    {
        string id = null, name = null, description = null, icon = null, image = null, preview = null, version = null, remote = null, homepage = null, license = null, createdAt = null, updatedAt = null;
        var types = new TypesDiffGoo();
        var designs = new DesignsDiffGoo();
        var files = new FilesDiffGoo();
        var folders = new FoldersDiffGoo();
        var tags = new List<TagGoo>();
        var concepts = new List<ConceptGoo>();
        var ports = new List<PortGoo>();
        var authors = new List<AuthorGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id)) representation.Id = id;
        if (DA.GetData(3, ref name)) representation.Name = name;
        if (DA.GetData(4, ref description)) representation.Description = description;
        if (DA.GetData(5, ref icon)) representation.Icon = icon;
        if (DA.GetData(6, ref image)) representation.Image = image;
        if (DA.GetData(7, ref preview)) representation.Preview = preview;
        if (DA.GetData(8, ref version)) representation.Version = version;
        if (DA.GetData(9, ref remote)) representation.Remote = remote;
        if (DA.GetData(10, ref homepage)) representation.Homepage = homepage;
        if (DA.GetData(11, ref license)) representation.License = license;
        if (DA.GetData(12, ref types)) representation.Types = types.Value.DeepClone();
        if (DA.GetData(13, ref designs)) representation.Designs = designs.Value.DeepClone();
        if (DA.GetDataList(14, tags))
        {
            representation.Tags = new TagsDiff
            {
                Modified = tags.Select(t => new TagDiffUpdate
                {
                    Tag = t.Value.DeepClone(),
                    Diff = new TagDiff
                    {
                        Id = t.Value.Id,
                        Name = t.Value.Name,
                        Description = t.Value.Description,
                        Icon = t.Value.Icon,
                        Attributes = t.Value.Attributes,
                    },
                }).ToList(),
            };
        }
        if (DA.GetDataList(15, concepts))
        {
            representation.Concepts = new ConceptsDiff
            {
                Modified = concepts.Select(c => new ConceptDiffUpdate
                {
                    Concept = c.Value.DeepClone(),
                    Diff = new ConceptDiff
                    {
                        Id = c.Value.Id,
                        Name = c.Value.Name,
                        Description = c.Value.Description,
                        Icon = c.Value.Icon,
                        Attributes = c.Value.Attributes,
                    },
                }).ToList(),
            };
        }
        if (DA.GetDataList(16, ports)) representation.Ports = ports.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(17, authors)) representation.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(18, attributes)) representation.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetData(19, ref files)) representation.Files = files.Value.DeepClone();
        if (DA.GetData(20, ref folders)) representation.Folders = folders.Value.DeepClone();
        if (DA.GetData(21, ref createdAt)) representation.CreatedAt = createdAt;
        if (DA.GetData(22, ref updatedAt)) representation.ModificationdAt = updatedAt;
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, KitDiff representation)
    {
        if (representation.ShouldSerializeId()) DA.SetData(2, representation.Id);
        if (representation.ShouldSerializeName()) DA.SetData(3, representation.Name);
        if (representation.ShouldSerializeDescription()) DA.SetData(4, representation.Description);
        if (representation.ShouldSerializeIcon()) DA.SetData(5, representation.Icon);
        if (representation.ShouldSerializeImage()) DA.SetData(6, representation.Image);
        if (representation.ShouldSerializePreview()) DA.SetData(7, representation.Preview);
        if (representation.ShouldSerializeVersion()) DA.SetData(8, representation.Version);
        if (representation.ShouldSerializeRemote()) DA.SetData(9, representation.Remote);
        if (representation.ShouldSerializeHomepage()) DA.SetData(10, representation.Homepage);
        if (representation.ShouldSerializeLicense()) DA.SetData(11, representation.License);
        if (representation.ShouldSerializeTypes()) DA.SetData(12, representation.Types is not null ? new TypesDiffGoo(representation.Types.DeepClone()) : null);
        if (representation.ShouldSerializeDesigns()) DA.SetData(13, representation.Designs is not null ? new DesignsDiffGoo(representation.Designs.DeepClone()) : null);

        if (representation.ShouldSerializeTags())
        {
            var resolvedTags = (representation.Tags?.Added ?? new List<Tag>())
                .Concat((representation.Tags?.Modified ?? new List<TagDiffUpdate>())
                    .Select(update =>
                    {
                        if (update.Diff is null)
                        {
                            return ((Tag)update.Tag).DeepClone();
                        }

                        return new Tag
                        {
                            Id = update.Diff.Id ?? update.Tag.Id,
                            Name = update.Diff.Name ?? string.Empty,
                            Description = update.Diff.Description,
                            Icon = update.Diff.Icon,
                            Attributes = update.Diff.Attributes?.Added ?? new List<Attribute>(),
                        };
                    }))
                .Select(tag => new TagGoo(tag.DeepClone()))
                .ToList();
            DA.SetDataList(14, resolvedTags);
        }

        if (representation.ShouldSerializeConcepts())
        {
            var resolvedConcepts = (representation.Concepts?.Added ?? new List<Concept>())
                .Concat((representation.Concepts?.Modified ?? new List<ConceptDiffUpdate>())
                    .Select(update =>
                    {
                        if (update.Diff is null)
                        {
                            return ((Concept)update.Concept).DeepClone();
                        }

                        return new Concept
                        {
                            Id = update.Diff.Id ?? update.Concept.Id,
                            Name = update.Diff.Name ?? string.Empty,
                            Description = update.Diff.Description,
                            Icon = update.Diff.Icon,
                            Attributes = update.Diff.Attributes?.Added ?? new List<Attribute>(),
                        };
                    }))
                .Select(concept => new ConceptGoo(concept.DeepClone()))
                .ToList();
            DA.SetDataList(15, resolvedConcepts);
        }

        if (representation.ShouldSerializePorts())
        {
            var resolvedPorts = (representation.Ports?.Added ?? new List<Port>())
                .Concat((representation.Ports?.Modified ?? new List<PortDiffUpdate>())
                    .Where(update => update.Diff is not null)
                    .Select(update => ((Port)update.Diff!).DeepClone()))
                .Select(port => new PortGoo(port.DeepClone()))
                .ToList();
            DA.SetDataList(16, resolvedPorts);
        }

        if (representation.ShouldSerializeAuthors())
        {
            var resolvedAuthors = (representation.Authors?.Added ?? new List<Author>())
                .Concat((representation.Authors?.Modified ?? new List<AuthorDiffUpdate>())
                    .Select(update =>
                    {
                        if (update.Diff is null)
                        {
                            return ((Author)update.Author).DeepClone();
                        }

                        return new Author
                        {
                            Id = update.Diff.Id ?? update.Author.Id,
                            Name = update.Diff.Name ?? string.Empty,
                            Email = update.Diff.Email ?? string.Empty,
                            Attributes = update.Diff.Attributes ?? new List<Attribute>(),
                        };
                    }))
                .Select(author => new AuthorGoo(author.DeepClone()))
                .ToList();
            DA.SetDataList(17, resolvedAuthors);
        }

        if (representation.ShouldSerializeAttributes())
        {
            var resolvedAttributes = (representation.Attributes?.Added ?? new List<Attribute>())
                .Concat((representation.Attributes?.Modified ?? new List<AttributeDiffUpdate>())
                    .Where(update => update.Diff is not null)
                    .Select(update => ((Attribute)update.Diff!).DeepClone()))
                .Select(attribute => new AttributeGoo(attribute.DeepClone()))
                .ToList();
            DA.SetDataList(18, resolvedAttributes);
        }

        if (representation.ShouldSerializeFiles()) DA.SetData(19, representation.Files is not null ? new FilesDiffGoo(representation.Files.DeepClone()) : null);
        if (representation.ShouldSerializeFolders()) DA.SetData(20, representation.Folders is not null ? new FoldersDiffGoo(representation.Folders.DeepClone()) : null);
        if (representation.ShouldSerializeCreatedAt()) DA.SetData(21, representation.CreatedAt);
        if (representation.ShouldSerializeModificationdAt()) DA.SetData(22, representation.ModificationdAt);
    }
}

public class SerializeKitDiffComponent : SerializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public SerializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B4");
}

public class DeserializeKitDiffComponent : DeserializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public DeserializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B5");
}

public class KitsDiffGoo : DiffGoo<KitsDiff>
{
    public KitsDiffGoo() { }
    public KitsDiffGoo(KitsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("KitsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<KitsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class KitsDiffParam : DiffParam<KitsDiffGoo, KitsDiff>
{
    protected override string RepresentationName => "KitsDiff";
    protected override string RepresentationNickname => "KDs";
    protected override string RepresentationDescription => "Kit collection diff";
    protected override string IconResourceName => "kitsdiff_24x24";
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C3");
}

public class KitsDiffComponent : DiffComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C4");
    protected override string RepresentationName => "KitsDiff";
    protected override string RepresentationNickname => "KDs";
    protected override string RepresentationDescription => "Construct, deconstruct or modify a collection of kit diffs.";
    protected override string IconResourceName => "kitsdiff_24x24";

    protected override void RegisterRepresentationInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed kit ids.", GH_ParamAccess.list);
        pManager.AddParameter(new KitDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated kit diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new KitParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added kits.", GH_ParamAccess.list);
    }

    protected override void RegisterRepresentationOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitIdParam() { Access = GH_ParamAccess.list }, "Removed", "Rm*", "The optional removed kit ids.", GH_ParamAccess.list);
        pManager.AddParameter(new KitDiffParam() { Access = GH_ParamAccess.list }, "Updated", "Up*", "The optional updated kit diffs.", GH_ParamAccess.list);
        pManager.AddParameter(new KitParam() { Access = GH_ParamAccess.list }, "Added", "Ad*", "The optional added kits.", GH_ParamAccess.list);
    }

    protected override void GetRepresentationData(IGH_DataAccess DA, KitsDiff representation)
    {
        var removed = new List<KitIdGoo>();
        var updated = new List<KitDiffGoo>();
        var added = new List<KitGoo>();

        if (DA.GetDataList(2, removed)) representation.Removed = removed.Select(r => r.Value.DeepClone()).ToList();
        if (DA.GetDataList(3, updated)) representation.Modified = updated.Select(u => new KitDiffUpdate { Kit = new KitId { Id = u.Value.Id ?? "" }, Diff = u.Value.DeepClone() }).ToList();
        if (DA.GetDataList(4, added)) representation.Added = added.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetRepresentationData(IGH_DataAccess DA, KitsDiff representation)
    {
        DA.SetDataList(2, representation.Removed.Select(r => new KitIdGoo(r.DeepClone())).ToList());
        DA.SetDataList(3, representation.Modified.Select(u => new KitDiffGoo((u.Diff ?? new KitDiff { Id = u.Kit.Id }).DeepClone())).ToList());
        DA.SetDataList(4, representation.Added.Select(a => new KitGoo(a.DeepClone())).ToList());
    }

}

public class SerializeKitsDiffComponent : SerializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public SerializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C5");
}

public class DeserializeKitsDiffComponent : DeserializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public DeserializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C6");
}

#endregion 🧳Kit

#region 🎲Change
// Implementations MUST expose params and passthrough components for semio change entities.

public class AuthorDiffGoo : DiffGoo<AuthorDiff> { public AuthorDiffGoo() { } public AuthorDiffGoo(AuthorDiff value) : base(value) { } }
public class AuthorDiffParam : DiffParam<AuthorDiffGoo, AuthorDiff>
{
    protected override string RepresentationName => "AuthorDiff";
    protected override string RepresentationNickname => "AuD";
    protected override string RepresentationDescription => "Author diff";
    protected override string IconResourceName => "author_24x24";
    public override Guid ComponentGuid => new("8F239855-4077-4672-9C62-2162F6B9A301");
}

public class BenchmarkDiffGoo : DiffGoo<BenchmarkDiff> { public BenchmarkDiffGoo() { } public BenchmarkDiffGoo(BenchmarkDiff value) : base(value) { } }
public class BenchmarkDiffParam : DiffParam<BenchmarkDiffGoo, BenchmarkDiff>
{
    protected override string RepresentationName => "BenchmarkDiff";
    protected override string RepresentationNickname => "BmD";
    protected override string RepresentationDescription => "Benchmark diff";
    protected override string IconResourceName => "benchmark_24x24";
    public override Guid ComponentGuid => new("D02285B2-53A9-45EA-8F38-FCF0D6A2A301");
}

public class PortDiffGoo : DiffGoo<PortDiff> { public PortDiffGoo() { } public PortDiffGoo(PortDiff value) : base(value) { } }
public class PortDiffParam : DiffParam<PortDiffGoo, PortDiff>
{
    protected override string RepresentationName => "PortDiff";
    protected override string RepresentationNickname => "PoD";
    protected override string RepresentationDescription => "Port diff";
    protected override string IconResourceName => "port_24x24";
    public override Guid ComponentGuid => new("A5D2AAE4-B8C6-41FD-A64F-1D4A0B27A301");
}

public class PropDiffGoo : DiffGoo<PropDiff> { public PropDiffGoo() { } public PropDiffGoo(PropDiff value) : base(value) { } }
public class PropDiffParam : DiffParam<PropDiffGoo, PropDiff>
{
    protected override string RepresentationName => "PropDiff";
    protected override string RepresentationNickname => "PrD";
    protected override string RepresentationDescription => "Prop diff";
    protected override string IconResourceName => "prop_24x24";
    public override Guid ComponentGuid => new("60B4A67D-2C6D-4730-A739-5EA66123A301");
}

public class TagDiffGoo : DiffGoo<TagDiff> { public TagDiffGoo() { } public TagDiffGoo(TagDiff value) : base(value) { } }
public class TagDiffParam : DiffParam<TagDiffGoo, TagDiff>
{
    protected override string RepresentationName => "TagDiff";
    protected override string RepresentationNickname => "TgD";
    protected override string RepresentationDescription => "Tag diff";
    protected override string IconResourceName => "tag_24x24";
    public override Guid ComponentGuid => new("50E1C62B-81EF-45F3-8462-57D7C68FA301");
}

public class ConceptDiffGoo : DiffGoo<ConceptDiff> { public ConceptDiffGoo() { } public ConceptDiffGoo(ConceptDiff value) : base(value) { } }
public class ConceptDiffParam : DiffParam<ConceptDiffGoo, ConceptDiff>
{
    protected override string RepresentationName => "ConceptDiff";
    protected override string RepresentationNickname => "CnD";
    protected override string RepresentationDescription => "Concept diff";
    protected override string IconResourceName => "concept_24x24";
    public override Guid ComponentGuid => new("8DCE6E2B-4EAD-421C-B7D2-E9F5D164A301");
}

public class LayerDiffGoo : DiffGoo<LayerDiff> { public LayerDiffGoo() { } public LayerDiffGoo(LayerDiff value) : base(value) { } }
public class LayerDiffParam : DiffParam<LayerDiffGoo, LayerDiff>
{
    protected override string RepresentationName => "LayerDiff";
    protected override string RepresentationNickname => "LyD";
    protected override string RepresentationDescription => "Layer diff";
    protected override string IconResourceName => "layer_24x24";
    public override Guid ComponentGuid => new("4C44B36F-2854-4B53-8E34-1AD0E8DEA301");
}

public class GroupDiffGoo : DiffGoo<GroupDiff> { public GroupDiffGoo() { } public GroupDiffGoo(GroupDiff value) : base(value) { } }
public class GroupDiffParam : DiffParam<GroupDiffGoo, GroupDiff>
{
    protected override string RepresentationName => "GroupDiff";
    protected override string RepresentationNickname => "GrD";
    protected override string RepresentationDescription => "Group diff";
    protected override string IconResourceName => "group_24x24";
    public override Guid ComponentGuid => new("810B8571-BC34-485C-BAD7-C5FAEF8CA301");
}

public class StatDiffGoo : DiffGoo<StatDiff> { public StatDiffGoo() { } public StatDiffGoo(StatDiff value) : base(value) { } }
public class StatDiffParam : DiffParam<StatDiffGoo, StatDiff>
{
    protected override string RepresentationName => "StatDiff";
    protected override string RepresentationNickname => "StD";
    protected override string RepresentationDescription => "Stat diff";
    protected override string IconResourceName => "stat_24x24";
    public override Guid ComponentGuid => new("C9A3F76F-30A9-4EF1-A92B-2A83AA4CA301");
}

public class AttributeChangeGoo : ChangeGoo<AttributeChange> { public AttributeChangeGoo() { } public AttributeChangeGoo(AttributeChange value) : base(value) { } }
public class AttributeChangeParam : ChangeParam<AttributeChangeGoo, AttributeChange>
{
    protected override string RepresentationName => "AttributeChange";
    protected override string RepresentationNickname => "AtC";
    protected override string RepresentationDescription => "Attribute change";
    protected override string IconResourceName => "attribute_diff_24x24";
    public override Guid ComponentGuid => new("0DA5D764-5322-47A2-A0BC-551C2BEACB01");
}
public class AttributeChangeComponent : ChangeComponent<AttributeParam, AttributeGoo, Attribute, AttributeDiffParam, AttributeDiffGoo, AttributeDiff, AttributeChangeParam, AttributeChangeGoo, AttributeChange>
{
    public override Guid ComponentGuid => new("0DA5D764-5322-47A2-A0BC-551C2BEACB02");
    protected override string EntityName => "Attribute";
    protected override string EntityNickname => "At";
    protected override string IconResourceName => "attribute_diff_24x24";
}

public class AuthorChangeGoo : ChangeGoo<AuthorChange> { public AuthorChangeGoo() { } public AuthorChangeGoo(AuthorChange value) : base(value) { } }
public class AuthorChangeParam : ChangeParam<AuthorChangeGoo, AuthorChange>
{
    protected override string RepresentationName => "AuthorChange";
    protected override string RepresentationNickname => "AuC";
    protected override string RepresentationDescription => "Author change";
    protected override string IconResourceName => "author_24x24";
    public override Guid ComponentGuid => new("3E6E5C8F-6D0D-4CA6-A06A-6F2CCB8DF101");
}
public class AuthorChangeComponent : ChangeComponent<AuthorParam, AuthorGoo, Author, AuthorDiffParam, AuthorDiffGoo, AuthorDiff, AuthorChangeParam, AuthorChangeGoo, AuthorChange>
{
    public override Guid ComponentGuid => new("3E6E5C8F-6D0D-4CA6-A06A-6F2CCB8DF102");
    protected override string EntityName => "Author";
    protected override string EntityNickname => "Au";
    protected override string IconResourceName => "author_24x24";
}

public class FileChangeGoo : ChangeGoo<FileChange> { public FileChangeGoo() { } public FileChangeGoo(FileChange value) : base(value) { } }
public class FileChangeParam : ChangeParam<FileChangeGoo, FileChange>
{
    protected override string RepresentationName => "FileChange";
    protected override string RepresentationNickname => "FlC";
    protected override string RepresentationDescription => "File change";
    protected override string IconResourceName => "file_diff_24x24";
    public override Guid ComponentGuid => new("8F2B2F13-64C0-4A7D-9F5E-9DBA5E2F4101");
}
public class FileChangeComponent : ChangeComponent<FileParam, FileGoo, File, FileDiffParam, FileDiffGoo, FileDiff, FileChangeParam, FileChangeGoo, FileChange>
{
    public override Guid ComponentGuid => new("8F2B2F13-64C0-4A7D-9F5E-9DBA5E2F4102");
    protected override string EntityName => "File";
    protected override string EntityNickname => "Fl";
    protected override string IconResourceName => "file_diff_24x24";
}

public class FolderChangeGoo : ChangeGoo<FolderChange> { public FolderChangeGoo() { } public FolderChangeGoo(FolderChange value) : base(value) { } }
public class FolderChangeParam : ChangeParam<FolderChangeGoo, FolderChange>
{
    protected override string RepresentationName => "FolderChange";
    protected override string RepresentationNickname => "FoC";
    protected override string RepresentationDescription => "Folder change";
    protected override string IconResourceName => "folder_diff_24x24";
    public override Guid ComponentGuid => new("EEA0E8CF-178F-42CD-8ECA-5E3365B82101");
}
public class FolderChangeComponent : ChangeComponent<FolderParam, FolderGoo, Folder, FolderDiffParam, FolderDiffGoo, FolderDiff, FolderChangeParam, FolderChangeGoo, FolderChange>
{
    public override Guid ComponentGuid => new("EEA0E8CF-178F-42CD-8ECA-5E3365B82102");
    protected override string EntityName => "Folder";
    protected override string EntityNickname => "Fo";
    protected override string IconResourceName => "folder_diff_24x24";
}

public class BenchmarkChangeGoo : ChangeGoo<BenchmarkChange> { public BenchmarkChangeGoo() { } public BenchmarkChangeGoo(BenchmarkChange value) : base(value) { } }
public class BenchmarkChangeParam : ChangeParam<BenchmarkChangeGoo, BenchmarkChange>
{
    protected override string RepresentationName => "BenchmarkChange";
    protected override string RepresentationNickname => "BmC";
    protected override string RepresentationDescription => "Benchmark change";
    protected override string IconResourceName => "benchmark_24x24";
    public override Guid ComponentGuid => new("07D84C07-EE9C-4E70-B28D-D089D58D5101");
}
public class BenchmarkChangeComponent : ChangeComponent<BenchmarkParam, BenchmarkGoo, Benchmark, BenchmarkDiffParam, BenchmarkDiffGoo, BenchmarkDiff, BenchmarkChangeParam, BenchmarkChangeGoo, BenchmarkChange>
{
    public override Guid ComponentGuid => new("07D84C07-EE9C-4E70-B28D-D089D58D5102");
    protected override string EntityName => "Benchmark";
    protected override string EntityNickname => "Bm";
    protected override string IconResourceName => "benchmark_24x24";
}

public class QualityChangeGoo : ChangeGoo<QualityChange> { public QualityChangeGoo() { } public QualityChangeGoo(QualityChange value) : base(value) { } }
public class QualityChangeParam : ChangeParam<QualityChangeGoo, QualityChange>
{
    protected override string RepresentationName => "QualityChange";
    protected override string RepresentationNickname => "QlC";
    protected override string RepresentationDescription => "Quality change";
    protected override string IconResourceName => "quality_diff_24x24";
    public override Guid ComponentGuid => new("B5F78DDB-53A8-4A29-97A3-76F8D46AC101");
}
public class QualityChangeComponent : ChangeComponent<QualityParam, QualityGoo, Quality, QualityDiffParam, QualityDiffGoo, QualityDiff, QualityChangeParam, QualityChangeGoo, QualityChange>
{
    public override Guid ComponentGuid => new("B5F78DDB-53A8-4A29-97A3-76F8D46AC102");
    protected override string EntityName => "Quality";
    protected override string EntityNickname => "Ql";
    protected override string IconResourceName => "quality_diff_24x24";
}

public class PortChangeGoo : ChangeGoo<PortChange> { public PortChangeGoo() { } public PortChangeGoo(PortChange value) : base(value) { } }
public class PortChangeParam : ChangeParam<PortChangeGoo, PortChange>
{
    protected override string RepresentationName => "PortChange";
    protected override string RepresentationNickname => "PoC";
    protected override string RepresentationDescription => "Port change";
    protected override string IconResourceName => "port_24x24";
    public override Guid ComponentGuid => new("D7FCA4AA-7099-4D0F-B66B-177A41F4E101");
}
public class PortChangeComponent : ChangeComponent<PortParam, PortGoo, Port, PortDiffParam, PortDiffGoo, PortDiff, PortChangeParam, PortChangeGoo, PortChange>
{
    public override Guid ComponentGuid => new("D7FCA4AA-7099-4D0F-B66B-177A41F4E102");
    protected override string EntityName => "Port";
    protected override string EntityNickname => "Po";
    protected override string IconResourceName => "port_24x24";
}

public class PropChangeGoo : ChangeGoo<PropChange> { public PropChangeGoo() { } public PropChangeGoo(PropChange value) : base(value) { } }
public class PropChangeParam : ChangeParam<PropChangeGoo, PropChange>
{
    protected override string RepresentationName => "PropChange";
    protected override string RepresentationNickname => "PrC";
    protected override string RepresentationDescription => "Prop change";
    protected override string IconResourceName => "prop_24x24";
    public override Guid ComponentGuid => new("7C6D159C-5C29-4A15-A9D0-4DB9E66D7101");
}
public class PropChangeComponent : ChangeComponent<PropParam, PropGoo, Prop, PropDiffParam, PropDiffGoo, PropDiff, PropChangeParam, PropChangeGoo, PropChange>
{
    public override Guid ComponentGuid => new("7C6D159C-5C29-4A15-A9D0-4DB9E66D7102");
    protected override string EntityName => "Prop";
    protected override string EntityNickname => "Pr";
    protected override string IconResourceName => "prop_24x24";
}

public class TagChangeGoo : ChangeGoo<TagChange> { public TagChangeGoo() { } public TagChangeGoo(TagChange value) : base(value) { } }
public class TagChangeParam : ChangeParam<TagChangeGoo, TagChange>
{
    protected override string RepresentationName => "TagChange";
    protected override string RepresentationNickname => "TgC";
    protected override string RepresentationDescription => "Tag change";
    protected override string IconResourceName => "tag_24x24";
    public override Guid ComponentGuid => new("17C0BE7A-3D72-4933-A8A6-B0D0B7AA8101");
}
public class TagChangeComponent : ChangeComponent<TagParam, TagGoo, Tag, TagDiffParam, TagDiffGoo, TagDiff, TagChangeParam, TagChangeGoo, TagChange>
{
    public override Guid ComponentGuid => new("17C0BE7A-3D72-4933-A8A6-B0D0B7AA8102");
    protected override string EntityName => "Tag";
    protected override string EntityNickname => "Tg";
    protected override string IconResourceName => "tag_24x24";
}

public class ConceptChangeGoo : ChangeGoo<ConceptChange> { public ConceptChangeGoo() { } public ConceptChangeGoo(ConceptChange value) : base(value) { } }
public class ConceptChangeParam : ChangeParam<ConceptChangeGoo, ConceptChange>
{
    protected override string RepresentationName => "ConceptChange";
    protected override string RepresentationNickname => "CnC";
    protected override string RepresentationDescription => "Concept change";
    protected override string IconResourceName => "concept_24x24";
    public override Guid ComponentGuid => new("CAD88735-2F40-46CB-928B-369D84928101");
}
public class ConceptChangeComponent : ChangeComponent<ConceptParam, ConceptGoo, Concept, ConceptDiffParam, ConceptDiffGoo, ConceptDiff, ConceptChangeParam, ConceptChangeGoo, ConceptChange>
{
    public override Guid ComponentGuid => new("CAD88735-2F40-46CB-928B-369D84928102");
    protected override string EntityName => "Concept";
    protected override string EntityNickname => "Cn";
    protected override string IconResourceName => "concept_24x24";
}

public class RepresentationChangeGoo : ChangeGoo<RepresentationChange> { public RepresentationChangeGoo() { } public RepresentationChangeGoo(RepresentationChange value) : base(value) { } }
public class RepresentationChangeParam : ChangeParam<RepresentationChangeGoo, RepresentationChange>
{
    protected override string RepresentationName => "RepresentationChange";
    protected override string RepresentationNickname => "MoC";
    protected override string RepresentationDescription => "Representation change";
    protected override string IconResourceName => "representation_diff_24x24";
    public override Guid ComponentGuid => new("71A8A53D-C8B8-43E4-918D-1D7DD7D59101");
}
public class RepresentationChangeComponent : ChangeComponent<RepresentationParam, RepresentationGoo, Representation, RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff, RepresentationChangeParam, RepresentationChangeGoo, RepresentationChange>
{
    public override Guid ComponentGuid => new("71A8A53D-C8B8-43E4-918D-1D7DD7D59102");
    protected override string EntityName => "Representation";
    protected override string EntityNickname => "Mo";
    protected override string IconResourceName => "representation_diff_24x24";
}

public class ConnectorChangeGoo : ChangeGoo<ConnectorChange> { public ConnectorChangeGoo() { } public ConnectorChangeGoo(ConnectorChange value) : base(value) { } }
public class ConnectorChangeParam : ChangeParam<ConnectorChangeGoo, ConnectorChange>
{
    protected override string RepresentationName => "ConnectorChange";
    protected override string RepresentationNickname => "CoC";
    protected override string RepresentationDescription => "Connector change";
    protected override string IconResourceName => "connector_diff_24x24";
    public override Guid ComponentGuid => new("7A10C866-F666-497D-8FD7-1D1AD39CA101");
}
public class ConnectorChangeComponent : ChangeComponent<ConnectorParam, ConnectorGoo, Connector, ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff, ConnectorChangeParam, ConnectorChangeGoo, ConnectorChange>
{
    public override Guid ComponentGuid => new("7A10C866-F666-497D-8FD7-1D1AD39CA102");
    protected override string EntityName => "Connector";
    protected override string EntityNickname => "Co";
    protected override string IconResourceName => "connector_diff_24x24";
}

public class TypeChangeGoo : ChangeGoo<TypeChange> { public TypeChangeGoo() { } public TypeChangeGoo(TypeChange value) : base(value) { } }
public class TypeChangeParam : ChangeParam<TypeChangeGoo, TypeChange>
{
    protected override string RepresentationName => "TypeChange";
    protected override string RepresentationNickname => "TyC";
    protected override string RepresentationDescription => "Type change";
    protected override string IconResourceName => "type_diff_24x24";
    public override Guid ComponentGuid => new("A305B34B-8153-467E-A9F8-5FA158CA9101");
}
public class TypeChangeComponent : ChangeComponent<TypeParam, TypeGoo, Type, TypeDiffParam, TypeDiffGoo, TypeDiff, TypeChangeParam, TypeChangeGoo, TypeChange>
{
    public override Guid ComponentGuid => new("A305B34B-8153-467E-A9F8-5FA158CA9102");
    protected override string EntityName => "Type";
    protected override string EntityNickname => "Ty";
    protected override string IconResourceName => "type_diff_24x24";
}

public class LayerChangeGoo : ChangeGoo<LayerChange> { public LayerChangeGoo() { } public LayerChangeGoo(LayerChange value) : base(value) { } }
public class LayerChangeParam : ChangeParam<LayerChangeGoo, LayerChange>
{
    protected override string RepresentationName => "LayerChange";
    protected override string RepresentationNickname => "LyC";
    protected override string RepresentationDescription => "Layer change";
    protected override string IconResourceName => "layer_24x24";
    public override Guid ComponentGuid => new("A61BEAB7-92A4-4862-9F4A-BBB80CD9A101");
}
public class LayerChangeComponent : ChangeComponent<LayerParam, LayerGoo, Layer, LayerDiffParam, LayerDiffGoo, LayerDiff, LayerChangeParam, LayerChangeGoo, LayerChange>
{
    public override Guid ComponentGuid => new("A61BEAB7-92A4-4862-9F4A-BBB80CD9A102");
    protected override string EntityName => "Layer";
    protected override string EntityNickname => "Ly";
    protected override string IconResourceName => "layer_24x24";
}

public class PieceChangeGoo : ChangeGoo<PieceChange> { public PieceChangeGoo() { } public PieceChangeGoo(PieceChange value) : base(value) { } }
public class PieceChangeParam : ChangeParam<PieceChangeGoo, PieceChange>
{
    protected override string RepresentationName => "PieceChange";
    protected override string RepresentationNickname => "PcC";
    protected override string RepresentationDescription => "Piece change";
    protected override string IconResourceName => "piece_diff_24x24";
    public override Guid ComponentGuid => new("836AAB61-44B5-4881-8C66-EFF8DBD4B101");
}
public class PieceChangeComponent : ChangeComponent<PieceParam, PieceGoo, Piece, PieceDiffParam, PieceDiffGoo, PieceDiff, PieceChangeParam, PieceChangeGoo, PieceChange>
{
    public override Guid ComponentGuid => new("836AAB61-44B5-4881-8C66-EFF8DBD4B102");
    protected override string EntityName => "Piece";
    protected override string EntityNickname => "Pc";
    protected override string IconResourceName => "piece_diff_24x24";
}

public class GroupChangeGoo : ChangeGoo<GroupChange> { public GroupChangeGoo() { } public GroupChangeGoo(GroupChange value) : base(value) { } }
public class GroupChangeParam : ChangeParam<GroupChangeGoo, GroupChange>
{
    protected override string RepresentationName => "GroupChange";
    protected override string RepresentationNickname => "GrC";
    protected override string RepresentationDescription => "Group change";
    protected override string IconResourceName => "group_24x24";
    public override Guid ComponentGuid => new("9FE9F093-3EFF-47CE-BF1D-31E5AFD5C101");
}
public class GroupChangeComponent : ChangeComponent<GroupParam, GroupGoo, SemioGroup, GroupDiffParam, GroupDiffGoo, GroupDiff, GroupChangeParam, GroupChangeGoo, GroupChange>
{
    public override Guid ComponentGuid => new("9FE9F093-3EFF-47CE-BF1D-31E5AFD5C102");
    protected override string EntityName => "Group";
    protected override string EntityNickname => "Gr";
    protected override string IconResourceName => "group_24x24";
}

public class SideChangeGoo : ChangeGoo<SideChange> { public SideChangeGoo() { } public SideChangeGoo(SideChange value) : base(value) { } }
public class SideChangeParam : ChangeParam<SideChangeGoo, SideChange>
{
    protected override string RepresentationName => "SideChange";
    protected override string RepresentationNickname => "SdC";
    protected override string RepresentationDescription => "Side change";
    protected override string IconResourceName => "side_diff_24x24";
    public override Guid ComponentGuid => new("2B8B5287-8F3D-4CF9-BEEA-DF04E778D101");
}
public class SideChangeComponent : ChangeComponent<SideParam, SideGoo, Side, SideDiffParam, SideDiffGoo, SideDiff, SideChangeParam, SideChangeGoo, SideChange>
{
    public override Guid ComponentGuid => new("2B8B5287-8F3D-4CF9-BEEA-DF04E778D102");
    protected override string EntityName => "Side";
    protected override string EntityNickname => "Sd";
    protected override string IconResourceName => "side_diff_24x24";
}

public class ConnectionChangeGoo : ChangeGoo<ConnectionChange> { public ConnectionChangeGoo() { } public ConnectionChangeGoo(ConnectionChange value) : base(value) { } }
public class ConnectionChangeParam : ChangeParam<ConnectionChangeGoo, ConnectionChange>
{
    protected override string RepresentationName => "ConnectionChange";
    protected override string RepresentationNickname => "CnC";
    protected override string RepresentationDescription => "Connection change";
    protected override string IconResourceName => "connection_diff_24x24";
    public override Guid ComponentGuid => new("59352953-A18D-4339-A81B-125A15C3E101");
}
public class ConnectionChangeComponent : ChangeComponent<ConnectionParam, ConnectionGoo, Connection, ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff, ConnectionChangeParam, ConnectionChangeGoo, ConnectionChange>
{
    public override Guid ComponentGuid => new("59352953-A18D-4339-A81B-125A15C3E102");
    protected override string EntityName => "Connection";
    protected override string EntityNickname => "Cn";
    protected override string IconResourceName => "connection_diff_24x24";
}

public class StatChangeGoo : ChangeGoo<StatChange> { public StatChangeGoo() { } public StatChangeGoo(StatChange value) : base(value) { } }
public class StatChangeParam : ChangeParam<StatChangeGoo, StatChange>
{
    protected override string RepresentationName => "StatChange";
    protected override string RepresentationNickname => "StC";
    protected override string RepresentationDescription => "Stat change";
    protected override string IconResourceName => "stat_24x24";
    public override Guid ComponentGuid => new("D9190DF2-A738-4FE2-9244-410C70A3F101");
}
public class StatChangeComponent : ChangeComponent<StatParam, StatGoo, Stat, StatDiffParam, StatDiffGoo, StatDiff, StatChangeParam, StatChangeGoo, StatChange>
{
    public override Guid ComponentGuid => new("D9190DF2-A738-4FE2-9244-410C70A3F102");
    protected override string EntityName => "Stat";
    protected override string EntityNickname => "St";
    protected override string IconResourceName => "stat_24x24";
}

public class DesignChangeGoo : ChangeGoo<DesignChange> { public DesignChangeGoo() { } public DesignChangeGoo(DesignChange value) : base(value) { } }
public class DesignChangeParam : ChangeParam<DesignChangeGoo, DesignChange>
{
    protected override string RepresentationName => "DesignChange";
    protected override string RepresentationNickname => "DeC";
    protected override string RepresentationDescription => "Design change";
    protected override string IconResourceName => "design_diff_24x24";
    public override Guid ComponentGuid => new("4F16F428-3E34-4D2D-B067-27A20A06A101");
}
public class DesignChangeComponent : ChangeComponent<DesignParam, DesignGoo, Design, DesignDiffParam, DesignDiffGoo, DesignDiff, DesignChangeParam, DesignChangeGoo, DesignChange>
{
    public override Guid ComponentGuid => new("4F16F428-3E34-4D2D-B067-27A20A06A102");
    protected override string EntityName => "Design";
    protected override string EntityNickname => "De";
    protected override string IconResourceName => "design_diff_24x24";
}

public class KitChangeGoo : ChangeGoo<KitChange> { public KitChangeGoo() { } public KitChangeGoo(KitChange value) : base(value) { } }
public class KitChangeParam : ChangeParam<KitChangeGoo, KitChange>
{
    protected override string RepresentationName => "KitChange";
    protected override string RepresentationNickname => "KtC";
    protected override string RepresentationDescription => "Kit change";
    protected override string IconResourceName => "kitdiff_24x24";
    public override Guid ComponentGuid => new("4B4F48E0-28BA-476A-B7A5-BC325F3CB101");
}
public class KitChangeComponent : ChangeComponent<KitParam, KitGoo, Kit, KitDiffParam, KitDiffGoo, KitDiff, KitChangeParam, KitChangeGoo, KitChange>
{
    public override Guid ComponentGuid => new("4B4F48E0-28BA-476A-B7A5-BC325F3CB102");
    protected override string EntityName => "Kit";
    protected override string EntityNickname => "Kt";
    protected override string IconResourceName => "kitdiff_24x24";
}

#endregion 🎲Change

#region 📌Scripting
// Callers MUST use these helpers for C# script component integration.

public abstract class ScriptingComponent : Component
{
    public ScriptingComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Scripting")
    { }
}

public class EncodeTextComponent : ScriptingComponent
{
    public EncodeTextComponent()
        : base("Encode Text", ">Txt", "Encode a text.")
    { }
    public override Guid ComponentGuid => new("FBDDF723-80BD-4AF9-A1EE-450A27D50ABE");

    protected override Bitmap Icon => Resources.encode_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to encode.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Mode", "Mo", "0: url safe encoding ()\n1: base64 encoding\n2: replace only", GH_ParamAccess.item, 0);

        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that will be replaced after encoding.", GH_ParamAccess.list);

        pManager.AddTextParameter("Replace", "Re", "Placeholder text that replaces the forbidden text after encoding.", GH_ParamAccess.list);

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        var mode = 0;
        var forbidden = new List<string>();
        var replace = new List<string>();
        DA.GetData(0, ref text);
        DA.GetData(1, ref mode);
        DA.GetDataList(2, forbidden);
        DA.GetDataList(3, replace);
        DA.SetData(0, Semio.Utility.Encode(text, (EncodeMode)mode, new Tuple<List<string>, List<string>>(forbidden, replace)));
    }
}

public class DecodeTextComponent : ScriptingComponent
{
    public DecodeTextComponent() : base("Decode Text", "<Txt", "Decode a text.") { }
    public override Guid ComponentGuid => new("E7158D28-87DE-493F-8D78-923265C3E211");
    protected override Bitmap Icon => Resources.decode_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text to decode.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Mode", "Mo", "0: url safe encoding ()\n1: base64 encoding\n2: replace only", GH_ParamAccess.item, 0);

        pManager.AddTextParameter("Replace", "Re", "Placeholder text that was used to encode forbidden text after encoding and is restored before decoding. It will be applied sequentially. Make sure to invert the order of your original list.", GH_ParamAccess.list);

        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that gets restored before decoding.", GH_ParamAccess.list);

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Decoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        var mode = 0;
        var replace = new List<string>();
        var forbidden = new List<string>();
        DA.GetData(0, ref text);
        DA.GetData(1, ref mode);
        DA.GetDataList(2, replace);
        DA.GetDataList(3, forbidden);
        DA.SetData(0, Semio.Utility.Decode(text, (EncodeMode)mode, new Tuple<List<string>, List<string>>(replace, forbidden)));
    }
}

public class ImportRepresentationComponent : ScriptingComponent
{
    public ImportRepresentationComponent() : base("Import Representation", "ImpRepresentation", "Imports a Rhino representation object from a semio file.") { }
    public override Guid ComponentGuid => new("0E2A82A4-494E-4D38-9E32-FD26A1B6EC6D");
    protected override Bitmap Icon => Resources.representation_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Md", "Representation consumed by Import Representation for unit-aware import scaling.", GH_ParamAccess.item);
        pManager.AddParameter(new FileParam(), "File", "Fi", "Semio file consumed by Import Representation that contains a Rhino file blob.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new Param_RepresentationObject(), "Rhino RepresentationObject", "Mo*", "Imported Rhino representation objects produced by Import Representation.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        RepresentationGoo representationGoo = null;
        if (!DA.GetData(0, ref representationGoo) || representationGoo?.Value is null) return;

        FileGoo fileGoo = null;
        if (!DA.GetData(1, ref fileGoo) || fileGoo?.Value is null) return;

        try
        {
            var importedRhinoObjects = Utility.ImportRhinoDocumentObjectsFromSemioFile(fileGoo.Value, representationGoo.Value);
            DA.SetDataList(0, importedRhinoObjects.Select(importedRhinoObject => new RhinoRepresentationObjectData(importedRhinoObject)));
        }
        catch (Exception exception)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, exception.Message);
        }
    }
}

public class RepresentationObjectToGroupComponent : ScriptingComponent
{
    public RepresentationObjectToGroupComponent() : base("RepresentationObject To Group", "Mo→Gr", "RepresentationObject To Group translates imported Rhino representation objects into a single native Rhino/Grasshopper group.") { }
    public override Guid ComponentGuid => new("9C74A31E-3B07-48EC-A6C9-B16A3F1EA9DD");
    protected override Bitmap Icon => Resources.group_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new Param_RepresentationObject(), "Rhino RepresentationObject", "Mo*", "Rhino representation objects consumed by RepresentationObject To Group.", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new Param_Group(), "Group", "Gr", "Single native Rhino/Grasshopper group produced by RepresentationObject To Group.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var representationObjectInputs = new List<object>();
        if (!DA.GetDataList(0, representationObjectInputs)) return;

        var rhinoRepresentationObjects = new List<Utility.RhinoRepresentationObject>();
        foreach (var representationObjectInput in representationObjectInputs)
        {
            if (!Utility.TryResolveRhinoRepresentationContext(representationObjectInput, out var rhinoRepresentationObject))
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "Input must be the Rhino RepresentationObject output of Import Representation.");
                return;
            }
            rhinoRepresentationObjects.Add(rhinoRepresentationObject);
        }

        try
        {
            var nativeGroupData = BuildNativeRhinoGeometryGroup(rhinoRepresentationObjects);
            DA.SetData(0, nativeGroupData);
        }
        catch (Exception exception)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, exception.Message);
        }
    }

    private sealed class LayerGroupNode
    {
        public LayerGroupNode(string name) => Name = name;

        public string Name { get; }
        public Dictionary<string, LayerGroupNode> Children { get; } = new(StringComparer.OrdinalIgnoreCase);
        public List<IGH_GeometricGoo> Geometries { get; } = new();
    }

    private static GH_GeometryGroup BuildNativeRhinoGeometryGroup(List<Utility.RhinoRepresentationObject> rhinoRepresentationObjects)
    {
        var rootNode = new LayerGroupNode(BuildNativeRootGroupName(rhinoRepresentationObjects));
        foreach (var rhinoRepresentationObject in rhinoRepresentationObjects)
        {
            var representation = rhinoRepresentationObject.Representation;
            var representationObjects = ResolveRepresentationObjects(rhinoRepresentationObject).ToList();
            if (representationObjects.Count == 0)
                continue;

            foreach (var representationObject in representationObjects)
            {
                var sourceGeometry = representationObject?.Geometry;
                if (sourceGeometry is null)
                    continue;

                var geometricGoo = GH_Convert.ToGeometricGoo(sourceGeometry.Duplicate());
                if (geometricGoo is null)
                    continue;

                var layerPath = ResolveLayerPath(representation, representationObject);
                AddGeometryToLayerTree(rootNode, layerPath, geometricGoo);
            }
        }

        return BuildGeometryGroup(rootNode);
    }

    private static IEnumerable<Rhino.FileIO.File3dmObject> ResolveRepresentationObjects(Utility.RhinoRepresentationObject rhinoRepresentationObject)
    {
        if (rhinoRepresentationObject?.RepresentationObject is not null)
            return new[] { rhinoRepresentationObject.RepresentationObject };
        return rhinoRepresentationObject?.Representation?.Objects?.Where(representationObject => representationObject is not null) ?? Enumerable.Empty<Rhino.FileIO.File3dmObject>();
    }

    private static string BuildNativeRootGroupName(List<Utility.RhinoRepresentationObject> rhinoRepresentationObjects)
    {
        var representationCount = rhinoRepresentationObjects.Count;
        return $"Imported Rhino Group ({representationCount} representation object{(representationCount == 1 ? string.Empty : "s")})";
    }

    private static string ResolveLayerPath(Rhino.FileIO.File3dm representation, Rhino.FileIO.File3dmObject representationObject)
    {
        var layerIndex = representationObject.Attributes?.LayerIndex ?? -1;
        if (layerIndex < 0 || layerIndex >= representation.Layers.Count)
            return string.Empty;

        var layer = representation.Layers[layerIndex];
        if (layer is null || layer.IsDeleted)
            return string.Empty;
        if (!string.IsNullOrWhiteSpace(layer.FullPath))
            return layer.FullPath;
        if (!string.IsNullOrWhiteSpace(layer.Name))
            return layer.Name;
        return string.Empty;
    }

    private static void AddGeometryToLayerTree(LayerGroupNode rootNode, string layerPath, IGH_GeometricGoo geometry)
    {
        var currentNode = rootNode;
        var layerParts = (layerPath ?? string.Empty)
            .Split(new[] { "::", "/" }, StringSplitOptions.RemoveEmptyEntries)
            .Select(layerPart => layerPart.Trim())
            .Where(layerPart => !string.IsNullOrWhiteSpace(layerPart));
        foreach (var layerPart in layerParts)
        {
            if (!currentNode.Children.TryGetValue(layerPart, out var childNode))
            {
                childNode = new LayerGroupNode(layerPart);
                currentNode.Children[layerPart] = childNode;
            }
            currentNode = childNode;
        }
        currentNode.Geometries.Add(geometry);
    }

    private static GH_GeometryGroup BuildGeometryGroup(LayerGroupNode node)
    {
        var group = new GH_GeometryGroup
        {
            Name = node.Name
        };

        foreach (var childNode in node.Children.Values.OrderBy(child => child.Name, StringComparer.OrdinalIgnoreCase))
            group.Objects.Add(BuildGeometryGroup(childNode));

        foreach (var geometry in node.Geometries)
            group.Objects.Add(geometry);

        return group;
    }
}

public class GroupToRepresentationObjectComponent : ScriptingComponent
{
    public GroupToRepresentationObjectComponent() : base("Group To Representation Object", "Gr→Mo", "Group To Representation Object extracts individual Rhino representation objects from a native Rhino/Grasshopper group.") { }
    public override Guid ComponentGuid => new("A2B1C3D4-E5F6-4789-B1C2-D3E4F5A6B7C8");
    protected override Bitmap Icon => Resources.group_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new Param_Group(), "Group", "Gr", "Group consumed by Group To Representation Object.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new Param_RepresentationObject(), "Rhino RepresentationObject", "Mo*", "Rhino representation objects produced by Group To Representation Object.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        GH_GeometryGroup group = null;
        if (!DA.GetData(0, ref group) || group is null)
            return;

        try
        {
            var representationObjectDatas = Utility.ExtractRhinoRepresentationObjectDataFromGeometryGroup(group).ToList();
            DA.SetDataList(0, representationObjectDatas);
        }
        catch (Exception exception)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, exception.Message);
        }
    }
}

public class ObjectsToTextComponent : ScriptingComponent
{
    public ObjectsToTextComponent() : base("Objects to Text", "Objs→Txt", "Converts a list of objects to a human-readable text.") { }
    public override Guid ComponentGuid => new("3BE61561-8290-4965-A9A6-38ACB4EC5182");
    protected override Bitmap Icon => Resources.objects_convert_text_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddGenericParameter("Objects", "Ob+", "Objects to humanize.", GH_ParamAccess.list);
    }
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Humanized Text", "Tx", "Human-readable text.", GH_ParamAccess.item);
    }
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var objects = new List<object>();
        DA.GetDataList(0, objects);
        var humanizedText = objects.Humanize();
        DA.SetData(0, humanizedText);
    }
}

public class NormalizeTextComponent : ScriptingComponent
{
    public NormalizeTextComponent() : base("Normalize Text", "⇒Txt", "Normalizes a text to different formats.") { }
    public override Guid ComponentGuid => new("1417BD04-7271-4EFD-A32C-99B1D2FC8A9E");
    protected override Bitmap Icon => Resources.text_normalize_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to normalize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Strict", "St", "Strictly alphanumerical text that either strips characters or turn them into underscores.", GH_ParamAccess.item);
        pManager.AddTextParameter("Title", "Ti", "Titelized text by capitalizing and unifying casing.", GH_ParamAccess.item);
        pManager.AddTextParameter("Underscore", "Un", "Underscorized text by lowercasing everything and replacing spaces with underscores.", GH_ParamAccess.item);
        pManager.AddTextParameter("Kebab", "Kb", "Kebaberized text by lowercasing everything and replacing spaces with dashes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Pascal", "Pa", "Pascalized text by capitalizing and removing spaces.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        var strict = Regex.Replace(text.Dehumanize().Underscore(), @"[^a-zA-Z0-9_]", "");
        var title = text.Titleize();
        var underscore = text.Underscore();
        var kebab = text.Kebaberize();
        var pascal = text.Pascalize();
        DA.SetData(0, strict);
        DA.SetData(1, title);
        DA.SetData(2, underscore);
        DA.SetData(3, kebab);
        DA.SetData(4, pascal);
    }
}

public class TruncateTextComponent : ScriptingComponent
{
    public TruncateTextComponent() : base("Truncate Text", "…Txt", "Truncates text by length and an optional termination.") { }
    public override Guid ComponentGuid => new("C15BFCE9-0EF7-4367-8310-EF47CE0B8013");
    protected override Bitmap Icon => Resources.text_truncate_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to truncate.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Length", "Le", "Maximum length of the text.", GH_ParamAccess.item);
        pManager.AddTextParameter("Termination", "Tr", "Optional termination to append to the truncated text.", GH_ParamAccess.item, "…");

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Strict", "St", "Fixed length truncated text including the truncation text length.", GH_ParamAccess.item);
        pManager.AddTextParameter("Characters", "Crs", "Fixed alphanumeric character length truncated text including the truncation text length", GH_ParamAccess.item);
        pManager.AddTextParameter("Words", "Wds", "Fixed word length truncated text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        string text = "";
        var length = 0;
        var termination = "…";
        DA.GetData(0, ref text);
        DA.GetData(1, ref length);
        DA.GetData(2, ref termination);
        var strict = text.Truncate(length, termination, Truncator.FixedLength);
        var characters = text.Truncate(length, termination, Truncator.FixedNumberOfCharacters);
        var words = text.Truncate(length, termination, Truncator.FixedNumberOfWords);
        DA.SetData(0, strict);
        DA.SetData(1, characters);
        DA.SetData(2, words);
    }
}

#endregion 📌Scripting

#region ⭐Engine
// Local kit persistence uses semio-store via Semio.Store.StoreKitIO (same as Semio).

public readonly struct Unit
{
}

public readonly struct PersistenceRequest<TInput>
{
    public string Directory { get; }
    public TInput Input { get; }

    public PersistenceRequest(string directory, TInput input)
    {
        Directory = directory;
        Input = input;
    }
}

public readonly struct UpdateKitInput
{
    public string Directory { get; }
    public KitDiff Diff { get; }

    public UpdateKitInput(string directory, KitDiff diff)
    {
        Directory = directory;
        Diff = diff;
    }
}

public readonly struct UpdateKitOutput
{
    public string Directory { get; }
    public Kit Kit { get; }

    public UpdateKitOutput(string directory, Kit kit)
    {
        Directory = directory;
        Kit = kit;
    }
}

public abstract class KitOperationComponent<TInput, TOutput> : Component
{
    protected KitOperationComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual string RunDescription => "True to start the operation.";
    protected virtual string SuccessDescription => "True if the operation was successful.";
    protected virtual void RegisterKitInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterKitInputParams(pManager);
        pManager.AddBooleanParameter("Run", "R", RunDescription, GH_ParamAccess.item, false);
    }
    protected virtual void RegisterKitOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", SuccessDescription, GH_ParamAccess.item);
        RegisterKitOutputParams(pManager);
    }
    protected virtual bool TryGetInput(IGH_DataAccess DA, out TInput input)
    {
        input = default!;
        return true;
    }
    protected abstract TOutput Run(TInput input);
    protected virtual void SetOutput(IGH_DataAccess DA, TOutput response) { }
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var run = false;
        DA.GetData(Params.Input.Count - 1, ref run);
        if (!run) return;
        if (!TryGetInput(DA, out var input)) return;
        try
        {
            var response = Run(input);
            SetOutput(DA, response);
            DA.SetData(0, true);
        }
        catch (Exception e)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, e.Message);
            DA.SetData(0, false);
        }
    }
}

#region ⛑️Persistence
// Load/save kits through semio-store (see Semio/Store/StoreKitIO in the net bundle).

public static class KitRuntimeState
{
    public static Kit? StaticKit { get; set; }
    public static string StaticKitDirectory { get; set; } = "";
}

public abstract class PersistenceComponent<TPersistentInput, TResponse> : KitOperationComponent<PersistenceRequest<TPersistentInput>, TResponse>
{
    protected PersistenceComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual void RegisterPersitenceInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterKitInputParams(GH_InputParamManager pManager)
    {
        RegisterPersitenceInputParams(pManager);
        var amountCustomParams = pManager.ParamCount;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path of the local kit.\n" +
            "If none is provided, it will use the directory of the current Grasshopper file.",
            GH_ParamAccess.item);
        pManager[amountCustomParams].Optional = true;
    }
    protected virtual void RegisterPersitenceOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterKitOutputParams(GH_OutputParamManager pManager)
    {
        RegisterPersitenceOutputParams(pManager);
    }
    protected virtual bool TryGetPersistentInput(IGH_DataAccess DA, out TPersistentInput input)
    {
        input = default!;
        return true;
    }

    protected string ResolveKitDirectory(IGH_DataAccess DA)
    {
        var directory = "";
        if (!DA.GetData(Params.Input.Count - 2, ref directory) || string.IsNullOrEmpty(directory))
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        return directory;
    }

    protected override bool TryGetInput(IGH_DataAccess DA, out PersistenceRequest<TPersistentInput> input)
    {
        if (!TryGetPersistentInput(DA, out var persistentInput))
        {
            input = default;
            return false;
        }
        var directory = ResolveKitDirectory(DA);
        input = new PersistenceRequest<TPersistentInput>(directory, persistentInput);
        return true;
    }
    protected abstract TResponse RunOnKit(string directory, TPersistentInput input);
    protected override TResponse Run(PersistenceRequest<TPersistentInput> input) => RunOnKit(input.Directory, input.Input);
}

public class LoadKitComponent : PersistenceComponent<Unit, Kit>
{
    public LoadKitComponent() : base("Load Kit", "/Kit", "Load a kit from a local directory.") { }
    protected override string RunDescription => "True to load the kit.";
    protected override string SuccessDescription => "True if the kit was successfully loaded. False otherwise.";
    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");
    protected override Bitmap Icon => Resources.kit_load_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterPersitenceOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Local Directory", "Di",
            "The local directory of the kit.",
            GH_ParamAccess.item);
    }

    protected override Kit RunOnKit(string directory, Unit input) => StoreKitIO.LoadKitFromFolder(directory);

    protected override void SetOutput(IGH_DataAccess DA, Kit response)
    {
        DA.SetData(1, new KitGoo(response));
        var directory = "";
        DA.GetData(Params.Input.Count - 2, ref directory);
        if (string.IsNullOrEmpty(directory))
        {
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        }
        DA.SetData(2, directory);
    }
}

public class SaveKitComponent : PersistenceComponent<Kit, Kit>
{
    public SaveKitComponent() : base("Save Kit", "Kit/", "Save a kit to a local directory.") { }
    protected override string RunDescription => "True to save the kit.";
    protected override string SuccessDescription => "True if the kit was successfully saved. False otherwise.";
    public override Guid ComponentGuid => new("A7E3B651-581E-4595-8DAC-132F10BD87FC");
    protected override Bitmap Icon => Resources.kit_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterPersitenceInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "Kt", "The kit to save.", GH_ParamAccess.item);
    }

    protected override bool TryGetPersistentInput(IGH_DataAccess DA, out Kit input)
    {
        KitGoo? kitGoo = null;
        if (!DA.GetData(0, ref kitGoo) || kitGoo is null)
        {
            input = default!;
            return false;
        }
        input = kitGoo.Value;
        return true;
    }

    protected override Kit RunOnKit(string directory, Kit input)
    {
        StoreKitIO.SaveKitToFolder(input, directory);
        return input;
    }
}

public class UpdateKitComponent : KitOperationComponent<UpdateKitInput, UpdateKitOutput>
{
    public UpdateKitComponent() : base("Update Kit", "Kit↻", "Apply a kit diff and persist the result to the local kit folder (semio-store).") { }
    protected override string RunDescription => "True to update the kit.";
    protected override string SuccessDescription => "True if the kit was successfully updated. False otherwise.";
    public override Guid ComponentGuid => new("B7104D9E-E2BD-4FBE-9D04-A4527B978AEE");
    protected override Bitmap Icon => Resources.kit_diff_24x24;
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override void RegisterKitInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitDiffParam(), "Kit Diff", "KtΔ", "The diff to apply.", GH_ParamAccess.item);
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path of the local kit.\n" +
            "If none is provided, it will use the directory of the current Grasshopper file.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
    }

    protected override void RegisterKitOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Local Directory", "Di", "The local directory of the kit.", GH_ParamAccess.item);
    }

    protected override bool TryGetInput(IGH_DataAccess DA, out UpdateKitInput input)
    {
        KitDiffGoo? diffGoo = null;
        if (!DA.GetData(0, ref diffGoo) || diffGoo is null)
        {
            input = default;
            return false;
        }
        var directory = "";
        if (!DA.GetData(1, ref directory) || string.IsNullOrWhiteSpace(directory))
        {
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        }
        input = new UpdateKitInput(directory, diffGoo.Value);
        return true;
    }

    protected override UpdateKitOutput Run(UpdateKitInput input)
    {
        var directory = input.Directory;
        var diff = input.Diff;

        var baseKit = KitRuntimeState.StaticKit is not null && KitRuntimeState.StaticKitDirectory == directory
            ? KitRuntimeState.StaticKit
            : StoreKitIO.LoadKitFromFolder(directory);
        var updatedKit = Kit.ApplyDiff(baseKit, diff);
        StoreKitIO.SaveKitToFolder(updatedKit, directory);
        KitRuntimeState.StaticKit = updatedKit;
        KitRuntimeState.StaticKitDirectory = directory;
        return new UpdateKitOutput(directory, updatedKit);
    }

    protected override void SetOutput(IGH_DataAccess DA, UpdateKitOutput response)
    {
        DA.SetData(1, new KitGoo(response.Kit));
        DA.SetData(2, response.Directory);
    }
}

#endregion ⛑️Persistence

#endregion ⭐Engine

public class FlattenDesignComponent : ScriptingComponent
{
    public FlattenDesignComponent() : base("Flatten Design", "Flat", "Flattens a design.") { }
    public override Guid ComponentGuid => new("4A6F1D2C-8B3E-49A1-B72F-1D9C8B5F6C3E");
    protected override Bitmap Icon => Resources.design_flatten_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "Id", "Design Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignChangeParam(), "DesignChange", "Dch", "Flattened design change report diff payload.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo kitGoo = null;
        if (!DA.GetData(0, ref kitGoo)) return;
        string designId = "";
        if (!DA.GetData(1, ref designId)) return;

        var result = Kit.FlattenDesign(kitGoo.Value, designId);
        if (!result.Ok || result.Diff is null)
        {
            foreach (var error in result.Errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error.Message);
            return;
        }

        foreach (var warning in result.Warnings)
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, warning.Message);

        foreach (var info in result.Infos)
            AddRuntimeMessage(GH_RuntimeMessageLevel.Remark, info.Message);

        DA.SetData(0, new DesignChangeGoo(result.Diff));
    }
}

/// <summary>📤Exports the 3D representation of a design to a format (.glb by default).</summary>
public class ExportDesignRepresentationComponent : ScriptingComponent
{
    public ExportDesignRepresentationComponent() : base("Export Design Representation", "ExpMdl", "Exports the 3D representation of a design to a format.") { }
    public override Guid ComponentGuid => new("B3D4E5F6-7A8B-49C1-A2D3-E4F5A6B7C8D9");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "Id", "Design ID", GH_ParamAccess.item);
        pManager.AddTextParameter("Format", "F", "Output format (.glb, .gltf, .obj, .stl, .3dm)", GH_ParamAccess.item, ".glb");
        pManager.AddTextParameter("Tags", "T", "Tags to filter representations", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddGenericParameter("RepresentationBytes", "B", "Exported representation as byte array", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo kitGoo = null;
        if (!DA.GetData(0, ref kitGoo)) return;
        string designId = "";
        if (!DA.GetData(1, ref designId)) return;
        string format = ".glb";
        DA.GetData(2, ref format);
        var tagsList = new List<string>();
        DA.GetDataList(3, tagsList);

        try
        {
            var result = Kit.ExportDesignRepresentation(kitGoo.Value, designId, format, tagsList.ToArray());
            DA.SetData(0, result);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class ReplaceClusterWithDesignComponent : ScriptingComponent
{
    public ReplaceClusterWithDesignComponent() : base("Replace Cluster", "RepCl", "Replaces a cluster with a design.") { }
    public override Guid ComponentGuid => new("7D8E2F9A-4B1C-46A3-9F1E-3C8B5A7D2C1E");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "OriginalDesign", "OD", "Original Design", GH_ParamAccess.item);
        pManager.AddTextParameter("ClusterPieceIds", "Ids", "Cluster Piece Ids", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "ClusteredDesign", "CD", "Clustered Design", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectionParam(), "ExternalConnections", "EC", "External Connections", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignDiffParam(), "DesignDiff", "Df", "Design Diff", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo originalDesignGoo = null;
        if (!DA.GetData(0, ref originalDesignGoo)) return;
        List<string> clusterPieceIds = new List<string>();
        if (!DA.GetDataList(1, clusterPieceIds)) return;
        DesignGoo clusteredDesignGoo = null;
        if (!DA.GetData(2, ref clusteredDesignGoo)) return;
        List<ConnectionGoo> externalConnectionsGoo = new List<ConnectionGoo>();
        if (!DA.GetDataList(3, externalConnectionsGoo)) return;

        var result = Kit.ReplaceClusterWithDesign(originalDesignGoo.Value, clusterPieceIds, clusteredDesignGoo.Value, externalConnectionsGoo.Select(c => c.Value).ToList());
        DA.SetData(0, new DesignDiffGoo(result));
    }
}

public class FindPieceInDesignComponent : ScriptingComponent
{
    public FindPieceInDesignComponent() : base("Find Piece In Design", "FPiD", "Finds a piece in a design by its ID.") { }
    public override Guid ComponentGuid => new("8E9D0F1C-2A3B-45A9-8F7E-6C5B4A3D2E1F");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "G", "Piece ID", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "P", "Found Piece", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo designGoo = null;
        if (!DA.GetData(0, ref designGoo)) return;
        string pieceId = "";
        if (!DA.GetData(1, ref pieceId)) return;

        var result = Piece.FindInDesign(designGoo.Value, pieceId);
        DA.SetData(0, new PieceGoo(result));
    }
}

public class FindReplacableTypesForPieceInDesignComponent : ScriptingComponent
{
    public FindReplacableTypesForPieceInDesignComponent() : base("Find Replacable Types (Piece)", "FRepTP", "Finds replacable types for a piece in a design.") { }
    public override Guid ComponentGuid => new("9A0B1C2D-3E4F-56A7-B8C9-D0E1F2A3B4C5");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "DG", "Design ID", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "PG", "Piece ID", GH_ParamAccess.item);
        pManager.AddTextParameter("Variants", "V", "Variants", GH_ParamAccess.list);

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Types", "T", "Replacable Types", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo kitGoo = null;
        if (!DA.GetData(0, ref kitGoo)) return;
        string designId = "";
        if (!DA.GetData(1, ref designId)) return;
        string pieceId = "";
        if (!DA.GetData(2, ref pieceId)) return;
        List<string> variants = new List<string>();
        DA.GetDataList(3, variants);

        var result = Kit.FindReplacableTypesForPieceInDesign(kitGoo.Value, designId, pieceId, variants.Count > 0 ? variants.ToArray() : null);
        DA.SetDataList(0, result.Select(r => new TypeGoo(r)));
    }
}


public class FindTagComponent : ScriptingComponent
{
    public FindTagComponent() : base("FindTag", "FTag", "Finds a tag by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234560");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TagParam(), "Tags", "T", "Tags", GH_ParamAccess.list);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TagParam(), "Tag", "T", "Tag", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<TagGoo> in0 = new List<TagGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Tag.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new TagGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConceptComponent : ScriptingComponent
{
    public FindConceptComponent() : base("FindConcept", "FConcept", "Finds a concept by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234561");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConceptParam(), "Concepts", "C", "Concepts", GH_ParamAccess.list);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConceptParam(), "Concept", "C", "Concept", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<ConceptGoo> in0 = new List<ConceptGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Concept.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new ConceptGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindRepresentationComponent : ScriptingComponent
{
    public FindRepresentationComponent() : base("FindRepresentation", "FRepresentation", "Finds a representation by tag IDs.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234562");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representations", "M", "Representations", GH_ParamAccess.list);
        pManager.AddTextParameter("TagIds", "G", "TagIds", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "M", "Representation", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<RepresentationGoo> in0 = new List<RepresentationGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        List<string> in1 = new List<string>();
        if (!DA.GetDataList(1, in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Representation.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new RepresentationGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectorComponent : ScriptingComponent
{
    public FindConnectorComponent() : base("FindConnector", "FConnector", "Finds a connector by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234563");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorParam(), "Connectors", "C", "Connectors", GH_ParamAccess.list);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorParam(), "Connector", "C", "Connector", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<ConnectorGoo> in0 = new List<ConnectorGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connector.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new ConnectorGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectorInTypeComponent : ScriptingComponent
{
    public FindConnectorInTypeComponent() : base("FindConnectorInType", "FCInType", "Finds a connector in a type by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234564");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorParam(), "Connector", "C", "Connector", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        TypeGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connector.FindInType(in0.Value, in1);
            DA.SetData(0, result != null ? new ConnectorGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindPieceComponent : ScriptingComponent
{
    public FindPieceComponent() : base("FindPiece", "FPiece", "Finds a piece by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234565");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Pieces", GH_ParamAccess.list);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "P", "Piece", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<PieceGoo> in0 = new List<PieceGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Piece.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new PieceGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectionComponent : ScriptingComponent
{
    public FindConnectionComponent() : base("FindConnection", "FConn", "Finds a connection by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234566");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<ConnectionGoo> in0 = new List<ConnectionGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connection.Find(in0.Select(x => x.Value).ToList(), in1);
            DA.SetData(0, result != null ? new ConnectionGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindPieceConnectionsComponent : ScriptingComponent
{
    public FindPieceConnectionsComponent() : base("FindPieceConnections", "FPConn", "Finds connections for a piece.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234567");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
        pManager.AddTextParameter("PieceId", "G", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        List<ConnectionGoo> in0 = new List<ConnectionGoo>();
        if (!DA.GetDataList(0, in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connection.FindByPiece(in0.Select(x => x.Value).ToList(), in1);
            DA.SetDataList(0, result?.Select(r => new ConnectionGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectorForPieceInConnectionComponent : ScriptingComponent
{
    public FindConnectorForPieceInConnectionComponent() : base("FindConnectorForPieceInConnection", "FCFPIC", "Finds a connector for a piece in a connection.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234568");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "G", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorParam(), "Connector", "C", "Connector", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        TypeGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        ConnectionGoo in1 = null;
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Connector.FindForPieceInConnection(in0.Value, in1.Value, in2);
            DA.SetData(0, result != null ? new ConnectorGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectionInDesignComponent : ScriptingComponent
{
    public FindConnectionInDesignComponent() : base("FindConnectionInDesign", "FCID", "Finds a connection in a design by ID.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234569");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connection.FindInDesign(in0.Value, in1);
            DA.SetData(0, result != null ? new ConnectionGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectionsInDesignComponent : ScriptingComponent
{
    public FindConnectionsInDesignComponent() : base("FindConnectionsInDesign", "FCsID", "Finds connections in a design by IDs.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456A");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("Ids", "G", "Ids", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        List<string> in1 = new List<string>();
        if (!DA.GetDataList(1, in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connection.FindManyInDesign(in0.Value, in1);
            DA.SetDataList(0, result?.Select(r => new ConnectionGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindPieceConnectionsInDesignComponent : ScriptingComponent
{
    public FindPieceConnectionsInDesignComponent() : base("FindPieceConnectionsInDesign", "FPCID", "Finds connections for a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456B");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "G", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Connection.FindByPieceInDesign(in0.Value, in1);
            DA.SetDataList(0, result?.Select(r => new ConnectionGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindStaleConnectionsInDesignComponent : ScriptingComponent
{
    public FindStaleConnectionsInDesignComponent() : base("FindStaleConnectionsInDesign", "FSCID", "Finds stale connections in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456C");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connections", "C", "Connections", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;

        try
        {
            var result = Connection.FindStaleInDesign(in0.Value);
            DA.SetDataList(0, result?.Select(r => new ConnectionGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindFileInKitComponent : ScriptingComponent
{
    public FindFileInKitComponent() : base("FindFileInKit", "FFIK", "Finds a file in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456D");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FileParam(), "File", "F", "File", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindFile(in0.Value, in1);
            DA.SetData(0, result != null ? new FileGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindTagInKitComponent : ScriptingComponent
{
    public FindTagInKitComponent() : base("FindTagInKit", "FTIK", "Finds a tag in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456E");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TagParam(), "Tag", "T", "Tag", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindTag(in0.Value, in1);
            DA.SetData(0, result != null ? new TagGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConceptInKitComponent : ScriptingComponent
{
    public FindConceptInKitComponent() : base("FindConceptInKit", "FCIK", "Finds a concept in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123456F");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConceptParam(), "Concept", "C", "Concept", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindConcept(in0.Value, in1);
            DA.SetData(0, result != null ? new ConceptGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindTypeInKitComponent : ScriptingComponent
{
    public FindTypeInKitComponent() : base("FindTypeInKit", "FTyIK", "Finds a type in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234570");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindType(in0.Value, in1);
            DA.SetData(0, result != null ? new TypeGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindDesignInKitComponent : ScriptingComponent
{
    public FindDesignInKitComponent() : base("FindDesignInKit", "FDIK", "Finds a design in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234571");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindDesign(in0.Value, in1);
            DA.SetData(0, result != null ? new DesignGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindPortInKitComponent : ScriptingComponent
{
    public FindPortInKitComponent() : base("FindPortInKit", "FPIK", "Finds a port in a kit.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234572");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "G", "Id", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "P", "Port", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;

        try
        {
            var result = Kit.FindPort(in0.Value, in1);
            DA.SetData(0, result != null ? new PortGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindPieceTypeInDesignComponent : ScriptingComponent
{
    public FindPieceTypeInDesignComponent() : base("FindPieceTypeInDesign", "FPTID", "Finds the type of a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234573");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "P", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.FindPieceTypeInDesign(in0.Value, in1, in2);
            DA.SetData(0, result != null ? new TypeGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindParentPieceInDesignComponent : ScriptingComponent
{
    public FindParentPieceInDesignComponent() : base("FindParentPieceInDesign", "FPPID", "Finds the parent piece of a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234574");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "P", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "P", "Piece", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.FindParentPieceInDesign(in0.Value, in1, in2);
            DA.SetData(0, result != null ? new PieceGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindParentConnectionForPieceInDesignComponent : ScriptingComponent
{
    public FindParentConnectionForPieceInDesignComponent() : base("FindParentConnectionForPieceInDesign", "FPCFPI", "Finds the parent connection for a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234575");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "P", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.FindParentConnectionForPieceInDesign(in0.Value, in1, in2);
            DA.SetData(0, result != null ? new ConnectionGoo(result) : null);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindChildrenPiecesInDesignComponent : ScriptingComponent
{
    public FindChildrenPiecesInDesignComponent() : base("FindChildrenPiecesInDesign", "FCPID", "Finds the children pieces of a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234576");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "P", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Pieces", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.FindChildrenPiecesInDesign(in0.Value, in1, in2);
            DA.SetDataList(0, result?.Select(r => new PieceGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindUsedConnectorsByPieceInDesignComponent : ScriptingComponent
{
    public FindUsedConnectorsByPieceInDesignComponent() : base("FindUsedConnectorsByPieceInDesign", "FUCBPID", "Finds used connectors by a piece in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234577");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceId", "P", "PieceId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectorParam(), "Connectors", "C", "Connectors", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.FindUsedConnectorsByPieceInDesign(in0.Value, in1, in2);
            DA.SetDataList(0, result?.Select(r => new ConnectorGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindReplacableTypesForPiecesInDesignComponent : ScriptingComponent
{
    public FindReplacableTypesForPiecesInDesignComponent() : base("FindReplacableTypesForPiecesInDesign", "FRepTPs", "Finds replacable types for pieces in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF01234578");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "DG", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("PieceIds", "PG", "PieceIds", GH_ParamAccess.list);
        pManager.AddTextParameter("Variants", "V", "Variants", GH_ParamAccess.list);

    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Types", "T", "Types", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        List<string> in2 = new List<string>();
        if (!DA.GetDataList(2, in2) && !Params.Input[2].Optional) return;
        List<string> in3 = new List<string>();
        if (!DA.GetDataList(3, in3) && !Params.Input[3].Optional) return;

        try
        {
            var result = Kit.FindReplacableTypesForPiecesInDesign(in0.Value, in1, in2.ToArray(), in3.Count > 0 ? in3.ToArray() : null);
            DA.SetDataList(0, result?.Select(r => new TypeGoo(r)));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class FindConnectionPiecesInDesignComponent : ScriptingComponent
{
    public FindConnectionPiecesInDesignComponent() : base("Find Connection Pieces In Design", "FCPsID", "Finds the connected and connecting pieces of a connection in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123457A");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Connecting", "Cg", "Connecting Piece", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Connected", "Cd", "Connected Piece", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        DesignGoo in0 = null;
        if (!DA.GetData(0, ref in0)) return;
        ConnectionGoo in1 = null;
        if (!DA.GetData(1, ref in1)) return;

        try
        {
            var result = Connection.FindPiecesInDesign(in0.Value, in1.Value);
            DA.SetData(0, new PieceGoo(result.connecting));
            DA.SetData(1, new PieceGoo(result.connected));
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

public class SumQualityInDesignComponent : ScriptingComponent
{
    public SumQualityInDesignComponent() : base("SumQualityInDesign", "SQID", "Sums the values of a quality across all pieces in a design.") { }
    public override Guid ComponentGuid => new("A1B2C3D4-1234-5678-90AB-CDEF0123457B");
    protected override Bitmap Icon => null;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "D", "DesignId", GH_ParamAccess.item);
        pManager.AddTextParameter("QualityId", "Q", "QualityId", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("Sum", "S", "Sum of quality values", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo in0 = null;
        if (!DA.GetData(0, ref in0) && !Params.Input[0].Optional) return;
        string in1 = "";
        if (!DA.GetData(1, ref in1) && !Params.Input[1].Optional) return;
        string in2 = "";
        if (!DA.GetData(2, ref in2) && !Params.Input[2].Optional) return;

        try
        {
            var result = Kit.SumQualityInDesign(in0.Value, in1, in2);
            DA.SetData(0, result);
        }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

#region ⏰ExportDesignToBlocks
// Exports a design to native Rhino block definitions and instances.
// Every type becomes a block definition, every piece becomes a block instance.

/// <summary>
/// 📤Exports a design to native Rhino block instances.
/// Every type becomes a block definition and every piece becomes a block instance.
/// Piece planes are computed via BFS over connections using connector geometry.
/// </summary>
public class ExportDesignToBlocksComponent : ScriptingComponent
{
    public ExportDesignToBlocksComponent() : base("Export Design To Blocks", "Des→Blk", "Exports a design to native Rhino block instances. Every type becomes a block definition and every piece becomes a block instance.") { }
    public override Guid ComponentGuid => new("C4D5E6F7-8A9B-4C1D-A2E3-F4A5B6C7D8EA");
    protected override Bitmap Icon => null;
    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit containing types, designs, and files.", GH_ParamAccess.item);
        pManager.AddTextParameter("DesignId", "Id", "Design ID to export.", GH_ParamAccess.item);
        pManager.AddTextParameter("Tags", "T", "Tags to filter representations per type.", GH_ParamAccess.list);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddGeometryParameter("BlockInstances", "BI*", "Native Rhino block instances for each piece.", GH_ParamAccess.list);
        pManager.AddTextParameter("PieceIds", "PG*", "Piece IDs corresponding to each block instance.", GH_ParamAccess.list);
        pManager.AddTextParameter("TypeNames", "TN*", "Type names corresponding to each block instance.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        KitGoo kitGoo = null;
        if (!DA.GetData(0, ref kitGoo) || kitGoo?.Value is null) return;
        string designId = "";
        if (!DA.GetData(1, ref designId)) return;
        var tagsList = new List<string>();
        DA.GetDataList(2, tagsList);

        try
        {
            var kit = kitGoo.Value;
            var design = Kit.FindDesign(kit, designId);
            var pieces = design.Pieces ?? new List<Piece>();
            var connections = design.Connections ?? new List<Connection>();
            var types = kit.Types ?? new List<Type>();

            if (pieces.Count == 0)
            {
                DA.SetDataList(0, new List<object>());
                DA.SetDataList(1, new List<string>());
                DA.SetDataList(2, new List<string>());
                return;
            }

            var typesDict = new Dictionary<string, Type>();
            foreach (var t in types) typesDict[t.Id] = t;
            var piecesDict = new Dictionary<string, Piece>();
            foreach (var p in pieces) piecesDict[p.Id] = p;

            #region 🖇️ExportDesignToBlocks_PlanePropagation
            // Build adjacency for connection-based plane propagation
            var adjacency = new Dictionary<string, List<(Connection connection, string neighborId)>>();
            foreach (var p in pieces) adjacency[p.Id] = new List<(Connection, string)>();
            foreach (var conn in connections)
            {
                var connectedId = conn.Parent.Piece.Id;
                var connectingId = conn.Child.Piece.Id;
                if (adjacency.ContainsKey(connectedId))
                    adjacency[connectedId].Add((conn, connectingId));
                if (adjacency.ContainsKey(connectingId))
                    adjacency[connectingId].Add((conn, connectedId));
            }

            var piecePlanes = new Dictionary<string, Semio.Plane>();
            var visited = new HashSet<string>();
            var queue = new Queue<string>();

            Type GetTypeLocal(string typeId) => typesDict.TryGetValue(typeId, out var t) ? t : null;
            Connector GetConnectorLocal(Type type, string connectorId)
            {
                if (type == null) return null;
                if (string.IsNullOrEmpty(connectorId))
                    return type.Connectors?.Count > 0 ? type.Connectors[0] : null;
                return type.Connectors?.FirstOrDefault(c => c.Id == connectorId);
            }

            var identityPlane = new Semio.Plane
            {
                Origin = new Semio.Point { X = 0, Y = 0, Z = 0 },
                XAxis = new Semio.Vector { X = 1, Y = 0, Z = 0 },
                YAxis = new Semio.Vector { X = 0, Y = 1, Z = 0 }
            };

            foreach (var p in pieces)
            {
                if (p.Plane != null && p.Center != null)
                {
                    piecePlanes[p.Id] = p.Plane;
                    visited.Add(p.Id);
                    queue.Enqueue(p.Id);
                }
            }

            if (queue.Count == 0 && pieces.Count > 0)
            {
                piecePlanes[pieces[0].Id] = identityPlane;
                visited.Add(pieces[0].Id);
                queue.Enqueue(pieces[0].Id);
            }

            while (queue.Count > 0)
            {
                var currentId = queue.Dequeue();
                var currentPlane = piecePlanes[currentId];
                if (!adjacency.TryGetValue(currentId, out var edges)) continue;
                foreach (var edge in edges)
                {
                    if (visited.Contains(edge.neighborId)) continue;
                    var conn = edge.connection;
                    var isParent = conn.Parent.Piece.Id == currentId;
                    if (!isParent) continue;

                    var childId = edge.neighborId;
                    var parentPiece = piecesDict[currentId];
                    var childPiece = piecesDict[childId];
                    var parentType = parentPiece.Type != null ? GetTypeLocal(parentPiece.Type.Id) : null;
                    var childType = childPiece.Type != null ? GetTypeLocal(childPiece.Type.Id) : null;
                    var parentConnector = GetConnectorLocal(parentType, conn.Parent.Connector?.Id);
                    var childConnector = GetConnectorLocal(childType, conn.Child.Connector?.Id);

                    if (parentConnector != null && childConnector != null &&
                        parentConnector.Point != null && parentConnector.Direction != null &&
                        childConnector.Point != null && childConnector.Direction != null)
                    {
                        piecePlanes[childId] = Utility.ComputeChildPlane(
                            currentPlane, parentConnector.Point, parentConnector.Direction,
                            childConnector.Point, childConnector.Direction,
                            conn.Gap, conn.Shift, conn.Rise,
                            conn.Rotation, conn.Turn, conn.Tilt);
                    }
                    else
                    {
                        piecePlanes[childId] = currentPlane;
                    }

                    visited.Add(childId);
                    queue.Enqueue(childId);
                }
            }

            foreach (var p in pieces)
            {
                if (!visited.Contains(p.Id))
                    piecePlanes[p.Id] = identityPlane;
            }
            #endregion 🖇️ExportDesignToBlocks_PlanePropagation

            #region ⏰ExportDesignToBlocks_BlockDefinitions
            // Create block definitions per type in the active Rhino document
            var doc = RhinoDoc.ActiveDoc;
            if (doc == null)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "No active Rhino document.");
                return;
            }

            var typeBlockIndices = new Dictionary<string, int>();
            var tags = tagsList.ToArray();

            foreach (var piece in pieces)
            {
                var typeId = piece.Type?.Id;
                if (string.IsNullOrEmpty(typeId) || typeBlockIndices.ContainsKey(typeId)) continue;
                if (!typesDict.TryGetValue(typeId, out var type)) continue;

                var blockName = $"semio::{type.Name}::{type.Id}";
                var existingDef = doc.InstanceDefinitions.Find(blockName);
                if (existingDef != null && !existingDef.IsDeleted)
                {
                    typeBlockIndices[typeId] = existingDef.Index;
                    continue;
                }

                var geometries = new List<GeometryBase>();
                var objAttributes = new List<Rhino.DocObjects.ObjectAttributes>();

                var representation = Kit.ExportFindMatchingRepresentation(kit, type, tags);
                if (representation != null)
                {
                    var file = kit.Files?.FirstOrDefault(f => f.Id == representation.File.Id);
                    if (file?.Blob != null)
                    {
                        try
                        {
                            var rhinoContext = Utility.ImportRhinoRepresentationContextFromBlob(file.Blob, file.Name);
                            var sourceObjects = rhinoContext.Representation.Objects
                                .Where(o => o?.Geometry != null)
                                .ToList();
                            foreach (var sourceObj in sourceObjects)
                            {
                                geometries.Add(sourceObj.Geometry.Duplicate());
                                var attr = new Rhino.DocObjects.ObjectAttributes();
                                if (sourceObj.Attributes != null)
                                {
                                    attr.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromObject;
                                    attr.ObjectColor = sourceObj.Attributes.ObjectColor;
                                }
                                objAttributes.Add(attr);
                            }
                        }
                        catch
                        {
                            // Fallback: empty block definition with placeholder box
                        }
                    }
                }

                if (geometries.Count == 0)
                {
                    var box = new Box(Rhino.Geometry.Plane.WorldXY, new Interval(-0.5, 0.5), new Interval(-0.5, 0.5), new Interval(0, 1.0));
                    geometries.Add(box.ToBrep());
                    objAttributes.Add(new Rhino.DocObjects.ObjectAttributes());
                }

                var blockIdx = doc.InstanceDefinitions.Add(blockName, type.Description ?? type.Name, Point3d.Origin, geometries, objAttributes);
                if (blockIdx < 0)
                {
                    AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, $"Failed to create block definition for type '{type.Name}'.");
                    continue;
                }
                typeBlockIndices[typeId] = blockIdx;
            }
            #endregion ⏰ExportDesignToBlocks_BlockDefinitions

            #region 🎯ExportDesignToBlocks_BlockInstances
            // Create block instances per piece
            var blockInstances = new List<IGH_GeometricGoo>();
            var pieceIds = new List<string>();
            var typeNamesList = new List<string>();

            foreach (var piece in pieces)
            {
                var typeId = piece.Type?.Id;
                if (string.IsNullOrEmpty(typeId) || !typeBlockIndices.TryGetValue(typeId, out var blockIdx)) continue;
                if (!piecePlanes.TryGetValue(piece.Id, out var semioPlane)) continue;

                var rhinoPlane = RhinoConverter.Convert(semioPlane);

                var scale = piece.Scale ?? 1.0f;
                var xform = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, rhinoPlane);
                if (Math.Abs(scale - 1.0f) > 1e-6)
                    xform = xform * Transform.Scale(Point3d.Origin, scale);

                var idef = doc.InstanceDefinitions[blockIdx];
                var instanceRef = new InstanceReferenceGeometry(idef.Id, xform);

                blockInstances.Add(GH_Convert.ToGeometricGoo(instanceRef));
                pieceIds.Add(piece.Id);
                typeNamesList.Add(typesDict.TryGetValue(typeId, out var tp) ? tp.Name : typeId);
            }
            #endregion 🎯ExportDesignToBlocks_BlockInstances

            DA.SetDataList(0, blockInstances);
            DA.SetDataList(1, pieceIds);
            DA.SetDataList(2, typeNamesList);
        }
        catch (Exception ex)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message);
        }
    }
}

#endregion ⏰ExportDesignToBlocks

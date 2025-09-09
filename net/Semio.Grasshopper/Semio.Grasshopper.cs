#region Header

//Semio.Grasshopper.cs
//2020-2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Affero General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Affero General Public License for more details.

//You should have received a copy of the GNU Affero General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion Header

#region TODOs

// TODO: Think of modeling components that are resilient to future schema changes.
// TODO: Refactor EngineComponent with GetInput and GetPersitanceInput etc. Very confusing. Probably no abstracting is better.
// TODO: GetProps and SetProps (includes children) is not consistent with the prop naming in python (does not include children).
// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();
// TODO: IsInvalid is used to check null state which is not clean.
// Think of a better way to handle this.
// The invalid check happen twice and code is duplicated.
// TODO: Figure out why cast from Piece to Text is not triggering the casts. ToString has somehow has precedence.
// TODO: NameM.ToLower() doesn't work for composite names. E.g. "DiagramPoint" -> "diagrampoint".
// TODO: Implement a status check and wait for the engine to be ready

#endregion

using System;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Net.Http;
using GH_IO.Serialization;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Humanizer;
using Rhino;
using Rhino.Geometry;
using Semio;
using System.Text.RegularExpressions;

namespace Semio.Grasshopper;

#region Constants

public static class Constants
{
    public const string Category = Semio.Constants.Name;
    public const string Version = "6.0.0";
}

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => Semio.Constants.Name;
    public override Bitmap Icon => Resources.semio_24x24;
    public override Bitmap AssemblyIcon => Resources.semio_24x24;
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
        Instances.ComponentServer.AddCategoryIcon("semio", Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion Constants

#region Utility

public static class Utility
{
    public static string? GetGrasshopperLibraryDirectory()
    {
        try
        {
            return Path.GetDirectoryName(typeof(Utility).Assembly.Location);
        }
        catch
        {
            return Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
        }
    }

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
        Point childPoint, Vector childDirection, float gap, float shift, float raise, float rotation, float turn,
        float tilt)
    {
        var parentPointR = new Vector3d(parentPoint.Convert());
        var parentDirectionR = parentDirection.Convert();
        var revertedChildPointR = new Vector3d(childPoint.Convert());
        revertedChildPointR.Reverse();
        //var gapDirectionR = new Vector3d(parentDirectionR);
        var reverseChildDirectionR = childDirection.Convert();
        reverseChildDirectionR.Reverse();
        var rotationRad = RhinoMath.ToRadians(rotation);
        var turnRad = RhinoMath.ToRadians(turn);
        var tiltRad = RhinoMath.ToRadians(tilt);

        // orient

        // If directions are same, then there are infinite solutions.
        var areDirectionsSame = parentDirectionR.IsParallelTo(childDirection.Convert(), Semio.Constants.Tolerance) == 1;

        // Rhino tends to pick the "wrong" direction when the vectors are parallel.
        // E.g when z=0 it picks to flip the object around the y-axis instead of a rotation around the z-axis.
        Transform directionT;
        if (areDirectionsSame)
        {
            // Idea taken from: // https://github.com/dfki-ric/pytransform3d/blob/143943b028fc776adfc6939b1d7c2c6edeaa2d90/pytransform3d/rotations/_utils.py#L253
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
        //gapDirection.Transform(rotateT);

        var turnT = Transform.Rotation(turnRad, turnAxis, new Point3d());
        orientationT = turnT * orientationT;
        //gapDirection.Transform(turnT);

        var tiltT = Transform.Rotation(tiltRad, tiltAxis, new Point3d());
        orientationT = tiltT * orientationT;
        //gapDirection.Transform(tiltT);

        // move

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

        // to parent

        var parentPlaneR = parentPlane.Convert();
        var parentPlaneT = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, parentPlaneR);
        childPlaneR.Transform(parentPlaneT);

        return childPlaneR.Convert();
    }
}

#endregion Utility

#region Converters

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
}

#endregion Converters

#region Bases

public abstract class ModelGoo<TModel> : GH_Goo<TModel> where TModel : Model<TModel>, new()
{
    public ModelGoo() { Value = new TModel(); }
    public ModelGoo(TModel value) { Value = value; }
    public override bool IsValid => Value is not null;
    public override string TypeName => typeof(TModel).Name;
    public override string TypeDescription => typeof(TModel).Name;

    public override IGH_Goo Duplicate()
    {
        var duplicate = CreateDuplicate();
        if (Value is not null)
            duplicate.Value = Value.DeepClone();
        return duplicate;
    }

    protected abstract ModelGoo<TModel> CreateDuplicate();

    public override string ToString() => Value?.ToString() ?? string.Empty;
    public override bool Write(GH_IWriter writer)
    {
        if (Value is not null)
            writer.SetString("model", Value.Serialize());
        return base.Write(writer);
    }
    public override bool Read(GH_IReader reader)
    {
        var serialized = reader.GetString("model");
        if (!string.IsNullOrEmpty(serialized))
        {
            var deserialized = serialized.Deserialize<TModel>();
            if (deserialized is not null)
                Value = deserialized;
        }
        return base.Read(reader);
    }
    protected virtual bool CustomCastTo<Q>(ref Q target) => false;
    protected virtual bool CustomCastFrom(object source) => false;
    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TModel)) && Value is not null)
        {
            target = (Q)(object)Value;
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;
        if (source is TModel model)
        {
            Value = model;
            return true;
        }
        return CustomCastFrom(source);
    }
}

public abstract class ModelParam<TGoo, TModel> : GH_PersistentParam<TGoo> where TGoo : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    protected ModelParam(string name, string nickname, string description) : base(name, nickname, description, Constants.Category, "Params") { }
    protected override Bitmap Icon => GetParamIcon();
    protected abstract Bitmap GetParamIcon();
    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => GH_GetterResult.cancel;
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => GH_GetterResult.cancel;
}

public abstract class EnumGoo<TEnum> : GH_Goo<TEnum> where TEnum : struct, Enum
{
    public EnumGoo() { }
    public EnumGoo(TEnum value) => Value = value;
    public override bool IsValid => true;
    public override IGH_Goo Duplicate() => CreateDuplicate();
    protected abstract EnumGoo<TEnum> CreateDuplicate();
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

public abstract class EnumParam<TEnumGoo, TEnum> : GH_Param<TEnumGoo>
    where TEnumGoo : EnumGoo<TEnum>, new()
    where TEnum : struct, Enum
{
    protected EnumParam(string name, string nickname, string description, Guid guid) : base(name, nickname, description, "Semio", "Param", GH_ParamAccess.item)
    {
        ComponentGuid = guid;
    }
    public override Guid ComponentGuid { get; }
}
public abstract class Component : GH_Component
{
    public Component(string name, string nickname, string description, string subcategory) : base(
        name, nickname, description, Constants.Category, subcategory)
    { }
}

public abstract class ModelComponent<TParam, TGoo, TModel> : Component
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected ModelComponent(string name, string nickname, string description) : base(name, nickname, description, "Modeling") { }

    protected override Bitmap Icon => GetComponentIcon();
    public override GH_Exposure Exposure => GH_Exposure.primary;
    public override Guid ComponentGuid => GetComponentGuid();

    protected abstract Bitmap GetComponentIcon();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelName(), GetModelCode() + "?",
            $"The optional {GetModelName().ToLower()} to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?",
            $"Whether the {GetModelName().ToLower()} should be validated.", GH_ParamAccess.item);
        RegisterModelInputs(pManager);
        for (var i = 0; i < pManager.ParamCount; i++)
            pManager[i].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelName(), GetModelCode(),
            $"The constructed or modified {GetModelName().ToLower()}.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?",
            $"True if the {GetModelName().ToLower()} is valid. Null if no validation was performed.", GH_ParamAccess.item);
        RegisterModelOutputs(pManager);
    }

    protected abstract void RegisterModelInputs(GH_InputParamManager pManager);
    protected abstract void RegisterModelOutputs(GH_OutputParamManager pManager);
    protected abstract string GetModelCode();
    protected abstract string GetModelName();

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        TGoo? modelGoo = null;
        var validate = false;

        if (DA.GetData(0, ref modelGoo))
            modelGoo = (TGoo?)modelGoo?.Duplicate();
        DA.GetData(1, ref validate);

        var model = modelGoo?.Value ?? new TModel();
        ProcessModelInputs(DA, model);

        model = ProcessModel(model);

        if (validate)
        {
            var (isValid, errors) = model.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        var outputGoo = new TGoo { Value = model };
        DA.SetData(0, outputGoo);
        ProcessModelOutputs(DA, model);
    }

    protected abstract void ProcessModelInputs(IGH_DataAccess DA, TModel model);
    protected abstract void ProcessModelOutputs(IGH_DataAccess DA, TModel model);
    protected virtual TModel ProcessModel(TModel model) => model;
}

public abstract class IdGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public IdGoo() : base() { }
    public IdGoo(TModel value) : base(value) { }
}

public abstract class IdParam<TGoo, TModel> : ModelParam<TGoo, TModel> where TGoo : IdGoo<TModel> where TModel : Model<TModel>, new()
{
    protected IdParam(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class IdComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected IdComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) { }
    protected override void ProcessModelInputs(IGH_DataAccess DA, TModel model) { }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, TModel model) { }
    protected override string GetModelName() => GetModelCode().Replace("Id", "").Replace("ID", "") + "Id";
}

public abstract class DiffGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public DiffGoo() : base() { }
    public DiffGoo(TModel value) : base(value) { }
}

public abstract class DiffParam<TGoo, TModel> : ModelParam<TGoo, TModel> where TGoo : DiffGoo<TModel> where TModel : Model<TModel>, new()
{
    protected DiffParam(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

public abstract class DiffComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DiffComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) { }
    protected override void ProcessModelInputs(IGH_DataAccess DA, TModel model) { }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, TModel model) { }
    protected override string GetModelName() => GetModelCode().Replace("Diff", "").Replace("sDiff", "s") + "Diff";
}
public abstract class SerializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected SerializeComponent(string name, string nickname, string description) : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.secondary;
    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelTypeName(), GetModelNickname(), GetModelDescription(), GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "T", "Serialized text", GH_ParamAccess.item);
    }

    protected abstract string GetModelTypeName();
    protected abstract string GetModelNickname();
    protected abstract string GetModelDescription();

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var goo = default(TGoo);
        if (!DA.GetData(0, ref goo) || goo?.Value is null) return;

        var serialized = goo.Value.Serialize();
        DA.SetData(0, serialized);
    }
}

public abstract class DeserializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DeserializeComponent(string name, string nickname, string description) : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "T", "Serialized text", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelTypeName(), GetModelNickname(), GetModelDescription(), GH_ParamAccess.item);
    }

    protected abstract string GetModelTypeName();
    protected abstract string GetModelNickname();
    protected abstract string GetModelDescription();

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        if (!DA.GetData(0, ref text) || string.IsNullOrEmpty(text)) return;

        var model = text.Deserialize<TModel>();
        if (model is not null)
        {
            var goo = new TGoo();
            goo.Value = model;
            DA.SetData(0, goo);
        }
    }
}

public abstract class SerializeDiffComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected SerializeDiffComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => Resources.semio_24x24;
}

public abstract class DeserializeDiffComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DeserializeDiffComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => Resources.semio_24x24;
}

public abstract class SerializeIdComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected SerializeIdComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => Resources.semio_24x24;
}

public abstract class DeserializeIdComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DeserializeIdComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => Resources.semio_24x24;
}

public abstract class EntityGoo<TEntity, TEntityDiff, TEntityId> : ModelGoo<TEntity>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityGoo() : base() { }
    public EntityGoo(TEntity value) : base(value) { }
}

public abstract class EntityParam<TGoo, TEntity, TEntityDiff, TEntityId> : ModelParam<TGoo, TEntity>
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityParam(string name, string nickname, string description) : base(name, nickname, description) { }
}

public abstract class EntityComponent<TParam, TGoo, TEntity, TEntityDiff, TEntityId> : ModelComponent<TParam, TGoo, TEntity>
    where TParam : EntityParam<TGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityComponent(string name, string nickname, string description) : base(name, nickname, description) { }
}

public abstract class EntityIdGoo<TEntity, TEntityDiff, TEntityId> : IdGoo<TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityIdGoo() : base() { }
    public EntityIdGoo(TEntityId value) : base(value) { }
}

public abstract class EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId> : IdParam<TIdGoo, TEntityId>
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityIdParam(string name, string nickname, string description) : base(name, nickname, description) { }
}

public abstract class EntityIdComponent<TIdParam, TIdGoo, TEntity, TEntityDiff, TEntityId> : IdComponent<TIdParam, TIdGoo, TEntityId>
    where TIdParam : EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityIdComponent(string name, string nickname, string description) : base(name, nickname, description) { }
}

public abstract class EntityDiffGoo<TEntity, TEntityDiff, TEntityId> : DiffGoo<TEntityDiff>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityDiffGoo() : base() { }
    public EntityDiffGoo(TEntityDiff value) : base(value) { }
}

public abstract class EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffParam<TDiffGoo, TEntityDiff>
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityDiffParam(string name, string nickname, string description) : base(name, nickname, description) { }
}

public abstract class EntityDiffComponent<TDiffParam, TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffComponent<TDiffParam, TDiffGoo, TEntityDiff>
    where TDiffParam : EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityDiffComponent(string name, string nickname, string description) : base(name, nickname, description) { }
}

#endregion Bases

#region Attribute

public class AttributeIdGoo : IdGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }

    protected override ModelGoo<AttributeId> CreateDuplicate() => new AttributeIdGoo();

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new AttributeId { Key = str };
            return true;
        }
        return false;
    }
}

public class AttributeIdParam : IdParam<AttributeIdGoo, AttributeId>
{
    public AttributeIdParam() : base("AttributeId", "AI", "AttributeId parameter") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B93");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeIdComponent : IdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public AttributeIdComponent() : base("The ID of the attribute.", "AtI", "AttributeId component") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B92");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "AI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeIdParam(), "AtI", "AI", "The ID of the attribute.", GH_ParamAccess.item);
    }
}

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }

    protected override ModelGoo<AttributeDiff> CreateDuplicate() => new AttributeDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<AttributeDiff>() ?? new AttributeDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffParam() : base("AttributeDiff", "AD", "AttributeDiff parameter") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffComponent() : base("A diff for attributes.", "ADf", "AttributeDiff component") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B96");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "AD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeDiffParam(), "ADf", "AD", "A diff for attributes.", GH_ParamAccess.item);
    }
}

public class SerializeAttributeDiffComponent : SerializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public SerializeAttributeDiffComponent() : base("Serialize AttributeDiff", "SAD", "Serialize AttributeDiff to JSON") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B97");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelDescription() => "Serialize AttributeDiff to JSON";
    protected override string GetModelNickname() => "SAD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAttributeDiffComponent : DeserializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public DeserializeAttributeDiffComponent() : base("Deserialize AttributeDiff", "DAD", "Deserialize JSON to AttributeDiff") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B98");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelDescription() => "Deserialize JSON to AttributeDiff";
    protected override string GetModelNickname() => "DAD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class AttributeGoo : ModelGoo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }

    
    
    
    protected override ModelGoo<Attribute> CreateDuplicate() => new AttributeGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
public class AttributeParam : ModelParam<AttributeGoo, Attribute>
{
    public AttributeParam() : base("Atr", "At", "A attribute is a key value pair with an an optional definition.") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeComponent : ModelComponent<AttributeParam, AttributeGoo, Attribute>
{
    public AttributeComponent() : base("Attribute", "Atr", "Construct, deconstruct or modify an attribute") { }
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "At";
    protected override string GetModelName() => "Atr";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "The key of the attribute", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "V", "The value of the attribute", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "D", "The definition of the attribute", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "The key of the attribute", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "V", "The value of the attribute", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "D", "The definition of the attribute", GH_ParamAccess.item);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Attribute model)
    {
        string? key = null, value = null, definition = null;
        if (DA.GetData(2, ref key) && key != null) model.Key = key;
        if (DA.GetData(3, ref value) && value != null) model.Value = value;
        if (DA.GetData(4, ref definition) && definition != null) model.Definition = definition;
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Attribute model)
    {
        DA.SetData(2, model.Key);
        DA.SetData(3, model.Value);
        DA.SetData(4, model.Definition);
    }
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public SerializeAttributeComponent() : base("Serialize Attribute", ">>A", "Serialize Attribute to JSON") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelDescription() => "Serialize Attribute to JSON";
    protected override string GetModelNickname() => "SA";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public DeserializeAttributeComponent() : base("Deserialize Attribute", "<<A", "Deserialize JSON to Attribute") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8975-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelDescription() => "Deserialize JSON to Attribute";
    protected override string GetModelNickname() => "DA";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion

#region Representation

public class RepresentationIdGoo : IdGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }

    
    
    
    protected override ModelGoo<RepresentationId> CreateDuplicate() => new RepresentationIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is RepresentationDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is RepresentationGoo reprGoo)
        {
            Value = reprGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new RepresentationId { Tags = new List<string> { str } };
            return true;
        }
        return false;
    }
}

public class RepresentationIdParam : IdParam<RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdParam() : base("Rep", "Rp", "The identifier of a representation.") { }
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationIdComponent : IdComponent<RepresentationIdParam, RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdComponent() : base("The identifier of a representation.", "Rep", "RepresentationId component") { }
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "RI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationIdParam(), "Rep", "Rp", "The identifier of a representation.", GH_ParamAccess.item);
    }
}

public class RepresentationDiffGoo : DiffGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<RepresentationDiff> CreateDuplicate() => new RepresentationDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(string.Join(",", Value.Tags));
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<RepresentationDiff>() ?? new RepresentationDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationDiffParam : DiffParam<RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffParam() : base("RDf", "RD", "A diff for representations.") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationDiffComponent : DiffComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffComponent() : base("A diff for representations.", "RDf", "RepresentationDiff component") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AA");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "RD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationDiffParam(), "RDf", "RD", "A diff for representations.", GH_ParamAccess.item);
    }
}

public class SerializeRepresentationDiffComponent : SerializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public SerializeRepresentationDiffComponent() : base("Serialize RepresentationDiff", "SRD", "Serialize RepresentationDiff to JSON") { }
    public override Guid ComponentGuid => new("71E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override string GetModelDescription() => "Serialize RepresentationDiff to JSON";
    protected override string GetModelNickname() => "SRD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationDiffComponent : DeserializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public DeserializeRepresentationDiffComponent() : base("Deserialize RepresentationDiff", "DRD", "Deserialize JSON to RepresentationDiff") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AC");
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override string GetModelDescription() => "Deserialize JSON to RepresentationDiff";
    protected override string GetModelNickname() => "DRD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class RepresentationsDiffGoo : DiffGoo<RepresentationsDiff>
{
    public RepresentationsDiffGoo() { }
    public RepresentationsDiffGoo(RepresentationsDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<RepresentationsDiff> CreateDuplicate() => new RepresentationsDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("RepresentationsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<RepresentationsDiff>() ?? new RepresentationsDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationsDiffParam : DiffParam<RepresentationsDiffGoo, RepresentationsDiff>
{
    public RepresentationsDiffParam() : base("RepresentationsDiff", "RsD", "RepresentationsDiff parameter") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationsDiffComponent : DiffComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public RepresentationsDiffComponent() : base("RepresentationsDiff", "RsD", "RepresentationsDiff component") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AD");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "RsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationsDiffParam(), "RsDf", "RsD", "A diff for multiple representations.", GH_ParamAccess.item);
    }
}

public class SerializeRepresentationsDiffComponent : SerializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public SerializeRepresentationsDiffComponent() : base("Serialize RepresentationsDiff", "SRsD", "Serialize RepresentationsDiff to JSON") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AE");
    protected override string GetModelTypeName() => "RepresentationsDiff";
    protected override string GetModelDescription() => "Serialize RepresentationsDiff to JSON";
    protected override string GetModelNickname() => "SRsD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationsDiffComponent : DeserializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public DeserializeRepresentationsDiffComponent() : base("Deserialize RepresentationsDiff", "DRsD", "Deserialize JSON to RepresentationsDiff") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AF");
    protected override string GetModelTypeName() => "RepresentationsDiff";
    protected override string GetModelDescription() => "Deserialize JSON to RepresentationsDiff";
    protected override string GetModelNickname() => "DRsD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class RepresentationGoo : ModelGoo<Representation>
{
    public RepresentationGoo() { }
    public RepresentationGoo(Representation value) : base(value) { }

    
    
    
    protected override ModelGoo<Representation> CreateDuplicate() => new RepresentationGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Representation { Tags = new List<string> { str } };
            return true;
        }
        return false;
    }
}

public class RepresentationParam : ModelParam<RepresentationGoo, Representation>
{
    public RepresentationParam() : base("Rep", "Rp", "A representation is a link to a resource that describes a type for a certain level of detail and tags.") { }
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationComponent : ModelComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public RepresentationComponent() : base("Representation", "Rep", "Construct, deconstruct or modify a representation") { }
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "R";
    protected override string GetModelName() => "Representation";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "Ur", "The Unique Resource Locator (URL) to the resource of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Tags*", "Tg*", "The optional tags to group representations. No tags means default.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the representation.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "Ur", "The Unique Resource Locator (URL) to the resource of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Tags*", "Tg*", "The optional tags to group representations. No tags means default.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the representation.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Representation model)
    {
        string? url = null, description = null;
        var tags = new List<string>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref url) && url != null) model.Url = url;
        if (DA.GetData(3, ref description) && description != null) model.Description = description;
        if (DA.GetDataList(4, tags)) model.Tags = tags;
        if (DA.GetDataList(5, attributes)) model.Attributes = attributes.ConvertAll(a => a.Value);
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Representation model)
    {
        DA.SetData(2, model.Url);
        DA.SetData(3, model.Description);
        DA.SetDataList(4, model.Tags);
        DA.SetDataList(5, model.Attributes.ConvertAll(a => new AttributeGoo(a)));
    }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public SerializeRepresentationComponent() : base("Serialize Representation", "SR", "Serialize Representation to JSON") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");

    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rep";
    protected override string GetModelDescription() => "Serialize Representation";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public DeserializeRepresentationComponent() : base("Deserialize Representation", "DR", "Deserialize JSON to Representation") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");

    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rep";
    protected override string GetModelDescription() => "Deserialize Representation";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Representation

#region File

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }

    
    
    
    protected override ModelGoo<FileId> CreateDuplicate() => new FileIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FileId { Url = str };
            return true;
        }
        return false;
    }
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    public FileIdParam() : base("Fil", "Fl", "The identifier of a file.") { }
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FileIdComponent : IdComponent<FileIdParam, FileIdGoo, FileId>
{
    public FileIdComponent() : base("The identifier of a file.", "Fil", "FileId component") { }
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E8");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "FI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FileIdParam(), "Fil", "Fl", "The identifier of a file.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FileIdParam(), "Fil", "Fl", "The identifier of a file.", GH_ParamAccess.item);
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<FileDiff> CreateDuplicate() => new FileDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<FileDiff>() ?? new FileDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    public FileDiffParam() : base("FDf", "FD", "A diff for files.") { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public FileDiffComponent() : base("A diff for files.", "FDf", "FileDiff component") { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "FD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FileDiffParam(), "FDf", "FD", "A diff for files.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FileDiffParam(), "FDf", "FD", "A diff for files.", GH_ParamAccess.item);
}

public class SerializeFileDiffComponent : SerializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public SerializeFileDiffComponent() : base("Serialize FileDiff", "SFD", "Serialize FileDiff to JSON") { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F2");

    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FDiff";
    protected override string GetModelDescription() => "Serialize File difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFileDiffComponent : DeserializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public DeserializeFileDiffComponent() : base("Deserialize FileDiff", "DFD", "Deserialize JSON to FileDiff") { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F3");

    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FDiff";
    protected override string GetModelDescription() => "Deserialize File difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class FilesDiffGoo : DiffGoo<FilesDiff>
{
    public FilesDiffGoo() { }
    public FilesDiffGoo(FilesDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<FilesDiff> CreateDuplicate() => new FilesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FilesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<FilesDiff>() ?? new FilesDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FilesDiffParam : DiffParam<FilesDiffGoo, FilesDiff>
{
    public FilesDiffParam() : base("FilesDiff", "FsD", "FilesDiff parameter") { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A1");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FilesDiffComponent : DiffComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public FilesDiffComponent() : base("FilesDiff", "FsD", "FilesDiff component") { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A2");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "FsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FilesDiffParam(), "FsDf", "FsD", "A diff for multiple files.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FilesDiffParam(), "FsDf", "FsD", "A diff for multiple files.", GH_ParamAccess.item);
}

public class SerializeFilesDiffComponent : SerializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public SerializeFilesDiffComponent() : base("Serialize FilesDiff", "SFsD", "Serialize FilesDiff to JSON") { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A3");

    protected override string GetModelTypeName() => "FilesDiff";
    protected override string GetModelNickname() => "FsDiff";
    protected override string GetModelDescription() => "Serialize Files difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFilesDiffComponent : DeserializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public DeserializeFilesDiffComponent() : base("Deserialize FilesDiff", "DFsD", "Deserialize JSON to FilesDiff") { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A4");

    protected override string GetModelTypeName() => "FilesDiff";
    protected override string GetModelNickname() => "FsDiff";
    protected override string GetModelDescription() => "Deserialize Files difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class FileGoo : ModelGoo<File>
{
    public FileGoo() { }
    public FileGoo(File value) : base(value) { }

    
    
    
    protected override ModelGoo<File> CreateDuplicate() => new FileGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new File { Url = str };
            return true;
        }
        return false;
    }
}

public class FileParam : ModelParam<FileGoo, File>
{
    public FileParam() : base("Fil", "Fl", "A file with content.") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FileComponent : ModelComponent<FileParam, FileGoo, File>
{
    public FileComponent() : base("File", "Fil", "Construct, deconstruct or modify a file") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "F";
    protected override string GetModelName() => "File";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "Ur", "The url of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dat", "Da", "The data URI of the file.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Siz?", "Sz?", "The optional size of the file in bytes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Has?", "Hs?", "The optional hash of the file.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "Ur", "The url of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dat", "Da", "The data URI of the file.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Siz?", "Sz?", "The optional size of the file in bytes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Has?", "Hs?", "The optional hash of the file.", GH_ParamAccess.item);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, File model)
    {
        string? url = null, data = null, hash = null;
        int size = 0;

        if (DA.GetData(2, ref url) && url != null) model.Url = url;
        if (DA.GetData(3, ref data) && data != null) model.Data = data;
        if (DA.GetData(4, ref size)) model.Size = size;
        if (DA.GetData(5, ref hash) && hash != null) model.Hash = hash;
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, File model)
    {
        DA.SetData(2, model.Url);
        DA.SetData(3, model.Data);
        DA.SetData(4, model.Size);
        DA.SetData(5, model.Hash);
    }
}

public class SerializeFileComponent : SerializeComponent<FileParam, FileGoo, File>
{
    public SerializeFileComponent() : base("Serialize File", ">>F", "Serialize File") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FA");

    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "Serialize File";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFileComponent : DeserializeComponent<FileParam, FileGoo, File>
{
    public DeserializeFileComponent() : base("Deserialize File", "<<F", "Deserialize File") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FB");

    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "Deserialize File";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion File

#region DiagramPoint

public class DiagramPointGoo : ModelGoo<DiagramPoint>
{
    public DiagramPointGoo() { }
    public DiagramPointGoo(DiagramPoint value) : base(value) { }

    
    
    
    protected override ModelGoo<DiagramPoint> CreateDuplicate() => new DiagramPointGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.X, Value.Y, 0));
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        Point3d point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new DiagramPoint { X = (float)point.X, Y = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class DiagramPointParam : ModelParam<DiagramPointGoo, DiagramPoint>
{
    public DiagramPointParam() : base("DiagramPoint", "DP", "DiagramPoint parameter") { }
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DiagramPointComponent : ModelComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DiagramPointComponent() : base("DiagramPoint", "DPt", "Construct, deconstruct or modify a diagram point") { }
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "DP";
    protected override string GetModelName() => "DiagramPoint";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("X", "X", "The X coordinate", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The Y coordinate", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("X", "X", "The X coordinate", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The Y coordinate", GH_ParamAccess.item);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, DiagramPoint model)
    {
        double x = 0, y = 0;
        if (DA.GetData(2, ref x)) model.X = (float)x;
        if (DA.GetData(3, ref y)) model.Y = (float)y;
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, DiagramPoint model)
    {
        DA.SetData(2, model.X);
        DA.SetData(3, model.Y);
    }
}

public class SerializeDiagramPointComponent : SerializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public SerializeDiagramPointComponent() : base("Serialize DiagramPoint", "SDP", "Serialize DiagramPoint to JSON") { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");

    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "Serialize Diagram Point";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDiagramPointComponent : DeserializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DeserializeDiagramPointComponent() : base("Deserialize DiagramPoint", "DDP", "Deserialize JSON to DiagramPoint") { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A9A");

    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "Deserialize Diagram Point";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion DiagramPoint

#region Port

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }

    
    
    
    protected override ModelGoo<PortId> CreateDuplicate() => new PortIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortDiffGoo)))
        {
            target = (Q)(object)new PortDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortGoo)))
        {
            target = (Q)(object)new PortGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortGoo portGoo)
        {
            Value = portGoo.Value;
            return true;
        }
        if (source is PortDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
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
    public PortIdParam() : base("Por", "Po", "The optional local identifier of the port within the type. No id means the default port.") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortIdComponent : IdComponent<PortIdParam, PortIdGoo, PortId>
{
    public PortIdComponent() : base("The optional local identifier of the port within the type. No id means the default port.", "Por", "PortId component") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortIdParam(), "Por", "Po", "The optional local identifier of the port within the type. No id means the default port.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortIdParam(), "Por", "Po", "The optional local identifier of the port within the type. No id means the default port.", GH_ParamAccess.item);
}

public class PortDiffGoo : DiffGoo<PortDiff>
{
    public PortDiffGoo() { }
    public PortDiffGoo(PortDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<PortDiff> CreateDuplicate() => new PortDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortGoo)))
        {
            target = (Q)(object)new PortGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PortGoo portGoo)
        {
            Value = portGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PortDiff>() ?? new PortDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PortDiffParam : DiffParam<PortDiffGoo, PortDiff>
{
    public PortDiffParam() : base("PortDiff", "PD", "PortDiff parameter") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortDiffComponent : DiffComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public PortDiffComponent() : base("A diff for ports.", "PDf", "PortDiff component") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortDiffParam(), "PDf", "PD", "A diff for ports.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortDiffParam(), "PDf", "PD", "A diff for ports.", GH_ParamAccess.item);
}

public class SerializePortDiffComponent : SerializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public SerializePortDiffComponent() : base("Serialize PortDiff", "SPD", "Serialize PortDiff to JSON") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B4");

    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Serialize Port difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortDiffComponent : DeserializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public DeserializePortDiffComponent() : base("Deserialize PortDiff", "DPD", "Deserialize JSON to PortDiff") { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B5");

    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Deserialize Port difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PortGoo : ModelGoo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }

    
    
    
    protected override ModelGoo<Port> CreateDuplicate() => new PortGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            if (Value.Direction is null || Value.Point is null) return false;
            target = (Q)(object)new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortDiffGoo)))
        {
            target = (Q)(object)new PortDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PortDiffGoo diffGoo)
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
            Value = str.Deserialize<Port>() ?? new Port();
            return true;
        }
        return false;
    }
}

public class PortParam : ModelParam<PortGoo, Port>
{
    public PortParam() : base("Por", "Po", "A port is a connection point (with a direction) of a type.") { }
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortComponent : ModelComponent<PortParam, PortGoo, Port>
{
    public PortComponent() : base("Port", "Por", "Construct, deconstruct or modify a port") { }
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "P";
    protected override string GetModelName() => "Port";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Idn?", "Id?", "The optional local identifier of the port within the type. No id means the default port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the port.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Man?", "Ma?", "Whether the port is mandatory. A mandatory port must be connected in a design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Fam?", "Fa?", "The optional family of the port. This allows to define explicit compatibility with other ports.", GH_ParamAccess.item);
        pManager.AddTextParameter("CFas*", "CF*", "The optional other compatible families of the port. An empty list means this port is compatible with all other ports.", GH_ParamAccess.list);
        pManager.AddPointParameter("Pnt", "Pt", "The connection point of the port that is attracted to another connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Drn", "Dr", "The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the port.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the port.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Idn?", "Id?", "The optional local identifier of the port within the type. No id means the default port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the port.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Man?", "Ma?", "Whether the port is mandatory. A mandatory port must be connected in a design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Fam?", "Fa?", "The optional family of the port. This allows to define explicit compatibility with other ports.", GH_ParamAccess.item);
        pManager.AddTextParameter("CFas*", "CF*", "The optional other compatible families of the port. An empty list means this port is compatible with all other ports.", GH_ParamAccess.list);
        pManager.AddPointParameter("Pnt", "Pt", "The connection point of the port that is attracted to another connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Drn", "Dr", "The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the port.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the port.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Port model)
    {
        string? id = null, description = null, family = null;
        bool mandatory = false;
        var compatibleFamilies = new List<string>();
        Point3d point = Point3d.Unset;
        Vector3d direction = Vector3d.Unset;
        double t = 0;
        var props = new List<PropGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref id) && id != null) model.Id = id;
        if (DA.GetData(3, ref description) && description != null) model.Description = description;
        if (DA.GetData(4, ref mandatory)) model.Mandatory = mandatory;
        if (DA.GetData(5, ref family) && family != null) model.Family = family;
        if (DA.GetDataList(6, compatibleFamilies)) model.CompatibleFamilies = compatibleFamilies;
        if (DA.GetData(7, ref point) && point.IsValid) model.Point = new Point { X = (float)point.X, Y = (float)point.Y, Z = (float)point.Z };
        if (DA.GetData(8, ref direction) && direction.IsValid) model.Direction = new Vector { X = (float)direction.X, Y = (float)direction.Y, Z = (float)direction.Z };
        if (DA.GetData(9, ref t)) model.T = (float)t;
        if (DA.GetDataList(10, props)) model.Props = props.ConvertAll(p => p.Value);
        if (DA.GetDataList(11, attributes)) model.Attributes = attributes.ConvertAll(a => a.Value);
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Port model)
    {
        DA.SetData(2, model.Id);
        DA.SetData(3, model.Description);
        DA.SetData(4, model.Mandatory);
        DA.SetData(5, model.Family);
        DA.SetDataList(6, model.CompatibleFamilies);
        DA.SetData(7, model.Point != null ? new Point3d(model.Point.X, model.Point.Y, model.Point.Z) : Point3d.Unset);
        DA.SetData(8, model.Direction != null ? new Vector3d(model.Direction.X, model.Direction.Y, model.Direction.Z) : Vector3d.Unset);
        DA.SetData(9, model.T);
        DA.SetDataList(10, model.Props.ConvertAll(p => new PropGoo(p)));
        DA.SetDataList(11, model.Attributes.ConvertAll(a => new AttributeGoo(a)));
    }
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public SerializePortComponent() : base("Serialize Port", "SP", "Serialize Port to JSON") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Serialize Port";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public DeserializePortComponent() : base("Deserialize Port", "DP", "Deserialize JSON to Port") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Deserialize Port";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PortsDiffGoo : DiffGoo<PortsDiff>
{
    public PortsDiffGoo() { }
    public PortsDiffGoo(PortsDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<PortsDiff> CreateDuplicate() => new PortsDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PortsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PortsDiff>() ?? new PortsDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PortsDiffParam : DiffParam<PortsDiffGoo, PortsDiff>
{
    public PortsDiffParam() : base("PortsDiff", "PSD", "PortsDiff parameter") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C0");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortsDiffComponent : DiffComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public PortsDiffComponent() : base("PortsDiff", "PSD", "PortsDiff component") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C1");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortsDiffParam(), "PsDf", "PsD", "A diff for multiple ports.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortsDiffParam(), "PsDf", "PsD", "A diff for multiple ports.", GH_ParamAccess.item);
}

public class SerializePortsDiffComponent : SerializeComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public SerializePortsDiffComponent() : base("Serialize PortsDiff", "SPSD", "Serialize PortsDiff to JSON") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C2");
    protected override string GetModelTypeName() => "PortsDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Serialize Ports difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortsDiffComponent : DeserializeComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public DeserializePortsDiffComponent() : base("Deserialize PortsDiff", "DPSD", "Deserialize JSON to PortsDiff") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C3");
    protected override string GetModelTypeName() => "PortsDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Deserialize Ports difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Port

#region Author

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }

    
    
    
    protected override ModelGoo<AuthorId> CreateDuplicate() => new AuthorIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AuthorId { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    public AuthorIdParam() : base("Aut", "Au", "The id of the author.") { }
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1C");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AuthorIdComponent : IdComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public AuthorIdComponent() : base("The id of the author.", "Aut", "AuthorId component") { }
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1D");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "AuI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new AuthorIdParam(), "Aut", "Au", "The id of the author.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new AuthorIdParam(), "Aut", "Au", "The id of the author.", GH_ParamAccess.item);
}

public class AuthorGoo : ModelGoo<Author>
{
    public AuthorGoo() { }
    public AuthorGoo(Author value) : base(value) { }

    
    
    
    protected override ModelGoo<Author> CreateDuplicate() => new AuthorGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Author { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorParam : ModelParam<AuthorGoo, Author>
{
    public AuthorParam() : base("Author", "A", "Author parameter") { }
    public override Guid ComponentGuid => new("9F52380B-1812-42F7-9DAD-952C2F7A635A");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AuthorComponent : ModelComponent<AuthorParam, AuthorGoo, Author>
{
    public AuthorComponent() : base("Author", "Au", "Construct, deconstruct or modify an author") { }
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "Au";
    protected override string GetModelName() => "Author";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "The name of the author", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "E", "The email of the author", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "The name of the author", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "E", "The email of the author", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Author model)
    {
        string? name = null, email = null;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref name) && name != null) model.Name = name;
        if (DA.GetData(3, ref email) && email != null) model.Email = email;
        if (DA.GetDataList(4, attributes)) model.Attributes = attributes.ConvertAll(a => a.Value);
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Author model)
    {
        DA.SetData(2, model.Name);
        DA.SetData(3, model.Email);
        DA.SetDataList(4, model.Attributes.ConvertAll(a => new AttributeGoo(a)));
    }
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public SerializeAuthorComponent() : base("Serialize Author", "SA", "Serialize Author to JSON") { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "Serialize Author";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public DeserializeAuthorComponent() : base("Deserialize Author", "DA", "Deserialize JSON to Author") { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634879");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "Deserialize Author";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Author

#region Location

public class LocationGoo : ModelGoo<Location>
{
    public LocationGoo() { }
    public LocationGoo(Location value) : base(value) { }

    
    
    
    protected override ModelGoo<Location> CreateDuplicate() => new LocationGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.Longitude, Value.Latitude, 0));
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        var point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Location { Longitude = (float)point.X, Latitude = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class LocationParam : ModelParam<LocationGoo, Location>
{
    public LocationParam() : base("Location", "L", "Location parameter") { }
    public override Guid ComponentGuid => new("CA9DA889-398E-469B-BF1B-AD2BDFCA7957");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class LocationComponent : ModelComponent<LocationParam, LocationGoo, Location>
{
    public LocationComponent() : base("Location", "L", "Construct, deconstruct or modify a location") { }
    public override Guid ComponentGuid => new("6F2EDF42-6E10-4944-8B05-4D41F4876ED0");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "L";
    protected override string GetModelName() => "Location";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lng", "The longitude of the location", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "Lat", "The latitude of the location", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lng", "The longitude of the location", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "Lat", "The latitude of the location", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Location model)
    {
        double longitude = 0, latitude = 0;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref longitude)) model.Longitude = (float)longitude;
        if (DA.GetData(3, ref latitude)) model.Latitude = (float)latitude;
        if (DA.GetDataList(4, attributes)) model.Attributes = attributes.ConvertAll(a => a.Value);
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Location model)
    {
        DA.SetData(2, model.Longitude);
        DA.SetData(3, model.Latitude);
        DA.SetDataList(4, model.Attributes.ConvertAll(a => new AttributeGoo(a)));
    }
}

public class SerializeLocationComponent : SerializeComponent<LocationParam, LocationGoo, Location>
{
    public SerializeLocationComponent() : base("Serialize Location", "SL", "Serialize Location to JSON") { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D466");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Serialize Location";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeLocationComponent : DeserializeComponent<LocationParam, LocationGoo, Location>
{
    public DeserializeLocationComponent() : base("Deserialize Location", "DL", "Deserialize JSON to Location") { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D467");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Deserialize Location";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Location

#region Type

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }

    
    
    
    protected override ModelGoo<TypeId> CreateDuplicate() => new TypeIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new TypeId { Name = str };
            return true;
        }
        return false;
    }
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    public TypeIdParam() : base("Typ", "Ty", "The identifier of the type within the kit.") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeIdComponent : IdComponent<TypeIdParam, TypeIdGoo, TypeId>
{
    public TypeIdComponent() : base("The identifier of the type within the kit.", "Typ", "TypeId component") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "TI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypeIdParam(), "Typ", "Ty", "The identifier of the type within the kit.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypeIdParam(), "Typ", "Ty", "The identifier of the type within the kit.", GH_ParamAccess.item);
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<TypeDiff> CreateDuplicate() => new TypeDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<TypeDiff>() ?? new TypeDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypeDiffParam : DiffParam<TypeDiffGoo, TypeDiff>
{
    public TypeDiffParam() : base("TypeDiff", "TD", "TypeDiff parameter") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public TypeDiffComponent() : base("TypeDiff", "TD", "TypeDiff component") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "TD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypeDiffParam(), "TDf", "TD", "A diff for types.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypeDiffParam(), "TDf", "TD", "A diff for types.", GH_ParamAccess.item);
}

public class SerializeTypeDiffComponent : SerializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public SerializeTypeDiffComponent() : base("Serialize TypeDiff", "STD", "Serialize TypeDiff to JSON") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C5");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TDiff";
    protected override string GetModelDescription() => "Type difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypeDiffComponent : DeserializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public DeserializeTypeDiffComponent() : base("Deserialize TypeDiff", "DTD", "Deserialize JSON to TypeDiff") { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C6");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TDiff";
    protected override string GetModelDescription() => "Type difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class TypesDiffGoo : DiffGoo<TypesDiff>
{
    public TypesDiffGoo() { }
    public TypesDiffGoo(TypesDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<TypesDiff> CreateDuplicate() => new TypesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("TypesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<TypesDiff>() ?? new TypesDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypesDiffParam : DiffParam<TypesDiffGoo, TypesDiff>
{
    public TypesDiffParam() : base("TypesDiff", "TSD", "TypesDiff parameter") { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypesDiffComponent : DiffComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public TypesDiffComponent() : base("TypesDiff", "TSD", "TypesDiff component") { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "TsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypesDiffParam(), "TsDf", "TsD", "A diff for multiple types.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypesDiffParam(), "TsDf", "TsD", "A diff for multiple types.", GH_ParamAccess.item);
}

public class SerializeTypesDiffComponent : SerializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public SerializeTypesDiffComponent() : base("Serialize TypesDiff", "STSD", "Serialize TypesDiff to JSON") { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B8");
    protected override string GetModelTypeName() => "TypesDiff";
    protected override string GetModelNickname() => "TsDiff";
    protected override string GetModelDescription() => "Types difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypesDiffComponent : DeserializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public DeserializeTypesDiffComponent() : base("Deserialize TypesDiff", "DTSD", "Deserialize JSON to TypesDiff") { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B9");
    protected override string GetModelTypeName() => "TypesDiff";
    protected override string GetModelNickname() => "TsDiff";
    protected override string GetModelDescription() => "Types difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }

    
    
    
    protected override ModelGoo<Type> CreateDuplicate() => new TypeGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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
                Type = new TypeId { Name = Value.Name, Variant = Value.Variant }
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new Type { Name = piece.Value.Type.Name, Variant = piece.Value.Type.Variant };
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

public class TypeParam : ModelParam<TypeGoo, Type>
{
    public TypeParam() : base("Typ", "Ty", "A type is a reusable element that can be connected with other types over ports.") { }
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public TypeComponent() : base("Type", "Typ", "Construct, deconstruct or modify a type") { }
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "T";
    protected override string GetModelName() => "Type";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Nam", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Ico?", "Ic?", "The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.", GH_ParamAccess.item);
        pManager.AddTextParameter("Img?", "Im?", "The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.", GH_ParamAccess.item);
        pManager.AddTextParameter("Vnt?", "Vn?", "The optional variant of the type. No variant means the default variant.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stk?", "St?", "The optional number of items in stock. 2147483647 (=2^31-1) means infinite stock.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Vir?", "Vi?", "Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Sca?", "Sc?", "Whether the type is scalable.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mir?", "Mi?", "Whether the type is mirrorable.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri?", "Ur?", "The optional Unique Resource Identifier (URI) of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Loc?", "Lo?", "The optional location of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unt", "Ut", "The length unit of the point and the direction of the ports of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Reps*", "Rp*", "The optional representations of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Pors*", "Po*", "The optional ports of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Aut*", "Au*", "The optional authors of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the type.", GH_ParamAccess.list);
        pManager.AddTextParameter("Con*", "Co*", "The optional concepts of the type.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Nam", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Ico?", "Ic?", "The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.", GH_ParamAccess.item);
        pManager.AddTextParameter("Img?", "Im?", "The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.", GH_ParamAccess.item);
        pManager.AddTextParameter("Vnt?", "Vn?", "The optional variant of the type. No variant means the default variant.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stk?", "St?", "The optional number of items in stock. 2147483647 (=2^31-1) means infinite stock.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Vir?", "Vi?", "Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Sca?", "Sc?", "Whether the type is scalable.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mir?", "Mi?", "Whether the type is mirrorable.", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri?", "Ur?", "The optional Unique Resource Identifier (URI) of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Loc?", "Lo?", "The optional location of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unt", "Ut", "The length unit of the point and the direction of the ports of the type.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Reps*", "Rp*", "The optional representations of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Pors*", "Po*", "The optional ports of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Aut*", "Au*", "The optional authors of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the type.", GH_ParamAccess.list);
        pManager.AddTextParameter("Con*", "Co*", "The optional concepts of the type.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Type model)
    {
        string? name = null, description = null, icon = null, image = null, variant = null, uri = null, unit = null;
        int stock = 0;
        bool virt = false, scalable = false, mirrorable = false;
        LocationGoo? location = null;
        var representations = new List<RepresentationGoo>();
        var ports = new List<PortGoo>();
        var props = new List<PropGoo>();
        var authors = new List<AuthorIdGoo>();
        var attributes = new List<AttributeGoo>();
        var concepts = new List<string>();

        if (DA.GetData(2, ref name) && name != null) model.Name = name;
        if (DA.GetData(3, ref description) && description != null) model.Description = description;
        if (DA.GetData(4, ref icon) && icon != null) model.Icon = icon.Replace('\\', '/');
        if (DA.GetData(5, ref image) && image != null) model.Image = image.Replace('\\', '/');
        if (DA.GetData(6, ref variant) && variant != null) model.Variant = variant;
        if (DA.GetData(7, ref stock)) model.Stock = stock;
        if (DA.GetData(8, ref virt)) model.Virtual = virt;
        if (DA.GetData(9, ref scalable)) model.Scalable = scalable;
        if (DA.GetData(10, ref mirrorable)) model.Mirrorable = mirrorable;
        if (DA.GetData(11, ref uri) && uri != null) model.Uri = uri;
        if (DA.GetData(12, ref location) && location != null) model.Location = location.Value;
        if (DA.GetData(13, ref unit) && unit != null) model.Unit = unit;
        else if (model.Unit == "") try { model.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); } catch (Exception) { model.Unit = "m"; }
        if (DA.GetDataList(14, representations)) model.Representations = representations.ConvertAll(r => r.Value);
        if (DA.GetDataList(15, ports)) model.Ports = ports.ConvertAll(p => p.Value);
        if (DA.GetDataList(16, props)) model.Props = props.ConvertAll(p => p.Value);
        if (DA.GetDataList(17, authors)) model.Authors = authors.ConvertAll(a => a.Value);
        if (DA.GetDataList(18, attributes)) model.Attributes = attributes.ConvertAll(a => a.Value);
        if (DA.GetDataList(19, concepts)) model.Concepts = concepts;
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Type model)
    {
        DA.SetData(2, model.Name);
        DA.SetData(3, model.Description);
        DA.SetData(4, model.Icon);
        DA.SetData(5, model.Image);
        DA.SetData(6, model.Variant);
        DA.SetData(7, model.Stock);
        DA.SetData(8, model.Virtual);
        DA.SetData(9, model.Scalable);
        DA.SetData(10, model.Mirrorable);
        DA.SetData(11, model.Uri);
        DA.SetData(12, model.Location != null ? new LocationGoo(model.Location) : null);
        DA.SetData(13, model.Unit);
        DA.SetDataList(14, model.Representations.ConvertAll(r => new RepresentationGoo(r)));
        DA.SetDataList(15, model.Ports.ConvertAll(p => new PortGoo(p)));
        DA.SetDataList(16, model.Props.ConvertAll(p => new PropGoo(p)));
        DA.SetDataList(17, model.Authors.ConvertAll(a => new AuthorIdGoo(a)));
        DA.SetDataList(18, model.Attributes.ConvertAll(a => new AttributeGoo(a)));
        DA.SetDataList(19, model.Concepts);
    }
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public SerializeTypeComponent() : base("Serialize Type", "ST", "Serialize Type to JSON") { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "Type";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public DeserializeTypeComponent() : base("Deserialize Type", "DT", "Deserialize JSON to Type") { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673B");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "Type";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Type

#region Piece

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }

    
    
    
    protected override ModelGoo<PieceId> CreateDuplicate() => new PieceIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
    public PieceIdParam() : base("Pce", "Pc", "The optional local identifier of the piece within the design. No id means the default piece.") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceIdComponent : IdComponent<PieceIdParam, PieceIdGoo, PieceId>
{
    public PieceIdComponent() : base("The optional local identifier of the piece within the design. No id means the default piece.", "Pce", "PieceId component") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PiI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PieceIdParam(), "Pce", "Pc", "The optional local identifier of the piece within the design. No id means the default piece.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PieceIdParam(), "Pce", "Pc", "The optional local identifier of the piece within the design. No id means the default piece.", GH_ParamAccess.item);
}

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<PieceDiff> CreateDuplicate() => new PieceDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<PieceDiff>() ?? new PieceDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PieceDiffParam : DiffParam<PieceDiffGoo, PieceDiff>
{
    public PieceDiffParam() : base("PieceDiff", "PD", "PieceDiff parameter") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public PieceDiffComponent() : base("PieceDiff", "PD", "PieceDiff component") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PiD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PieceDiffParam(), "PDf", "PD", "A diff for pieces.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PieceDiffParam(), "PDf", "PD", "A diff for pieces.", GH_ParamAccess.item);
}

public class SerializePieceDiffComponent : SerializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public SerializePieceDiffComponent() : base("Serialize PieceDiff", "SPD", "Serialize PieceDiff to JSON") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D6");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Piece difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePieceDiffComponent : DeserializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public DeserializePieceDiffComponent() : base("Deserialize PieceDiff", "DPD", "Deserialize JSON to PieceDiff") { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D7");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Piece difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PiecesDiffGoo : DiffGoo<PiecesDiff>
{
    public PiecesDiffGoo() { }
    public PiecesDiffGoo(PiecesDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<PiecesDiff> CreateDuplicate() => new PiecesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PiecesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PiecesDiff>() ?? new PiecesDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PiecesDiffParam : DiffParam<PiecesDiffGoo, PiecesDiff>
{
    public PiecesDiffParam() : base("PiecesDiff", "PsD", "PiecesDiff parameter") { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C7");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PiecesDiffComponent : DiffComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public PiecesDiffComponent() : base("PiecesDiff", "PsD", "PiecesDiff component") { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C8");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "PsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PiecesDiffParam(), "PsDf", "PsD", "A diff for multiple pieces.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PiecesDiffParam(), "PsDf", "PsD", "A diff for multiple pieces.", GH_ParamAccess.item);
}

public class SerializePiecesDiffComponent : SerializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public SerializePiecesDiffComponent() : base("Serialize PiecesDiff", "SPsD", "Serialize PiecesDiff to JSON") { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C9");
    protected override string GetModelTypeName() => "PiecesDiff";
    protected override string GetModelNickname() => "PsDiff";
    protected override string GetModelDescription() => "Pieces difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePiecesDiffComponent : DeserializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public DeserializePiecesDiffComponent() : base("Deserialize PiecesDiff", "DPsD", "Deserialize JSON to PiecesDiff") { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6CA");
    protected override string GetModelTypeName() => "PiecesDiff";
    protected override string GetModelNickname() => "PsDiff";
    protected override string GetModelDescription() => "Pieces difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }

    
    
    
    protected override ModelGoo<Piece> CreateDuplicate() => new PieceGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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
            type.Value = new Type { Name = Value.Type.Name, Variant = Value.Type.Variant };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Type = new TypeId { Name = type.Value.Name, Variant = type.Value.Variant }
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

public class PieceParam : ModelParam<PieceGoo, Piece>
{
    public PieceParam() : base("Pce", "Pc", "A piece is a 3d-instance of a type in a design.") { }
    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceComponent : ModelComponent<PieceParam, PieceGoo, Piece>
{
    public PieceComponent() : base("Piece", "Pce", "Construct, deconstruct or modify a piece") { }
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "Pce";
    protected override string GetModelName() => "Piece";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Pce", "Pc", "Piece to modify", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id?", "The optional local identifier of the piece within the design. No id means the default piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Typ?", "Ty?", "The optional type of the piece. Either type or design must be set.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Dsn?", "Dn?", "The optional design of this piece. Either type or design must be set.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Pln?", "Pn?", "The optional plane of the piece. When pieces are connected only one piece can have a plane.", GH_ParamAccess.item);
        pManager.AddParameter(new DiagramPointParam(), "Cen?", "Ce?", "The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Hid?", "Hi?", "Whether the piece is hidden. A hidden piece is not visible in the model.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Lck?", "Lk?", "Whether the piece is locked. A locked piece cannot be edited.", GH_ParamAccess.item);
        pManager.AddTextParameter("Col?", "Cl?", "The optional hex color of the piece.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scl?", "Sc?", "The optional scale factor of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Mir?", "Mp?", "The optional mirror plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the piece.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the piece.", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Pce", "Pc", "The piece", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id?", "The optional local identifier of the piece within the design. No id means the default piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Dsc?", "Dc?", "The optional human-readable description of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Typ?", "Ty?", "The optional type of the piece. Either type or design must be set.", GH_ParamAccess.item);
        pManager.AddParameter(new DesignIdParam(), "Dsn?", "Dn?", "The optional design of this piece. Either type or design must be set.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Pln?", "Pn?", "The optional plane of the piece. When pieces are connected only one piece can have a plane.", GH_ParamAccess.item);
        pManager.AddParameter(new DiagramPointParam(), "Cen?", "Ce?", "The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Hid?", "Hi?", "Whether the piece is hidden. A hidden piece is not visible in the model.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Lck?", "Lk?", "Whether the piece is locked. A locked piece cannot be edited.", GH_ParamAccess.item);
        pManager.AddTextParameter("Col?", "Cl?", "The optional hex color of the piece.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Scl?", "Sc?", "The optional scale factor of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Mir?", "Mp?", "The optional mirror plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp*", "Pp*", "The optional properties of the piece.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr*", "At*", "The optional attributes of the piece.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Piece piece)
    {
        string id = null; if (DA.GetData(1, ref id) && !string.IsNullOrEmpty(id)) piece.Id = id;
        string description = null; if (DA.GetData(2, ref description) && !string.IsNullOrEmpty(description)) piece.Description = description;
        TypeIdGoo typeGoo = null; if (DA.GetData(3, ref typeGoo)) piece.Type = typeGoo.Value;
        DesignIdGoo designGoo = null; if (DA.GetData(4, ref designGoo)) piece.Design = designGoo.Value;
        Plane plane = new Plane(); if (DA.GetData(5, ref plane)) piece.Plane = new Semio.Plane { Origin = new Point { X = plane.Origin.X, Y = plane.Origin.Y, Z = plane.Origin.Z }, XAxis = new Vector { X = plane.XAxis.X, Y = plane.XAxis.Y, Z = plane.XAxis.Z }, YAxis = new Vector { X = plane.YAxis.X, Y = plane.YAxis.Y, Z = plane.YAxis.Z } };
        DiagramPointGoo centerGoo = null; if (DA.GetData(6, ref centerGoo)) piece.Center = centerGoo.Value;
        bool hidden = false; if (DA.GetData(7, ref hidden)) piece.Hidden = hidden;
        bool locked = false; if (DA.GetData(8, ref locked)) piece.Locked = locked;
        string color = null; if (DA.GetData(9, ref color) && !string.IsNullOrEmpty(color)) piece.Color = color;
        double scale = 1.0; if (DA.GetData(10, ref scale)) piece.Scale = (float)scale;
        Plane mirrorPlane = new Plane(); if (DA.GetData(11, ref mirrorPlane)) piece.MirrorPlane = new Semio.Plane { Origin = new Point { X = mirrorPlane.Origin.X, Y = mirrorPlane.Origin.Y, Z = mirrorPlane.Origin.Z }, XAxis = new Vector { X = mirrorPlane.XAxis.X, Y = mirrorPlane.XAxis.Y, Z = mirrorPlane.XAxis.Z }, YAxis = new Vector { X = mirrorPlane.YAxis.X, Y = mirrorPlane.YAxis.Y, Z = mirrorPlane.YAxis.Z } };
        var propGoos = new List<PropGoo>(); if (DA.GetDataList(12, propGoos)) piece.Props = propGoos.Select(g => g.Value).ToList();
        var attributeGoos = new List<AttributeGoo>(); if (DA.GetDataList(13, attributeGoos)) piece.Attributes = attributeGoos.Select(g => g.Value).ToList();
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Piece piece)
    {
        DA.SetData(0, new PieceGoo(piece));
        DA.SetData(1, piece.Id);
        DA.SetData(2, piece.Description);
        DA.SetData(3, piece.Type != null ? new TypeIdGoo(piece.Type) : null);
        DA.SetData(4, piece.Design != null ? new DesignIdGoo(piece.Design) : null);
        if (piece.Plane != null) { var plane = new Rhino.Geometry.Plane(); plane.Origin = new Point3d(piece.Plane.Origin.X, piece.Plane.Origin.Y, piece.Plane.Origin.Z); plane.XAxis = new Vector3d(piece.Plane.XAxis.X, piece.Plane.XAxis.Y, piece.Plane.XAxis.Z); plane.YAxis = new Vector3d(piece.Plane.YAxis.X, piece.Plane.YAxis.Y, piece.Plane.YAxis.Z); DA.SetData(5, plane); }
        DA.SetData(6, piece.Center != null ? new DiagramPointGoo(piece.Center) : null);
        DA.SetData(7, piece.Hidden);
        DA.SetData(8, piece.Locked);
        DA.SetData(9, piece.Color);
        DA.SetData(10, piece.Scale);
        if (piece.MirrorPlane != null) { var mirrorPlane = new Rhino.Geometry.Plane(); mirrorPlane.Origin = new Point3d(piece.MirrorPlane.Origin.X, piece.MirrorPlane.Origin.Y, piece.MirrorPlane.Origin.Z); mirrorPlane.XAxis = new Vector3d(piece.MirrorPlane.XAxis.X, piece.MirrorPlane.XAxis.Y, piece.MirrorPlane.XAxis.Z); mirrorPlane.YAxis = new Vector3d(piece.MirrorPlane.YAxis.X, piece.MirrorPlane.YAxis.Y, piece.MirrorPlane.YAxis.Z); DA.SetData(11, mirrorPlane); }
        DA.SetDataList(12, piece.Props.Select(p => new PropGoo(p)));
        DA.SetDataList(13, piece.Attributes.Select(a => new AttributeGoo(a)));
    }
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public SerializePieceComponent() : base("Serialize Piece", "SP", "Serialize Piece to JSON") { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public DeserializePieceComponent() : base("Deserialize Piece", "DP", "Deserialize JSON to Piece") { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Piece

#region Side

public class SideDiffGoo : DiffGoo<SideDiff>
{
    public SideDiffGoo() { }
    public SideDiffGoo(SideDiff value) : base(value) { }

    
    
    
    protected override ModelGoo<SideDiff> CreateDuplicate() => new SideDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("SideDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<SideDiff>() ?? new SideDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class SideDiffParam : DiffParam<SideDiffGoo, SideDiff>
{
    public SideDiffParam() : base("SideDiff", "SD", "SideDiff parameter") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class SideDiffComponent : DiffComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public SideDiffComponent() : base("SideDiff", "SD", "SideDiff component") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "SD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new SideDiffParam(), "SDf", "SD", "A diff for sides.", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new SideDiffParam(), "SDf", "SD", "A diff for sides.", GH_ParamAccess.item);

}

public class SerializeSideDiffComponent : SerializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public SerializeSideDiffComponent() : base("Serialize SideDiff", "SSD", "Serialize SideDiff to JSON") { }
    public override Guid ComponentGuid => new("B1C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
    protected override string GetModelTypeName() => "SideDiff";
    protected override string GetModelNickname() => "SDiff";
    protected override string GetModelDescription() => "Side difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeSideDiffComponent : DeserializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public DeserializeSideDiffComponent() : base("Deserialize SideDiff", "DSD", "Deserialize JSON to SideDiff") { }
    public override Guid ComponentGuid => new("B2C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
    protected override string GetModelTypeName() => "SideDiff";
    protected override string GetModelNickname() => "SDiff";
    protected override string GetModelDescription() => "Side difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class SideGoo : ModelGoo<Side>
{
    public SideGoo() { }
    public SideGoo(Side value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Piece.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Side { Piece = new PieceId { Id = str } };
            return true;
        }
        return false;
    }
    
    
    
    protected override ModelGoo<Side> CreateDuplicate() => new SideGoo(Value);
}

public class SideParam : ModelParam<SideGoo, Side>
{
    public SideParam() : base("Side", "S", "Side parameter") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class SideComponent : ModelComponent<SideParam, SideGoo, Side>
{
    public SideComponent() : base("Side", "Sde", "Construct, deconstruct or modify a side") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "Sid";
    protected override string GetModelName() => "Side";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SideParam(), "Sde", "Sd", "Side to modify", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pce", "Pc", "The piece ID of the side", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP", "The design piece ID of the side", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Por", "Po", "The port ID of the side", GH_ParamAccess.item);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SideParam(), "Sde", "Sd", "The side", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pce", "Pc", "The piece ID of the side", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP", "The design piece ID of the side", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam(), "Por", "Po", "The port ID of the side", GH_ParamAccess.item);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Side side)
    {
        PieceIdGoo pieceGoo = null; if (DA.GetData(1, ref pieceGoo)) side.Piece = pieceGoo.Value;
        PieceIdGoo designPieceGoo = null; if (DA.GetData(2, ref designPieceGoo)) side.DesignPiece = designPieceGoo.Value;
        PortIdGoo portGoo = null; if (DA.GetData(3, ref portGoo)) side.Port = portGoo.Value;
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Side side)
    {
        DA.SetData(0, new SideGoo(side));
        DA.SetData(1, new PieceIdGoo(side.Piece));
        DA.SetData(2, side.DesignPiece != null ? new PieceIdGoo(side.DesignPiece) : null);
        DA.SetData(3, new PortIdGoo(side.Port));
    }
}

public class SerializeSideComponent : SerializeComponent<SideParam, SideGoo, Side>
{
    public SerializeSideComponent() : base("Serialize Side", "SS", "Serialize Side to JSON") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E7");
    protected override string GetModelTypeName() => "Side";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Side of a piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeSideComponent : DeserializeComponent<SideParam, SideGoo, Side>
{
    public DeserializeSideComponent() : base("Deserialize Side", "DS", "Deserialize JSON to Side") { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E8");
    protected override string GetModelTypeName() => "Side";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Side of a piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Side

#region Connection

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<ConnectionId>() ?? new ConnectionId();
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
    
    
    
    protected override ModelGoo<ConnectionId> CreateDuplicate() => new ConnectionIdGoo(Value);
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    public ConnectionIdParam() : base("ConId", "Cn", "The local identifier of the connection within the design.") { }
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionIdComponent : IdComponent<ConnectionIdParam, ConnectionIdGoo, ConnectionId>
{
    public ConnectionIdComponent() : base("The local identifier of the connection within the design.", "ConId", "ConnectionId component") { }
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "CI";
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<ConnectionDiff>() ?? new ConnectionDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    
    
    
    protected override ModelGoo<ConnectionDiff> CreateDuplicate() => new ConnectionDiffGoo(Value);
}

public class ConnectionDiffParam : DiffParam<ConnectionDiffGoo, ConnectionDiff>
{
    public ConnectionDiffParam() : base("ConnectionDiff", "CD", "ConnectionDiff parameter") { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public ConnectionDiffComponent() : base("ConnectionDiff", "CD", "ConnectionDiff component") { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "CD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionDiffParam(), "CDf", "CD", "A diff for connections.", GH_ParamAccess.item);

}

public class SerializeConnectionDiffComponent : SerializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public SerializeConnectionDiffComponent() : base("Serialize ConnectionDiff", "SCD", "Serialize ConnectionDiff to JSON") { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F6");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CDiff";
    protected override string GetModelDescription() => "Connection difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionDiffComponent : DeserializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public DeserializeConnectionDiffComponent() : base("Deserialize ConnectionDiff", "DCD", "Deserialize JSON to ConnectionDiff") { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F7");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CDiff";
    protected override string GetModelDescription() => "Connection difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class ConnectionsDiffGoo : DiffGoo<ConnectionsDiff>
{
    public ConnectionsDiffGoo() { }
    public ConnectionsDiffGoo(ConnectionsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<ConnectionsDiff>() ?? new ConnectionsDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    
    
    
    protected override ModelGoo<ConnectionsDiff> CreateDuplicate() => new ConnectionsDiffGoo(Value);
}

public class ConnectionsDiffParam : DiffParam<ConnectionsDiffGoo, ConnectionsDiff>
{
    public ConnectionsDiffParam() : base("ConnectionsDiff", "CsD", "ConnectionsDiff parameter") { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D8");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionsDiffComponent : DiffComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public ConnectionsDiffComponent() : base("ConnectionsDiff", "CsD", "ConnectionsDiff component") { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D9");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "CsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionsDiffParam(), "CsDf", "CsD", "A diff for multiple connections.", GH_ParamAccess.item);

}

public class SerializeConnectionsDiffComponent : SerializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public SerializeConnectionsDiffComponent() : base("Serialize ConnectionsDiff", "SCsD", "Serialize ConnectionsDiff to JSON") { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DA");
    protected override string GetModelTypeName() => "ConnectionsDiff";
    protected override string GetModelNickname() => "CsDiff";
    protected override string GetModelDescription() => "Connections difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionsDiffComponent : DeserializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public DeserializeConnectionsDiffComponent() : base("Deserialize ConnectionsDiff", "DCsD", "Deserialize JSON to ConnectionsDiff") { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DB");
    protected override string GetModelTypeName() => "ConnectionsDiff";
    protected override string GetModelNickname() => "CsDiff";
    protected override string GetModelDescription() => "Connections difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class ConnectionGoo : ModelGoo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<Connection>() ?? new Connection();
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
    
    
    
    protected override ModelGoo<Connection> CreateDuplicate() => new ConnectionGoo(Value);
}

public class ConnectionParam : ModelParam<ConnectionGoo, Connection>
{
    public ConnectionParam() : base("Connection", "C", "Connection parameter") { }
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public ConnectionComponent() : base("Connection", "Con", "Construct, deconstruct or modify a connection") { }
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "Con";
    protected override string GetModelName() => "Connection";

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "Con", "Connection to modify", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Co", "The connected side", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cn", "The connecting side", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "The description of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "G", "The gap of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "S", "The shift of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "R", "The rise of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Ro", "The rotation of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "T", "The turn of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Ti", "The tilt of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X", "The X coordinate of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The Y coordinate of the connection", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp", "Pp", "A property is a value with an optional unit for a quality.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "Con", "The connection", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Co", "The connected side", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cn", "The connecting side", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "The description of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "G", "The gap of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "S", "The shift of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "R", "The rise of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Ro", "The rotation of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "T", "The turn of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Ti", "The tilt of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X", "The X coordinate of the connection", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The Y coordinate of the connection", GH_ParamAccess.item);
        pManager.AddParameter(new PropParam(), "Prp", "Pp", "A property is a value with an optional unit for a quality.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Atr", "At", "A attribute is a key value pair with an an optional definition.", GH_ParamAccess.list);
    }

    protected override void ProcessModelInputs(IGH_DataAccess DA, Connection connection)
    {
        SideGoo connectedGoo = null; if (DA.GetData(1, ref connectedGoo)) connection.Connected = connectedGoo.Value;
        SideGoo connectingGoo = null; if (DA.GetData(2, ref connectingGoo)) connection.Connecting = connectingGoo.Value;
        string description = null; if (DA.GetData(3, ref description) && !string.IsNullOrEmpty(description)) connection.Description = description;
        double gap = 0; if (DA.GetData(4, ref gap)) connection.Gap = (float)gap;
        double shift = 0; if (DA.GetData(5, ref shift)) connection.Shift = (float)shift;
        double rise = 0; if (DA.GetData(6, ref rise)) connection.Rise = (float)rise;
        double rotation = 0; if (DA.GetData(7, ref rotation)) connection.Rotation = (float)rotation;
        double turn = 0; if (DA.GetData(8, ref turn)) connection.Turn = (float)turn;
        double tilt = 0; if (DA.GetData(9, ref tilt)) connection.Tilt = (float)tilt;
        double x = 0; if (DA.GetData(10, ref x)) connection.X = (float)x;
        double y = 1; if (DA.GetData(11, ref y)) connection.Y = (float)y;
        var propGoos = new List<PropGoo>(); if (DA.GetDataList(12, propGoos)) connection.Props = propGoos.Select(g => g.Value).ToList();
        var attributeGoos = new List<AttributeGoo>(); if (DA.GetDataList(13, attributeGoos)) connection.Attributes = attributeGoos.Select(g => g.Value).ToList();
    }

    protected override void ProcessModelOutputs(IGH_DataAccess DA, Connection connection)
    {
        DA.SetData(0, new ConnectionGoo(connection));
        DA.SetData(1, new SideGoo(connection.Connected));
        DA.SetData(2, new SideGoo(connection.Connecting));
        DA.SetData(3, connection.Description);
        DA.SetData(4, connection.Gap);
        DA.SetData(5, connection.Shift);
        DA.SetData(6, connection.Rise);
        DA.SetData(7, connection.Rotation);
        DA.SetData(8, connection.Turn);
        DA.SetData(9, connection.Tilt);
        DA.SetData(10, connection.X);
        DA.SetData(11, connection.Y);
        DA.SetDataList(12, connection.Props.Select(p => new PropGoo(p)));
        DA.SetDataList(13, connection.Attributes.Select(a => new AttributeGoo(a)));
    }
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public SerializeConnectionComponent() : base("Serialize Connection", "SC", "Serialize Connection to JSON") { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "Connection between pieces";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public DeserializeConnectionComponent() : base("Deserialize Connection", "DC", "Deserialize JSON to Connection") { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "Connection between pieces";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Connection

#region Design

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new DesignId { Name = str };
            return true;
        }
        return false;
    }
    
    
    
    protected override ModelGoo<DesignId> CreateDuplicate() => new DesignIdGoo(Value);
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    public DesignIdParam() : base("Dsn", "Dn", "The local identifier of the design within the kit.") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignIdComponent : IdComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public DesignIdComponent() : base("The local identifier of the design within the kit.", "Dsn", "DesignId component") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "DI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignIdParam(), "Dsn", "Dn", "The local identifier of the design within the kit.", GH_ParamAccess.item);

}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<DesignDiff>() ?? new DesignDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    
    
    
    protected override ModelGoo<DesignDiff> CreateDuplicate() => new DesignDiffGoo(Value);
}

public class DesignDiffParam : DiffParam<DesignDiffGoo, DesignDiff>
{
    public DesignDiffParam() : base("DesignDiff", "DD", "DesignDiff parameter") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DesignDiffComponent() : base("DesignDiff", "DD", "DesignDiff component") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A8");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "DD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignDiffParam(), "DDf", "DD", "A diff for designs.", GH_ParamAccess.item);

}

public class SerializeDesignDiffComponent : SerializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public SerializeDesignDiffComponent() : base("Serialize DesignDiff", "SDD", "Serialize DesignDiff to JSON") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A9");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DDiff";
    protected override string GetModelDescription() => "Design difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignDiffComponent : DeserializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DeserializeDesignDiffComponent() : base("Deserialize DesignDiff", "DDD", "Deserialize JSON to DesignDiff") { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4AA");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DDiff";
    protected override string GetModelDescription() => "Design difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DesignsDiffGoo : DiffGoo<DesignsDiff>
{
    public DesignsDiffGoo() { }
    public DesignsDiffGoo(DesignsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("DesignsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<DesignsDiff>() ?? new DesignsDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    
    
    
    protected override ModelGoo<DesignsDiff> CreateDuplicate() => new DesignsDiffGoo(Value);
}

public class DesignsDiffParam : DiffParam<DesignsDiffGoo, DesignsDiff>
{
    public DesignsDiffParam() : base("DesignsDiff", "DsD", "DesignsDiff parameter") { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8E9");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignsDiffComponent : DiffComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public DesignsDiffComponent() : base("DesignsDiff", "DsD", "DesignsDiff component") { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EA");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "DsD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignsDiffParam(), "DsDf", "DsD", "A diff for multiple designs.", GH_ParamAccess.item);

}

public class SerializeDesignsDiffComponent : SerializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public SerializeDesignsDiffComponent() : base("Serialize DesignsDiff", "SDsD", "Serialize DesignsDiff to JSON") { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EB");
    protected override string GetModelTypeName() => "DesignsDiff";
    protected override string GetModelNickname() => "DsDiff";
    protected override string GetModelDescription() => "Designs difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignsDiffComponent : DeserializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public DeserializeDesignsDiffComponent() : base("Deserialize DesignsDiff", "DDsD", "Deserialize JSON to DesignsDiff") { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EC");
    protected override string GetModelTypeName() => "DesignsDiff";
    protected override string GetModelNickname() => "DsDiff";
    protected override string GetModelDescription() => "Designs difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DesignGoo : ModelGoo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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

    
    
    
    protected override ModelGoo<Design> CreateDuplicate() => new DesignGoo(Value);
}

public class DesignParam : ModelParam<DesignGoo, Design>
{
    public DesignParam() : base("Design", "D", "Design parameter") { }
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignComponent : ModelComponent<DesignParam, DesignGoo, Design>
{
    public DesignComponent() : base("Design", "Dsn", "Design component") { }
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "D";
    protected override string GetModelName() => "Design";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "Variant", "Variant", GH_ParamAccess.item);
        pManager.AddTextParameter("View", "View", "View", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Image", "Image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "Concepts", "Concepts", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Authors", "Authors", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "Location", "Location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Scalable", "Scalable", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mirrorable", "Mirrorable", "Mirrorable", GH_ParamAccess.item);
        pManager.AddParameter(new LayerParam(), "Layers", "Layers", "Layers", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Groups", "Groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Connections", "Connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Props", "Props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "Stats", "Stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "Variant", "Variant", GH_ParamAccess.item);
        pManager.AddTextParameter("View", "View", "View", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Image", "Image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "Concepts", "Concepts", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Authors", "Authors", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "Location", "Location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Scalable", "Scalable", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mirrorable", "Mirrorable", "Mirrorable", GH_ParamAccess.item);
        pManager.AddParameter(new LayerParam(), "Layers", "Layers", "Layers", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Groups", "Groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Connections", "Connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Props", "Props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "Stats", "Stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Design model)
    {
        string name = ""; if (DA.GetData(1, ref name)) model.Name = name;
        string variant = ""; if (DA.GetData(2, ref variant)) model.Variant = variant;
        string view = ""; if (DA.GetData(3, ref view)) model.View = view;
        string description = ""; if (DA.GetData(4, ref description)) model.Description = description;
        string icon = ""; if (DA.GetData(5, ref icon)) model.Icon = icon.Replace('\\', '/');
        string image = ""; if (DA.GetData(6, ref image)) model.Image = image.Replace('\\', '/');
        var concepts = new List<string>(); if (DA.GetDataList(7, concepts)) model.Concepts = concepts;
        var authors = new List<AuthorIdGoo>(); if (DA.GetDataList(8, authors)) model.Authors = authors.Select(x => x.Value).ToList();
        LocationGoo? location = null; if (DA.GetData(9, ref location) && location != null) model.Location = location.Value;
        string unit = ""; if (DA.GetData(10, ref unit)) model.Unit = unit;
        bool scalable = false; if (DA.GetData(11, ref scalable)) model.Scalable = scalable;
        bool mirrorable = false; if (DA.GetData(12, ref mirrorable)) model.Mirrorable = mirrorable;
        var layers = new List<LayerGoo>(); if (DA.GetDataList(13, layers)) model.Layers = layers.Select(x => x.Value).ToList();
        var pieces = new List<PieceGoo>(); if (DA.GetDataList(14, pieces)) model.Pieces = pieces.Select(x => x.Value).ToList();
        var groups = new List<GroupGoo>(); if (DA.GetDataList(15, groups)) model.Groups = groups.Select(x => x.Value).ToList();
        var connections = new List<ConnectionGoo>(); if (DA.GetDataList(16, connections)) model.Connections = connections.Select(x => x.Value).ToList();
        var props = new List<PropGoo>(); if (DA.GetDataList(17, props)) model.Props = props.Select(x => x.Value).ToList();
        var stats = new List<StatGoo>(); if (DA.GetDataList(18, stats)) model.Stats = stats.Select(x => x.Value).ToList();
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(19, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
        if (model.Unit == "")
            try { model.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { model.Unit = "m"; }
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Design model)
    {
        DA.SetData(0, new DesignGoo(model));
        DA.SetData(1, model.Name);
        DA.SetData(2, model.Variant);
        DA.SetData(3, model.View);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetData(6, model.Image);
        DA.SetDataList(7, model.Concepts);
        DA.SetDataList(8, model.Authors.Select(x => new AuthorIdGoo(x)));
        DA.SetData(9, model.Location != null ? new LocationGoo(model.Location) : null);
        DA.SetData(10, model.Unit);
        DA.SetData(11, model.Scalable);
        DA.SetData(12, model.Mirrorable);
        DA.SetDataList(13, model.Layers.Select(x => new LayerGoo(x)));
        DA.SetDataList(14, model.Pieces.Select(x => new PieceGoo(x)));
        DA.SetDataList(15, model.Groups.Select(x => new GroupGoo(x)));
        DA.SetDataList(16, model.Connections.Select(x => new ConnectionGoo(x)));
        DA.SetDataList(17, model.Props.Select(x => new PropGoo(x)));
        DA.SetDataList(18, model.Stats.Select(x => new StatGoo(x)));
        DA.SetDataList(19, model.Attributes.Select(x => new AttributeGoo(x)));
    }
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public SerializeDesignComponent() : base("Serialize Design", "SD", "Serialize Design to JSON") { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "Design";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public DeserializeDesignComponent() : base("Deserialize Design", "DD", "Deserialize JSON to Design") { }
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "Design";
    protected override Guid GetComponentGuid() => ComponentGuid;
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB59");
}

#endregion Design

#region Kit

public class KitIdGoo : IdGoo<KitId>
{
    public KitIdGoo() { }
    public KitIdGoo(KitId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new KitId { Name = str };
            return true;
        }
        return false;
    }

    
    
    
    protected override ModelGoo<KitId> CreateDuplicate() => new KitIdGoo(Value);
}

public class KitIdParam : IdParam<KitIdGoo, KitId>
{
    public KitIdParam() : base("KitId", "KId", "The local identifier of the kit.") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B0");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitIdComponent : IdComponent<KitIdParam, KitIdGoo, KitId>
{
    public KitIdComponent() : base("The local identifier of the kit.", "KitId", "KitId component") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B1");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "KI";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitIdParam(), "KitId", "KId", "Kit identifier", GH_ParamAccess.item);

}

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<KitDiff>() ?? new KitDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    
    
    
    protected override ModelGoo<KitDiff> CreateDuplicate() => new KitDiffGoo(Value);
}

public class KitDiffParam : DiffParam<KitDiffGoo, KitDiff>
{
    public KitDiffParam() : base("KitDiff", "KD", "KitDiff parameter") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public KitDiffComponent() : base("KitDiff", "KD", "KitDiff component") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B3");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "KD";
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitDiffParam(), "KitDiff", "KDiff", "Kit difference", GH_ParamAccess.item);

}

public class SerializeKitDiffComponent : SerializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public SerializeKitDiffComponent() : base("Serialize KitDiff", "SKD", "Serialize KitDiff to JSON") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B4");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KDiff";
    protected override string GetModelDescription() => "Kit difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitDiffComponent : DeserializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public DeserializeKitDiffComponent() : base("Deserialize KitDiff", "DKD", "Deserialize JSON to KitDiff") { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B5");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KDiff";
    protected override string GetModelDescription() => "Kit difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class KitGoo : ModelGoo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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

    
    
    
    protected override ModelGoo<Kit> CreateDuplicate() => new KitGoo(Value);
}

public class KitParam : ModelParam<KitGoo, Kit>
{
    public KitParam() : base("Kit", "K", "Kit parameter") { }
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public KitComponent() : base("Kit", "Kit", "Kit component") { }
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "K";
    protected override string GetModelName() => "Kit";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Version", "Version", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Image", "Image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "Concepts", "Concepts", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "Remote", "Remote", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Homepage", "Homepage", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "License", "License", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorParam(), "Authors", "Authors", "Authors", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Groups", "Groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Connections", "Connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Props", "Props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "Stats", "Stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Preview", "Preview", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Qualities", "Qualities", "Qualities", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "Types", "Types", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Designs", "Designs", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Version", "Version", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Image", "Image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "Concepts", "Concepts", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "Remote", "Remote", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Homepage", "Homepage", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "License", "License", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorParam(), "Authors", "Authors", "Authors", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "Groups", "Groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Connections", "Connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Props", "Props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "Stats", "Stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Preview", "Preview", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Qualities", "Qualities", "Qualities", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "Types", "Types", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Designs", "Designs", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Kit model)
    {
        string name = ""; if (DA.GetData(1, ref name)) model.Name = name;
        string version = ""; if (DA.GetData(2, ref version)) model.Version = version;
        string description = ""; if (DA.GetData(3, ref description)) model.Description = description;
        string icon = ""; if (DA.GetData(4, ref icon)) model.Icon = icon.Replace('\\', '/');
        string image = ""; if (DA.GetData(5, ref image)) model.Image = image.Replace('\\', '/');
        var concepts = new List<string>(); if (DA.GetDataList(6, concepts)) model.Concepts = concepts;
        string remote = ""; if (DA.GetData(7, ref remote)) model.Remote = remote;
        string homepage = ""; if (DA.GetData(8, ref homepage)) model.Homepage = homepage;
        string license = ""; if (DA.GetData(9, ref license)) model.License = license;
        var authors = new List<AuthorGoo>(); if (DA.GetDataList(10, authors)) model.Authors = authors.Select(x => x.Value).ToList();
        var pieces = new List<PieceGoo>(); if (DA.GetDataList(11, pieces)) model.Pieces = pieces.Select(x => x.Value).ToList();
        var groups = new List<GroupGoo>(); if (DA.GetDataList(12, groups)) model.Groups = groups.Select(x => x.Value).ToList();
        var connections = new List<ConnectionGoo>(); if (DA.GetDataList(13, connections)) model.Connections = connections.Select(x => x.Value).ToList();
        var props = new List<PropGoo>(); if (DA.GetDataList(14, props)) model.Props = props.Select(x => x.Value).ToList();
        var stats = new List<StatGoo>(); if (DA.GetDataList(15, stats)) model.Stats = stats.Select(x => x.Value).ToList();
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(16, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
        string preview = ""; if (DA.GetData(17, ref preview)) model.Preview = preview.Replace('\\', '/');
        var qualities = new List<QualityGoo>(); if (DA.GetDataList(18, qualities)) model.Qualities = qualities.Select(x => x.Value).ToList();
        var types = new List<TypeGoo>(); if (DA.GetDataList(19, types)) model.Types = types.Select(x => x.Value).ToList();
        var designs = new List<DesignGoo>(); if (DA.GetDataList(20, designs)) model.Designs = designs.Select(x => x.Value).ToList();
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Kit model)
    {
        DA.SetData(0, new KitGoo(model));
        DA.SetData(1, model.Name);
        DA.SetData(2, model.Version);
        DA.SetData(3, model.Description);
        DA.SetData(4, model.Icon);
        DA.SetData(5, model.Image);
        DA.SetDataList(6, model.Concepts);
        DA.SetData(7, model.Remote);
        DA.SetData(8, model.Homepage);
        DA.SetData(9, model.License);
        DA.SetDataList(10, model.Authors.Select(x => new AuthorGoo(x)));
        DA.SetDataList(11, model.Pieces.Select(x => new PieceGoo(x)));
        DA.SetDataList(12, model.Groups.Select(x => new GroupGoo(x)));
        DA.SetDataList(13, model.Connections.Select(x => new ConnectionGoo(x)));
        DA.SetDataList(14, model.Props.Select(x => new PropGoo(x)));
        DA.SetDataList(15, model.Stats.Select(x => new StatGoo(x)));
        DA.SetDataList(16, model.Attributes.Select(x => new AttributeGoo(x)));
        DA.SetData(17, model.Preview);
        DA.SetDataList(18, model.Qualities.Select(x => new QualityGoo(x)));
        DA.SetDataList(19, model.Types.Select(x => new TypeGoo(x)));
        DA.SetDataList(20, model.Designs.Select(x => new DesignGoo(x)));
    }
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public SerializeKitComponent() : base("Serialize Kit", "SK", "Serialize Kit to JSON") { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "Kit";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public DeserializeKitComponent() : base("Deserialize Kit", "DK", "Deserialize JSON to Kit") { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4166");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "Kit";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class KitsDiffGoo : DiffGoo<KitsDiff>
{
    public KitsDiffGoo() { }
    public KitsDiffGoo(KitsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("KitsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<KitsDiff>() ?? new KitsDiff();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    
    
    
    protected override ModelGoo<KitsDiff> CreateDuplicate() => new KitsDiffGoo(Value);
}

public class KitsDiffParam : DiffParam<KitsDiffGoo, KitsDiff>
{
    public KitsDiffParam() : base("KitsDiff", "KsD", "KitsDiff parameter") { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitsDiffComponent : DiffComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public KitsDiffComponent() : base("KitsDiff", "KsD", "KitsDiff component") { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "KsD";
}

public class SerializeKitsDiffComponent : SerializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public SerializeKitsDiffComponent() : base("Serialize KitsDiff", "SKsD", "Serialize KitsDiff to JSON") { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C5");
    protected override string GetModelTypeName() => "KitsDiff";
    protected override string GetModelNickname() => "KsDiff";
    protected override string GetModelDescription() => "Kits difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitsDiffComponent : DeserializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public DeserializeKitsDiffComponent() : base("Deserialize KitsDiff", "DKsD", "Deserialize JSON to KitsDiff") { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C6");
    protected override string GetModelTypeName() => "KitsDiff";
    protected override string GetModelNickname() => "KsDiff";
    protected override string GetModelDescription() => "Kits difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Kit

#region Quality

public class QualityKindGoo : EnumGoo<QualityKind>
{
    public QualityKindGoo() { }
    public QualityKindGoo(QualityKind value) : base(value) { }

    protected override EnumGoo<QualityKind> CreateDuplicate() => new QualityKindGoo(Value);
    
    
}

public class QualityKindParam : EnumParam<QualityKindGoo, QualityKind>
{
    public QualityKindParam() : base("QualityKind", "QK", "QualityKind parameter", new("A1B2C3D4-E5F6-4A5B-9C8D-7E6F5A4B3C2D")) { }
}

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<QualityId>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);

    
    
    
    protected override ModelGoo<QualityId> CreateDuplicate() => new QualityIdGoo(Value);
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    public QualityIdParam() : base("QualityId", "QI", "QualityId parameter") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityIdComponent : IdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public QualityIdComponent() : base("A quality id is a key for a quality.", "Qal", "QualityId component") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "QI";
}

public class SerializeQualityIdComponent : SerializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public SerializeQualityIdComponent() : base("Serialize QualityId", "SQI", "Serialize QualityId to JSON") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CA");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QId";
    protected override string GetModelDescription() => "Quality identifier";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityIdComponent : DeserializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public DeserializeQualityIdComponent() : base("Deserialize QualityId", "DQI", "Deserialize JSON to QualityId") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CB");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QId";
    protected override string GetModelDescription() => "Quality identifier";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class QualityDiffGoo : DiffGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<QualityDiff>();
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

    
    
    
    protected override ModelGoo<QualityDiff> CreateDuplicate() => new QualityDiffGoo(Value);
}

public class QualityDiffParam : DiffParam<QualityDiffGoo, QualityDiff>
{
    public QualityDiffParam() : base("QualityDiff", "QD", "QualityDiff parameter") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DA");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public QualityDiffComponent() : base("QualityDiff", "QD", "QualityDiff component") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DB");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "QD";
}

public class SerializeQualityDiffComponent : SerializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public SerializeQualityDiffComponent() : base("Serialize QualityDiff", "SQD", "Serialize QualityDiff to JSON") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DC");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QDiff";
    protected override string GetModelDescription() => "Quality difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityDiffComponent : DeserializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public DeserializeQualityDiffComponent() : base("Deserialize QualityDiff", "DQD", "Deserialize JSON to QualityDiff") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DD");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QDiff";
    protected override string GetModelDescription() => "Quality difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class QualityGoo : ModelGoo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<Quality>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new((Quality)idGoo.Value);

    
    
    
    protected override ModelGoo<Quality> CreateDuplicate() => new QualityGoo(Value);
}

public class QualityParam : ModelParam<QualityGoo, Quality>
{
    public QualityParam() : base("Quality", "Q", "Quality parameter") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public QualityComponent() : base("Quality", "Qal", "Quality component") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "Q";
    protected override string GetModelName() => "Quality";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Q", "Quality", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Uri", "Uri", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Scalable", "Scalable", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kind", "Kind", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "SI", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Imperial", "Imperial", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Default", "Default", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Formula", "Formula", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam(), "Benchmarks", "Benchmarks", "Benchmarks", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Q", "Quality", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Uri", "Uri", "Uri", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Scalable", "Scalable", "Scalable", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Kind", "Kind", "Kind", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "SI", GH_ParamAccess.item);
        pManager.AddTextParameter("Imperial", "Imperial", "Imperial", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Default", "Default", "Default", GH_ParamAccess.item);
        pManager.AddTextParameter("Formula", "Formula", "Formula", GH_ParamAccess.item);
        pManager.AddParameter(new BenchmarkParam(), "Benchmarks", "Benchmarks", "Benchmarks", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Quality model)
    {
        string key = ""; if (DA.GetData(1, ref key)) model.Key = key;
        string name = ""; if (DA.GetData(2, ref name)) model.Name = name;
        string description = ""; if (DA.GetData(3, ref description)) model.Description = description;
        string uri = ""; if (DA.GetData(4, ref uri)) model.Uri = uri;
        bool scalable = false; if (DA.GetData(5, ref scalable)) model.Scalable = scalable;
        int kind = 0; if (DA.GetData(6, ref kind)) model.Kind = (QualityKind)kind;
        string si = ""; if (DA.GetData(7, ref si)) model.SI = si;
        string imperial = ""; if (DA.GetData(8, ref imperial)) model.Imperial = imperial;
        double min = 0.0; if (DA.GetData(9, ref min)) model.Min = (float)min;
        bool minExcluded = false; if (DA.GetData(10, ref minExcluded)) model.MinExcluded = minExcluded;
        double max = 0.0; if (DA.GetData(11, ref max)) model.Max = (float)max;
        bool maxExcluded = false; if (DA.GetData(12, ref maxExcluded)) model.MaxExcluded = maxExcluded;
        double defaultValue = 0.0; if (DA.GetData(13, ref defaultValue)) model.Default = (float)defaultValue;
        string formula = ""; if (DA.GetData(14, ref formula)) model.Formula = formula;
        var benchmarks = new List<BenchmarkGoo>(); if (DA.GetDataList(15, benchmarks)) model.Benchmarks = benchmarks.Select(x => x.Value).ToList();
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(16, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Quality model)
    {
        DA.SetData(0, new QualityGoo(model));
        DA.SetData(1, model.Key);
        DA.SetData(2, model.Name);
        DA.SetData(3, model.Description);
        DA.SetData(4, model.Uri);
        DA.SetData(5, model.Scalable);
        DA.SetData(6, (int)model.Kind);
        DA.SetData(7, model.SI);
        DA.SetData(8, model.Imperial);
        DA.SetData(9, model.Min);
        DA.SetData(10, model.MinExcluded);
        DA.SetData(11, model.Max);
        DA.SetData(12, model.MaxExcluded);
        DA.SetData(13, model.Default);
        DA.SetData(14, model.Formula);
        DA.SetDataList(15, model.Benchmarks.Select(x => new BenchmarkGoo(x)));
        DA.SetDataList(16, model.Attributes.Select(x => new AttributeGoo(x)));
    }
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public SerializeQualityComponent() : base("Serialize Quality", "SQ", "Serialize Quality to JSON") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "Quality";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() : base("Deserialize Quality", "DQ", "Deserialize JSON to Quality") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "Quality";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Quality

#region Benchmark

public class BenchmarkGoo : ModelGoo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Benchmark { Name = str };
            return true;
        }
        return false;
    }

    
    
    
    protected override ModelGoo<Benchmark> CreateDuplicate() => new BenchmarkGoo(Value);
}

public class BenchmarkParam : ModelParam<BenchmarkGoo, Benchmark>
{
    public BenchmarkParam() : base("Benchmark", "B", "Benchmark parameter") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class BenchmarkComponent : ModelComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public BenchmarkComponent() : base("Benchmark", "Bmk", "Benchmark component") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "B";
    protected override string GetModelName() => "Benchmark";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new BenchmarkParam(), "Benchmark", "B", "Benchmark", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new BenchmarkParam(), "Benchmark", "B", "Benchmark", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Icon", "Icon", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Benchmark model)
    {
        string name = ""; if (DA.GetData(1, ref name)) model.Name = name;
        string icon = ""; if (DA.GetData(2, ref icon)) model.Icon = icon;
        double min = 0.0; if (DA.GetData(3, ref min)) model.Min = (float)min;
        bool minExcluded = false; if (DA.GetData(4, ref minExcluded)) model.MinExcluded = minExcluded;
        double max = 0.0; if (DA.GetData(5, ref max)) model.Max = (float)max;
        bool maxExcluded = false; if (DA.GetData(6, ref maxExcluded)) model.MaxExcluded = maxExcluded;
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(7, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Benchmark model)
    {
        DA.SetData(0, new BenchmarkGoo(model));
        DA.SetData(1, model.Name);
        DA.SetData(2, model.Icon);
        DA.SetData(3, model.Min);
        DA.SetData(4, model.MinExcluded);
        DA.SetData(5, model.Max);
        DA.SetData(6, model.MaxExcluded);
        DA.SetDataList(7, model.Attributes.Select(x => new AttributeGoo(x)));
    }
}

public class SerializeBenchmarkComponent : SerializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public SerializeBenchmarkComponent() : base("Serialize Benchmark", "SB", "Serialize Benchmark to JSON") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Benchmark";
    protected override string GetModelNickname() => "B";
    protected override string GetModelDescription() => "Benchmark";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeBenchmarkComponent : DeserializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public DeserializeBenchmarkComponent() : base("Deserialize Benchmark", "DB", "Deserialize JSON to Benchmark") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Benchmark";
    protected override string GetModelNickname() => "B";
    protected override string GetModelDescription() => "Benchmark";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Benchmark

#region Prop

public class PropGoo : ModelGoo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Prop>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    
    
    
    protected override ModelGoo<Prop> CreateDuplicate() => new PropGoo(Value);
}

public class PropParam : ModelParam<PropGoo, Prop>
{
    public PropParam() : base("Prop", "P", "Prop parameter") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PropComponent : ModelComponent<PropParam, PropGoo, Prop>
{
    public PropComponent() : base("Prop", "Prp", "Prop component") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "P";
    protected override string GetModelName() => "Prop";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PropParam(), "Prop", "P", "Property", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Value", "Value", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PropParam(), "Prop", "P", "Property", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Value", "Value", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Prop model)
    {
        string key = ""; if (DA.GetData(1, ref key)) model.Key = key;
        string value = ""; if (DA.GetData(2, ref value)) model.Value = value;
        string unit = ""; if (DA.GetData(3, ref unit)) model.Unit = unit;
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(4, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Prop model)
    {
        DA.SetData(0, new PropGoo(model));
        DA.SetData(1, model.Key);
        DA.SetData(2, model.Value);
        DA.SetData(3, model.Unit);
        DA.SetDataList(4, model.Attributes.Select(x => new AttributeGoo(x)));
    }
}

public class SerializePropComponent : SerializeComponent<PropParam, PropGoo, Prop>
{
    public SerializePropComponent() : base("Serialize Prop", "SP", "Serialize Prop to JSON") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Property";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePropComponent : DeserializeComponent<PropParam, PropGoo, Prop>
{
    public DeserializePropComponent() : base("Deserialize Prop", "DP", "Deserialize JSON to Prop") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Property";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Prop

#region Stat

public class StatGoo : ModelGoo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Stat>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override ModelGoo<Stat> CreateDuplicate() => new StatGoo(Value);
}

public class StatParam : ModelParam<StatGoo, Stat>
{
    public StatParam() : base("Stat", "S", "Stat parameter") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class StatComponent : ModelComponent<StatParam, StatGoo, Stat>
{
    public StatComponent() : base("Stat", "Stt", "Stat component") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "S";
    protected override string GetModelName() => "Stat";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new StatParam(), "Stat", "S", "Statistic", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new StatParam(), "Stat", "S", "Statistic", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Key", "Key", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Unit", "Unit", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Min", "Min", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MinExcluded", "MinExcluded", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Max", "Max", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MaxExcluded", "MaxExcluded", GH_ParamAccess.item);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Stat model)
    {
        string key = ""; if (DA.GetData(1, ref key)) model.Key = key;
        string unit = ""; if (DA.GetData(2, ref unit)) model.Unit = unit;
        double min = 0; if (DA.GetData(3, ref min)) model.Min = (float)min;
        bool minExcluded = false; if (DA.GetData(4, ref minExcluded)) model.MinExcluded = minExcluded;
        double max = 0; if (DA.GetData(5, ref max)) model.Max = (float)max;
        bool maxExcluded = false; if (DA.GetData(6, ref maxExcluded)) model.MaxExcluded = maxExcluded;
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Stat model)
    {
        DA.SetData(0, new StatGoo(model));
        DA.SetData(1, model.Key);
        DA.SetData(2, model.Unit);
        DA.SetData(3, model.Min);
        DA.SetData(4, model.MinExcluded);
        DA.SetData(5, model.Max);
        DA.SetData(6, model.MaxExcluded);
    }
}

public class SerializeStatComponent : SerializeComponent<StatParam, StatGoo, Stat>
{
    public SerializeStatComponent() : base("Serialize Stat", "SS", "Serialize Stat to JSON") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Statistic";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeStatComponent : DeserializeComponent<StatParam, StatGoo, Stat>
{
    public DeserializeStatComponent() : base("Deserialize Stat", "DS", "Deserialize JSON to Stat") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Statistic";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Stat

#region Layer

public class LayerGoo : ModelGoo<Layer>
{
    public LayerGoo() { }
    public LayerGoo(Layer value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Layer>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override ModelGoo<Layer> CreateDuplicate() => new LayerGoo(Value);
}

public class LayerParam : ModelParam<LayerGoo, Layer>
{
    public LayerParam() : base("Layer", "L", "Layer parameter") { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class LayerComponent : ModelComponent<LayerParam, LayerGoo, Layer>
{
    public LayerComponent() : base("Layer", "Lyr", "Layer component") { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "L";
    protected override string GetModelName() => "Layer";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new LayerParam(), "Layer", "L", "Layer", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Color", "Color", GH_ParamAccess.item);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new LayerParam(), "Layer", "L", "Layer", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Color", "Color", GH_ParamAccess.item);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Layer model)
    {
        string name = ""; if (DA.GetData(1, ref name)) model.Name = name;
        string description = ""; if (DA.GetData(2, ref description)) model.Description = description;
        string color = ""; if (DA.GetData(3, ref color)) model.Color = color;
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Layer model)
    {
        DA.SetData(0, new LayerGoo(model));
        DA.SetData(1, model.Name);
        DA.SetData(2, model.Description);
        DA.SetData(3, model.Color);
    }
}

public class SerializeLayerComponent : SerializeComponent<LayerParam, LayerGoo, Layer>
{
    public SerializeLayerComponent() : base("Serialize Layer", "SL", "Serialize Layer to JSON") { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Layer";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeLayerComponent : DeserializeComponent<LayerParam, LayerGoo, Layer>
{
    public DeserializeLayerComponent() : base("Deserialize Layer", "DL", "Deserialize JSON to Layer") { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Layer";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Layer

#region Group

public class GroupGoo : ModelGoo<Group>
{
    public GroupGoo() { }
    public GroupGoo(Group value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Group>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override ModelGoo<Group> CreateDuplicate() => new GroupGoo(Value);
}

public class GroupParam : ModelParam<GroupGoo, Group>
{
    public GroupParam() : base("Group", "G", "Group parameter") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class GroupComponent : ModelComponent<GroupParam, GroupGoo, Group>
{
    public GroupComponent() : base("Group", "Grp", "Group component") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override string GetModelCode() => "G";
    protected override string GetModelName() => "Group";
    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new GroupParam(), "Group", "G", "Group", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddTextParameter("Color", "Color", "Color", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new GroupParam(), "Group", "G", "Group", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Name", "Name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Description", "Description", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pieces", "Pieces", GH_ParamAccess.list);
        pManager.AddTextParameter("Color", "Color", "Color", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "Attributes", "Attributes", GH_ParamAccess.list);
    }
    protected override void ProcessModelInputs(IGH_DataAccess DA, Group model)
    {
        string name = ""; if (DA.GetData(1, ref name)) model.Name = name;
        string description = ""; if (DA.GetData(2, ref description)) model.Description = description;
        var pieces = new List<PieceIdGoo>(); if (DA.GetDataList(3, pieces)) model.Pieces = pieces.Select(x => x.Value).ToList();
        string color = ""; if (DA.GetData(4, ref color)) model.Color = color;
        var attributes = new List<AttributeGoo>(); if (DA.GetDataList(5, attributes)) model.Attributes = attributes.Select(x => x.Value).ToList();
    }
    protected override void ProcessModelOutputs(IGH_DataAccess DA, Group model)
    {
        DA.SetData(0, new GroupGoo(model));
        DA.SetData(1, model.Name);
        DA.SetData(2, model.Description);
        DA.SetDataList(3, model.Pieces.Select(x => new PieceIdGoo(x)));
        DA.SetData(4, model.Color);
        DA.SetDataList(5, model.Attributes.Select(x => new AttributeGoo(x)));
    }
}

public class SerializeGroupComponent : SerializeComponent<GroupParam, GroupGoo, Group>
{
    public SerializeGroupComponent() : base("Serialize Group", ">>G", "Serialize a group") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "G";
    protected override string GetModelDescription() => "Group";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeGroupComponent : DeserializeComponent<GroupParam, GroupGoo, Group>
{
    public DeserializeGroupComponent() : base("Deserialize Group", "<<G", "Deserialize a group") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "G";
    protected override string GetModelDescription() => "Group";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Group

#region Scripting

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
        pManager[1].Optional = true;
        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that will be replaced after encoding.", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Replace", "Re", "Placeholder text that replaces the forbidden text after encoding.", GH_ParamAccess.list);
        pManager[3].Optional = true;
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
        pManager[1].Optional = true;
        pManager.AddTextParameter("Replace", "Re", "Placeholder text that was used to encode forbidden text after encoding and is restored before decoding. It will be applied sequentially. Make sure to invert the order of your original list.", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that gets restored before decoding.", GH_ParamAccess.list);
        pManager[3].Optional = true;
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
        pManager[2].Optional = true;
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

#endregion Scripting

#region Engine

public abstract class EngineComponent : Component
{
    protected EngineComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual string RunDescription => "True to start the operation.";
    protected virtual string SuccessDescription => "True if the operation was successful.";
    protected virtual void RegisterEngineInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterEngineInputParams(pManager);
        pManager.AddBooleanParameter("Run", "R", RunDescription, GH_ParamAccess.item, false);
    }
    protected virtual void RegisterEngineOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", SuccessDescription, GH_ParamAccess.item);
        RegisterEngineOutputParams(pManager);
    }
    protected virtual dynamic? GetInput(IGH_DataAccess DA) => null;
    protected abstract dynamic? Run(dynamic? input = null);
    protected virtual void SetOutput(IGH_DataAccess DA, dynamic response) { }
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var run = false;
        DA.GetData(Params.Input.Count - 1, ref run);
        if (!run) return;
        var input = GetInput(DA);
        try
        {
            var response = Run(input);
            SetOutput(DA, response);
            DA.SetData(0, true);
        }
        catch (ClientException e)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, e.Message);
            DA.SetData(0, false);
        }
        catch (ServerException e)
        {
            string serializedInput = input != null ? Semio.Utility.Serialize(input) : "";
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error,
                "The engine didn't like it ¯\\_(ツ)_/¯\n" +
                "If you want, you can report this under: https://github.com/usalu/semio/issues\n" +
                $"Or write me an email: {Semio.Constants.Email}\n\n" +
                "ServerError: " + e.Message + "\n" +
                "Semio.Release: " + Semio.Constants.Release + "\n" +
                "Semio.Grasshopper: " + Constants.Version + "\n" +
                (serializedInput != ""
                    ? "Input: " + (serializedInput.Length < 1000
                        ? serializedInput
                        : serializedInput.Substring(0, 1000) + "\n...\n")
                    : ""));
            DA.SetData(0, false);
        }
    }
    protected override void BeforeSolveInstance()
    {
        base.BeforeSolveInstance();
        var processes = Process.GetProcessesByName("semio-engine");
        if (processes.Length == 0)
        {
            var path = Path.Combine(Utility.GetGrasshopperLibraryDirectory() ?? string.Empty,
                "semio-engine.exe");
            var engine = Process.Start(path);
            // lightweight way to kill child process when parent process is killed
            // https://stackoverflow.com/questions/3342941/kill-child-process-when-parent-process-is-killed#4657392
            AppDomain.CurrentDomain.DomainUnload += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            AppDomain.CurrentDomain.ProcessExit += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            AppDomain.CurrentDomain.UnhandledException += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            // TODO: Implement a status check and wait for the engine to be ready
            Thread.Sleep(1000);
        }
    }
}

#region Persistence

public abstract class PersistenceComponent : EngineComponent
{
    protected PersistenceComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual void RegisterPersitenceInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterEngineInputParams(GH_InputParamManager pManager)
    {
        RegisterPersitenceInputParams(pManager);
        var amountCustomParams = pManager.ParamCount;
        pManager.AddTextParameter("Uri", "Ur?",
            "Optional Unique Resource Identifier (URI) of the kit. This can be an absolute path to a local kit or a url to a remote kit.\n" +
            "If none is provided, it will try to see if the Grasshopper script is executed inside a local kit.",
            GH_ParamAccess.item);
        pManager[amountCustomParams].Optional = true;
    }
    protected virtual void RegisterPersitenceOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterEngineOutputParams(GH_OutputParamManager pManager)
    {
        RegisterPersitenceOutputParams(pManager);
    }
    protected virtual dynamic? GetPersistentInput(IGH_DataAccess DA) => null;
    protected override dynamic? GetInput(IGH_DataAccess DA)
    {
        var uri = "";
        if (!DA.GetData(Params.Input.Count - 2, ref uri))
            uri = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        return new { Uri = uri, Input = GetPersistentInput(DA) };
    }
    protected abstract dynamic? RunOnKit(string url, dynamic? input);
    protected override dynamic? Run(dynamic? input = null) => input != null ? RunOnKit(input.Uri, input.Input) : null;
}

public class LoadKitComponent : PersistenceComponent
{
    public LoadKitComponent() : base("Load Kit", "/Kit", "Load a kit.") { }
    protected override string RunDescription => "True to load the kit.";
    protected override string SuccessDescription => "True if the kit was successfully loaded. False otherwise.";
    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");
    protected override Bitmap Icon => Resources.kit_load_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterPersitenceOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Local Directory", "Di",
            "The optional local directory of the kit. This applies to local kits and cached remote kits.",
            GH_ParamAccess.item);
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input) => Api.GetKit(uri);

    protected override void SetOutput(IGH_DataAccess DA, dynamic response)
    {
        DA.SetData(1, new KitGoo(response));
        var uri = "";
        DA.GetData(0, ref uri);
        string directory;
        if (uri == "")
        {
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        }
        else if (uri.StartsWith("http") && uri.EndsWith(".zip"))
        {
            var encodedUri = Semio.Utility.Encode(uri);
            var userPath = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
            directory = Path.Combine(userPath, ".semio", "cache", encodedUri);
        }
        else directory = uri;
        DA.SetData(2, directory);
    }
}

#endregion

#endregion Engine

#region Meta

#endregion Meta


